use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use axum::extract::State;
use axum::http::header::HeaderValue;
use axum::http::header::CACHE_CONTROL;
use axum::response::sse::Event;
use axum::response::sse::Sse;
use axum::response::IntoResponse;
use clap::Parser;
use serde::Deserialize;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::filter::EnvFilter;

#[derive(Parser)]
#[command(author, version, about)]
struct CommandLine {
    /// A path to a configuration YAML file.
    #[arg(short, long)]
    config: PathBuf,

    /// Parameters used when rendering `config.event-source`.
    #[arg(short, long, value_parser = parse_kv::<String, String>)]
    data: Vec<(String, String)>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cl = CommandLine::parse();

    let config: Config = serde_yaml::from_reader(std::fs::File::open(&cl.config)?)?;
    let listen = config.listen.clone();

    // Start a web server to serve static resources and events to a web browser
    // that will be opened by `open::that()`.
    let workdir = std::fs::canonicalize(&cl.config)?.parent().unwrap().to_owned();
    let handle = tokio::spawn(async move {
        serve(workdir, config, cl.data).await
    });

    // Load the web UI.
    open::that(format!("http://{}/logview/index.html", listen))?;

    let _ = handle.await;

    Ok(())
}

// command-line options

// Took from the cookbook in the clap project.
fn parse_kv<K, V>(s: &str) -> Result<(K, V), Box<dyn Error + Send + Sync + 'static>>
where
    K: std::str::FromStr,
    K::Err: Error + Send + Sync + 'static,
    V: std::str::FromStr,
    V::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

// server

macro_rules! into_router {
    ($service:expr) => {
        axum::Router::new().fallback_service(axum::routing::get_service($service))
    };
}

async fn serve(workdir: PathBuf, config: Config, data: Vec<(String, String)>) {
    let addr = config.listen.clone();

    let mut router = axum::Router::new()
        .route("/logs", axum::routing::get(logs))
        .with_state(Arc::new(AppState {
            workdir: workdir.clone(),
            event_source: config.event_source.clone(),
            data,
        }));

    for mount in config.mounts.iter() {
        match mount.source {
            MountSource::Fs(ref path) => {
                let path = workdir.join(path);
                if path.is_dir() {
                    let service = ServeDir::new(path);
                    router = router.nest(&mount.target, into_router!(service));
                } else if path.is_file() {
                    let service = ServeFile::new_with_mime(path, &mime::TEXT_HTML_UTF_8);
                    router = router.nest(&mount.target, into_router!(service));
                }
            }
        }
    }

    router = router
        .layer(SetResponseHeaderLayer::overriding(
            CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        ))
        .layer(TraceLayer::new_for_http());

    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn logs(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut child = match state.event_source {
        EventSource::Command(ref command) => {
            // Render the command.
            let template = mustache::compile_str(command).unwrap();
            let mut builder = mustache::MapBuilder::new();
            for (key, value) in state.data.iter() {
                builder = builder.insert_str(key, value);
            }
            let data = builder.build();
            let command = template.render_data_to_string(&data).unwrap();

            // Perform the command.
            let words = shell_words::split(&command).unwrap();
            let (prog, args) = words.split_first().unwrap();
            tokio::process::Command::new(prog)
                .args(args)
                .stderr(std::process::Stdio::piped())
                .current_dir(&state.workdir)
                .spawn()
                .unwrap()
        }
    };

    // Feed logs coming from STDERR to the web server.
    let pid = child.id().unwrap();
    let stderr = child.stderr.take().unwrap();
    let stream = async_stream::stream! {
        tracing::info!(pid, "spawned");
        yield Result::<_, Infallible>::Ok(Event::default().event("spawned").json_data(&pid).unwrap());
        let mut lines = BufReader::new(stderr).lines();
        while let Some(log) = lines.next_line().await.unwrap() {
            tracing::debug!(pid, log);
            yield Result::<_, Infallible>::Ok(Event::default().event("log").data(log));
        }
        yield Result::<_, Infallible>::Ok(Event::default().event("terminated").json_data(&pid).unwrap());
        tracing::info!(pid, "terminated");
    };
    Sse::new(stream).keep_alive(Default::default())
}

struct AppState {
    workdir: PathBuf,
    event_source: EventSource,
    data: Vec<(String, String)>,
}

// config

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
struct Config {
    #[serde(default = "Config::default_listen")]
    listen: SocketAddr,
    #[serde(default = "Config::default_mounts")]
    mounts: Vec<MountConfig>,
    #[serde(with = "serde_yaml::with::singleton_map")]
    event_source: EventSource,
}

impl Config {
    fn default_listen() -> SocketAddr {
        "127.0.0.1:3333".parse().unwrap()
    }

    fn default_mounts() -> Vec<MountConfig> {
        vec![MountConfig {
            target: "/logview".to_string(),
            source: MountSource::Fs(PathBuf::from(".")),
        }]
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
struct MountConfig {
    target: String,
    #[serde(with = "serde_yaml::with::singleton_map")]
    source: MountSource,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
enum MountSource {
    Fs(PathBuf),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
enum EventSource {
    Command(String),
}
