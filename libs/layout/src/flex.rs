use num_traits::Zero;

use crate::flow::FlowContainer;
use crate::logger;
use crate::style::*;
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

pub(crate) struct FlexContainer {
    direction: FlexDirection,
    lines: Vec<FlexLine>,
}

impl FlexContainer {
    pub(crate) fn new(nodes: &[LayoutNodeRef], style: &Style, avail: &AvailableSize) -> Self {
        let mut builder = FlexLineBuilder::new(style, avail);
        for node in nodes {
            builder.process_node(node);
        }
        FlexContainer {
            direction: style.flex.direction,
            lines: builder.build(),
        }
    }

    pub(crate) fn inspect<W>(&self, write: &mut W, depth: usize) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        writeln!(write, "{:indent$}flex:", "", indent = depth)?;
        for line in self.lines.iter() {
            line.inspect(write, depth + 1)?;
        }
        Ok(())
    }

    pub(crate) fn render<R>(&self, renderer: &mut R)
    where
        R: VisualRenderer,
    {
        for line in self.lines.iter() {
            line.render(renderer, self.direction);
        }
    }
}

struct FlexLineBuilder<'a> {
    style: &'a Style,
    avail: &'a AvailableSize,
    items: Vec<FlexItem>,
}

impl<'a> FlexLineBuilder<'a> {
    fn new(style: &'a Style, avail: &'a AvailableSize) -> Self {
        FlexLineBuilder {
            style,
            avail,
            items: vec![],
        }
    }

    fn process_node(&mut self, node: &LayoutNodeRef) {
        match node {
            LayoutNodeRef::Element(element) => self.process_element(element),
            LayoutNodeRef::Text(text) => self.process_text(text),
        }
    }

    fn process_element(&mut self, element: &LayoutElement) {
        let item = element.build_flex_item(self.avail);
        self.items.push(item);
    }

    fn process_text(&mut self, text: &LayoutText) {
        logger::warn!("TODO: not implemented: {}", text);
    }

    fn build(mut self) -> Vec<FlexLine> {
        let mut lines = self.build_lines();

        // TODO: Distribute free space

        if self.style.flex.wrap.is_reverse() {
            self.reverse_lines(&mut lines);
        }
        if self.style.flex.direction.is_reverse() {
            self.reverse_items(&mut lines);
        }

        lines
    }

    fn build_lines(&mut self) -> Vec<FlexLine> {
        let multiline = self.style.flex.wrap.is_multiline();
        let dir = self.style.flex.direction;

        let mut items = std::mem::take(&mut self.items);

        Self::reoder_items(&mut items);

        let mut lines = vec![];
        let mut flows: Vec<FlexItemBond> = vec![];
        let avail_size = self.avail.main_size(dir);
        let mut main_advance = LayoutLength::zero();
        let mut cross_advance = LayoutLength::zero();

        for item in items.into_iter() {
            let main_size = item.main_size(dir);
            if multiline && main_advance + main_size > avail_size {
                let cross_size = flows
                    .iter()
                    .map(|flow| flow.cross_size(dir))
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .expect("`flows` must be a non-empty");
                // TODO: Distribute free space
                let line = FlexLine::new(cross_advance, cross_size, std::mem::take(&mut flows));
                cross_advance += cross_size;
                lines.push(line);
                main_advance = LayoutLength::zero();
            }
            flows.push(FlexItemBond::new(main_advance, item));
            main_advance += main_size;
        }

        if !flows.is_empty() {
            let cross_size = flows
                .iter()
                .map(|flow| flow.cross_size(dir))
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .expect("`flows` must be a non-empty");
            // TODO: Distribute free space
            let line = FlexLine::new(cross_advance, cross_size, std::mem::take(&mut flows));
            lines.push(line);
        }

        lines
    }

    fn reoder_items(items: &mut [FlexItem]) {
        logger::warn!("TODO: reorder items: {}", items.len());
    }

    fn reverse_lines(&self, lines: &mut [FlexLine]) {
        let dir = self.style.flex.direction;
        let avail_size = self.avail.cross_size(dir);

        for line in lines.iter_mut() {
            line.advance = avail_size - (line.advance + line.cross_size);
        }
    }

    fn reverse_items(&self, lines: &mut [FlexLine]) {
        let dir = self.style.flex.direction;
        let avail_size = self.avail.main_size(dir);

        for line in lines.iter_mut() {
            for flow in line.flows.iter_mut() {
                flow.advance = avail_size - (flow.advance + flow.main_size(dir));
            }
        }
    }
}

struct FlexLine {
    advance: LayoutLength,
    cross_size: LayoutLength,
    flows: Vec<FlexItemBond>,
}

impl FlexLine {
    fn new(advance: LayoutLength, cross_size: LayoutLength, flows: Vec<FlexItemBond>) -> Self {
        FlexLine {
            advance,
            cross_size,
            flows,
        }
    }

    fn inspect<W>(&self, write: &mut W, depth: usize) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        writeln!(
            write,
            "{:indent$}flex-line: {:?} {:?}",
            "",
            self.advance,
            self.cross_size,
            indent = depth
        )?;
        for flow in self.flows.iter() {
            flow.inspect(write, depth + 1)?
        }
        Ok(())
    }

    fn render<T: VisualRenderer>(&self, renderer: &mut T, dir: FlexDirection) {
        let origin = renderer.get_origin();
        let v = if dir.is_row() {
            LayoutVector2D::new(LayoutLength::zero(), self.advance).to_visual()
        } else {
            LayoutVector2D::new(self.advance, LayoutLength::zero()).to_visual()
        };
        renderer.set_origin(origin + v);
        for flow in self.flows.iter() {
            flow.render(renderer, dir)
        }
        renderer.set_origin(origin);
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
        writeln!(write, "{:indent$}{}", "", self, indent = depth)?;
        self.item.inspect(write, depth + 1)
    }

    fn render<T: VisualRenderer>(&self, renderer: &mut T, dir: FlexDirection) {
        let origin = renderer.get_origin();
        let v = if dir.is_row() {
            LayoutVector2D::new(self.advance, LayoutLength::zero()).to_visual()
        } else {
            LayoutVector2D::new(LayoutLength::zero(), self.advance).to_visual()
        };
        renderer.set_origin(origin + v);
        self.item.render(renderer);
        renderer.set_origin(origin);
    }

    fn main_size(&self, dir: FlexDirection) -> LayoutLength {
        self.item.main_size(dir)
    }

    fn cross_size(&self, dir: FlexDirection) -> LayoutLength {
        self.item.cross_size(dir)
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
        FlexItem {
            box_model,
            container,
        }
    }

    fn inline_size(&self) -> LayoutLength {
        self.box_model.margin_box().width()
    }

    fn block_size(&self) -> LayoutLength {
        self.box_model.margin_box().height()
    }

    fn main_size(&self, dir: FlexDirection) -> LayoutLength {
        if dir.is_row() {
            self.inline_size()
        } else {
            self.block_size()
        }
    }

    fn cross_size(&self, dir: FlexDirection) -> LayoutLength {
        if dir.is_row() {
            self.block_size()
        } else {
            self.inline_size()
        }
    }

    fn inspect<W>(&self, write: &mut W, depth: usize) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        writeln!(write, "{:indent$}{}", "", self, indent = depth)?;
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
                color: self.style.background.color,
                images: vec![], // TODO
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
        solver.apply_style(&self.style).solve_constraints();

        solver.geom
    }
}

impl FlexDirection {
    fn is_row(&self) -> bool {
        matches!(self, FlexDirection::Row | FlexDirection::RowReverse)
    }

    pub(crate) fn is_reverse(&self) -> bool {
        matches!(
            self,
            FlexDirection::RowReverse | FlexDirection::ColumnReverse
        )
    }
}

impl FlexWrap {
    fn is_multiline(&self) -> bool {
        !matches!(self, FlexWrap::Nowrap)
    }

    fn is_reverse(&self) -> bool {
        matches!(self, FlexWrap::WrapReverse)
    }
}

impl AvailableSize {
    fn main_size(&self, dir: FlexDirection) -> LayoutLength {
        if dir.is_row() {
            self.width.unwrap_or_default()
        } else {
            self.height.unwrap_or_default()
        }
    }

    fn cross_size(&self, dir: FlexDirection) -> LayoutLength {
        if dir.is_row() {
            self.height.unwrap_or_default()
        } else {
            self.width.unwrap_or_default()
        }
    }
}
