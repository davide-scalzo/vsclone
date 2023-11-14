use bevy::prelude::*;

pub struct PlayerPlugin;

mod systems;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_player)
            .add_systems(Update, systems::move_player);
    }
}
