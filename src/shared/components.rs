use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub value: f32,
}

#[derive(Component)]
pub struct Attack {
    pub damage: f32,
    pub range: f32,
}

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component, Clone, Copy)]
pub struct Direction {
    pub x: f32,
    pub y: f32,
}
