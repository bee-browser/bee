use anyhow::Result;
use bee_htmltokenizer::*;
use std::fmt::Write;
use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let mut html = String::new();
    io::stdin().read_to_string(&mut html)?;
    let mut tokenizer = Tokenizer::new();
    tokenizer.feed_data(html.encode_utf16().collect());
    tokenizer.feed_end();
    loop {
        match tokenizer.next_token() {
            Ok(Token::Doctype) => {
                let name = tokenizer.doctype_name().map(|s| s.escape_debug());
                let public_id = tokenizer.doctype_public_id()
                    .map(|s| s.escape_debug());
                let system_id = tokenizer.doctype_system_id()
                    .map(|s| s.escape_debug());
                let force_quirks = tokenizer.force_quirks();
                print!(r#"#DOCTYPE:"#);
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
            Ok(Token::StartTag) => {
                let tag_name = tokenizer.tag_name().escape_debug();
                let mut attrs = String::new();
                for (name, value) in tokenizer.attrs() {
                    write!(&mut attrs, r#" "{}"="{}""#,
                           name.escape_debug(), value.escape_debug())?;
                }
                if tokenizer.is_empty_tag() {
                    println!(r#"<{}{}/>"#, tag_name, attrs);
                } else {
                    println!(r#"<{}{}>"#, tag_name, attrs);
                }
            }
            Ok(Token::EndTag) => {
                let tag_name = tokenizer.tag_name().escape_debug();
                println!(r#"</{}>"#, tag_name)
            },
            Ok(Token::Text) => {
                let text = tokenizer.text().escape_default();
                println!(r#"#text:"{}""#, text)
            }
            Ok(Token::Comment) => {
                let comment = tokenizer.comment().escape_default();
                println!(r#"#comment:"{}""#, comment)
            }
            Ok(Token::End) => {
                return Ok(());
            }
            Err(err) => {
                eprintln!("ERROR: {}", err);
            }
        }
    }
}
