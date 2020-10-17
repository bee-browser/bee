use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use assert_json_diff::assert_json_eq_no_panic;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;

use bee_layout::*;

fn layout_test(scenario_path: &str, expected_path: &str) {
    validate(interpret(scenario_path), expected_path);
}

fn interpret(scenario_path: &str) -> TestResult {
    let mut service = LayoutService::new();
    let mut node_map = HashMap::new();

    let msgs = load_scenario(scenario_path);

    let mut results = Vec::new();

    for msg in msgs.into_iter() {
        match msg {
            LayoutMessage::CreateElement { id, style, children, label } => {
                let children = children.iter()
                    .map(|id| node_map.get(id).unwrap())
                    .cloned()
                    .collect();
                let node = bee_layout::new_element(style, children, label);
                node_map.insert(id, node);
            }
            LayoutMessage::CreateText { id, text, label } => {
                let node = bee_layout::new_text(text, label);
                node_map.insert(id, node);
            }
            LayoutMessage::Visualize { width, height } => {
                let root = node_map.get(&NodeId(0)).unwrap();
                let visual_tree = service.build_visual_tree(root.clone(), width, height);
                let mut renderer = TestRenderer::new();
                visual_tree.render(&mut renderer);
                results.push(TestResult {
                    layout_tree: root.clone(),
                    visual_tree,
                    paint_messages: renderer.messages,
                });
            }
        }
    }

    results.pop().unwrap()
}

fn validate(result: TestResult, expected_path: &str) {
    let actual = serde_json::to_value(result.paint_messages).unwrap();
    let expected = load_expected(expected_path);
    match assert_json_eq_no_panic(&actual, &expected) {
        Ok(_) => (),
        //<coverage:exclude>
        Err(msg) => {
            println!("{}", msg);
            result.layout_tree.inspect();
            result.visual_tree.inspect();
            panic!("Assertion failed");
        }
        //</coverage:exclude>
    }
}

fn load_scenario(path: &str) -> Vec<LayoutMessage> {
    let file = File::open(path).unwrap_or_else(|err| {
        panic!("{}: {}", path, err);  //<coverage:exclude/>
    });
    serde_json::from_reader(BufReader::new(file)).unwrap_or_else(|err| {
        panic!("{}: {}", path, err);  //<coverage:exclude/>
    })
}

fn load_expected(path: &str) -> serde_json::Value {
    let file = File::open(path).unwrap_or_else(|err| {
        panic!("{}: {}", path, err);  //<coverage:exclude/>
    });
    serde_yaml::from_reader(BufReader::new(file)).unwrap_or_else(|err| {
        panic!("{}: {}", path, err);  //<coverage:exclude/>
    })
}

struct TestResult {
    layout_tree: LayoutNodeHandle,
    visual_tree: VisualRoot,
    paint_messages: Vec<PaintMessage>,
}

struct TestRenderer {
    transform: Transform2D,
    messages: Vec<PaintMessage>,
}

impl TestRenderer {
    fn new() -> Self {
        TestRenderer {
            transform: Default::default(),
            messages: vec![],
        }
    }
}

impl VisualRenderer for TestRenderer {
    fn start_render(&mut self, width: Length, height: Length) {
        self.messages.push(PaintMessage::Start { width, height });
    }

    fn end_render(&mut self) {
        self.messages.push(PaintMessage::End);
    }

    fn render_box(&mut self, model: &VisualBoxModel) {
        let rect = self.transform.outer_transformed_box(model.border_box()).to_rect();
        if rect.is_empty() {
            return;
        }
        if !model.background_color().is_transparent() {
            self.messages.push(PaintMessage::FillRect {
                rect,
                color: model.background_color(),
            });
        }
        if model.border().is_visible() {
            self.messages.push(PaintMessage::DrawBorder {
                rect,
                border: BoxEdge::new(model.border()),
            });
        }
    }

    fn translate_coord(&mut self, v: Vector2D) {
        self.transform = self.transform.then_translate(v);
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

include!(concat!(env!("OUT_DIR"), "/layout_test.codegen.rs"));
