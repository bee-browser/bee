mod box2d;
mod length;
mod point2d;
mod rect;
mod size2d;
mod vector2d;

pub use crate::box2d::Box2D;
pub use crate::length::Length;
pub use crate::point2d::Point2D;
pub use crate::rect::Rect;
pub use crate::size2d::Size2D;
pub use crate::vector2d::Vector2D;

type Quad<T> = (T, T, T, T);  // internal quad type to make developer's life easier
