use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    plugins::enemy::{Enemy, EnemyHit},
    shared::components::{Damage, Direction, Health, Speed},
};

use super::{Projectile, ProjectileTimer, ShootProjectileEvent};

pub fn spawn_projectile(
    mut commands: Commands,
    mut ev_shoot_projectile: EventReader<ShootProjectileEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_shoot_projectile.read() {
        commands.spawn((
            Name::new("Player Bullet"),
            Projectile,
            Speed(ev.speed),
            ev.direction,
            Damage(ev.damage),
            ProjectileTimer {
                timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(
                    ev.from.translation.x,
                    ev.from.translation.y,
                    0.,
                )),
                ..default()
            },
        ));

        commands.spawn(AudioBundle {
            source: asset_server.load("audio/footstep00.ogg"),
            ..default()
        });
    }
}

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
            transform.translation.y += speed.0 * time.delta_seconds() * direction.y;
            transform.translation.x += speed.0 * time.delta_seconds() * direction.x;
        }
    }
}

pub fn check_hit(
    mut commands: Commands,
    mut targets_query: Query<(Entity, &mut Health, &Transform), With<Enemy>>,
    bullets_query: Query<(Entity, &Transform, &Damage), With<Projectile>>,
    mut hit_enemy_event: EventWriter<EnemyHit>,
) {
    for (bullet, bullet_pos, damage) in bullets_query.iter() {
        for (target, mut health, target_pos) in targets_query.iter_mut() {
            let dist = bullet_pos.translation.distance(target_pos.translation);
            if dist < 16.0 {
                commands.entity(bullet).despawn();
                health.value -= damage.0;

                hit_enemy_event.send(EnemyHit {
                    location: Vec3 {
                        x: bullet_pos.translation.x,
                        y: bullet_pos.translation.y,
                        z: 2.0,
                    },
                    damage: damage.0,
                });

                if health.value <= 0.0 {
                    commands.entity(target).despawn();
                }
                break;
            }
        }
    }
}
