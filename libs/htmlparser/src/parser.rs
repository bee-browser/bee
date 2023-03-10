use bee_htmltokenizer::Tokenizer;

use crate::treebuilder::Control;
use crate::treebuilder::DomTreeBuilder;
use crate::treebuilder::TreeBuilder;

pub struct Parser<W> {
    tokenizer: Tokenizer<'static>,
    tree_builder: TreeBuilder<W>,
}

impl<T> Parser<T>
where
    T: DomTreeBuilder,
{
    pub fn new(builder: T) -> Parser<T> {
        Parser {
            tokenizer: Tokenizer::new(),
            tree_builder: TreeBuilder::new(builder),
        }
    }

    pub fn feed_data(&mut self, data: Vec<u16>) {
        tracing::debug!(data = String::from_utf16_lossy(&data));
        self.tokenizer.feed_data(data);
    }

    pub fn feed_end(&mut self) {
        self.tokenizer.feed_end();
    }

    pub fn parse(&mut self) {
        loop {
            self.tokenizer
                .set_in_html_namespace(self.tree_builder.in_html_namespace());
            let token = self.tokenizer.next_token();
            tracing::debug!(?token);
            let ctrl = self.tree_builder.handle_token(token);
            match ctrl {
                Control::Reprocess => unreachable!(),
                Control::Continue => {
                    // TODO: Perform something if any.
                    continue;
                }
                Control::SwitchTo(state, tag_name) => {
                    self.tokenizer.set_initial_state(state);
                    self.tokenizer.set_last_start_tag(tag_name);
                }
                Control::ExecuteScript => {
                    // TODO: Execute the script.
                }
                Control::Done => {
                    // TODO: Perform something if any.
                    break;
                }
            }
        }
    }
}
