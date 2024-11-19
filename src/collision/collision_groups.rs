use bevy_rapier2d::geometry::Group;

impl LocalGroupNames for Group {}
pub trait LocalGroupNames {
    const PLAYER: Group = Group::GROUP_1;

    const TEST_ENEMY: Group = Group::GROUP_11;
    const TEST_ENEMY_SENSOR: Group = Group::GROUP_12;

    // const ENTITIES: Group;
    // const SENSORS: Group;
}
