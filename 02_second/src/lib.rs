mod foundations;
mod operators;

use foundations::*;

struct Genus2 {
    hole_radius: f64,
    border_radius: f64,
    center_distance: f64
}

impl DistanceField for Genus2 {
    fn distance(&self, point: Vector3) -> f64 {
        0.0
    }
}