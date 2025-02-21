use core::ops::{Add, Sub};

use num_traits::Zero;

use crate::Box2D;
use crate::Length;
use crate::Point2D;
use crate::Quad;
use crate::Size2D;
use crate::Vector2D;

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Rect<T, U> {
    pub origin: Point2D<T, U>,
    pub size: Size2D<T, U>,
}

impl<T, U> Rect<T, U> {
    pub fn new(origin: Point2D<T, U>, size: Size2D<T, U>) -> Self {
        Rect { origin, size }
    }
}

impl<T: PartialOrd + Zero, U> Rect<T, U> {
    pub fn is_empty(&self) -> bool {
        self.size.is_empty()
    }
}

impl<T: Copy + Add<Output = T>, U> Rect<T, U> {
    pub fn translate(&self, v: Vector2D<T, U>) -> Self {
        Rect::new(self.origin + v, self.size.clone())
    }
}

impl<T: Clone, U> Clone for Rect<T, U> {
    fn clone(&self) -> Self {
        Rect::new(self.origin.clone(), self.size.clone())
    }
}

impl<T: Default, U> Default for Rect<T, U> {
    fn default() -> Self {
        Rect::new(Default::default(), Default::default())
    }
}

impl<T, U> From<Quad<T>> for Rect<T, U> {
    fn from(quad: Quad<T>) -> Self {
        Rect::new((quad.0, quad.1).into(), (quad.2, quad.3).into())
    }
}

impl<T, U> From<Quad<Length<T, U>>> for Rect<T, U> {
    fn from(quad: Quad<Length<T, U>>) -> Self {
        Rect::new((quad.0, quad.1).into(), (quad.2, quad.3).into())
    }
}

impl<T: Copy + Sub<Output = T>, U> From<Box2D<T, U>> for Rect<T, U> {
    fn from(box2d: Box2D<T, U>) -> Self {
        Rect::new(box2d.min, box2d.size())
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T, U> Serialize for Rect<T, U>
    where
        T: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            (
                &self.origin.x,
                &self.origin.y,
                &self.size.width,
                &self.size.height,
            )
                .serialize(serializer)
        }
    }

    impl<'de, T, U> Deserialize<'de> for Rect<T, U>
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
        use super::Rect;

        use serde_test::{Token, assert_tokens};

        // TODO: remove PartialEq
        #[derive(Debug, PartialEq)]
        enum Px {}

        #[test]
        fn test_serde() {
            let v: Rect<i32, Px> = (0, 1, 2, 3).into();
            assert_tokens(
                &v,
                &[
                    Token::Tuple { len: 4 },
                    Token::I32(0),
                    Token::I32(1),
                    Token::I32(2),
                    Token::I32(3),
                    Token::TupleEnd,
                ],
            );
        }
    }
}
