use std::sync::Arc;

use euclid::num::Zero;
use tracing::{warn};

use crate::BoxConstraintSolver;
use crate::Length;
use crate::LayoutNodeRef;
use crate::LayoutElement;
use crate::LayoutText;
use crate::SolvedBoxGeometry;
use crate::Vector2D;
use crate::VisualBoxModel;
use crate::VisualRenderer;
use crate::spec::*;
use crate::style::*;

impl LayoutElement {
    pub(crate) fn build_flow(&self, avail: &AvailableSize) -> Arc<FlowContainer> {
        debug_assert!(matches!(self.spec.container, ContainerSpec::Flow));
        Arc::new(FlowContainer::new(&self.children, avail))
    }

    fn build_block(&self, avail: &AvailableSize) -> VisualBlock {
        let solved_geom = self.solve_block_box_geometry(avail);

        let box_model = VisualBoxModel {
            style: self.style.clone(),
            geometry: solved_geom.determine(),
        };

        let new_avail = AvailableSize {
            width: Some(Length::new(box_model.content_box().width())),
            height: Some(Length::new(box_model.content_box().height())),
        };

        let container = BlockContainer::new(&self.children, &new_avail);

        // TODO: update height

        VisualBlock::new(box_model, container)
    }

    fn solve_block_box_geometry(&self, avail: &AvailableSize) -> SolvedBoxGeometry {
        let mut solver = BoxConstraintSolver::new(avail);
        solver
            .apply_style(&self.style)
            .solve_constraints();

        solver.geom
    }
}

pub(crate) struct FlowContainer {
    container: BlockContainer,
    // TODO: floating boxes
}

impl FlowContainer {
    fn new(nodes: &[LayoutNodeRef], avail: &AvailableSize) -> Self {
        FlowContainer {
            container: BlockContainer::new(nodes, avail),
        }
    }

    pub(crate) fn inspect<T>(&self, write: &mut T, depth: usize) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        write!(write, "{:indent$}{}\n", "", "flow-root", indent=depth)?;
        self.container.inspect(write, depth + 1)?;
        Ok(())
    }

    pub(crate) fn render<T: VisualRenderer>(&self, renderer: &mut T) {
        self.container.render_blocks(renderer);
        self.container.render_floats(renderer);
        self.container.render_inlines(renderer);
    }
}

struct BlockContainer {
    children: Vec<BlockFlow>,
}

impl BlockContainer {
    fn new(nodes: &[LayoutNodeRef], avail: &AvailableSize) -> Self {
        let mut builder = BlockFlowBuilder::new(avail);
        for node in nodes {
            builder.process_node(node);
        }

        BlockContainer {
            children: builder.build(),
        }
    }

    fn inspect<T>(&self, write: &mut T, depth: usize) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        for flow in self.children.iter() {
            flow.inspect(write, depth)?;
        }
        Ok(())
    }

    fn render_blocks<T: VisualRenderer>(&self, renderer: &mut T) {
        for flow in self.children.iter() {
            flow.render(renderer);
        }
    }

    fn render_floats<T: VisualRenderer>(&self, _renderer: &mut T) {
        // TODO
    }

    fn render_inlines<T: VisualRenderer>(&self, _renderer: &mut T) {
        // TODO
    }
}

struct BlockFlowBuilder<'a> {
    avail: &'a AvailableSize,
    advance: Length,
    flows: Vec<BlockFlow>,
}

impl<'a> BlockFlowBuilder<'a> {
    fn new(avail: &'a AvailableSize) -> Self {
        BlockFlowBuilder {
            avail,
            advance: Length::zero(),
            flows: vec![],
        }
    }

    fn process_node(&mut self, node: &LayoutNodeRef) {
        match node {
            LayoutNodeRef::Element(ref element) => self.process_element(element),
            LayoutNodeRef::Text(ref text) => self.process_text(text),
        }
    }

    fn process_element(&mut self, element: &LayoutElement) {
        match element.spec.node {
            NodeSpec::Block => self.process_block_element(element),
            NodeSpec::Inline => self.process_inline_element(element),
            NodeSpec::Float => self.process_float_element(element),
            _ => (),
        }
    }

    fn process_block_element(&mut self, element: &LayoutElement) {
        let block = element.build_block(&self.avail);
        let height = block.box_model.geometry.margin_box.height();
        self.flows.push(BlockFlow::new(self.advance, block));
        self.advance += Length::new(height);
    }

    fn process_inline_element(&mut self, _element:  &LayoutElement) {
        warn!("TODO: not implemented");
    }

    fn process_float_element(&mut self, _element:  &LayoutElement) {
        warn!("TODO: not implemented");
    }

    fn process_text(&mut self, _text: &LayoutText) {
        warn!("TODO: not implemented");
    }

    fn build(self) -> Vec<BlockFlow> {
        self.flows
    }
}

struct BlockFlow {
    advance: Length,
    block: VisualBlock,
}

impl BlockFlow {
    fn new(advance: Length, block: VisualBlock) -> Self {
        BlockFlow { advance, block }
    }

    fn inspect<T>(&self, write: &mut T, depth: usize) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        self.block.inspect(write, depth)
    }

    fn render<T: VisualRenderer>(&self, renderer: &mut T) {
        let v = Vector2D::from_lengths(Length::zero(), self.advance);
        renderer.translate_coord(v);
        self.block.render(renderer);
        renderer.translate_coord(-v);
    }
}

struct VisualBlock {
    box_model: VisualBoxModel,
    container: BlockContainer,
}

impl VisualBlock {
    #[inline]
    fn new(box_model: VisualBoxModel, container: BlockContainer) -> Self {
        VisualBlock { box_model, container }
    }

    fn inspect<T>(&self, write: &mut T, depth: usize) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        write!(write, "{:indent$}{}\n", "", self, indent=depth)?;
        self.container.inspect(write, depth + 1)?;
        Ok(())
    }

    fn render<T: VisualRenderer>(&self, renderer: &mut T) {
        renderer.render_box(&self.box_model);

        let v = self.box_model.content_box().min.to_vector();
        renderer.translate_coord(v);
        self.container.render_blocks(renderer);
        renderer.translate_coord(-v);
    }
}

impl std::fmt::Display for VisualBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "block: {:?}", self.box_model)
    }
}
