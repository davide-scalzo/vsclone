use bevy::prelude::*;

mod plugins;
mod shared;

fn main() {
    App::new()
        .add_plugins((
            plugins::setup::SetupPlugin,
            plugins::camera::CameraPlugin,
            plugins::player::PlayerPlugin,
            plugins::enemy::EnemyPlugin,
        ))
        .add_plugins(plugins::dev::DevPlugin)
        .run();
}
