use bevy::transform::components::Transform;

pub trait DistanceFrom  {
    fn distance_from(&self, t: &Transform) -> f32;
}

impl DistanceFrom for Transform {
    fn distance_from(&self, t: &Transform) -> f32 {
        let x_dist = self.translation.x - t.translation.x;
        let y_dist = self.translation.y - t.translation.y;
    
        (x_dist * x_dist + y_dist * y_dist).sqrt()
    }
}