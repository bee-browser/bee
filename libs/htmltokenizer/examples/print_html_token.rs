use std::fmt::Write;
use std::io;
use std::io::Read;

use anyhow::Result;

use htmltokenizer::Tokenizer;
use htmltokenizer::token::*;

fn main() -> Result<()> {
    let mut html = String::new();
    io::stdin().read_to_string(&mut html)?;
    let mut tokenizer = Tokenizer::new();
    let data: Vec<u16> = html.encode_utf16().collect();
    tokenizer.feed_data(&data);
    tokenizer.feed_end();
    loop {
        match tokenizer.next_token() {
            Token::Doctype(doctype) => {
                print!(r#"#DOCTYPE:"#);
                if let Some(name) = doctype.name {
                    print!(r#" "{}""#, name.escape_debug());
                }
                if let Some(public_id) = doctype.public_id {
                    print!(r#" public-id="{}""#, public_id.escape_debug());
                }
                if let Some(system_id) = doctype.system_id {
                    print!(r#" system-id="{}""#, system_id.escape_debug());
                }
                println!(r#" force-quirks={}"#, doctype.force_quirks);
            }
            Token::StartTag(tag) => {
                let mut attrs_str = String::new();
                for (name, value) in tag.attrs() {
                    write!(
                        &mut attrs_str,
                        r#" "{}"="{}""#,
                        name.escape_debug(),
                        value.escape_debug()
                    )
                    .unwrap();
                }
                if tag.self_closing {
                    println!(r#"<{}{}/>"#, tag.name.escape_debug(), attrs_str);
                } else {
                    println!(r#"<{}{}>"#, tag.name.escape_debug(), attrs_str);
                }
            }
            Token::EndTag(tag) => {
                println!(r#"</{}>"#, tag.name.escape_debug());
            }
            Token::Comment(comment) => {
                println!(r#"#comment:"{}""#, comment.data.escape_debug());
            }
            Token::Null(text) | Token::Whitespace(text) | Token::Text(text) => {
                println!(r#"#text:"{}""#, text.data.escape_debug());
            }
            Token::Error(err) => {
                eprintln!("ERROR: {err}");
            }
            Token::End => break,
        }
    }

    Ok(())
}
