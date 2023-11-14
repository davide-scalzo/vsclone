use bevy::prelude::*;

pub struct EnemyPlugin;

mod systems;

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_enemies)
            .add_systems(Update, systems::move_enemies);
    }
}
