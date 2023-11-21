use std::time::Duration;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    plugins::player::Player,
    shared::components::{self, Speed},
};

use super::{DamageIndicatorTimer, Enemy, EnemyHit, EnemySpawnConfig};

pub fn spawn_enemies(
    mut commands: Commands,
    win_query: Query<&Window>,
    mut spawn_timer: ResMut<EnemySpawnConfig>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(window) = win_query.get_single() {
        spawn_timer.timer.tick(time.delta());

        if spawn_timer.timer.finished() {
            let width = window.width();
            let height = window.height();

            let mut rng = rand::thread_rng();

            let enemy_x = rng.gen::<f32>() * width - width / 2.;
            let enemy_y = rng.gen::<f32>() * height - height / 2.;

            let mut transform = Transform::from_scale(Vec3::splat(2.0));

            transform.translation.x = enemy_x;
            transform.translation.y = enemy_y;

            let texture_handle = asset_server.load("sprites/tile_0108.png");

            commands.spawn((
                Name::new("Enemy"),
                Enemy,
                components::Speed(100.0),
                components::Health { value: 80.0 },
                components::Attack {
                    damage: 45.0,
                    range: 1.0,
                },
                SpriteBundle {
                    texture: texture_handle,
                    transform,
                    ..default()
                },
            ));
        }
    }
}

pub fn setup_enemy_timer(mut commands: Commands) {
    commands.insert_resource(EnemySpawnConfig {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    })
}

// TODO - figure out how to get enemies to not overlap (boids-like movement?)
pub fn move_enemies(
    mut enemy_query: Query<(&mut Transform, &Speed), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok(target_location) = player_query.get_single() {
        for (mut enemy_loc, enemy_speed) in enemy_query.iter_mut() {
            let raw_vector = Vec2 {
                x: target_location.translation.x - enemy_loc.translation.x,
                y: target_location.translation.y - enemy_loc.translation.y,
            };

            let vector = raw_vector.normalize_or_zero();
            let new_loc = Vec3 {
                x: enemy_loc.translation.x + vector.x * enemy_speed.0 * time.delta_seconds(),
                y: enemy_loc.translation.y + vector.y * enemy_speed.0 * time.delta_seconds(),
                z: 0.0,
            };
            enemy_loc.translation = new_loc;
        }
    }
}

// TODO - Check how to implement fading and moving animations (maybe follow enemy transform?)
pub fn show_damage_indicator(
    mut commands: Commands,
    mut ev_shoot_projectile: EventReader<EnemyHit>,
) {
    for ev in ev_shoot_projectile.read() {
        let timer = DamageIndicatorTimer {
            timer: Timer::new(Duration::from_millis(400), TimerMode::Once),
        };
        let text_bundle = Text2dBundle {
            text: Text::from_section(
                (ev.damage as u16).to_string(),
                TextStyle {
                    color: Color::WHITE,
                    ..default()
                },
            ),
            transform: Transform {
                translation: ev.location,
                ..default()
            },
            ..default()
        };
        commands.spawn((Name::new("Damage indicator timer"), text_bundle, timer));
    }
}

pub fn despawn_damage_indicators(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DamageIndicatorTimer)>, // TODO Check if despawning the identity fully clears all related components
    time: Res<Time>,
) {
    for (entity, mut dmg_timer) in query.iter_mut() {
        dmg_timer.timer.tick(time.delta());

        if dmg_timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
