use bevy::prelude::*;

pub fn get_distance(a: &Transform, b: &Transform) -> f32 {
    let x_dist = a.translation.x - b.translation.x;
    let y_dist = a.translation.y - b.translation.y;

    (x_dist * x_dist + y_dist * y_dist).sqrt()
}
