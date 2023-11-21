use bevy::prelude::*;

// TODO
// - Follow player, maybe with easing
// - Zoom?
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Main Camera"), Camera2dBundle::default()));
}
