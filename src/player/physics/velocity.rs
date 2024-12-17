use std::{ops::{Add, AddAssign, Div, Mul, Sub, SubAssign}, time::Duration};

use bevy::{math::Vec2, prelude::{Component, Query, Res}, reflect::Reflect, time::Time};
use bevy_rapier2d::prelude::KinematicCharacterController;

use super::{option_add_assign::OptionAddAssignExtension, Scaler};


#[derive(Component, Reflect, Debug, Default, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub linear: Vec2
}

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut KinematicCharacterController, &Velocity)>) {
    for (mut controller, velocity) in query.iter_mut() {
        OptionAddAssignExtension::add_assign(&mut controller.translation, *velocity * time.delta()); 
    }
}

impl Velocity {
    pub const ZERO: Self = Self { linear: Vec2::ZERO };
    pub const LEFT: Self = Self { linear: Vec2::NEG_X };
    pub const RIGHT: Self = Self { linear: Vec2::X };
    pub const UP: Self = Self { linear: Vec2::Y };
    pub const DOWN: Self = Self { linear: Vec2::NEG_Y };
}

impl Add for Velocity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { linear: self.linear + rhs.linear }
    }
}

impl AddAssign for Velocity {
    fn add_assign(&mut self, rhs: Self) {
        self.linear += rhs.linear;
    }
}

impl Sub for Velocity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { linear: self.linear - rhs.linear }
    }
}

impl SubAssign for Velocity {
    fn sub_assign(&mut self, rhs: Self) {
        self.linear -= rhs.linear;
    }
}


impl<T> Mul<T> for Velocity 
where 
    Vec2: Mul<T, Output = Vec2>,
    T: Scaler,
{
    type Output = Velocity;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output{ linear: self.linear * rhs }
    }
}

impl Mul<Velocity> for f32 {
    type Output = Velocity;

    fn mul(self, rhs: Velocity) -> Self::Output {rhs * self}
}

impl<T> Div<T> for Velocity 
where
    Vec2: Div<T, Output = Vec2>,
    T: Scaler,
{
    type Output = Velocity;
    
    fn div(self, rhs: T) -> Self::Output {
        Self::Output{ linear: self.linear / rhs }
    }
}

impl Div<Velocity> for f32
{
    type Output = Velocity;
    
    fn div(self, rhs: Velocity) -> Self::Output {rhs / self}
}

// #! TODO: make a distance type
impl Mul<Duration> for Velocity {
    type Output = Vec2;

    fn mul(self, rhs: Duration) -> Self::Output {
        self.linear * rhs.as_secs_f32()
    }
}

impl Mul<Velocity> for Duration {
    type Output = Vec2;

    fn mul(self, rhs: Velocity) -> Self::Output {
        rhs * self
    }
}