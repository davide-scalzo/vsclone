use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DevPlugin;

use crate::shared::components;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new())
            .register_type::<components::Speed>()
            .register_type::<components::Position>()
            .register_type::<components::Health>();
    }
}
