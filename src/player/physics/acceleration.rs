use std::{ops::{Add, AddAssign, Div, Mul, Sub, SubAssign}, time::Duration};

use bevy::{log::info, math::Vec2, prelude::{Component, Query, Res}, reflect::Reflect, time::Time};
use super::{Velocity, Scaler};


#[derive(Component, Reflect, Debug, Default, Clone, Copy, PartialEq)]
pub struct Acceleration {
    pub linear: Vec2
}

pub fn reset_acceleration(mut query: Query<&mut Acceleration>) {
    for mut acceleration in query.iter_mut() {
        *acceleration = Acceleration::ZERO;
    }
}

pub fn apply_acceleration(time: Res<Time>, mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut velocity, accelleration) in query.iter_mut() {
        *velocity += *accelleration * time.delta();
    }
}

impl Acceleration {
    pub const ZERO: Self = Self { linear: Vec2::ZERO };
    pub const LEFT: Self = Self { linear: Vec2::NEG_X };
    pub const RIGHT: Self = Self { linear: Vec2::X };
    pub const UP: Self = Self { linear: Vec2::Y };
    pub const DOWN: Self = Self { linear: Vec2::NEG_Y };
}

impl Add for Acceleration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { linear: self.linear + rhs.linear }
    }
}

impl AddAssign for Acceleration {
    fn add_assign(&mut self, rhs: Self) {
        self.linear += rhs.linear;
    }
}

impl Sub for Acceleration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { linear: self.linear - rhs.linear }
    }
}

impl SubAssign for Acceleration {
    fn sub_assign(&mut self, rhs: Self) {
        self.linear -= rhs.linear;
    }
}

impl<T> Mul<T> for Acceleration 
where 
    Vec2: Mul<T, Output = Vec2>,
    T: Scaler,
{
    type Output = Acceleration;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output{ linear: self.linear * rhs }
    }
}

impl Mul<Acceleration> for f32 {
    type Output = Acceleration;

    fn mul(self, rhs: Acceleration) -> Self::Output {rhs * self}
}

impl<T> Div<T> for Acceleration 
where Vec2: 
    Div<T, Output = Vec2>,
    T: Scaler,
{
    type Output = Acceleration;
    
    fn div(self, rhs: T) -> Self::Output {
        Self::Output{ linear: self.linear / rhs }
    }
}

impl Div<Acceleration> for f32
{
    type Output = Acceleration;
    
    fn div(self, rhs: Acceleration) -> Self::Output {rhs / self}
}

impl Mul<Duration> for Acceleration
{
    type Output = Velocity;

    fn mul(self, rhs: Duration) -> Self::Output {
        Self::Output{ linear: self.linear * rhs.as_secs_f32() }
    }
}

impl Mul<Acceleration> for Duration
{
    type Output = Velocity;

    fn mul(self, rhs: Acceleration) -> Self::Output {
        rhs * self
    }
}

