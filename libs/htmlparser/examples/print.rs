use anyhow::Result;
use bee_htmlparser::*;
use std::fmt::Write;
use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let mut html = String::new();
    io::stdin().read_to_string(&mut html)?;
    let mut parser = Parser::new();
    parser.feed_data(html.encode_utf16().collect());
    parser.feed_end();
    loop {
        match parser.next_event() {
            Ok(Event::Doctype) => {
                let depth = parser.depth();
                let name = parser.doctype_name().map(|s| s.escape_debug());
                let public_id = parser.doctype_public_id().map(|s| s.escape_debug());
                let system_id = parser.doctype_system_id().map(|s| s.escape_debug());
                let force_quirks = parser.force_quirks();
                print!(r#"{:depth$}#DOCTYPE:"#, depth=depth);
                if let Some(name) = name {
                    print!(r#" "{}""#, name);
                }
                if let Some(public_id) = public_id {
                    print!(r#" public-id="{}""#, public_id);
                }
                if let Some(system_id) = system_id {
                    print!(r#" system-id="{}""#, system_id);
                }
                println!(r#" force-quirks={}"#, force_quirks);
            }
            Ok(Event::StartTag) => {
                let depth = parser.depth();
                let tag_name = parser.tag_name().escape_debug();
                let mut attrs = String::new();
                for (name, value) in parser.attrs() {
                    write!(&mut attrs, r#" "{}"="{}""#,
                           name.escape_debug(), value.escape_debug())?;
                }
                if parser.is_empty_tag() {
                    println!(r#"{:depth$}<{}{}/>"#, "",
                             tag_name, attrs, depth=depth)
                } else {
                    println!(r#"{:depth$}<{}{}>"#, "",
                             tag_name, attrs, depth=depth)
                }
            }
            Ok(Event::EndTag) => {
                let depth = parser.depth();
                let tag_name = parser.tag_name().escape_debug();
                println!(r#"{:depth$}</{}>"#, "",
                         tag_name, depth=depth)
            },
            Ok(Event::Text) => {
                let depth = parser.depth();
                let text = parser.text().escape_default();
                println!(r#"{:depth$}#text:"{}""#, "",
                         text, depth=depth)
            }
            Ok(Event::Comment) => {
                let depth = parser.depth();
                let comment = parser.comment().escape_default();
                println!(r#"{:depth$}#comment:"{}""#, "",
                         comment, depth=depth)
            }
            Ok(Event::End) => {
                return Ok(());
            }
            Err(err) => {
                eprintln!("ERROR: {}", err);
            }
        }
    }
}
