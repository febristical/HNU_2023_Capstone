use crate::linear::*;

enum Shape {
    Box{width: f32, height: f32, depth: f32},
    Ball{radius: f32},
    Torus{inner: f32, outer: f32},
    Genus{inner: f32, outer: f32, index: f32}
}

enum Unary {
    Bump{bumps: Vec<f32>, width: u32, height: u32},
    Blow{radius: f32},
    Hair{density: f32, length: f32},
    Twist{ratio: f32, direction: Vector3},
    Roll{angle: f32, origin: Vector3},
    Stretch{amount: f32, direction: Vector3},
    Translate{vector: Vector3},
    Rotate{angle: f32, axis: Vector3},
    Scale{factor: f32},
    None,
}