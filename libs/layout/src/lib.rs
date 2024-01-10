mod canvas;
mod flex;
mod flow;
mod logger;
mod spec;
mod style;

#[cfg(feature = "service")]
pub mod service;

use std::collections::BTreeMap;
use std::sync::Arc;

use num_traits::Bounded;
use num_traits::Zero;
#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;

use geometry;

use crate::flex::FlexContainer;
use crate::flow::FlowContainer;
use crate::spec::*;
pub use crate::style::*;

pub type Decimal = f32;
pub type Integer = i32;

pub mod units {
    #[derive(Debug)]
    pub struct Px;
}

// Geometric types used in layout trees.
pub type LayoutLength = geometry::Length<Decimal, units::Px>;
type LayoutPoint2D = geometry::Point2D<Decimal, units::Px>;
pub type LayoutSize2D = geometry::Size2D<Decimal, units::Px>;
pub type LayoutBox2D = geometry::Box2D<Decimal, units::Px>;
pub type LayoutRect = geometry::Rect<Decimal, units::Px>;
pub type LayoutVector2D = geometry::Vector2D<Decimal, units::Px>;

// Geometric types used for rendering box models.
pub type VisualLength = geometry::Length<Integer, units::Px>;
type VisualPoint2D = geometry::Point2D<Integer, units::Px>;
pub type VisualSize2D = geometry::Size2D<Integer, units::Px>;
pub type VisualBox2D = geometry::Box2D<Integer, units::Px>;
pub type VisualRect = geometry::Rect<Integer, units::Px>;
pub type VisualVector2D = geometry::Vector2D<Integer, units::Px>;

pub trait ToVisual {
    type VisualType;

    fn to_visual(&self) -> Self::VisualType;
}

impl ToVisual for LayoutLength {
    type VisualType = VisualLength;

    fn to_visual(&self) -> Self::VisualType {
        VisualLength::new(self.value().floor() as Integer)
    }
}

impl ToVisual for LayoutPoint2D {
    type VisualType = VisualPoint2D;

    fn to_visual(&self) -> Self::VisualType {
        VisualPoint2D::new(self.x.to_visual(), self.y.to_visual())
    }
}

impl ToVisual for LayoutSize2D {
    type VisualType = VisualSize2D;

    fn to_visual(&self) -> Self::VisualType {
        VisualSize2D::new(self.width.to_visual(), self.height.to_visual())
    }
}

impl ToVisual for LayoutBox2D {
    type VisualType = VisualBox2D;

    fn to_visual(&self) -> Self::VisualType {
        VisualBox2D::new(self.min.to_visual(), self.max.to_visual())
    }
}

impl ToVisual for LayoutVector2D {
    type VisualType = VisualVector2D;

    fn to_visual(&self) -> Self::VisualType {
        VisualVector2D::new(self.x.to_visual(), self.y.to_visual())
    }
}

pub fn new_element(
    style: Arc<Style>,
    children: Vec<LayoutNodeHandle>,
    label: String,
) -> LayoutNodeHandle {
    let children = children.into_iter().map(|handle| handle.0).collect();
    LayoutNodeHandle(LayoutNodeRef::Element(Arc::new(LayoutElement::new(
        style, children, label,
    ))))
}

pub fn new_text(text: String, label: String) -> LayoutNodeHandle {
    LayoutNodeHandle(LayoutNodeRef::Text(Arc::new(LayoutText::new(text, label))))
}

pub fn build_visual_tree(layout_root: LayoutNodeHandle, width: usize, height: usize) -> VisualRoot {
    if let LayoutNodeRef::Element(ref element) = layout_root.0 {
        let width = LayoutLength::new(width as f32);
        let height = LayoutLength::new(height as f32);
        let root_box: LayoutBox2D =
            (LayoutLength::zero(), LayoutLength::zero(), width, height).into();

        let box_model = BoxModel {
            style: element.style.clone(),
            geometry: BoxGeometry {
                margin_box: root_box.clone(),
                border_box: root_box.clone(),
                padding_box: root_box.clone(),
                content_box: root_box.clone(),
            },
            background: BoxBackground {
                color: element.style.background.color.clone(),
                images: vec![], // TODO
            },
        };

        let avail = AvailableSize {
            width: Some(width),
            height: Some(height),
        };

        let flow = element.build_flow(&avail);

        let layers = element.build_layers_for_children(&avail, &avail).into_vec();

        VisualRoot {
            box_model,
            flow,
            layers,
        }
    } else {
        unreachable!();
    }
}

#[derive(Clone)]
pub struct LayoutNodeHandle(LayoutNodeRef); // opaque

impl LayoutNodeHandle {
    //<coverage:exclude>
    pub fn inspect<T>(&self, write: &mut T) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        self.0.inspect(write, 0)
    }
    //</coverage:exclude>
}

#[derive(Clone)]
enum LayoutNodeRef {
    Element(Arc<LayoutElement>),
    Text(Arc<LayoutText>),
}

impl LayoutNodeRef {
    fn maybe_element(&self) -> Option<&Arc<LayoutElement>> {
        match *self {
            Self::Element(ref element) => Some(element),
            _ => None,
        }
    }

    //<coverage:exclude>
    fn inspect<T>(&self, write: &mut T, depth: usize) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        match *self {
            LayoutNodeRef::Element(ref element) => element.inspect(write, depth),
            LayoutNodeRef::Text(ref text) => text.inspect(write, depth),
        }
    }
    //</coverage:exclude>
}

struct LayoutElement {
    spec: Spec,
    style: Arc<Style>,
    children: Vec<LayoutNodeRef>,
    label: String,
}

impl LayoutElement {
    fn new(style: Arc<Style>, children: Vec<LayoutNodeRef>, label: String) -> Self {
        LayoutElement {
            spec: Spec::determine_from(&style),
            style,
            children,
            label,
        }
    }

    fn build_layer_content(&self, avail: &AvailableSize) -> Arc<LayerContent> {
        Arc::new(LayerContent::new(
            self.spec.container,
            &self.children,
            &self.style,
            avail,
        ))
    }

    fn solve_box_geometry(&self, avail: &AvailableSize) -> SolvedBoxGeometry {
        let mut solver = BoxConstraintSolver::new(avail);
        solver.apply_style(&self.style).solve_constraints();

        solver.geom
    }

    fn build_layers(
        &self,
        initial_avail: &AvailableSize,
        avail: &AvailableSize,
    ) -> VisualLayersMap {
        match self.style.positioning {
            PositioningScheme::Static => self.build_layers_for_children(initial_avail, avail),
            PositioningScheme::Fixed => self.build_layers_for_layer(initial_avail, initial_avail),
            PositioningScheme::Absolute | PositioningScheme::Sticky =>
            // TODO
            {
                self.build_layers_for_layer(initial_avail, avail)
            }
            PositioningScheme::Relative => self.build_layers_for_children(initial_avail, avail), // TODO
        }
    }

    fn build_layers_for_layer(
        &self,
        initial_avail: &AvailableSize,
        avail: &AvailableSize,
    ) -> VisualLayersMap {
        let solved_geom = self.solve_box_geometry(avail);

        // TODO: layout in-flow child elements.
        let content = self.build_layer_content(&AvailableSize {
            width: solved_geom.width.value.clone(),
            height: solved_geom.height.value.clone(),
        });

        // TODO:
        // * update the position of the layer with the static position if it has not been solved.
        // * determine the height of the layer if it has not been solved.
        let box_model = BoxModel {
            style: self.style.clone(),
            geometry: solved_geom.determine(),
            background: BoxBackground {
                color: self.style.background.color.clone(),
                images: vec![], // TODO
            },
        };

        let new_avail = AvailableSize {
            width: Some(box_model.padding_box().width()),
            height: Some(box_model.padding_box().height()),
        };

        let mut layers = self.build_layers_for_children(initial_avail, &new_avail);

        match self.style.layer.z_index {
            LayerZIndex::Auto => {
                let layer = Arc::new(VisualLayer {
                    box_model,
                    stack_level: 0,
                    content,
                    child_layers: vec![],
                });
                layers.push_front(layer);
                layers
            }
            LayerZIndex::Index(stack_level) => {
                let layer = Arc::new(VisualLayer {
                    box_model,
                    stack_level,
                    content,
                    child_layers: layers.into_vec(),
                });
                let mut layers = VisualLayersMap::new();
                layers.push_back(layer);
                layers
            }
        }
    }

    fn build_layers_for_children(
        &self,
        initial_avail: &AvailableSize,
        avail: &AvailableSize,
    ) -> VisualLayersMap {
        self.children
            .iter()
            .filter_map(LayoutNodeRef::maybe_element)
            .map(|element| element.build_layers(initial_avail, avail))
            .fold(VisualLayersMap::new(), |mut acc, v| {
                acc.merge(v);
                acc
            })
    }

    //<coverage:exclude>
    fn inspect<T>(&self, write: &mut T, depth: usize) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        write!(write, "{:indent$}{}\n", "", self, indent = depth)?;
        for node_ref in self.children.iter() {
            node_ref.inspect(write, depth + 1)?;
        }
        Ok(())
    }
    //</coverage:exclude>
}

impl std::fmt::Display for LayoutElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "element: spec={:?} label=\"{}\"", self.spec, self.label)
    }
}

struct LayoutText {
    text: String,
    label: String,
}

impl LayoutText {
    fn new(text: String, label: String) -> Self {
        LayoutText { text, label }
    }

    fn inspect<T>(&self, write: &mut T, depth: usize) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        write!(write, "{:indent$}{}\n", "", self, indent = depth)
    }
}

impl std::fmt::Display for LayoutText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "text: text=\"{}\", label=\"{}\"",
            self.text.escape_debug().to_string(),
            self.label
        )
    }
}

// context node
pub struct VisualRoot {
    box_model: BoxModel,
    // bounding box
    flow: Arc<FlowContainer>,
    layers: Vec<Arc<VisualLayer>>,
}

impl VisualRoot {
    pub fn inspect<T>(&self, write: &mut T) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        write!(write, "root: {:?}\n", self.box_model)?;

        for layer in self.layers.iter().filter(|layer| layer.stack_level < 0) {
            layer.inspect(write, 1)?;
        }

        self.flow.inspect(write, 1)?;

        for layer in self.layers.iter().filter(|layer| layer.stack_level >= 0) {
            layer.inspect(write, 1)?;
        }

        Ok(())
    }

    pub fn render<T: VisualRenderer>(&self, renderer: &mut T) {
        renderer.start(self.box_model.margin_box().size().to_visual());

        let box_model = self.box_model.to_visual();
        if box_model.is_visible() {
            // background and borders
            renderer.render_box(box_model);
        }

        // negative layers
        let v = self.box_model.padding_box().min.to_visual().to_vector();
        renderer.set_origin(v);
        for layer in self.layers.iter().filter(|layer| layer.stack_level < 0) {
            layer.render(renderer);
        }

        // in-flow boxes
        let v = self.box_model.content_box().min.to_visual().to_vector();
        renderer.set_origin(v);
        self.flow.render(renderer);

        // non-negative layers
        let v = self.box_model.padding_box().min.to_visual().to_vector();
        renderer.set_origin(v);
        for layer in self.layers.iter().filter(|layer| layer.stack_level >= 0) {
            layer.render(renderer);
        }

        renderer.set_origin(VisualVector2D::zero());

        renderer.end();
    }
}

pub trait VisualRenderer {
    fn start(&mut self, size: VisualSize2D);
    fn end(&mut self);
    fn get_origin(&self) -> VisualVector2D;
    fn set_origin(&mut self, origin: VisualVector2D);
    fn render_box(&mut self, model: VisualBoxModel);
    fn render_asset(&mut self, asset_id: u64, content_box: VisualBox2D);
}

pub struct VisualBoxModel {
    pub border_box: VisualBox2D,
    pub background: VisualBackground,
    pub border: BoxQuad<Option<VisualBorder>>,
}

impl VisualBoxModel {
    pub fn is_visible(&self) -> bool {
        if self.border_box.is_empty() {
            false
        } else {
            self.background.is_visible() || self.border.is_visible()
        }
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct VisualBackground {
    #[serde(skip_serializing_if = "Color::is_transparent")]
    pub color: Color,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<VisualBackgroundImage>,
}

impl VisualBackground {
    pub fn is_visible(&self) -> bool {
        !self.is_transparent()
    }

    pub fn is_transparent(&self) -> bool {
        self.color.is_transparent() && self.images.is_empty()
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct VisualBackgroundImage {
    pub asset: Asset,
    pub crop_area: VisualBox2D,
    pub tile_size: VisualSize2D,
    pub position: VisualPoint2D,
    pub horizontal_spacing: VisualLength,
    pub vertical_spacing: VisualLength,
    pub horizontal_repeat: bool,
    pub vertical_repeat: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct VisualBorder {
    pub style: BorderStyle,
    pub width: VisualLength,
    pub color: Color,
}

impl BoxQuad<Option<VisualBorder>> {
    pub fn is_visible(&self) -> bool {
        self.any(Option::is_some)
    }
}

impl ToVisual for BackgroundStyle {
    type VisualType = VisualBackground;

    fn to_visual(&self) -> Self::VisualType {
        VisualBackground {
            color: self.color.clone(),
            images: vec![],
        }
    }
}

impl ToVisual for Border {
    type VisualType = VisualBorder;

    fn to_visual(&self) -> Self::VisualType {
        VisualBorder {
            style: self.style.clone(),
            width: self.width.to_visual(),
            color: self.color.clone(),
        }
    }
}

struct BoxModel {
    style: Arc<Style>,
    geometry: BoxGeometry,
    background: BoxBackground,
}

struct BoxGeometry {
    margin_box: LayoutBox2D,
    border_box: LayoutBox2D,
    padding_box: LayoutBox2D,
    content_box: LayoutBox2D,
}

struct BoxBackground {
    color: Color,
    images: Vec<BoxBackgroundImage>,
}

struct BoxBackgroundImage {
    asset: Asset,
    crop_area: LayoutBox2D,
    tile_size: LayoutSize2D,
    position: LayoutPoint2D,
    horizontal_spacing: LayoutLength,
    vertical_spacing: LayoutLength,
    horizontal_repeat: bool,
    vertical_repeat: bool,
}

impl Default for BoxGeometry {
    fn default() -> Self {
        Self {
            margin_box: LayoutBox2D::empty(),
            border_box: LayoutBox2D::empty(),
            padding_box: LayoutBox2D::empty(),
            content_box: LayoutBox2D::empty(),
        }
    }
}

impl ToVisual for BoxBackground {
    type VisualType = VisualBackground;

    fn to_visual(&self) -> Self::VisualType {
        VisualBackground {
            color: self.color.clone(),
            images: self.images.iter().map(|image| image.to_visual()).collect(),
        }
    }
}

impl ToVisual for BoxBackgroundImage {
    type VisualType = VisualBackgroundImage;

    fn to_visual(&self) -> Self::VisualType {
        VisualBackgroundImage {
            asset: self.asset.clone(),
            crop_area: self.crop_area.to_visual(),
            tile_size: self.tile_size.to_visual(),
            position: self.position.to_visual(),
            horizontal_spacing: self.horizontal_spacing.to_visual(),
            vertical_spacing: self.vertical_spacing.to_visual(),
            horizontal_repeat: self.horizontal_repeat,
            vertical_repeat: self.vertical_repeat,
        }
    }
}

impl BoxModel {
    pub fn margin_box(&self) -> &LayoutBox2D {
        &self.geometry.margin_box
    }

    pub fn border_box(&self) -> &LayoutBox2D {
        &self.geometry.border_box
    }

    pub fn padding_box(&self) -> &LayoutBox2D {
        &self.geometry.padding_box
    }

    pub fn content_box(&self) -> &LayoutBox2D {
        &self.geometry.content_box
    }
}

impl std::fmt::Debug for BoxModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.border_box())
    }
}

impl ToVisual for BoxModel {
    type VisualType = VisualBoxModel;

    fn to_visual(&self) -> Self::VisualType {
        VisualBoxModel {
            border_box: self.geometry.border_box.to_visual(),
            background: self.background.to_visual(),
            border: self.style.box_model.border.apply(|border| {
                if border.is_visible() {
                    Some(border.to_visual())
                } else {
                    None
                }
            }),
        }
    }
}

struct VisualLayer {
    box_model: BoxModel,
    stack_level: i32,
    content: Arc<LayerContent>,
    child_layers: Vec<Arc<VisualLayer>>,
}

impl VisualLayer {
    fn inspect<T>(&self, write: &mut T, depth: usize) -> std::io::Result<()>
    where
        T: std::io::Write + ?Sized,
    {
        write!(write, "{:indent$}{}\n", "", self, indent = depth)?;

        for layer in self
            .child_layers
            .iter()
            .filter(|layer| layer.stack_level < 0)
        {
            layer.inspect(write, depth + 1)?;
        }

        self.content.inspect(write, depth + 1)?;

        for layer in self
            .child_layers
            .iter()
            .filter(|layer| layer.stack_level >= 0)
        {
            layer.inspect(write, depth + 1)?;
        }

        Ok(())
    }

    fn render<T: VisualRenderer>(&self, renderer: &mut T) {
        let saved_origin = renderer.get_origin();

        let origin = match self.box_model.style.positioning {
            PositioningScheme::Fixed => Zero::zero(),
            _ => saved_origin,
        };

        let box_model = self.box_model.to_visual();

        if box_model.is_visible() {
            // background and borders
            renderer.render_box(box_model);
        }

        // negative layers
        let v = self.box_model.padding_box().min.to_visual().to_vector();
        renderer.set_origin(origin + v);
        for layer in self
            .child_layers
            .iter()
            .filter(|layer| layer.stack_level < 0)
        {
            layer.render(renderer);
        }

        // in-flow boxes
        let v = self.box_model.content_box().min.to_visual().to_vector();
        renderer.set_origin(origin + v);
        self.content.render(renderer);

        // non-negative layers
        let v = self.box_model.padding_box().min.to_visual().to_vector();
        renderer.set_origin(origin + v);
        for layer in self
            .child_layers
            .iter()
            .filter(|layer| layer.stack_level >= 0)
        {
            layer.render(renderer);
        }

        renderer.set_origin(saved_origin);
    }
}

impl std::fmt::Display for VisualLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "layer: {:?}, stack_level={}",
            self.box_model, self.stack_level
        )
    }
}

enum LayerContent {
    Flow(FlowContainer),
    Flex(FlexContainer),
}

impl LayerContent {
    fn new(
        spec: ContainerSpec,
        nodes: &[LayoutNodeRef],
        style: &Style,
        avail: &AvailableSize,
    ) -> Self {
        match spec {
            ContainerSpec::Flow => LayerContent::Flow(FlowContainer::new(nodes, avail)),
            ContainerSpec::Flex => LayerContent::Flex(FlexContainer::new(nodes, style, avail)),
            _ => unreachable!(),
        }
    }

    fn inspect<W>(&self, write: &mut W, depth: usize) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        match self {
            LayerContent::Flow(ref flow) => flow.inspect(write, depth),
            LayerContent::Flex(ref flex) => flex.inspect(write, depth),
        }
    }

    fn render<R>(&self, renderer: &mut R)
    where
        R: VisualRenderer,
    {
        match self {
            LayerContent::Flow(ref flow) => flow.render(renderer),
            LayerContent::Flex(ref flex) => flex.render(renderer),
        }
    }
}

struct BoxConstraintSolver {
    avail: AvailableSize,
    geom: SolvedBoxGeometry,
}

#[derive(Clone, Default)]
struct SolvedBoxGeometry {
    width: LayoutLengthWithRange,
    height: LayoutLengthWithRange,
    padding: BoxQuad<LayoutLength>,
    border: BoxQuad<LayoutLength>,
    margin: BoxQuad<Option<LayoutLength>>,
    offset: BoxQuad<Option<LayoutLength>>,
}

impl SolvedBoxGeometry {
    fn determine(self) -> BoxGeometry {
        let margin = self.margin.apply(|v| v.unwrap());
        let offset = self.offset.apply(|v| v.unwrap());

        let margin_min = LayoutPoint2D::new(offset.left, offset.top);
        let margin_max = margin_min
            + LayoutVector2D::new(
                margin.dw() + self.border.dw() + self.padding.dw() + self.width.value.unwrap(),
                margin.dh() + self.border.dh() + self.padding.dh() + self.height.value.unwrap(),
            );
        let margin_box = LayoutBox2D::new(margin_min, margin_max);
        let border_box =
            margin_box.shrink_edges((margin.left, margin.top, margin.right, margin.bottom));
        let padding_box = border_box.shrink_edges((
            self.border.left,
            self.border.top,
            self.border.right,
            self.border.bottom,
        ));
        let content_box = padding_box.shrink_edges((
            self.padding.left,
            self.padding.top,
            self.padding.right,
            self.padding.bottom,
        ));

        BoxGeometry {
            margin_box,
            border_box,
            padding_box,
            content_box,
        }
    }
}

#[derive(Clone, Default)]
struct LayoutLengthWithRange {
    value: Option<LayoutLength>,
    min: LayoutLength,
    max: LayoutLength,
}

impl LayoutLengthWithRange {
    fn subtract(&mut self, delta: LayoutLength) {
        if delta == LayoutLength::zero() {
            return;
        }
        if let Some(ref mut value) = self.value {
            *value -= delta;
            if *value < LayoutLength::zero() {
                *value = LayoutLength::zero();
            }
        }
        if self.min != LayoutLength::zero() {
            self.min -= delta;
            if self.min < LayoutLength::zero() {
                self.min = LayoutLength::zero();
            }
        }
        if self.max != LayoutLength::max_value() {
            self.max -= delta;
            if self.max < LayoutLength::zero() {
                self.max = LayoutLength::zero();
            }
        }
    }
}

impl BoxConstraintSolver {
    fn new(avail: &AvailableSize) -> Self {
        BoxConstraintSolver {
            avail: avail.clone(),
            geom: Default::default(),
        }
    }

    fn apply_style(&mut self, style: &Style) -> &mut Self {
        self.geom.width.value = style.box_model.width.resolve(&self.avail.width);
        self.geom.width.min = style.box_model.min_width.resolve(&self.avail.width);
        self.geom.width.max = style.box_model.max_width.resolve(&self.avail.width);
        if self.geom.width.max < self.geom.width.min {
            self.geom.width.max = self.geom.width.min;
        }

        self.geom.height.value = style.box_model.height.resolve(&self.avail.height);
        self.geom.height.min = style.box_model.min_height.resolve(&self.avail.height);
        self.geom.height.max = style.box_model.max_height.resolve(&self.avail.height);
        if self.geom.height.max < self.geom.height.min {
            self.geom.height.max = self.geom.height.min;
        }

        self.geom.padding = style.box_model.padding.resolve(&self.avail);
        self.geom.border = style.box_model.border.resolve();
        self.geom.margin = style.box_model.margin.resolve(&self.avail);

        let (dw, dh) = match style.box_model.box_sizing {
            BoxSizing::ContentBox => (LayoutLength::zero(), LayoutLength::zero()),
            BoxSizing::PaddingBox => (self.geom.padding.dw(), self.geom.padding.dh()),
            BoxSizing::BorderBox => (
                self.geom.padding.dw() + self.geom.border.dw(),
                self.geom.padding.dh() + self.geom.border.dh(),
            ),
        };

        self.geom.width.subtract(dw);
        self.geom.height.subtract(dh);

        self.geom.offset = match style.positioning {
            PositioningScheme::Static | PositioningScheme::Relative => box_quad!(
                Some(LayoutLength::zero()),
                Some(LayoutLength::zero()),
                None,
                Some(LayoutLength::zero())
            ),
            _ => style.layer.offset.resolve(&self.avail),
        };

        self
    }

    fn solve_constraints(&mut self) -> &mut Self {
        if let Some(avail_width) = self.avail.width {
            self.solve_horizontal_constraints(
                avail_width - self.geom.border.dw() - self.geom.padding.dw(),
            );
        }
        if let Some(avail_height) = self.avail.height {
            self.solve_vertical_constraints(
                avail_height - self.geom.border.dh() - self.geom.padding.dh(),
            );
        }
        self
    }

    fn solve_horizontal_constraints(&mut self, avail_width: LayoutLength) {
        match (
            self.geom.width.value,
            self.geom.offset.left,
            self.geom.offset.right,
        ) {
            // none of the three is 'auto'
            (Some(width), Some(left), Some(right)) => {
                let remaining = avail_width - width - left - right;
                match (self.geom.margin.left, self.geom.margin.right) {
                    (None, None) => {
                        if remaining < LayoutLength::zero() {
                            // TODO: RTL
                            self.geom.margin.left = Some(LayoutLength::zero());
                            self.geom.margin.right = Some(remaining);
                        } else {
                            // TODO: RTL
                            let half = remaining / 2.0;
                            self.geom.margin.left = Some(half);
                            self.geom.margin.right = Some(remaining - half);
                        }
                    }
                    (None, Some(right_margin)) => {
                        self.geom.margin.left = Some(remaining - right_margin);
                    }
                    (Some(left_margin), None) => {
                        self.geom.margin.right = Some(remaining - left_margin);
                    }
                    (Some(left_margin), Some(right_margin)) => {
                        // over-constrained.
                        // TODO: RTL
                        self.geom.offset.right =
                            Some(right + remaining - left_margin - right_margin);
                    }
                }
            }
            (Some(width), Some(left), None) => {
                let left_margin = *self.geom.margin.left.get_or_insert(LayoutLength::zero());
                let right_margin = *self.geom.margin.right.get_or_insert(LayoutLength::zero());
                self.geom.offset.right =
                    Some(avail_width - width - left - left_margin - right_margin);
            }
            (Some(width), None, Some(right)) => {
                let left_margin = *self.geom.margin.left.get_or_insert(LayoutLength::zero());
                let right_margin = *self.geom.margin.right.get_or_insert(LayoutLength::zero());
                self.geom.offset.left =
                    Some(avail_width - width - right - left_margin - right_margin);
            }
            (Some(width), None, None) => {
                let left_margin = *self.geom.margin.left.get_or_insert(LayoutLength::zero());
                let right_margin = *self.geom.margin.right.get_or_insert(LayoutLength::zero());
                // TODO: static-position, rtl
                let left = *self.geom.offset.left.get_or_insert(LayoutLength::zero());
                self.geom.offset.right =
                    Some(avail_width - width - left - left_margin - right_margin);
            }
            (None, Some(left), Some(right)) => {
                let left_margin = *self.geom.margin.left.get_or_insert(LayoutLength::zero());
                let right_margin = *self.geom.margin.right.get_or_insert(LayoutLength::zero());
                self.geom.width.value =
                    Some(avail_width - left - right - left_margin - right_margin);
            }
            (None, Some(left), None) => {
                let left_margin = *self.geom.margin.left.get_or_insert(LayoutLength::zero());
                let right_margin = *self.geom.margin.right.get_or_insert(LayoutLength::zero());
                // TODO: shrink-to-fit
                let width = *self.geom.width.value.get_or_insert(LayoutLength::zero());
                self.geom.offset.right =
                    Some(avail_width - width - left - left_margin - right_margin);
            }
            (None, None, Some(right)) => {
                let left_margin = *self.geom.margin.left.get_or_insert(LayoutLength::zero());
                let right_margin = *self.geom.margin.right.get_or_insert(LayoutLength::zero());
                // TODO: shrink-to-fit
                let width = *self.geom.width.value.get_or_insert(LayoutLength::zero());
                self.geom.offset.left =
                    Some(avail_width - width - right - left_margin - right_margin);
            }
            (None, None, None) => {
                let left_margin = *self.geom.margin.left.get_or_insert(LayoutLength::zero());
                let right_margin = *self.geom.margin.right.get_or_insert(LayoutLength::zero());
                // TODO: shrink-to-fit
                let width = *self.geom.width.value.get_or_insert(LayoutLength::zero());
                // TODO: static-position, rtl
                let left = *self.geom.offset.left.get_or_insert(LayoutLength::zero());
                self.geom.offset.right =
                    Some(avail_width - width - left - left_margin - right_margin);
            }
        }
    }

    fn solve_vertical_constraints(&mut self, avail_height: LayoutLength) {
        match (
            self.geom.height.value,
            self.geom.offset.top,
            self.geom.offset.bottom,
        ) {
            // none of the three is 'auto'
            (Some(height), Some(top), Some(bottom)) => {
                let remaining = avail_height - height - top - bottom;
                match (self.geom.margin.top, self.geom.margin.bottom) {
                    (None, None) => {
                        if remaining < LayoutLength::zero() {
                            self.geom.margin.top = Some(LayoutLength::zero());
                            self.geom.margin.bottom = Some(remaining);
                        } else {
                            let half = remaining / 2.0;
                            self.geom.margin.top = Some(half);
                            self.geom.margin.bottom = Some(remaining - half);
                        }
                    }
                    (None, Some(bottom_margin)) => {
                        self.geom.margin.top = Some(remaining - bottom_margin);
                    }
                    (Some(top_margin), None) => {
                        self.geom.margin.bottom = Some(remaining - top_margin);
                    }
                    (Some(top_margin), Some(bottom_margin)) => {
                        // over-constrained.
                        self.geom.offset.bottom =
                            Some(bottom + remaining - top_margin - bottom_margin);
                    }
                }
            }
            (Some(height), Some(top), None) => {
                let top_margin = *self.geom.margin.top.get_or_insert(LayoutLength::zero());
                let bottom_margin = *self.geom.margin.bottom.get_or_insert(LayoutLength::zero());
                self.geom.offset.bottom =
                    Some(avail_height - height - top - top_margin - bottom_margin);
            }
            (Some(height), None, Some(bottom)) => {
                let top_margin = *self.geom.margin.top.get_or_insert(LayoutLength::zero());
                let bottom_margin = *self.geom.margin.bottom.get_or_insert(LayoutLength::zero());
                self.geom.offset.top =
                    Some(avail_height - height - bottom - top_margin - bottom_margin);
            }
            (Some(height), None, None) => {
                let top_margin = *self.geom.margin.top.get_or_insert(LayoutLength::zero());
                let bottom_margin = *self.geom.margin.bottom.get_or_insert(LayoutLength::zero());
                // TODO: static-position
                let top = *self.geom.offset.top.get_or_insert(LayoutLength::zero());
                self.geom.offset.bottom =
                    Some(avail_height - height - top - top_margin - bottom_margin);
            }
            (None, Some(top), Some(bottom)) => {
                let top_margin = *self.geom.margin.top.get_or_insert(LayoutLength::zero());
                let bottom_margin = *self.geom.margin.bottom.get_or_insert(LayoutLength::zero());
                self.geom.height.value =
                    Some(avail_height - top - bottom - top_margin - bottom_margin);
            }
            (None, Some(top), None) => {
                let top_margin = *self.geom.margin.top.get_or_insert(LayoutLength::zero());
                let bottom_margin = *self.geom.margin.bottom.get_or_insert(LayoutLength::zero());
                // TODO: shrink-to-fit
                let height = *self.geom.height.value.get_or_insert(LayoutLength::zero());
                self.geom.offset.bottom =
                    Some(avail_height - height - top - top_margin - bottom_margin);
            }
            (None, None, Some(bottom)) => {
                let top_margin = *self.geom.margin.top.get_or_insert(LayoutLength::zero());
                let bottom_margin = *self.geom.margin.bottom.get_or_insert(LayoutLength::zero());
                // TODO: shrink-to-fit
                let height = *self.geom.height.value.get_or_insert(LayoutLength::zero());
                self.geom.offset.top =
                    Some(avail_height - height - bottom - top_margin - bottom_margin);
            }
            (None, None, None) => {
                let top_margin = *self.geom.margin.top.get_or_insert(LayoutLength::zero());
                let bottom_margin = *self.geom.margin.bottom.get_or_insert(LayoutLength::zero());
                // TODO: shrink-to-fit
                let height = *self.geom.height.value.get_or_insert(LayoutLength::zero());
                // TODO: static-position
                let top = *self.geom.offset.top.get_or_insert(LayoutLength::zero());
                self.geom.offset.bottom =
                    Some(avail_height - height - top - top_margin - bottom_margin);
            }
        }
    }
}

// TODO: Inefficient in the memory point of view.
struct VisualLayersMap(BTreeMap<i32, Vec<Arc<VisualLayer>>>);

impl VisualLayersMap {
    fn new() -> Self {
        VisualLayersMap(BTreeMap::new())
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn push_back(&mut self, layer: Arc<VisualLayer>) {
        self.0
            .entry(layer.stack_level)
            .and_modify(|e| e.push(layer.clone()))
            .or_insert_with(|| vec![layer]);
    }

    fn push_front(&mut self, layer: Arc<VisualLayer>) {
        self.0
            .entry(layer.stack_level)
            .and_modify(|e| e.insert(0, layer.clone()))
            .or_insert_with(|| vec![layer]);
    }

    fn merge(&mut self, other: VisualLayersMap) {
        if other.is_empty() {
            return;
        }
        for (stack_level, mut layers) in other.0.into_iter() {
            self.0
                .entry(stack_level)
                .and_modify(|e| e.append(&mut layers))
                .or_insert(layers);
        }
    }

    fn into_vec(self) -> Vec<Arc<VisualLayer>> {
        // TODO: self.0.into_values().collect()
        let mut result = vec![];
        for (_, mut layers) in self.0.into_iter() {
            result.append(&mut layers);
        }
        result
    }
}
