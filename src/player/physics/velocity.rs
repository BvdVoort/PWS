use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy::{math::Vec2, prelude::{Component, Query}, reflect::Reflect};
use bevy_rapier2d::prelude::KinematicCharacterController;

use super::option_add_assign::OptionAddAssignExtension;


#[derive(Component, Reflect, Default, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub linear: Vec2
}

pub fn apply_velocity(mut query: Query<(&mut KinematicCharacterController, &Velocity)>) {
    for (mut controller, velocity) in query.iter_mut() {
        controller.translation.add_assign(velocity.linear); 
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