use std::cmp::Ordering;
mod vector3;

use image::{
    ImageFormat::Png,
    ImageBuffer,
    Rgb, ImageResult
};

use serde::{
    Serialize,
    Deserialize
};

use vector3::Vector3;

struct Tracer {
    distance: f64,
    minimum: f64,
    count: usize
}

#[derive(Debug, Serialize, Deserialize)]
struct Camera {
    position: Vector3,
    origin: Vector3,
    righttop: Vector3,
    leftbottom: Vector3,
    width: usize,
    height: usize,
}

impl Camera {
    fn direction(&self, xratio: f64, yratio: f64) -> Vector3 {
        let guide: f64 = 1.0 - xratio - yratio;
        let long: Vector3 = guide * &self.origin 
            + xratio * &self.righttop 
            + yratio * &self.righttop;

        long.norm().recip() * long
    }
}

struct Scene {
    objects: Vec<Box<dyn DistanceField>>,
    camera: Camera
}

trait DistanceField {
    fn distance(&self, point: &Vector3) -> f64;
}

impl DistanceField for Scene {
    fn distance(&self, point: &Vector3) -> f64 {
        let objects = self.objects.iter();
        let distances = objects
            .map(
                |x|
                x.distance(point)
            );

        distances.min_by(
            |x, y|
            if x < y { Ordering::Less } else { Ordering::Greater })
            .unwrap()
    }
}

impl Scene {
    fn march(&self, x: usize, y: usize, tracer: Tracer) -> Tracer {
        let (xratio, yratio) = (
            x as f64 / self.camera.width as f64,
            y as f64 / self.camera.height as f64);
        let direction = self.camera.direction(xratio, yratio);
        tracer
    }
}

struct Genus {
    index: usize,
    hole_radius: f64,
    border_radius: f64
}