use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Main Camera"), Camera2dBundle::default()));
}
