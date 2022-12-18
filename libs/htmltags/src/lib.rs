impl HtmlTag {
    pub fn lookup(name: &str) -> Option<Self> {
        TAGS.get(name).copied()
    }
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

struct HtmlTagData {
    name: &'static str,
    deprecated: bool,
}
