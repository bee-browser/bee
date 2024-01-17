use std::ops::Add;

use num_traits::{Bounded, Zero};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Decimal;
use crate::Integer;
use crate::LayoutLength;

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Style {
    pub display: DisplayStyle,
    pub positioning: PositioningScheme,
    pub box_model: BoxModelStyle,
    pub background: BackgroundStyle,
    pub layer: LayerStyle,
    pub flex: FlexStyle,
    pub content: Option<ContentStyle>,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DisplayStyle {
    pub outside: DisplayOutside,
    pub inside: DisplayInside,
}

impl Default for DisplayStyle {
    fn default() -> Self {
        DisplayStyle {
            outside: DisplayOutside::None,
            inside: DisplayInside::None,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum DisplayOutside {
    None,
    Inline,
    Block,
    TableCaption,
    TableHeaderGroup,
    TableFooterGroup,
    TableRowGroup,
    TableRow,
    TableColumnGroup,
    TableColumn,
    TableCell,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum DisplayInside {
    None,
    Flow,
    FlowRoot,
    Table,
    TableRowGroup,
    TableRow,
    Canvas,
    Flex,
    Grid,
    Ruby,
}

#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum PositioningScheme {
    #[default]
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct BoxModelStyle {
    pub box_sizing: BoxSizing,
    pub width: ContentSize,
    pub min_width: ContentMinSize,
    pub max_width: ContentMaxSize,
    pub height: ContentSize,
    pub min_height: ContentMinSize,
    pub max_height: ContentMaxSize,
    pub padding: BoxQuad<Padding>,
    pub border: BoxQuad<Border>,
    pub margin: BoxQuad<Margin>,
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BoxSizing {
    #[default]
    ContentBox,
    BorderBox,
    PaddingBox,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum NumericSize {
    Pixel(LayoutLength),
    Scale(Decimal),
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ContentSize {
    #[default]
    Auto,
    MaxContent,
    MinContent,
    FitContent(NumericSize),
    Pixel(LayoutLength),
    Scale(Decimal),
    Calc(String), // TODO: Fn
}

impl ContentSize {
    #[inline]
    pub fn resolve(&self, base: &Option<LayoutLength>) -> Option<LayoutLength> {
        match (self, *base) {
            (ContentSize::Pixel(px), _) => Some(*px),
            (ContentSize::Scale(scale), Some(base)) => Some(base * *scale),
            _ => None,
        }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ContentMinSize {
    None,
    Auto,
    MaxContent,
    MinContent,
    FitContent(NumericSize),
    Pixel(LayoutLength),
    Scale(Decimal),
    Calc(String), // TODO: Fn
}

impl ContentMinSize {
    #[inline]
    pub fn resolve(&self, base: &Option<LayoutLength>) -> LayoutLength {
        match (self, *base) {
            (ContentMinSize::Pixel(px), _) => *px,
            (ContentMinSize::Scale(scale), Some(base)) => base * *scale,
            _ => LayoutLength::zero(),
        }
    }
}

impl Default for ContentMinSize {
    fn default() -> Self {
        ContentMinSize::Pixel(Default::default())
    }
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ContentMaxSize {
    #[default]
    None,
    Pixel(LayoutLength),
    Scale(Decimal),
    Calc(String), // TODO: Fn
}

impl ContentMaxSize {
    #[inline]
    pub fn resolve(&self, base: &Option<LayoutLength>) -> LayoutLength {
        match (self, *base) {
            (ContentMaxSize::Pixel(px), _) => *px,
            (ContentMaxSize::Scale(scale), Some(base)) => base * *scale,
            _ => LayoutLength::max_value(),
        }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Padding {
    Pixel(LayoutLength),
    Scale(Decimal),
    Calc(String), // TODO: Fn
}

impl Padding {
    #[inline]
    pub fn resolve(&self, base: &Option<LayoutLength>) -> LayoutLength {
        match (self, *base) {
            (Padding::Pixel(px), _) => *px,
            (Padding::Scale(scale), Some(base)) => base * *scale,
            _ => LayoutLength::zero(),
        }
    }
}

impl Default for Padding {
    fn default() -> Self {
        Padding::Pixel(Default::default())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BorderStyle {
    #[default]
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl BorderStyle {
    #[inline]
    pub fn is_visible(&self) -> bool {
        !matches!(self, BorderStyle::None | BorderStyle::Hidden)
    }
}

impl std::fmt::Display for BorderStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::None => write!(f, "none"),
            Self::Hidden => write!(f, "hidden"),
            Self::Dotted => write!(f, "dotted"),
            Self::Dashed => write!(f, "dashed"),
            Self::Solid => write!(f, "solid"),
            Self::Double => write!(f, "double"),
            Self::Groove => write!(f, "groove"),
            Self::Ridge => write!(f, "ridge"),
            Self::Inset => write!(f, "inset"),
            Self::Outset => write!(f, "outset"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Border {
    pub style: BorderStyle,
    pub width: LayoutLength,
    pub color: Color,
}

impl Border {
    #[inline]
    pub fn is_visible(&self) -> bool {
        self.style.is_visible() && self.width > LayoutLength::zero() && !self.color.is_transparent()
    }

    #[inline]
    pub fn resolve(&self) -> LayoutLength {
        match self.style {
            BorderStyle::None => LayoutLength::zero(),
            _ => self.width,
        }
    }
}

impl BoxQuad<Border> {
    pub fn is_visible(&self) -> bool {
        self.any(|border| border.is_visible())
    }

    pub fn widths(&self) -> BoxQuad<LayoutLength> {
        self.apply(|border| border.width)
    }
}

impl From<BoxQuad<Border>> for (LayoutLength, LayoutLength, LayoutLength, LayoutLength) {
    fn from(val: BoxQuad<Border>) -> Self {
        (
            val.top.width,
            val.right.width,
            val.bottom.width,
            val.left.width,
        )
    }
}

impl std::fmt::Display for Border {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {}", self.width, self.color)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Color(u8, u8, u8, u8);

impl Color {
    #[inline]
    pub fn red(&self) -> u8 {
        self.0
    }

    #[inline]
    pub fn green(&self) -> u8 {
        self.1
    }

    #[inline]
    pub fn blue(&self) -> u8 {
        self.2
    }

    #[inline]
    pub fn alpha(&self) -> u8 {
        self.3
    }

    pub fn is_transparent(&self) -> bool {
        self.alpha() == 0
    }

    pub fn is_opaque(&self) -> bool {
        self.alpha() == 255
    }

    pub fn is_translucent(&self) -> bool {
        !self.is_transparent() && !self.is_opaque()
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "rgba({}, {}, {}, {})",
            self.red(),
            self.green(),
            self.blue(),
            self.alpha()
        )
    }
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Margin {
    #[default]
    Auto,
    Pixel(LayoutLength),
    Scale(Decimal),
    Calc(String), // TODO: Fn
}

impl Margin {
    #[inline]
    pub fn resolve(&self, base: &Option<LayoutLength>) -> Option<LayoutLength> {
        match (self, *base) {
            (Margin::Pixel(px), _) => Some(*px),
            (Margin::Scale(scale), Some(base)) => Some(base * *scale),
            _ => None,
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct BoxQuad<T> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

impl<T> BoxQuad<T> {
    pub fn new(top: T, right: T, bottom: T, left: T) -> Self {
        BoxQuad {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn any<F>(&self, f: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        f(&self.top) || f(&self.right) || f(&self.bottom) || f(&self.left)
    }

    pub fn all<F>(&self, f: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        f(&self.top) && f(&self.right) && f(&self.bottom) && f(&self.left)
    }

    pub fn apply<B, F>(&self, f: F) -> BoxQuad<B>
    where
        F: Fn(&T) -> B,
    {
        BoxQuad::new(f(&self.top), f(&self.right), f(&self.bottom), f(&self.left))
    }
}

impl<T: Clone> From<T> for BoxQuad<T> {
    fn from(v: T) -> Self {
        BoxQuad::new(v.clone(), v.clone(), v.clone(), v.clone())
    }
}

impl<T> From<(T, T, T, T)> for BoxQuad<T> {
    fn from(quad: (T, T, T, T)) -> Self {
        BoxQuad::new(quad.0, quad.1, quad.2, quad.3)
    }
}

impl<T: Copy + Add<Output = T>> BoxQuad<T> {
    pub fn dw(&self) -> T {
        self.left + self.right
    }

    pub fn dh(&self) -> T {
        self.top + self.bottom
    }
}

impl<T: std::fmt::Display> std::fmt::Display for BoxQuad<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}]",
            self.top, self.right, self.bottom, self.left
        )
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for BoxQuad<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:?}, {:?}, {:?}, {:?}]",
            self.top, self.right, self.bottom, self.left
        )
    }
}

#[macro_export]
macro_rules! box_quad {
    ($v:expr) => {
        BoxQuad::new($v.clone(), $v.clone(), $v.clone(), $v.clone())
    };
    ($vert:expr, $hori:expr) => {
        BoxQuad::new($vert.clone(), $hori.clone(), $vert.clone(), $hori.clone())
    };
    ($top:expr, $right:expr, $bottom:expr, $left:expr) => {
        BoxQuad::new($top, $right, $bottom, $left)
    };
}

impl BoxQuad<Padding> {
    pub fn resolve(&self, avail: &AvailableSize) -> BoxQuad<LayoutLength> {
        self.apply(|padding| padding.resolve(&avail.width))
    }
}

impl BoxQuad<Border> {
    pub fn resolve(&self) -> BoxQuad<LayoutLength> {
        self.apply(|border| border.resolve())
    }
}

impl BoxQuad<Margin> {
    pub fn resolve(&self, avail: &AvailableSize) -> BoxQuad<Option<LayoutLength>> {
        self.apply(|margin| margin.resolve(&avail.width))
    }
}

impl BoxQuad<LayerOffset> {
    pub fn resolve(&self, avail: &AvailableSize) -> BoxQuad<Option<LayoutLength>> {
        BoxQuad::new(
            self.top.resolve(&avail.height),
            self.right.resolve(&avail.width),
            self.bottom.resolve(&avail.height),
            self.left.resolve(&avail.width),
        )
    }
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct BackgroundStyle {
    pub color: Color,
    pub images: Vec<BackgroundImage>,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct BackgroundImage {
    pub asset: Asset,
    pub attachment: BackgroundAttachment,
    pub clip: BackgroundClip,
    pub origin: BackgroundOrigin,
    pub position_x: BackgroundPosition,
    pub position_y: BackgroundPosition,
    pub repeat_x: BackgroundRepeat,
    pub repeat_y: BackgroundRepeat,
    pub width: BackgroundSize,
    pub height: BackgroundSize,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BackgroundAttachment {
    Fixed,
    Local,
    #[default]
    Scroll,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BackgroundClip {
    #[default]
    BorderBox,
    PaddingBox,
    ContentBox,
    Text,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BackgroundOrigin {
    BorderBox,
    #[default]
    PaddingBox,
    ContentBox,
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BackgroundPosition {
    Start(NumericSize),
    End(NumericSize),
}

impl Default for BackgroundPosition {
    fn default() -> Self {
        BackgroundPosition::Start(NumericSize::Pixel(Default::default()))
    }
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BackgroundRepeat {
    #[default]
    Repeat,
    Space,
    Round,
    NoRepeat,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BackgroundSize {
    #[default]
    Auto,
    Contain,
    Cover,
    Pixel(LayoutLength),
    Scale(Decimal),
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct LayerStyle {
    pub offset: BoxQuad<LayerOffset>,
    pub z_index: LayerZIndex,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum LayerOffset {
    #[default]
    Auto,
    Pixel(LayoutLength),
    Scale(Decimal),
    Calc(String), // TODO: Fn
}

impl LayerOffset {
    #[inline]
    pub fn resolve(&self, base: &Option<LayoutLength>) -> Option<LayoutLength> {
        match (self, *base) {
            (LayerOffset::Pixel(px), _) => Some(*px),
            (LayerOffset::Scale(scale), Some(base)) => Some(base * *scale),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum LayerZIndex {
    #[default]
    Auto,
    Index(i32),
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct FlexStyle {
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
    pub order: Integer,
    pub grow: Decimal,
    pub shrink: Decimal,
    pub basis: FlexBasis,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub align_content: AlignContent,
}

#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum FlexDirection {
    #[default]
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum FlexWrap {
    #[default]
    Nowrap,
    Wrap,
    WrapReverse,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum FlexBasis {
    #[default]
    Auto,
    Content,
    Pixel(LayoutLength),
    Scale(Decimal),
    Calc(String), // TODO: Fn
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum JustifyContent {
    #[default]
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    #[default]
    Stretch,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum AlignSelf {
    #[default]
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    #[default]
    Stretch,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct ContentStyle {
    pub asset: Asset,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Asset {
    pub id: u64,
    pub size: AssetSize, // natural size is required for computing the box size
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum AssetSize {
    Pixel(LayoutLength, LayoutLength),
    Ratio(Decimal, Decimal),
}

impl Default for AssetSize {
    fn default() -> Self {
        Self::Pixel(LayoutLength::new(300.0), LayoutLength::new(150.0))
    }
}

#[derive(Clone)]
pub struct AvailableSize {
    pub width: Option<LayoutLength>,
    pub height: Option<LayoutLength>,
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_border_style_serde() {
        let pairs = [
            (BorderStyle::None, r#""none""#),
            (BorderStyle::Hidden, r#""hidden""#),
            (BorderStyle::Dotted, r#""dotted""#),
            (BorderStyle::Dashed, r#""dashed""#),
            (BorderStyle::Solid, r#""solid""#),
            (BorderStyle::Double, r#""double""#),
            (BorderStyle::Groove, r#""groove""#),
            (BorderStyle::Ridge, r#""ridge""#),
            (BorderStyle::Inset, r#""inset""#),
            (BorderStyle::Outset, r#""outset""#),
        ];
        for pair in pairs.iter() {
            let result = assert_matches!(serde_json::to_string(&pair.0), Ok(v) => v);
            assert_eq!(result, pair.1);

            let result: BorderStyle = assert_matches!(serde_json::from_str(pair.1), Ok(v) => v);
            assert_eq!(result, pair.0);
        }
    }

    #[test]
    fn test_border_serde() {
        let border = Border {
            style: BorderStyle::Solid,
            width: LayoutLength::new(10.0),
            color: Color(0, 0, 0, 0),
        };

        let json = r#"{"style":"solid","width":10.0,"color":[0,0,0,0]}"#;

        let result = assert_matches!(serde_json::to_string(&border), Ok(v) => v);
        assert_eq!(result, json);

        let result: Border = assert_matches!(serde_json::from_str(json), Ok(v) => v);
        assert_eq!(result, border);
    }

    #[test]
    fn test_border_quad_serde() {
        let value = box_quad!(Border {
            style: BorderStyle::Solid,
            width: LayoutLength::new(10.0),
            color: Color(0, 0, 0, 0),
        });

        let json = r#"[{"style":"solid","width":10.0,"color":[0,0,0,0]},{"style":"solid","width":10.0,"color":[0,0,0,0]},{"style":"solid","width":10.0,"color":[0,0,0,0]},{"style":"solid","width":10.0,"color":[0,0,0,0]}]"#;

        let result = assert_matches!(serde_json::to_string(&value), Ok(v) => v);
        assert_eq!(result, json);

        let result: BoxQuad<Border> = assert_matches!(serde_json::from_str(json), Ok(v) => v);
        assert_eq!(result, value);
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::ser::SerializeTuple;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T> Serialize for BoxQuad<T>
    where
        T: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut tup = serializer.serialize_tuple(4)?;
            tup.serialize_element(&self.top)?;
            tup.serialize_element(&self.right)?;
            tup.serialize_element(&self.bottom)?;
            tup.serialize_element(&self.left)?;
            tup.end()
        }
    }

    impl<'de, T> Deserialize<'de> for BoxQuad<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let quad: (T, T, T, T) = Deserialize::deserialize(deserializer)?;
            Ok(quad.into())
        }
    }
}
