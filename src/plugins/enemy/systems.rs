use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

use crate::{
    plugins::player::Player,
    shared::components::{self, Speed},
};

use super::{Enemy, EnemySpawnConfig};

pub fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    win_query: Query<&Window>,
    mut spawn_timer: ResMut<EnemySpawnConfig>,
    time: Res<Time>,
) {
    if let Ok(window) = win_query.get_single() {
        spawn_timer.timer.tick(time.delta());

        if spawn_timer.timer.finished() {
            let width = window.width();
            let height = window.height();

            let mut rng = rand::thread_rng();

            let enemy_x = rng.gen::<f32>() * width - width / 2.;
            let enemy_y = rng.gen::<f32>() * height - height / 2.;

            commands.spawn((
                Name::new("Enemy"),
                Enemy,
                components::Speed { value: 100.0 },
                components::Health { value: 80.0 },
                components::Attack {
                    damage: 45.0,
                    range: 1.0,
                },
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Quad::new(Vec2 { x: 8.0, y: 8.0 }).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_translation(Vec3::new(enemy_x, enemy_y, 0.)),
                    ..default()
                },
            ));
        }
    }
}

pub fn setup_enemy_timer(mut commands: Commands) {
    commands.insert_resource(EnemySpawnConfig {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    })
}

pub fn move_enemies(
    mut enemy_query: Query<(&mut Transform, &Speed), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok(target_location) = player_query.get_single() {
        for (mut transform, speed) in enemy_query.iter_mut() {
            let raw_vector = Vec2 {
                x: target_location.translation.x - transform.translation.x,
                y: target_location.translation.y - transform.translation.y,
            };

            let vector = raw_vector.normalize_or_zero();
            transform.translation.x += vector.x * speed.value * time.delta_seconds();
            transform.translation.y += vector.y * speed.value * time.delta_seconds();
        }
    }
}
