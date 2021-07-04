use num_traits::Zero;
use tracing::warn;

use crate::BoxBackground;
use crate::BoxConstraintSolver;
use crate::BoxModel;
use crate::LayoutElement;
use crate::LayoutLength;
use crate::LayoutNodeRef;
use crate::LayoutText;
use crate::LayoutVector2D;
use crate::SolvedBoxGeometry;
use crate::ToVisual;
use crate::VisualRenderer;
use crate::flow::FlowContainer;
use crate::style::*;

pub(crate) struct FlexContainer {
    direction: FlexDirection,
    flows: Vec<FlexItemBond>,
}

impl FlexContainer {
    pub(crate) fn new(nodes: &[LayoutNodeRef], style: &Style, avail: &AvailableSize) -> Self {
        let mut builder = FlexLineBuilder::new(style, avail);
        for node in nodes {
            builder.process_node(node);
        }
        FlexContainer {
            direction: style.flex.direction,
            flows: builder.build(),
        }
    }

    pub(crate) fn inspect<W>(&self, write: &mut W, depth: usize) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        write!(write, "{:indent$}{}\n", "", "flex", indent=depth)?;
        for flow in self.flows.iter() {
            flow.inspect(write, depth + 1)?;
        }
        Ok(())
    }

    pub(crate) fn render<R>(&self, renderer: &mut R)
    where
        R: VisualRenderer,
    {
        for flow in self.flows.iter() {
            flow.render(renderer, self.direction);
        }
    }
}

struct FlexLineBuilder<'a> {
    style: &'a Style,
    avail: &'a AvailableSize,
    main_advance: LayoutLength,
    cross_advance: LayoutLength,
    flows: Vec<FlexItemBond>,
}

impl<'a> FlexLineBuilder<'a> {
    fn new(style: &'a Style, avail: &'a AvailableSize) -> Self {
        FlexLineBuilder {
            style,
            avail,
            main_advance: LayoutLength::zero(),
            cross_advance: LayoutLength::zero(),
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
        let item = element.build_flex_item(&self.avail);
        let main_size = item.main_size(element.style.flex.direction);
        self.flows.push(FlexItemBond::new(self.main_advance, item));
        self.main_advance += main_size;
    }

    fn process_text(&mut self, text: &LayoutText) {
        warn!("TODO: not implemented");
    }

    fn build(mut self) -> Vec<FlexItemBond> {
        let flows = std::mem::replace(&mut self.flows, vec![]);

        let flows: Vec<FlexItemBond> = if self.style.flex.direction.is_reverse() {
            let dir = self.style.flex.direction;
            let main_size = self.avail.main_size(dir);
            flows.into_iter()
                .map(|mut flow| {
                    flow.advance = main_size - (flow.advance + flow.item.main_size(dir));
                    flow
                })
                .collect()
        } else {
            flows
        };

        flows
    }
}

struct FlexItemBond {
    advance: LayoutLength,
    item: FlexItem,
}

impl FlexItemBond {
    fn new(advance: LayoutLength, item: FlexItem) -> Self {
        FlexItemBond { advance, item }
    }

    fn inspect<W>(&self, write: &mut W, depth: usize) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        write!(write, "{:indent$}{}\n", "", self, indent=depth)?;
        self.item.inspect(write, depth + 1)
    }

    fn render<T: VisualRenderer>(&self, renderer: &mut T, dir: FlexDirection) {
        let origin = renderer.get_origin();
        let v = match dir {
            FlexDirection::Row | FlexDirection::RowReverse =>
                LayoutVector2D::new(self.advance, LayoutLength::zero()).to_visual(),
            FlexDirection::Column | FlexDirection::ColumnReverse =>
                LayoutVector2D::new(LayoutLength::zero(), self.advance).to_visual(),
        };
        renderer.set_origin(origin + v);
        self.item.render(renderer);
        renderer.set_origin(origin);
    }
}

impl std::fmt::Display for FlexItemBond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "flex-flow: {:?}", self.advance)
    }
}

struct FlexItem {
    box_model: BoxModel,
    container: FlowContainer,
}

impl FlexItem {
    fn new(box_model: BoxModel, container: FlowContainer) -> Self {
        FlexItem { box_model, container }
    }

    fn inline_size(&self) -> LayoutLength {
        self.box_model.margin_box().width()
    }

    fn block_size(&self) -> LayoutLength {
        self.box_model.margin_box().height()
    }

    fn main_size(&self, dir: FlexDirection) -> LayoutLength {
        match dir {
            FlexDirection::Row | FlexDirection::RowReverse => self.inline_size(),
            FlexDirection::Column | FlexDirection::ColumnReverse => self.block_size(),
        }
    }

    fn cross_size(&self, dir: FlexDirection) -> LayoutLength {
        match dir {
            FlexDirection::Row | FlexDirection::RowReverse => self.block_size(),
            FlexDirection::Column | FlexDirection::ColumnReverse => self.inline_size(),
        }
    }

    fn inspect<W>(&self, write: &mut W, depth: usize) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        write!(write, "{:indent$}{}\n", "", self, indent=depth)?;
        self.container.inspect(write, depth + 1)?;
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
        self.container.render(renderer);

        renderer.set_origin(origin);
    }
}

impl std::fmt::Display for FlexItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "flex-item: {:?}", self.box_model)
    }
}

impl LayoutElement {
    fn build_flex_item(&self, avail: &AvailableSize) -> FlexItem {
        let solved_geom = self.solve_flex_item_box_geometry(avail);

        let box_model = BoxModel {
            style: self.style.clone(),
            geometry: solved_geom.determine(),
            background: BoxBackground {
                color: self.style.background.color.clone(),
                images: vec![],  // TODO
            },
        };

        let new_avail = AvailableSize {
            width: Some(box_model.content_box().width()),
            height: Some(box_model.content_box().height()),
        };

        let container = FlowContainer::new(&self.children, &new_avail);

        // TODO: update height

        FlexItem::new(box_model, container)
    }

    fn solve_flex_item_box_geometry(&self, avail: &AvailableSize) -> SolvedBoxGeometry {
        let mut solver = BoxConstraintSolver::new(avail);
        solver
            .apply_style(&self.style)
            .solve_constraints();

        solver.geom
    }
}

impl FlexDirection {
    pub(crate) fn is_reverse(&self) -> bool {
        match self {
            FlexDirection::RowReverse | FlexDirection::ColumnReverse => true,
            _ => false,
        }
    }
}

impl AvailableSize {
    fn main_size(&self, dir: FlexDirection) -> LayoutLength {
        match dir {
            FlexDirection::Row | FlexDirection::RowReverse =>
                self.width.unwrap_or_default(),
            FlexDirection::Column | FlexDirection::ColumnReverse =>
                self.height.unwrap_or_default(),
        }
    }

    fn cross_size(&self, dir: FlexDirection) -> LayoutLength {
        match dir {
            FlexDirection::Row | FlexDirection::RowReverse =>
                self.height.unwrap_or_default(),
            FlexDirection::Column | FlexDirection::ColumnReverse =>
                self.width.unwrap_or_default(),
        }
    }
}
