use std::{ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign}, time::Duration};

use bevy::math::Vec2;

use super::{Velocity, Scalar};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Distance {
    pub(super) meters: Vec2
}

impl Distance {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { meters: Vec2 { x, y }}
    }

    const fn only_x(x: f32) -> Self { Self::new(x, 0.0) }
    const fn only_y(y: f32) -> Self { Self::new(0.0, y) }

    pub const UP: Self = Self::only_y(1.0);
    pub const DOWN: Self = Self::only_y(-1.0);
    pub const RIGHT: Self = Self::only_x(1.0);
    pub const LEFT: Self = Self::only_x(-1.0);
}

// #? this is so you can += to charactercontroller translation
impl AddAssign<Distance> for Option<Vec2> {
    fn add_assign(&mut self, rhs: Distance) {
        *self = Some(self.unwrap_or_default() + rhs.meters);
    }
}


impl Add for Distance {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output{ meters: self.meters + rhs.meters }
    }
}

impl Sub for Distance {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { meters: self.meters - rhs.meters }
    }
}

impl Neg for Distance {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output { meters: -self.meters }
    }
}

impl AddAssign for Distance {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self { meters: self.meters + rhs.meters }
    }
}

impl SubAssign for Distance {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self { meters: self.meters - rhs.meters }
    }
}

// #! TODO: Mul<Distance> for T
impl<T> Mul<T> for Distance
where 
    T: Scalar,
    Vec2: Mul<T, Output = Vec2> 
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output { meters: self.meters * rhs }
    }
}

// #! TODO: Div<Distance> for T
impl<T> Div<T> for Distance
where 
    T: Scalar,
    Vec2: Div<T, Output = Vec2> 
{
    type Output = Self;
    
    fn div(self, rhs: T) -> Self::Output {
        Self::Output { meters: self.meters / rhs }
    }
}

impl Div<Duration> for Distance {
    type Output = Velocity;

    fn div(self, rhs: Duration) -> Self::Output {
        Self::Output { meters_per_second: self.meters / rhs.as_secs_f32() }
    }
}

impl From<Vec2> for Distance {
    fn from(meters: Vec2) -> Self {
        Self { meters }
    }
}