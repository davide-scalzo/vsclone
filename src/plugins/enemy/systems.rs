use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

use crate::{
    plugins::player::Player,
    shared::components::{self, Position, Speed},
};

use super::Enemy;

pub fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    win_query: Query<&Window>,
) {
    println!("Spawning enemies");

    if let Ok(window) = win_query.get_single() {
        let width = window.width();
        let height = window.height();

        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let enemy_x = rng.gen::<f32>() * width - width / 2.;
            let enemy_y = rng.gen::<f32>() * height - height / 2.;

            let position = components::Position {
                x: enemy_x,
                y: enemy_y,
            };

            commands.spawn((
                Name::new("Enemy"),
                Enemy,
                components::Speed { value: 100.0 },
                components::Health { value: 80.0 },
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Quad::new(Vec2 { x: 8.0, y: 8.0 }).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.)),
                    ..default()
                },
                position,
            ));
        }
    }
}

pub fn move_enemies(
    mut enemy_query: Query<(&mut Position, &mut Transform, &Speed), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Position, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok(target_location) = player_query.get_single() {
        for (mut position, mut transform, speed) in enemy_query.iter_mut() {
            let (x, y) = get_movement(&position, target_location);
            position.x += x * speed.value * time.delta_seconds();
            position.y += y * speed.value * time.delta_seconds();

            transform.translation.x = position.x;
            transform.translation.y = position.y;
        }
    }
}

fn get_movement(origin: &Position, destination: &Position) -> (f32, f32) {
    let x = destination.x - origin.x;
    let y = destination.y - origin.y;
    let magnitude = (x.powi(2) + y.powi(2)).sqrt();
    (x / magnitude, y / magnitude)
}
