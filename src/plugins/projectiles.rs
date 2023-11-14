use bevy::prelude::*;

pub struct ProjectilePlugin;

mod systems;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct ProjectileTimer {
    pub timer: Timer,
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::move_projectiles);
    }
}
