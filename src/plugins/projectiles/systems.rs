use bevy::prelude::*;

use crate::shared::components::{Direction, Speed};

use super::{Projectile, ProjectileTimer};

pub fn move_projectiles(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &Speed,
            &Direction,
            &mut ProjectileTimer,
        ),
        With<Projectile>,
    >,
    time: Res<Time>,
) {
    for (entity, mut transform, speed, direction, mut timer) in query.iter_mut() {
        timer.timer.tick(time.delta());

        if timer.timer.finished() {
            commands.entity(entity).despawn_recursive()
        } else {
            transform.translation.y += speed.value * time.delta_seconds() * direction.y;
            transform.translation.x += speed.value * time.delta_seconds() * direction.x;
        }
    }
}
