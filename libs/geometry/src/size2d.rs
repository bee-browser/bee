use num_traits::Zero;

use crate::Length;

#[cfg_attr(test, derive(Debug))]
pub struct Size2D<T, U> {
    pub width: Length<T, U>,
    pub height: Length<T, U>,
}

impl<T, U> Size2D<T, U> {
    pub fn new(width: Length<T, U>, height: Length<T, U>) -> Self {
        Size2D { width, height }
    }
}

impl<T: PartialOrd + Zero, U> Size2D<T, U> {
    pub fn empty() -> Self {
        Size2D::new(Length::zero(), Length::zero())
    }

    pub fn is_empty(&self) -> bool {
        self.width <= Length::zero() || self.height <= Length::zero()
    }

    pub fn set_empty(&mut self) {
        self.width.set_zero();
        self.height.set_zero();
    }
}

impl<T, U> Default for Size2D<T, U>
where
    T: Default,
{
    fn default() -> Self {
        Size2D::new(Default::default(), Default::default())
    }
}

impl<T: Clone, U> Clone for Size2D<T, U> {
    fn clone(&self) -> Self {
        Size2D::new(self.width.clone(), self.height.clone())
    }
}

impl<T, U> From<(T, T)> for Size2D<T, U> {
    fn from(tuple: (T, T)) -> Self {
        Size2D::new(tuple.0.into(), tuple.1.into())
    }
}

impl<T, U> From<(Length<T, U>, Length<T, U>)> for Size2D<T, U> {
    fn from(tuple: (Length<T, U>, Length<T, U>)) -> Self {
        Size2D::new(tuple.0, tuple.1)
    }
}

impl<T: PartialEq, U> PartialEq for Size2D<T, U> {
    fn eq(&self, rhs: &Self) -> bool {
        self.width == rhs.width && self.height == rhs.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]  // TODO: remove
    struct Px;

    #[test]
    fn test_clone() {
        let expected: Size2D<i32, Px> = Size2D::new(1.into(), 2.into());
        assert_eq!(expected, expected.clone());
    }

    fn test_from() {
        let expected: Size2D<i32, Px> = Size2D::new(1.into(), 2.into());
        assert_eq!(expected, (1, 2).into());
        assert_eq!(expected, (Length::new(1), Length::new(2)).into());
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T, U> Serialize for Size2D<T, U>
    where
        T: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            (&self.width, &self.height).serialize(serializer)
        }
    }

    impl<'de, T, U> Deserialize<'de> for Size2D<T, U>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let tuple: (T, T) = Deserialize::deserialize(deserializer)?;
            Ok(tuple.into())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use serde_test::{Token, assert_tokens};

        // TODO: remove PartialEq
        #[derive(Debug, PartialEq)]
        enum Px {}

        #[test]
        fn test_serde() {
            let v: Size2D<i32, Px> = (0, 1).into();
            assert_tokens(&v, &[
                Token::Tuple { len: 2 },
                Token::I32(0),
                Token::I32(1),
                Token::TupleEnd,
            ]);
        }
    }
}
