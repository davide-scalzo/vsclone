use bevy::prelude::*;

use crate::shared::components::Direction;

pub struct ProjectilePlugin;

mod systems;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct ProjectileTimer {
    pub timer: Timer,
}

#[derive(Event)]
pub struct ShootProjectileEvent {
    pub from: Transform,
    pub direction: Direction,
    pub speed: f32,
    pub damage: f32,
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                systems::move_projectiles,
                systems::check_hit,
                systems::spawn_projectile,
            ),
        )
        .add_event::<ShootProjectileEvent>();
    }
}
