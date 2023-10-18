mod foundation;
use foundation::*;

struct Tracer {
    distance: f64,
    minimum: f64,
    count: usize
}

struct Position {
    object: Box<dyn DistanceField>,
    position: Vector3
}

impl DistanceField for Position {
    fn distance(&self, point: Vector3) -> f64 {
        self.object.distance(point - self.position)
    }
}

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