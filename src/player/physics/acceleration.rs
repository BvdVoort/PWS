use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy::{math::Vec2, prelude::{Component, Query}, reflect::Reflect};
use super::Velocity;


#[derive(Component, Reflect, Default, Clone, Copy, PartialEq)]
pub struct Acceleration {
    pub linear: Vec2
}

pub fn reset_acceleration(mut query: Query<&mut Acceleration>) {
    for mut acceleration in query.iter_mut() {
        *acceleration = Acceleration::ZERO;
    }
}

pub fn apply_acceleration(mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut velocity, accelleration) in query.iter_mut() {
        velocity.linear += accelleration.linear;
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