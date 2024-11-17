mod test_enemy;

use test_enemy::TestEnemyPlugin;
use bevy::app::Plugin;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(TestEnemyPlugin);
    }
}