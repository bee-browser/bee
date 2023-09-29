use std::sync::Arc;

use num_traits::Zero;
use tracing::warn;

use crate::canvas::CanvasContainer;
use crate::flex::FlexContainer;
use crate::spec::*;
use crate::style::*;
use crate::BoxBackground;
use crate::BoxConstraintSolver;
use crate::BoxModel;
use crate::LayoutBox2D;
use crate::LayoutElement;
use crate::LayoutLength;
use crate::LayoutNodeRef;
use crate::LayoutText;
use crate::LayoutVector2D;
use crate::SolvedBoxGeometry;
use crate::ToVisual;
use crate::VisualRenderer;

impl LayoutElement {
    pub(crate) fn build_flow(&self, avail: &AvailableSize) -> Arc<FlowContainer> {
        debug_assert!(matches!(self.spec.container, ContainerSpec::Flow));
        Arc::new(FlowContainer::new(&self.children, avail))
    }

    fn build_block(&self, avail: &AvailableSize) -> Block {
        let solved_geom = self.solve_block_box_geometry(avail);

        let box_model = BoxModel {
            style: self.style.clone(),
            geometry: solved_geom.determine(),
            background: BoxBackground {
                color: self.style.background.color.clone(),
                images: vec![], // TODO
            },
        };

        let new_avail = AvailableSize {
            width: Some(box_model.content_box().width()),
            height: Some(box_model.content_box().height()),
        };

        let container =
            BlockContent::new(self.spec.container, &self.children, &self.style, &new_avail);

        // TODO: update height

        Block::new(box_model, container)
    }

    fn solve_block_box_geometry(&self, avail: &AvailableSize) -> SolvedBoxGeometry {
        let mut solver = BoxConstraintSolver::new(avail);
        solver.apply_style(&self.style).solve_constraints();

        solver.geom
    }
}

pub(crate) struct FlowContainer {
    container: BlockContainer,
    // TODO: floating boxes
}

impl FlowContainer {
    pub(crate) fn new(nodes: &[LayoutNodeRef], avail: &AvailableSize) -> Self {
        FlowContainer {
            container: BlockContainer::new(nodes, avail),
        }
    }

    pub(crate) fn inspect<W>(&self, write: &mut W, depth: usize) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        write!(write, "{:indent$}{}\n", "", "flow-root", indent = depth)?;
        self.container.inspect(write, depth + 1)?;
        Ok(())
    }

    pub(crate) fn render<T: VisualRenderer>(&self, renderer: &mut T) {
        self.container.render_blocks(renderer);
        self.container.render_floats(renderer);
        self.container.render_inlines(renderer);
    }
}

pub(crate) struct BlockContainer {
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

    pub(crate) fn inspect<W>(&self, write: &mut W, depth: usize) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
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
    advance: LayoutLength,
    flows: Vec<BlockFlow>,
}

impl<'a> BlockFlowBuilder<'a> {
    fn new(avail: &'a AvailableSize) -> Self {
        BlockFlowBuilder {
            avail,
            advance: LayoutLength::zero(),
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
        self.advance += height;
    }

    fn process_inline_element(&mut self, _element: &LayoutElement) {
        warn!("TODO: not implemented");
    }

    fn process_float_element(&mut self, _element: &LayoutElement) {
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
    advance: LayoutLength,
    block: Block,
}

impl BlockFlow {
    fn new(advance: LayoutLength, block: Block) -> Self {
        BlockFlow { advance, block }
    }

    fn inspect<W>(&self, write: &mut W, depth: usize) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        self.block.inspect(write, depth)
    }

    fn render<T: VisualRenderer>(&self, renderer: &mut T) {
        let origin = renderer.get_origin();
        let v = LayoutVector2D::new(LayoutLength::zero(), self.advance).to_visual();
        renderer.set_origin(origin + v);
        self.block.render(renderer);
        renderer.set_origin(origin);
    }
}

struct Block {
    box_model: BoxModel,
    content: BlockContent,
}

impl Block {
    #[inline]
    fn new(box_model: BoxModel, content: BlockContent) -> Self {
        Block { box_model, content }
    }

    fn inspect<W>(&self, write: &mut W, depth: usize) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        write!(write, "{:indent$}{}\n", "", self, indent = depth)?;
        self.content.inspect(write, depth + 1)?;
        Ok(())
    }

    fn render<T: VisualRenderer>(&self, renderer: &mut T) {
        let origin = renderer.get_origin();

        let box_model = self.box_model.to_visual();

        if box_model.is_visible() {
            renderer.render_box(box_model);
        }

        let v = self.box_model.content_box().min.to_visual().to_vector();
        renderer.set_origin(origin + v);
        self.content.render(renderer, self.box_model.content_box());

        renderer.set_origin(origin);
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "block: {:?}", self.box_model)
    }
}

enum BlockContent {
    None,
    Block(BlockContainer),
    Flow(FlowContainer),
    Flex(FlexContainer),
    Canvas(CanvasContainer),
}

impl BlockContent {
    fn new(
        spec: ContainerSpec,
        nodes: &[LayoutNodeRef],
        style: &Style,
        avail: &AvailableSize,
    ) -> Self {
        match spec {
            ContainerSpec::None => Self::None,
            ContainerSpec::Flow => Self::Flow(FlowContainer::new(nodes, avail)),
            ContainerSpec::Block => Self::Block(BlockContainer::new(nodes, avail)),
            ContainerSpec::Flex => Self::Flex(FlexContainer::new(nodes, style, avail)),
            ContainerSpec::Canvas => Self::Canvas(CanvasContainer::new(style)),
            spec => unreachable!("{:?}", spec),
        }
    }

    fn inspect<W>(&self, write: &mut W, depth: usize) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        match self {
            Self::None => write!(write, "{:indent$}none\n", "", indent = depth),
            Self::Block(ref block) => block.inspect(write, depth),
            Self::Flow(ref flow) => flow.inspect(write, depth),
            Self::Flex(ref flex) => flex.inspect(write, depth),
            Self::Canvas(ref canvas) => canvas.inspect(write, depth),
        }
    }

    fn render<R>(&self, renderer: &mut R, content_box: &LayoutBox2D)
    where
        R: VisualRenderer,
    {
        match self {
            Self::None => (),
            Self::Block(ref block) => block.render_blocks(renderer),
            Self::Flow(ref flow) => flow.render(renderer),
            Self::Flex(ref flex) => flex.render(renderer),
            Self::Canvas(ref canvas) => canvas.render(renderer, content_box),
        }
    }
}
