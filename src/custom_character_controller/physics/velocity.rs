use std::{ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign}, time::Duration};
use bevy::{math::Vec2, prelude::Component, reflect::Reflect};
use super::{Acceleration, Distance, Scalar};

#[derive(Component, Reflect, Debug, Default, PartialEq, Clone, Copy)]
pub struct Velocity {
    pub(super) meters_per_second: Vec2
}

impl Velocity {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { meters_per_second: Vec2 { x, y }}
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

    pub const ZERO: Self = Self { meters_per_second: Vec2::ZERO };

    pub fn get_horizontal(&self) -> Self { Self::horizontal(self.meters_per_second.x) }
    pub fn get_vertical(&self) -> Self { Self::vertical(self.meters_per_second.y) }

    pub fn set_horizontal(&mut self, value: f32) { self.meters_per_second.x = value; }
    pub fn set_vertical(&mut self, value: f32) { self.meters_per_second.y = value; }
}

impl Add for Velocity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { meters_per_second: self.meters_per_second + rhs.meters_per_second }
    }
}

impl Sub for Velocity {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { meters_per_second: self.meters_per_second - rhs.meters_per_second }
    }
}

impl Neg for Velocity {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output { meters_per_second: -self.meters_per_second }
    }
}

impl AddAssign for Velocity {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self { meters_per_second: self.meters_per_second + rhs.meters_per_second }
    }
}

impl SubAssign for Velocity {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self { meters_per_second: self.meters_per_second - rhs.meters_per_second }
    }
}

// #! TODO: Mul<Velocity> for T
impl<T> Mul<T> for Velocity
where 
    T: Scalar,
    Vec2: Mul<T, Output = Vec2> 
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output { meters_per_second: self.meters_per_second * rhs }
    }
}

// #! TODO: Div<Velocity> for T
impl<T> Div<T> for Velocity
where 
    T: Scalar,
    Vec2: Div<T, Output = Vec2> 
{
    type Output = Self;
    
    fn div(self, rhs: T) -> Self::Output {
        Self::Output { meters_per_second: self.meters_per_second / rhs }
    }
}

impl Div<Duration> for Velocity {
    type Output = Acceleration;

    fn div(self, rhs: Duration) -> Self::Output {
        Self::Output { meters_per_second_squared: self.meters_per_second / rhs.as_secs_f32() }
    }
}

impl Mul<Duration> for Velocity {
    type Output = Distance;

    fn mul(self, rhs: Duration) -> Self::Output {
        Self::Output { meters: self.meters_per_second * rhs.as_secs_f32() }
    }
}

impl Mul<Velocity> for Duration {
    type Output = Distance;

    fn mul(self, rhs: Velocity) -> Self::Output {
        Self::Output { meters: self.as_secs_f32() * rhs.meters_per_second }
    }
}

impl From<Vec2> for Velocity {
    fn from(meters_per_second: Vec2) -> Self {
        Self { meters_per_second }
    }
}