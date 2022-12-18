use anyhow::Result;
use bee_htmltokenizer::*;
use std::fmt::Write;
use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let mut html = String::new();
    io::stdin().read_to_string(&mut html)?;
    let mut tokenizer = Tokenizer::new(Printer);
    tokenizer.feed_data(html.encode_utf16().collect());
    tokenizer.feed_end();
    loop {
        match tokenizer.next_token() {
            Ok(()) => {
                return Ok(());
            }
            _ => (),
        }
    }
}

struct Printer;

impl TokenHandler for Printer {
    fn handle_doctype(&mut self, name: Option<&str>, public_id: Option<&str>, system_id: Option<&str>, force_quirks: bool) -> bool {
        print!(r#"#DOCTYPE:"#);
        if let Some(name) = name {
            print!(r#" "{}""#, name.escape_debug());
        }
        if let Some(public_id) = public_id {
            print!(r#" public-id="{}""#, public_id.escape_debug());
        }
        if let Some(system_id) = system_id {
            print!(r#" system-id="{}""#, system_id.escape_debug());
        }
        println!(r#" force-quirks={}"#, force_quirks);
        true
    }

    fn handle_start_tag(&mut self, name: TagKind, attrs: Attrs<'_>, self_closing: bool) -> bool {
        let mut attrs_str = String::new();
        for (name, value) in attrs {
            write!(&mut attrs_str, r#" "{}"="{}""#,
                   name.escape_debug(), value.escape_debug()).unwrap();
        }
        let name = match name {
            TagKind::Html(htmltag) => htmltag.name(),
            TagKind::Other(name) => name,
        };
        if self_closing {
            println!(r#"<{}{}/>"#, name.escape_debug(), attrs_str);
        } else {
            println!(r#"<{}{}>"#, name.escape_debug(), attrs_str);
        }
        true
    }

    fn handle_end_tag(&mut self, name: TagKind) -> bool {
        let name = match name {
            TagKind::Html(htmltag) => htmltag.name(),
            TagKind::Other(name) => name,
        };
        println!(r#"</{}>"#, name.escape_debug());
        true
    }

    fn handle_text(&mut self, text: &str) -> bool {
        println!(r#"#text:"{}""#, text.escape_debug());
        true
    }

    fn handle_comment(&mut self, comment: &str) -> bool {
        println!(r#"#comment:"{}""#, comment.escape_debug());
        true
    }

    fn handle_error(&mut self, err: Error) -> bool {
        eprintln!("ERROR: {}", err);
        true
    }
}
