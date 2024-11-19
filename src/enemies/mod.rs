mod test_enemy;
mod entity_bundles;

use test_enemy::TestEnemyPlugin;
use bevy::app::Plugin;

pub use entity_bundles::ObservableCollider;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(TestEnemyPlugin);
    }
}