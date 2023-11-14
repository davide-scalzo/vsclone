use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Reflect)]
pub struct Health {
    pub value: f32,
}

#[derive(Component, Reflect)]
pub struct Speed {
    pub value: f32,
}
