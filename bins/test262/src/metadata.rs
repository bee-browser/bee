use std::path::Path;
use std::sync::Arc;

use rustc_hash::FxHashSet;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    pub description: String,
    pub info: Option<String>,
    pub author: Option<String>,
    pub esid: Option<String>,
    pub es5id: Option<String>,
    pub es6id: Option<String>,
    pub negative: Option<Negative>,
    #[serde(default)]
    pub includes: Vec<String>,
    #[serde(default)]
    pub flags: FxHashSet<Flag>,
    #[serde(default)]
    pub locale: Vec<String>,
    #[serde(default)]
    pub features: FxHashSet<String>,
}

impl Metadata {
    pub fn extract(path: &Path) -> Option<Arc<Self>> {
        let content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(err) => panic!("{path:?}: {err:?}"),
        };
        let metadata = match content.split_once("/*---") {
            Some((_, metadata)) => metadata,
            None => panic!("{path:?}: No metadata contained"),
        };
        let metadata = match metadata.split_once("---*/") {
            Some((metadata, _)) => metadata,
            None => panic!("{path:?}: Metadata section must ends with ---*/"),
        };
        match serde_norway::from_str::<Metadata>(metadata) {
            Ok(metadata) => Some(Arc::new(metadata)),
            Err(err) => panic!("{path:?}: {err:?}"),
        }
    }

    pub fn is_module(&self) -> bool {
        self.flags.contains(&Flag::Module)
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Flag {
    #[serde(rename = "onlyStrict")]
    OnlyStrict,
    #[serde(rename = "noStrict")]
    NoStrict,
    #[serde(rename = "module")]
    Module,
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "async")]
    Async,
    #[serde(rename = "generated")]
    Generated,
    #[serde(rename = "CanBlockIsFalse")]
    CanBlockIsFalse,
    #[serde(rename = "CanBlockIsTrue")]
    CanBlockIsTrue,
    #[serde(rename = "non-deterministic")]
    NonDeterministic,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Negative {
    pub phase: Phase,
    #[serde(rename = "type")]
    pub error_type: ErrorType,
}

impl Negative {
    pub fn is_syntax_error_in_parse(&self) -> bool {
        matches!(self.phase, Phase::Parse) && matches!(self.error_type, ErrorType::SyntaxError)
    }

    pub fn is_runtime_error(&self) -> bool {
        matches!(self.phase, Phase::Runtime)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Phase {
    #[serde(rename = "parse")]
    Parse,
    #[serde(rename = "resolution")]
    Resolution,
    #[serde(rename = "runtime")]
    Runtime,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::enum_variant_names)]
pub enum ErrorType {
    SyntaxError,
    EvalError,
    RangeError,
    ReferenceError,
    TypeError,
    Test262Error,
}
