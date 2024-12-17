use std::{ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign}, time::Duration};
use bevy::{math::Vec2, prelude::Component, reflect::Reflect};
use super::{Scalar, Velocity};

#[derive(Component, Reflect, Debug, Default, PartialEq, Clone, Copy)]
pub struct Acceleration {
    pub(super) meters_per_second_squared: Vec2
}

impl Acceleration {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { meters_per_second_squared: Vec2 { x, y }}
    }

    pub const fn from_vec2(v: Vec2) -> Self {
        Self { meters_per_second_squared: v }
    }

    pub const fn horizontal(x: f32) -> Self { Self::new(x, 0.0) }
    pub const fn vertical(y: f32) -> Self { Self::new(0.0, y) }

    pub const fn up(value: f32) -> Self { Self::vertical(value) }
    pub const fn down(value: f32) -> Self { Self::vertical(-value) }
    pub const fn right(value: f32) -> Self { Self::horizontal(value) }
    pub const fn left(value: f32) -> Self { Self::horizontal(-value) }

    pub const UP: Self = Self::up(1.0);
    pub const DOWN: Self = Self::down(1.0);
    pub const RIGHT: Self = Self::right(1.0);
    pub const LEFT: Self = Self::left(1.0);

    pub const ZERO: Self = Self { meters_per_second_squared: Vec2::ZERO };

    pub fn get_horizontal(&self) -> Self { Self::horizontal(self.meters_per_second_squared.x) }
    pub fn get_vertical(&self) -> Self { Self::vertical(self.meters_per_second_squared.y) }
    
    pub fn get_horizontal_f32(&self) -> f32 { self.meters_per_second_squared.x } // should f32 be added as extra?
    pub fn get_vertical_f32(&self) -> f32 { self.meters_per_second_squared.y }

    pub fn set_horizontal(&mut self, value: f32) { self.meters_per_second_squared.x = value; }
    pub fn set_vertical(&mut self, value: f32) { self.meters_per_second_squared.y = value; }

}

impl Add for Acceleration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { meters_per_second_squared: self.meters_per_second_squared + rhs.meters_per_second_squared }
    }
}

impl Sub for Acceleration {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { meters_per_second_squared: self.meters_per_second_squared - rhs.meters_per_second_squared }
    }
}

impl Neg for Acceleration {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output { meters_per_second_squared: -self.meters_per_second_squared }
    }
}

impl AddAssign for Acceleration {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self { meters_per_second_squared: self.meters_per_second_squared + rhs.meters_per_second_squared }
    }
}

impl SubAssign for Acceleration {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self { meters_per_second_squared: self.meters_per_second_squared - rhs.meters_per_second_squared }
    }
}

// #! TODO: Mul<Velocity> for T
impl<T> Mul<T> for Acceleration
where 
    T: Scalar,
    Vec2: Mul<T, Output = Vec2> 
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output { meters_per_second_squared: self.meters_per_second_squared * rhs }
    }
}

// #! TODO: Div<Velocity> for T
impl<T> Div<T> for Acceleration
where 
    T: Scalar,
    Vec2: Div<T, Output = Vec2> 
{

    type Output = Self;
    
    fn div(self, rhs: T) -> Self::Output {
        Self::Output { meters_per_second_squared: self.meters_per_second_squared / rhs }
    }
}

impl Mul<Duration> for Acceleration {
    type Output = Velocity;

    fn mul(self, rhs: Duration) -> Self::Output {
        Self::Output { meters_per_second: self.meters_per_second_squared * rhs.as_secs_f32() }
    }
}

impl Mul<Acceleration> for Duration {
    type Output = Velocity;

    fn mul(self, rhs: Acceleration) -> Self::Output {
        Self::Output { meters_per_second: self.as_secs_f32() * rhs.meters_per_second_squared }
    }
}

impl From<Vec2> for Acceleration {
    fn from(meters_per_second_squared: Vec2) -> Self {
        Self { meters_per_second_squared }
    }
}