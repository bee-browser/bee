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
                let mut painter = Painter::new(&mut self.sink);
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

struct Painter<'a, T> {
    sink: &'a mut T,
    origin: VisualVector2D,
}

impl<'a, T> Painter<'a, T>
where
    T: JsonSink
{
    fn new(sink: &'a mut T) -> Self {
        Painter {
            sink,
            origin: VisualVector2D::zero(),
        }
    }

    fn send(&mut self, msg: PaintMessage) {
        self.sink.consume(serde_json::to_value(&msg).unwrap());
    }
}

impl<'a, T> VisualRenderer for Painter<'a, T>
where
    T: JsonSink
{
    fn start(&mut self, size: VisualSize2D) {
        self.send(PaintMessage::Start { size });
    }

    fn end(&mut self) {
        self.send(PaintMessage::End);
    }

    fn render_box(&mut self, model: VisualBoxModel) {
        let rect: VisualRect = model.border_box.translate(self.origin).into();
        if rect.is_empty() {
            return;
        }
        if !model.background.color.is_transparent() {
            self.send(PaintMessage::FillRect {
                rect: rect.clone(),
                color: model.background.color,
            });
        }
        for image in model.background.images.iter().rev() {
            // TODO:
        }
        if model.border.is_visible() {
            self.send(PaintMessage::DrawBorder {
                rect: rect.clone(),
                border: model.border.clone(),
            });
        }
    }

    fn translate_coord(&mut self, v: VisualVector2D) {
        self.origin += v;
    }
}

#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
enum PaintMessage {
    #[serde(rename = "paint.start")]
    Start {
        size: VisualSize2D,
    },
    #[serde(rename = "paint.end")]
    End,
    #[serde(rename = "paint.fill_rect")]
    FillRect {
        rect: VisualRect,
        color: Color,
    },
    #[serde(rename = "paint.draw_border")]
    DrawBorder {
        rect: VisualRect,
        border: BoxQuad<VisualBorder>,
    },
}
