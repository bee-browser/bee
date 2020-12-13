use core::ops::{Add, Sub};

use num_traits::Zero;

use crate::Length;
use crate::Point2D;
use crate::Quad;
use crate::Size2D;
use crate::Vector2D;

#[derive(Debug)]
pub struct Box2D<T, U> {
    pub min: Point2D<T, U>,
    pub max: Point2D<T, U>,
}

impl<T, U> Box2D<T, U> {
    pub fn new(min: Point2D<T, U>, max: Point2D<T, U>) -> Self {
        Box2D { min, max }
    }
}

impl<T: PartialOrd + Zero, U> Box2D<T, U> {
    pub fn empty() -> Self {
        Box2D::new(Point2D::origin(), Point2D::origin())
    }

    pub fn is_empty(&self) -> bool {
        self.min.x >= self.max.x || self.min.y >= self.max.y
    }
}

impl<T: Copy + Add<Output = T>, U> Box2D<T, U> {
    pub fn translate(&self, v: Vector2D<T, U>) -> Self {
        Box2D::new(self.min + v, self.max + v)
    }
}

impl<T: Copy + Sub<Output = T>, U> Box2D<T, U> {
    pub fn width(&self) -> Length<T, U> {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> Length<T, U> {
        self.max.y - self.min.y
    }

    pub fn size(&self) -> Size2D<T, U> {
        Size2D::new(self.width(), self.height())
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>, U> Box2D<T, U> {
    pub fn shrink_edges(&self, quad: Quad<Length<T, U>>) -> Self {
        (self.min.x + quad.0, self.min.y + quad.1,
         self.max.x - quad.2, self.max.y - quad.3).into()
    }

    pub fn expand_edges(&self, quad: Quad<Length<T, U>>) -> Self {
        (self.min.x - quad.0, self.min.y - quad.1,
         self.max.x + quad.2, self.max.y + quad.3).into()
    }
}

impl<T: Default, U> Default for Box2D<T, U> {
    fn default() -> Self {
        Box2D::new(Default::default(), Default::default())
    }
}

impl<T: Clone, U> Clone for Box2D<T, U> {
    fn clone(&self) -> Self {
        Box2D::new(self.min.clone(), self.max.clone())
    }
}

impl<T, U> From<Quad<T>> for Box2D<T, U> {
    fn from(quad: Quad<T>) -> Self {
        Box2D::new((quad.0, quad.1).into(), (quad.2, quad.3).into())
    }
}

impl<T, U> From<Quad<Length<T, U>>> for Box2D<T, U> {
    fn from(quad: Quad<Length<T, U>>) -> Self {
        Box2D::new((quad.0, quad.1).into(), (quad.2, quad.3).into())
    }
}

impl<T: PartialEq, U> PartialEq for Box2D<T, U> {
    fn eq(&self, rhs: &Self) -> bool {
        self.min == rhs.min && self.max == rhs.max
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T, U> Serialize for Box2D<T, U>
    where
        T: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            (&self.min.x, &self.min.y, &self.max.x, &self.max.y).serialize(serializer)
        }
    }

    impl<'de, T, U> Deserialize<'de> for Box2D<T, U>
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

    #[cfg(test)]
    mod tests {
        use super::Box2D;

        use serde_test::{Token, assert_tokens};

        // TODO: remove PartialEq
        #[derive(Debug, PartialEq)]
        enum Px {}

        #[test]
        fn test_serde() {
            let v: Box2D<i32, Px> = (0, 1, 2, 3).into();
            assert_tokens(&v, &[
                Token::Tuple { len: 4 },
                Token::I32(0),
                Token::I32(1),
                Token::I32(2),
                Token::I32(3),
                Token::TupleEnd,
            ]);
        }
    }
}
