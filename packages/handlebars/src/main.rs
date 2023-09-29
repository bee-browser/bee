use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use handlebars::Handlebars;
use tracing_subscriber::filter::EnvFilter;

/// Render a Handlebars template file with a data.
///
/// The following data will be passed to the template function compiled from the template file:
///
/// ```
/// {
///   "input": <INPUT>,
/// }
/// ```
#[derive(Parser)]
#[command(author, version, about)]
pub struct CommandLine {
    /// Enable HTML-escaping.
    #[arg(long)]
    escape: bool,

    /// A path to a folder containing partial template files to be loaded recursively.
    ///
    /// The name of every partial template file must end with the extension `.hbs`.  A template
    /// file can be specified in other template files by its relative path from <PARTIALS_DIR>
    /// without the extension.  For example, `html/partial.html.hbs` template file in
    /// <PARTIALS_DIR> can be rendered with `{{< html/partial.html }}`.
    #[arg(short, long)]
    partials_dir: Option<PathBuf>,

    /// A path to the template file to use.
    #[arg()]
    template: PathBuf,

    /// A JSON string used as the input to the template.
    ///
    /// Read it from STDIN when unspecified.
    #[arg()]
    input: Option<String>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cl = CommandLine::parse();

    let template = std::fs::read_to_string(&cl.template)?;

    let input: serde_json::Value = match cl.input {
        Some(ref json) => serde_json::from_str(json)?,
        None => serde_json::from_reader(std::io::stdin())?,
    };

    let data = serde_json::json!({
        "input": input,
        "template": cl.template.to_string_lossy(),
    });

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    if cl.escape {
        handlebars.register_escape_fn(handlebars::html_escape);
    } else {
        handlebars.register_escape_fn(handlebars::no_escape);
    }
    if let Some(ref partials_dir) = cl.partials_dir {
        handlebars.register_templates_directory(".hbs", partials_dir)?;
    }

    handlebars.register_helper("length", Box::new(length));

    handlebars.render_template_to_write(&template, &data, std::io::stdout())?;

    Ok(())
}

// helpers

fn length(
    h: &handlebars::Helper<'_, '_>,
    _: &Handlebars<'_>,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext<'_, '_>,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let value = h
        .param(0)
        .map(|v| v.value())
        .ok_or(handlebars::RenderError::new("param not found"))?;
    let len = match value {
        serde_json::Value::String(s) => s.len(),
        serde_json::Value::Array(a) => a.len(),
        serde_json::Value::Object(o) => o.len(),
        _ => panic!(),
    };
    out.write_fmt(format_args!("{len}"))?;
    Ok(())
}
