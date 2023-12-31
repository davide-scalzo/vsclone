use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use super::{Player, ShootingTimer};

use crate::{
    plugins::{enemy::Enemy, projectiles::ShootProjectileEvent},
    shared::{
        components::{Attack, Direction, Health, Speed},
        utils::macros::unwrap_or_return,
    },
};

// TODO - make Stats a component?
const BASE_DMG: f32 = 20.0;
const CRIT_RATE: f32 = 0.20;
const CRIT_DAMAGE: f32 = 1.5;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("sprites/tile_0098.png");

    commands.spawn((
        Name::new("Player"),
        Player,
        Speed(120.0),
        Health { value: 100.0 },
        Attack {
            damage: 45.0,
            range: 100.0,
        },
        SpriteBundle {
            texture: texture_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
    ));

    commands.insert_resource(ShootingTimer {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    })
}

fn get_movement_from_input(input: Res<Input<KeyCode>>) -> Vec2 {
    let mut input_vector = Vec2::ZERO;

    if input.pressed(KeyCode::W) {
        input_vector.y += 1.0;
    }

    if input.pressed(KeyCode::S) {
        input_vector.y -= 1.0;
    }
    if input.pressed(KeyCode::A) {
        input_vector.x -= 1.0;
    }

    if input.pressed(KeyCode::D) {
        input_vector.x += 1.0;
    }
    input_vector.normalize_or_zero()
}

pub fn move_player(
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let vector = get_movement_from_input(input);

    if let Ok((mut transform, speed)) = query.get_single_mut() {
        transform.translation.y += speed.0 * time.delta_seconds() * vector.y;
        transform.translation.x += speed.0 * time.delta_seconds() * vector.x;
    }
}

pub fn shoot_enemies(
    enemies_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    player_query: Query<(&Transform, &Attack), (With<Player>, Without<Enemy>)>,
    mut shoot_timer: ResMut<ShootingTimer>,
    time: Res<Time>,
    mut shoot_projectile: EventWriter<ShootProjectileEvent>,
) {
    shoot_timer.timer.tick(time.delta());
    if shoot_timer.timer.finished() {
        if let Ok((player_pos, attack)) = player_query.get_single() {
            let mut target_pos: Option<&Transform> = None;
            let mut target_dist: Option<f32> = None;
            for enemy_pos in enemies_query.iter() {
                let distance = enemy_pos.translation.distance(player_pos.translation);

                match target_dist {
                    None => {
                        target_dist = Some(distance);
                        target_pos = Some(enemy_pos);
                    }
                    Some(dist) => {
                        if distance < dist {
                            target_pos = Some(enemy_pos);
                            target_dist = Some(distance);
                        }
                    }
                }
            }

            let dist = unwrap_or_return!(target_dist);
            let target = unwrap_or_return!(target_pos);

            if dist > attack.range {
                return;
            }

            let vec = Vec2 {
                x: target.translation.x - player_pos.translation.x,
                y: target.translation.y - player_pos.translation.y,
            };

            let mut rng = rand::thread_rng();

            let test = rng.gen::<f32>();

            let damage = if test <= CRIT_RATE {
                BASE_DMG * CRIT_DAMAGE
            } else {
                BASE_DMG
            };

            let normalized_vec = vec.normalize_or_zero();
            let event = ShootProjectileEvent {
                from: *player_pos,
                direction: Direction {
                    x: normalized_vec.x,
                    y: normalized_vec.y,
                },
                speed: 500.0,
                damage,
            };
            shoot_projectile.send(event);
        }
    }
}
