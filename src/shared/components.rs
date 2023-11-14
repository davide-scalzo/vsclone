use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Health {
    pub value: f32,
}

#[derive(Component, Reflect)]
pub struct Attack {
    pub damage: f32,
    pub range: f32,
}

#[derive(Component, Reflect)]
pub struct Speed {
    pub value: f32,
}

#[derive(Component, Reflect)]
pub struct Damage {
    pub value: f32,
}

#[derive(Component, Reflect)]
pub struct Direction {
    pub x: f32,
    pub y: f32,
}
