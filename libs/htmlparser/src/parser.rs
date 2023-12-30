use htmltokenizer::InitialState;
use htmltokenizer::Tokenizer;

use crate::localnames::LocalName;
use crate::treebuilder::Control;
use crate::treebuilder::DomTreeBuilder;
use crate::treebuilder::TreeBuilder;
use crate::Namespace;
use crate::QuirksMode;

pub struct Parser<T>
where
    T: DomTreeBuilder,
{
    tokenizer: Tokenizer<'static>,
    tree_builder: TreeBuilder<T>,
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

    #[tracing::instrument(level = "debug", skip_all)]
    pub fn feed_data(&mut self, data: &[u16]) {
        tracing::debug!(data = String::from_utf16_lossy(data).escape_debug().to_string());
        self.tokenizer.feed_data(data);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub fn feed_end(&mut self) {
        self.tokenizer.feed_end();
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub fn set_quirks_mode(&mut self, quirks_mode: QuirksMode) {
        self.tree_builder.set_quirks_mode(quirks_mode);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub fn set_scripting(&mut self, scripting: bool) {
        self.tree_builder.set_scripting(scripting);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub fn set_context_element(&mut self, tag_name: &str, namespace: Namespace, node: T::NodeId) {
        tracing::debug!(tag_name, ?namespace, ?node);
        // TODO: The case-sensitivity of `tag_name` depends on the `namespace`.
        // TODO: `LocalName::lookup()` expects a lowercase tag name because the
        // HTML tokenizer converts all tag names to lowercase in the
        // tokenization process.
        let mut local_name = LocalName::lookup(tag_name.to_ascii_lowercase().as_str());
        if tag_name != local_name.name() {
            local_name = LocalName::Unknown;
        }
        if namespace == Namespace::Html {
            self.tokenizer.set_initial_state(match local_name {
                tag!(Title, Textarea) => InitialState::Rcdata,
                tag!(Style, Xmp, Iframe, Noembed, Noframes) => InitialState::Rawtext,
                tag!(Script) => InitialState::ScriptData,
                tag!(Noscript) => InitialState::Rawtext, // TODO: Check the scripting flag
                tag!(Plaintext) => InitialState::Plaintext,
                _ => InitialState::Data,
            });
        }
        self.tree_builder
            .set_context_element(local_name, namespace, node, tag_name);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub fn parse(&mut self) {
        loop {
            self.tokenizer
                .set_in_html_namespace(self.tree_builder.in_html_namespace());
            let token = self.tokenizer.next_token();
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
