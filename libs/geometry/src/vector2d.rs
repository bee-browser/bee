use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Neg;
use core::ops::Sub;

use num_traits::Zero;

use crate::Length;

#[derive(Debug)]
pub struct Vector2D<T, U> {
    pub x: Length<T, U>,
    pub y: Length<T, U>,
}

impl<T, U> Vector2D<T, U> {
    pub const fn new(x: Length<T, U>, y: Length<T, U>) -> Self {
        Vector2D { x, y }
    }
}

impl<T: Clone, U> Clone for Vector2D<T, U> {
    fn clone(&self) -> Self {
        Vector2D::new(self.x.clone(), self.y.clone())
    }
}

impl<T: Copy, U> Copy for Vector2D<T, U> {}

impl<T: Zero, U> Zero for Vector2D<T, U> {
    fn zero() -> Self {
        Vector2D::new(Length::zero(), Length::zero())
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }

    fn set_zero(&mut self) {
        self.x.set_zero();
        self.y.set_zero();
    }
}

impl<T: Add<Output = T>, U> Add for Vector2D<T, U> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub<Output = T>, U> Sub for Vector2D<T, U> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: AddAssign, U> AddAssign for Vector2D<T, U> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Neg<Output = T>, U> Neg for Vector2D<T, U> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector2D::new(-self.x, -self.y)
    }
}

impl<T: PartialEq, U> PartialEq for Vector2D<T, U> {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}
