use std::ops::Add;

use num_traits::Zero;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Number;
use crate::Length;
use crate::MAX_LENGTH;

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Style {
    pub display: DisplayStyle,
    pub positioning: PositioningScheme,
    pub box_model: BoxModelStyle,
    pub background: BackgroundStyle,
    pub layer: LayerStyle,
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
    Widget,
    Flex,
    Grid,
    Ruby,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum PositioningScheme {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Default for PositioningScheme {
    fn default() -> Self {
        PositioningScheme::Static
    }
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BoxSizing {
    ContentBox,
    BorderBox,
    PaddingBox,
}

impl Default for BoxSizing {
    fn default() -> Self {
        BoxSizing::ContentBox
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum NumericSize {
    Pixel(Length),
    Scale(Number),
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ContentSize {
    Auto,
    MaxContent,
    MinContent,
    FitContent(NumericSize),
    Pixel(Length),
    Scale(Number),
    Calc(String),  // TODO: Fn
}

impl ContentSize {
    #[inline]
    pub fn resolve(&self, base: &Option<Length>) -> Option<Length> {
        match (self, *base) {
            (ContentSize::Pixel(px), _) => Some(*px),
            (ContentSize::Scale(scale), Some(base)) => Some(base * *scale),
            _ => None,
        }
    }
}

impl Default for ContentSize {
    fn default() -> Self {
        ContentSize::Auto
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
    Pixel(Length),
    Scale(Number),
    Calc(String),  // TODO: Fn
}

impl ContentMinSize {
    #[inline]
    pub fn resolve(&self, base: &Option<Length>) -> Length {
        match (self, *base) {
            (ContentMinSize::Pixel(px), _) => *px,
            (ContentMinSize::Scale(scale), Some(base)) => base * *scale,
            _ => Length::zero(),
        }
    }
}

impl Default for ContentMinSize {
    fn default() -> Self {
        ContentMinSize::Pixel(Default::default())
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ContentMaxSize {
    None,
    Pixel(Length),
    Scale(Number),
    Calc(String),  // TODO: Fn
}

impl ContentMaxSize {
    #[inline]
    pub fn resolve(&self, base: &Option<Length>) -> Length {
        match (self, *base) {
            (ContentMaxSize::Pixel(px), _) => *px,
            (ContentMaxSize::Scale(scale), Some(base)) => base * *scale,
            _ => MAX_LENGTH,
        }
    }
}

impl Default for ContentMaxSize {
    fn default() -> Self {
        ContentMaxSize::None
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Padding {
    Pixel(Length),
    Scale(Number),
    Calc(String),  // TODO: Fn
}

impl Padding {
    #[inline]
    pub fn resolve(&self, base: &Option<Length>) -> Length {
        match (self, *base) {
            (Padding::Pixel(px), _) => *px,
            (Padding::Scale(scale), Some(base)) => base * *scale,
            _ => Length::zero(),
        }
    }
}

impl Default for Padding {
    fn default() -> Self {
        Padding::Pixel(Default::default())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BorderStyle {
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
        match *self {
            BorderStyle::None | BorderStyle::Hidden => false,
            _ => true,
        }
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        BorderStyle::None
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
    pub width: Length,
    pub color: Color,
}

impl Border {
    #[inline]
    pub fn is_visible(&self) -> bool {
        self.style.is_visible() && self.width > Length::zero() && !self.color.is_transparent()
    }

    #[inline]
    pub fn resolve(&self) -> Length {
        match self.style {
            BorderStyle::None => Length::zero(),
            _ => self.width,
        }
    }
}

impl BoxQuad<Border> {
    pub fn is_visible(&self) -> bool {
        self.any(|border| border.is_visible())
    }

    pub fn widths(&self) -> BoxQuad<Length> {
        self.apply(|border| border.width)
    }
}

impl Into<(Length, Length, Length, Length)> for BoxQuad<Border> {
    fn into(self) -> (Length, Length, Length, Length) {
        (self.top.width, self.right.width, self.bottom.width, self.left.width)
    }
}

impl std::fmt::Display for Border {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {}", self.width, self.color)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl Default for Color {
    fn default() -> Self {
        Color(0, 0, 0, 0)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.red(), self.green(), self.blue(), self.alpha())
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Margin {
    Auto,
    Pixel(Length),
    Scale(Number),
    Calc(String),  // TODO: Fn
}

impl Margin {
    #[inline]
    pub fn resolve(&self, base: &Option<Length>) -> Option<Length> {
        match (self, *base) {
            (Margin::Pixel(px), _) => Some(*px),
            (Margin::Scale(scale), Some(base)) => Some(base * *scale),
            _ => None,
        }
    }
}

impl Default for Margin {
    fn default() -> Self {
        Margin::Auto
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
        BoxQuad { top, right, bottom, left }
    }

    pub fn any<F>(&self, f: F) -> bool
    where
        F: Fn(&T) -> bool
    {
        f(&self.top) || f(&self.right) || f(&self.bottom) || f(&self.left)
    }

    pub fn all<F>(&self, f: F) -> bool
    where
        F: Fn(&T) -> bool
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
        write!(f, "[{}, {}, {}, {}]", self.top, self.right, self.bottom, self.left)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for BoxQuad<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}, {:?}, {:?}, {:?}]", self.top, self.right, self.bottom, self.left)
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
        BoxQuad::new($top, $right, $bottom,  $left)
    };
}

impl BoxQuad<Padding> {
    pub fn resolve(&self, avail: &AvailableSize) -> BoxQuad<Length> {
        self.apply(|padding| padding.resolve(&avail.width))
    }
}

impl BoxQuad<Border> {
    pub fn resolve(&self) -> BoxQuad<Length> {
        self.apply(|border| border.resolve())
    }
}

impl BoxQuad<Margin> {
    pub fn resolve(&self, avail: &AvailableSize) -> BoxQuad<Option<Length>> {
        self.apply(|margin| margin.resolve(&avail.width))
    }
}

impl BoxQuad<LayerOffset> {
    pub fn resolve(&self, avail: &AvailableSize) -> BoxQuad<Option<Length>> {
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
    pub media: VisualMedia,
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
#[cfg_attr(feature = "serde", serde(default))]
pub struct VisualMedia {
    // TODO: id
    pub size: VisualMediaSize,  // natural size is required for computing the box size
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum VisualMediaSize {
    Pixel(Length, Length),
    Ratio(Number, Number),
}

impl Default for VisualMediaSize {
    fn default() -> Self {
        VisualMediaSize::Pixel(Length::new(300.0), Length::new(150.0))
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BackgroundAttachment {
    Fixed,
    Local,
    Scroll,
}

impl Default for BackgroundAttachment {
    fn default() -> Self {
        BackgroundAttachment::Scroll
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BackgroundClip {
    BorderBox,
    PaddingBox,
    ContentBox,
    Text,
}

impl Default for BackgroundClip {
    fn default() -> Self {
        BackgroundClip::BorderBox
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BackgroundOrigin {
    BorderBox,
    PaddingBox,
    ContentBox,
}

impl Default for BackgroundOrigin {
    fn default() -> Self {
        BackgroundOrigin::PaddingBox
    }
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

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BackgroundRepeat {
    Repeat,
    Space,
    Round,
    NoRepeat,
}

impl Default for BackgroundRepeat {
    fn default() -> Self {
        BackgroundRepeat::Repeat
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum BackgroundSize {
    Auto,
    Contain,
    Cover,
    Pixel(Length),
    Scale(Number),
}

impl Default for BackgroundSize {
    fn default() -> Self {
        BackgroundSize::Auto
    }
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct LayerStyle {
    pub offset: BoxQuad<LayerOffset>,
    pub z_index: LayerZIndex,
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum LayerOffset {
    Auto,
    Pixel(Length),
    Scale(Number),
    Calc(String),  // TODO: Fn
}

impl LayerOffset {
    #[inline]
    pub fn resolve(&self, base: &Option<Length>) -> Option<Length> {
        match (self, *base) {
            (LayerOffset::Pixel(px), _) => Some(*px),
            (LayerOffset::Scale(scale), Some(base)) => Some(base * *scale),
            _ => None,
        }
    }
}

impl Default for LayerOffset {
    fn default() -> Self {
        LayerOffset::Auto
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum LayerZIndex {
    Auto,
    Index(i32),
}

impl Default for LayerZIndex {
    fn default() -> Self {
        LayerZIndex::Auto
    }
}

#[derive(Clone)]
pub struct AvailableSize {
    pub width: Option<Length>,
    pub height: Option<Length>,
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
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

            let result: BorderStyle =
                assert_matches!(serde_json::from_str(&pair.1), Ok(v) => v);
            assert_eq!(result, pair.0);
        }
    }

    #[test]
    fn test_border_serde() {
        let border = Border {
            style: BorderStyle::Solid,
            width: Length::new(10.0),
            color: Color(0, 0, 0, 0),
        };

        let json = r#"{"style":"solid","width":10.0,"color":[0,0,0,0]}"#;

        let result = assert_matches!(serde_json::to_string(&border), Ok(v) => v);
        assert_eq!(result, json);

        let result: Border = assert_matches!(serde_json::from_str(&json), Ok(v) => v);
        assert_eq!(result, border);
    }

    #[test]
    fn test_border_quad_serde() {
        let value = box_quad!(Border {
            style: BorderStyle::Solid,
            width: Length::new(10.0),
            color: Color(0, 0, 0, 0),
        });

        let json = r#"[{"style":"solid","width":10.0,"color":[0,0,0,0]},{"style":"solid","width":10.0,"color":[0,0,0,0]},{"style":"solid","width":10.0,"color":[0,0,0,0]},{"style":"solid","width":10.0,"color":[0,0,0,0]}]"#;

        let result = assert_matches!(serde_json::to_string(&value), Ok(v) => v);
        assert_eq!(result, json);

        let result: BoxQuad<Border> = assert_matches!(serde_json::from_str(&json), Ok(v) => v);
        assert_eq!(result, value);
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde::ser::SerializeTuple;

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
