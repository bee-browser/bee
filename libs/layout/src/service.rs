use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::*;

pub struct MessageInterpreter {
    node_map: HashMap<NodeId, LayoutNodeHandle>,
}

impl MessageInterpreter {
    pub fn new() -> Self {
        MessageInterpreter {
            node_map: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, json: &str) -> Result<()> {
        match serde_json::from_str(json)? {
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
                let mut painter = Painter::new();
                visual_tree.render(&mut painter);
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Deserialize)]
struct NodeId(usize);

#[derive(Deserialize)]
enum LayoutMessage {
    CreateElement {
        id: NodeId,
        style: Arc<Style>,
        children: Vec<NodeId>,
        label: String,
    },
    CreateText {
        id: NodeId,
        text: String,
        label: String,
    },
    Visualize {
        width: usize,
        height: usize,
    },
}

struct Painter {
    transform: Transform2D,
}

impl Painter {
    fn new() -> Self {
        Painter {
            transform: Default::default(),
        }
    }

    fn println(&self, msg: PaintMessage) {
        println!("{}", serde_json::to_string(&msg).expect("Failed to write"));
    }
}

impl VisualRenderer for Painter {
    fn start_render(&mut self, width: Length, height: Length) {
        self.println(PaintMessage::Start { width, height });
    }

    fn end_render(&mut self) {
        self.println(PaintMessage::End);
    }

    fn render_box(&mut self, model: &VisualBoxModel) {
        let rect = self.transform.outer_transformed_box(model.border_box()).to_rect();
        if rect.is_empty() {
            return;
        }
        if !model.background_color().is_transparent() {
            self.println(PaintMessage::FillRect {
                rect,
                color: model.background_color(),
            });
        }
        if model.border().is_visible() {
            self.println(PaintMessage::DrawBorder {
                rect,
                border: BoxEdge::new(model.border()),
            });
        }
    }

    fn translate_coord(&mut self, v: Vector2D) {
        self.transform = self.transform.then_translate(v);
    }
}

#[derive(Serialize)]
enum PaintMessage {
    Start {
        width: Length,
        height: Length,
    },
    End,
    FillRect {
        rect: Rect,
        color: Color,
    },
    DrawBorder {
        rect: Rect,
        border: BoxEdge<Border>,
    },
}

#[derive(Serialize)]
struct BoxEdge<T> {
    top: T,
    right: T,
    bottom: T,
    left: T,
}

impl<T: Copy> BoxEdge<T> {
    fn new(quad: &BoxQuad<T>) -> Self {
        BoxEdge {
            top: quad.top(),
            right: quad.right(),
            bottom: quad.bottom(),
            left: quad.left(),
        }
    }
}
