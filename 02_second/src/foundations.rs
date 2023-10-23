pub use image::{
    ImageFormat::Png,
    ImageBuffer,
    Rgba, ImageResult
};

pub use serde::{
    Serialize,
    Deserialize
};

pub use linear::Vector3;
pub use render::*;

pub mod render {
    use core::fmt;

    use crate::*;

    pub trait UVMap {
        fn uvmap(&self, point: Vector3) -> (f64, f64);
    }

    pub trait DistanceField { 
        fn distance(&self, point: Vector3) -> f64;
    }

    pub struct Tracer {
        pub current: Vector3,
        pub direction: Vector3,
        distance: f64,
        minimum: f64,
        count: usize
    }
    #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
    pub struct Camera {
        origin: Vector3,
        righttop: Vector3,
        leftbottom: Vector3,
        width: usize,
        height: usize,
    }

    impl Camera {
        fn direction(&self, xratio: f64, yratio: f64) -> Vector3 {
            let guide: f64 = 1.0 - xratio - yratio;
            let long: Vector3 =
                guide * self.origin + xratio * self.righttop + yratio * self.righttop;

            long.norm().recip() * long
        }
    }

    pub struct Scene {
        camera: Camera,
        objects: Vec<Box<dyn DistanceField>>
    }

    impl<'de> DistanceField for Scene {
        fn distance(&self, point: Vector3) -> f64 {
            self.objects
                .iter()
                .fold(
                    f64::MAX, 
                    |min, object|
                        object.distance(point).min(min)
                )
        }
    }

    impl Scene { // have to complete
        fn march(&self, x: usize, y: usize, tracer: Tracer) -> Tracer {
            let (xratio, yratio) = (
                x as f64 / self.camera.width as f64,
                y as f64 / self.camera.height as f64);
            let direction = self.camera.direction(xratio, yratio);
            tracer
        }
    }
}