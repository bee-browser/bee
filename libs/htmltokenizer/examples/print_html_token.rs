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
            Ok(Token::Doctype { name, public_id, system_id, force_quirks }) => {
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
            }
            Ok(Token::StartTag { name, attrs, self_closing }) => {
                let mut attrs_str = String::new();
                for (name, value) in attrs {
                    write!(&mut attrs_str, r#" "{}"="{}""#,
                           name.escape_debug(), value.escape_debug())?;
                }
                if self_closing {
                    println!(r#"<{}{}/>"#, name, attrs_str);
                } else {
                    println!(r#"<{}{}>"#, name, attrs_str);
                }
            }
            Ok(Token::EndTag { name }) => {
                println!(r#"</{}>"#, name.escape_debug())
            },
            Ok(Token::Text { text }) => {
                println!(r#"#text:"{}""#, text.escape_debug())
            }
            Ok(Token::Comment { comment }) => {
                println!(r#"#comment:"{}""#, comment.escape_debug())
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
