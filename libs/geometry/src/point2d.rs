use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

use num_traits::Zero;

use crate::length::Length;
use crate::vector2d::Vector2D;

#[derive(Debug)]
pub struct Point2D<T, U> {
    pub x: Length<T, U>,
    pub y: Length<T, U>,
}

impl<T, U> Point2D<T, U> {
    pub const fn new(x: Length<T, U>, y: Length<T, U>) -> Self {
        Point2D { x, y }
    }
}

impl<T: Clone, U> Point2D<T, U> {
    pub fn to_vector(&self) -> Vector2D<T, U> {
        Vector2D::new(self.x.clone(), self.y.clone())
    }
}

impl<T: Default, U> Default for Point2D<T, U> {
    fn default() -> Self {
        Point2D::new(Default::default(), Default::default())
    }
}

impl<T: Clone, U> Clone for Point2D<T, U> {
    fn clone(&self) -> Self {
        Point2D::new(self.x.clone(), self.y.clone())
    }
}

impl<T: Copy, U> Copy for Point2D<T, U> {}

impl<T, U> From<(T, T)> for Point2D<T, U> {
    fn from(tuple: (T, T)) -> Self {
        Point2D::new(tuple.0.into(), tuple.1.into())
    }
}

impl<T, U> From<(Length<T, U>, Length<T, U>)> for Point2D<T, U> {
    fn from(tuple: (Length<T, U>, Length<T, U>)) -> Self {
        Point2D::new(tuple.0, tuple.1)
    }
}

// <point> + <vector>
impl<T: Add<Output = T>, U> Add<Vector2D<T, U>> for Point2D<T, U> {
    type Output = Self;

    fn add(self, rhs: Vector2D<T, U>) -> Self::Output {
        Point2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

// <point> + <number>
impl<T: Copy + Add<Output = T>, U> Add<T> for Point2D<T, U> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Point2D::new(self.x + rhs, self.y + rhs)
    }
}

// <point> - <vector>
impl<T: Sub<Output = T>, U> Sub<Vector2D<T, U>> for Point2D<T, U> {
    type Output = Self;

    fn sub(self, rhs: Vector2D<T, U>) -> Self::Output {
        Point2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

// <point> - <number>
impl<T: Copy + Sub<Output = T>, U> Sub<T> for Point2D<T, U> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Point2D::new(self.x - rhs, self.y - rhs)
    }
}

// <point> * <number>
impl<T: Copy + Mul<Output = T>, U> Mul<T> for Point2D<T, U> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Point2D::new(self.x * rhs, self.y * rhs)
    }
}

// <point> / <number>
impl<T: Copy + Div<Output = T>, U> Div<T> for Point2D<T, U> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Point2D::new(self.x / rhs, self.y / rhs)
    }
}

// <point> % <number>
impl<T: Copy + Rem<Output = T>, U> Rem<T> for Point2D<T, U> {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Point2D::new(self.x % rhs, self.y % rhs)
    }
}

// <point> += <number>
impl<T: Copy + AddAssign, U> AddAssign<T> for Point2D<T, U> {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
    }
}

// <point> -= <number>
impl<T: Copy + SubAssign, U> SubAssign<T> for Point2D<T, U> {
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

// <point> *= <number>
impl<T: Copy + MulAssign, U> MulAssign<T> for Point2D<T, U> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

// <point> /= <number>
impl<T: Copy + DivAssign, U> DivAssign<T> for Point2D<T, U> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

// <point> %= <number>
impl<T: Copy + RemAssign, U> RemAssign<T> for Point2D<T, U> {
    fn rem_assign(&mut self, rhs: T) {
        self.x %= rhs;
        self.y %= rhs;
    }
}

// -<point>
impl<T: Neg<Output = T>, U> Neg for Point2D<T, U> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point2D::new(-self.x, -self.y)
    }
}

impl<T: PartialEq, U> PartialEq for Point2D<T, U> {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

impl<T: Zero, U> Point2D<T, U> {
    pub fn origin() -> Self {
        Point2D::new(Length::zero(), Length::zero())
    }

    pub fn is_origin(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }

    pub fn set_origin(&mut self) {
        self.x.set_zero();
        self.y.set_zero();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)] // TODO: remove
    struct Px;

    type I32Point2D = Point2D<i32, Px>;
    type I32Vector2D = Vector2D<i32, Px>;

    #[test]
    fn test_to_vector() {
        assert_eq!(
            I32Vector2D::new(1.into(), 2.into()),
            I32Point2D::new(1.into(), 2.into()).to_vector()
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(
            I32Point2D::new(Default::default(), Default::default()),
            Default::default()
        );
    }

    #[test]
    fn test_clone() {
        assert_eq!(
            I32Point2D::new(1.into(), 2.into()),
            I32Point2D::new(1.into(), 2.into()).clone()
        );
    }

    #[test]
    fn test_from() {
        let expected = I32Point2D::new(1.into(), 2.into());
        assert_eq!(expected, (1, 2).into());
        assert_eq!(expected, (Length::new(1), Length::new(2)).into());
    }

    #[test]
    fn test_add() {
        assert_eq!(
            I32Point2D::new(3.into(), 3.into()),
            I32Point2D::new(1.into(), 2.into()) + I32Vector2D::new(2.into(), 1.into())
        );
        assert_eq!(
            I32Point2D::new(2.into(), 3.into()),
            I32Point2D::new(1.into(), 2.into()) + 1
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            I32Point2D::new((-1).into(), 1.into()),
            I32Point2D::new(1.into(), 2.into()) - I32Vector2D::new(2.into(), 1.into())
        );
        assert_eq!(
            I32Point2D::new(0.into(), 1.into()),
            I32Point2D::new(1.into(), 2.into()) - 1
        );
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T: Serialize, U> Serialize for Point2D<T, U> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            (&self.x, &self.y).serialize(serializer)
        }
    }

    impl<'de, T, U> Deserialize<'de> for Point2D<T, U>
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
        use serde_test::{assert_tokens, Token};

        // TODO: remove PartialEq
        #[derive(Debug, PartialEq)]
        enum Px {}

        #[test]
        fn test_serde() {
            let v: Point2D<i32, Px> = (0, 1).into();
            assert_tokens(
                &v,
                &[
                    Token::Tuple { len: 2 },
                    Token::I32(0),
                    Token::I32(1),
                    Token::TupleEnd,
                ],
            );
        }
    }
}
