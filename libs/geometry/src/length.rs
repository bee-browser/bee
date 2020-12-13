use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

use num_traits::Zero;

#[derive(Debug)]
pub struct Length<T, U>(T, PhantomData<U>);

impl<T, U> Length<T, U> {
    pub const fn new(num: T) -> Self {
        Length(num, PhantomData)
    }
}

impl<T: Default, U> Default for Length<T, U> {
    fn default() -> Self {
        Length::new(Default::default())
    }
}

impl<T: Clone, U> Clone for Length<T, U> {
    fn clone(&self) -> Self {
        Length::new(self.0.clone())
    }
}

impl<T: Copy, U> Copy for Length<T, U> {}

impl<T, U> From<T> for Length<T, U> {
    fn from(num: T) -> Self {
        Length::new(num)
    }
}

// <length> + <length>
impl<T: Add<Output = T>, U> Add for Length<T, U> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Length::new(self.0 + rhs.0)
    }
}

// <length> + <number>
impl<T: Add<Output = T>, U> Add<T> for Length<T, U> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Length::new(self.0 + rhs)
    }
}

// <length> - <length>
impl<T: Sub<Output = T>, U> Sub for Length<T, U> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Length::new(self.0 - rhs.0)
    }
}

// <length> - <number>
impl<T: Sub<Output = T>, U> Sub<T> for Length<T, U> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Length::new(self.0 - rhs)
    }
}

// <length> * <number>
impl<T: Mul<Output = T>, U> Mul<T> for Length<T, U> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Length::new(self.0 * rhs)
    }
}

// <length> / <number>
impl<T: Div<Output = T>, U> Div<T> for Length<T, U> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Length::new(self.0 / rhs)
    }
}

// <length> % <number>
impl<T: Rem<Output = T>, U> Rem<T> for Length<T, U> {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Length::new(self.0 % rhs)
    }
}

// <length> += <length>
impl<T: AddAssign, U> AddAssign for Length<T, U> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

// <length> += <number>
impl<T: AddAssign, U> AddAssign<T> for Length<T, U> {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs;
    }
}

// <length> -= <length>
impl<T: SubAssign, U> SubAssign for Length<T, U> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

// <length> -= <number>
impl<T: SubAssign, U> SubAssign<T> for Length<T, U> {
    fn sub_assign(&mut self, rhs: T) {
        self.0 -= rhs;
    }
}

// <length> *= <length>
impl<T: MulAssign, U> MulAssign<T> for Length<T, U> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs;
    }
}

// <length> /= <length>
impl<T: DivAssign, U> DivAssign<T> for Length<T, U> {
    fn div_assign(&mut self, rhs: T) {
        self.0 /= rhs;
    }
}

// <length> %= <length>
impl<T: RemAssign, U> RemAssign<T> for Length<T, U> {
    fn rem_assign(&mut self, rhs: T) {
        self.0 %= rhs;
    }
}

// -<length>
impl<T: Neg<Output = T>, U> Neg for Length<T, U> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Length::new(-self.0)
    }
}

impl<T: Eq, U> Eq for Length<T, U> {}

impl<T: Ord, U> Ord for Length<T, U> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}

impl<T: PartialEq, U> PartialEq for Length<T, U> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.eq(&rhs.0)
    }
}

impl<T: PartialOrd, U> PartialOrd for Length<T, U> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}

impl<T: Hash, U> Hash for Length<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Zero, U> Zero for Length<T, U> {
    fn zero() -> Self {
        Length::new(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    fn set_zero(&mut self) {
        self.0.set_zero();
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T: Serialize, U> Serialize for Length<T, U> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.0.serialize(serializer)
        }
    }

    impl<'de, T, U> Deserialize<'de> for Length<T, U>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(Length(Deserialize::deserialize(deserializer)?, PhantomData))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Length;

        use serde_test::{Token, assert_tokens};

        #[derive(Debug)]
        enum Px {}

        #[test]
        fn test_serde() {
            let v: Length<i32, Px> = 0.into();
            assert_tokens(&v, &[Token::I32(0)]);
        }
    }
}
