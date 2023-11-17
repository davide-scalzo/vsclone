use bevy::prelude::*;

use crate::{
    plugins::enemy::Enemy,
    shared::{
        components::{Damage, Direction, Health, Speed},
        utils::traits::DistanceFrom,
    },
};

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

pub fn check_hit(
    mut commands: Commands,
    mut targets_query: Query<(Entity, &mut Health, &Transform), With<Enemy>>,
    bullets_query: Query<(Entity, &Transform, &Damage), With<Projectile>>,
) {
    for (bullet, bullet_pos, damage) in bullets_query.iter() {
        for (target, mut health, target_pos) in targets_query.iter_mut() {
            let dist = bullet_pos.distance_from(target_pos);
            if dist < 16.0 {
                commands.entity(bullet).despawn();
                health.value -= damage.value;
                if health.value <= 0.0 {
                    commands.entity(target).despawn();
                }
                break;
            }
        }
    }
}
