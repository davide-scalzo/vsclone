use bevy::prelude::*;

pub struct EnemyPlugin;

mod systems;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct EnemySpawnConfig {
    pub timer: Timer,
}

#[derive(Event)]
pub struct EnemyHit {
    pub location: Vec3,
    pub damage: f32,
}

#[derive(Component)]
pub struct DamageIndicatorTimer {
    pub timer: Timer,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_enemy_timer)
            .add_systems(
                Update,
                (
                    systems::move_enemies,
                    systems::spawn_enemies,
                    systems::show_damage_indicator,
                    systems::despawn_damage_indicators,
                ),
            )
            .add_event::<EnemyHit>();
    }
}
