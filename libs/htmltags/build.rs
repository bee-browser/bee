use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use indexmap::IndexMap;
use handlebars::Handlebars;
use serde::Deserialize;
use serde::Serialize;

const TAGS_YAML: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/tags.yaml");
const TEMPLATE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/codegen.rs.hbs");

fn main() {
    let tags: IndexMap<String, HtmlTagInfo> = serde_yaml::from_reader(File::open(TAGS_YAML).unwrap()).unwrap();

    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("codegen.rs", TEMPLATE).unwrap();

    let mut phf = phf_codegen::Map::new();
    for tag in tags.keys() {
        let symbol = format!("HtmlTag::{}", tag.to_ascii_uppercase());
        phf.entry(tag.as_str(), symbol.as_str());
    }

    let data = Data {
        num_tags: tags.len(),
        tags: tags.keys().map(|tag| tag.to_ascii_uppercase()).collect(),
        data: tags.iter().map(|(tag, data)| HtmlTagData {
            name: tag.clone(),
            deprecated: data.deprecated,
        }).collect(),
        phf: format!("{}", phf.build()),
    };

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let file = BufWriter::new(File::create(&path).unwrap());
    handlebars.render_to_write("codegen.rs", &data, file).unwrap();
}

#[derive(Deserialize)]
struct HtmlTagInfo {
    #[serde(default)]
    deprecated: bool,
}

#[derive(Serialize)]
struct Data {
    // handlebars-rust doesn't support `Array.length`.
    num_tags: usize,
    tags: Vec<String>,
    data: Vec<HtmlTagData>,
    phf: String,
}

#[derive(Serialize)]
struct HtmlTagData {
    name: String,
    deprecated: bool,
}
