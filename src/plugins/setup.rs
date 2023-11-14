use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Another colony builder".to_string(),
                        present_mode: PresentMode::AutoVsync,
                        mode: WindowMode::Windowed,
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        );
    }
}
