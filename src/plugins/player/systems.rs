use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use super::Player;

use crate::shared::components::{self, Position, Speed};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let position = components::Position { x: 0.0, y: 0.0 };
    commands.spawn((
        Name::new("Player"),
        Player,
        components::Speed { value: 200.0 },
        components::Health { value: 100.0 },
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(4.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.)),
            ..default()
        },
        position,
    ));
}

fn get_movement(input: Res<Input<KeyCode>>) -> (f32, f32) {
    let x_0: f32 = if input.pressed(KeyCode::A) { 1.0 } else { 0.0 };
    let x_1: f32 = if input.pressed(KeyCode::D) { 1.0 } else { 0.0 };

    let y_0: f32 = if input.pressed(KeyCode::W) { 1.0 } else { 0.0 };
    let y_1: f32 = if input.pressed(KeyCode::S) { 1.0 } else { 0.0 };

    let x = x_1 - x_0;
    let y = y_0 - y_1;
    let magnitude = (x.powi(2) + y.powi(2)).sqrt();
    if magnitude == 0.0 {
        return (0.0, 0.0);
    } else {
        return (x / magnitude, y / magnitude);
    }
}

pub fn move_player(
    mut query: Query<(&mut Transform, &Speed, &mut Position), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (x, y) = get_movement(input);

    if let Ok((mut transform, speed, mut position)) = query.get_single_mut() {
        position.y += speed.value * time.delta_seconds() * y;
        position.x += speed.value * time.delta_seconds() * x;

        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}
