use std::collections::HashMap;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::*;

pub struct MessageInterpreter<T> {
    sink: T,
    node_map: HashMap<NodeId, LayoutNodeHandle>,
    snapshots: Vec<(LayoutNodeHandle, VisualRoot)>,
}

impl<T> MessageInterpreter<T>
where
    T: JsonSink,
{
    pub fn new(sink: T) -> Self {
        MessageInterpreter {
            sink,
            node_map: HashMap::new(),
            snapshots: Vec::new(),
        }
    }

    // TODO: use BufRead as argument
    pub fn interpret(&mut self, json: &str) -> Result<()> {
        let msg = serde_json::from_str(json)
            .with_context(|| format!("Failed to parse: {}", json))?;
        match msg {
            LayoutMessage::CreateElement { id, style, children, label } => {
                let children = children.iter()
                    .map(|id| self.node_map.get(id).unwrap())
                    .cloned()
                    .collect();
                let node = new_element(style, children, label);
                self.node_map.insert(id, node);
            }
            LayoutMessage::CreateText { id, text, label } => {
                let node = new_text(text, label);
                self.node_map.insert(id, node);
            }
            LayoutMessage::Visualize { width, height } => {
                let root = self.node_map.get(&NodeId(0)).unwrap();
                let visual_tree = build_visual_tree(root.clone(), width, height);
                let mut painter = JsonRenderer::new(&mut self.sink);
                visual_tree.render(&mut painter);
                self.snapshots.push((root.clone(), visual_tree));
            }
        };
        Ok(())
    }

    pub fn inspect<W>(&self, write: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        for (ref layout, ref visual) in self.snapshots.iter() {
            write!(write, "----- layout tree\n")?;
            layout.inspect(write)?;
            write!(write, "----- visual tree\n")?;
            visual.inspect(write)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Deserialize)]
struct NodeId(usize);

#[derive(Deserialize)]
#[serde(tag = "type", content = "data")]
enum LayoutMessage {
    #[serde(rename = "layout.create_element")]
    CreateElement {
        id: NodeId,
        style: Arc<Style>,
        children: Vec<NodeId>,
        label: String,
    },
    #[serde(rename = "layout.create_text")]
    CreateText {
        id: NodeId,
        text: String,
        label: String,
    },
    #[serde(rename = "layout.visualize")]
    Visualize {
        width: usize,
        height: usize,
    },
}

pub trait JsonSink {
    fn consume(&mut self, json: serde_json::Value);
}

struct JsonRenderer<'a, T> {
    sink: &'a mut T,
    origin: VisualVector2D,
}

impl<'a, T> JsonRenderer<'a, T>
where
    T: JsonSink
{
    fn new(sink: &'a mut T) -> Self {
        JsonRenderer {
            sink,
            origin: VisualVector2D::zero(),
        }
    }

    fn send(&mut self, msg: RenderMessage) {
        self.sink.consume(serde_json::to_value(&msg).unwrap());
    }
}

impl<'a, T> VisualRenderer for JsonRenderer<'a, T>
where
    T: JsonSink
{
    fn start(&mut self, size: VisualSize2D) {
        self.send(RenderMessage::Start { size });
    }

    fn end(&mut self) {
        self.send(RenderMessage::End);
    }

    fn render_box(&mut self, model: VisualBoxModel) {
        self.send(RenderMessage::RenderBox {
            rect: model.border_box.translate(self.origin).into(),
            background: model.background,
            border: model.border,
        });
    }

    fn translate_coord(&mut self, v: VisualVector2D) {
        self.origin += v;
    }
}

#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
enum RenderMessage {
    #[serde(rename = "render.start")]
    Start {
        size: VisualSize2D,
    },
    #[serde(rename = "render.end")]
    End,
    #[serde(rename = "render.render_box")]
    RenderBox {
        rect: VisualRect,
        #[serde(skip_serializing_if = "VisualBackground::is_transparent")]
        background: VisualBackground,
        #[serde(skip_serializing_if = "has_no_visible_border")]
        border: BoxQuad<Option<VisualBorder>>,
    },
}

fn has_no_visible_border(border: &BoxQuad<Option<VisualBorder>>) -> bool {
    border.all(Option::is_none)
}
