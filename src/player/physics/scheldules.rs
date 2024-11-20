use bevy::ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerPhysicsConfigure;

#[derive(ScheduleLabel, Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerPhysicsCalculate;