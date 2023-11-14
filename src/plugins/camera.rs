use bevy::prelude::*;

pub struct CameraPlugin;

mod systems;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgba(0.16, 0.23, 0.25, 1.)))
            .add_systems(Startup, systems::spawn_camera);
    }
}
