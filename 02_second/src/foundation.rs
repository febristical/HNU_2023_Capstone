pub use image::{
    ImageFormat::Png,
    ImageBuffer,
    Rgb, ImageResult
};

pub use serde::{
    Serialize,
    Deserialize
};

pub use linear::Vector3;
pub use render::{DistanceField, Camera, Scene};

pub mod linear {
    use crate::*;
    use std::ops::{
        Add,
        Sub,
        Mul,
        Div
    };

    #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
    pub struct Vector3 (f64, f64, f64);

    impl Vector3 {
        pub fn cross(&self, rhs: Self) -> Self {
            Vector3(
                (self.1 * rhs.2) - (self.2 * rhs.1),
                (self.2 * rhs.0) - (self.0 * rhs.2),
                (self.0 * rhs.1) - (self.1 * rhs.0)
            )
        }

        pub fn norm(&self) -> f64 {
            (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
        }
    }
    
    impl Add for Vector3 {
        type Output = Vector3;

        fn add(self, rhs: Self) -> Vector3 {
            Vector3(
                self.0 + rhs.0,
                self.1 + rhs.1,
                self.2 + rhs.2
            )
        }
    }

    impl Sub for Vector3 {
        type Output = Vector3;

        fn sub(self, rhs: Self) -> Vector3 {
            Vector3(
                self.0 - rhs.0,
                self.1 - rhs.1,
                self.2 - rhs.2
            )
        }
    }

    impl Mul<Vector3> for f64 {
        type Output = Vector3;

        fn mul(self, rhs: Vector3) -> Self::Output {
            Vector3(
                self * rhs.0,
                self * rhs.1,
                self * rhs.2
            )
        }
    }

    impl Div<f64> for Vector3 {
        type Output = Vector3;

        fn div(self, rhs: f64) -> Self::Output {
            Vector3(
                self.0 / rhs,
                self.1 / rhs,
                self.2 / rhs
            )
        }
    }
}

pub mod render {
    use crate::*;

    pub trait DistanceField {
        fn distance(&self, point: Vector3) -> f64;
    }

    #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
    pub struct Camera {
        pub current: Vector3,
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
        objects: Vec<Box<dyn DistanceField>>,
        camera: Camera
    }

    impl DistanceField for Scene {
        fn distance(&self, point: Vector3) -> f64 {
            let objects = self.objects.iter();
            let distances = objects
                .map(
                    |x|
                    x.distance(point)
                );

            distances.fold(f64::MAX, |acc, x| acc.min(x))
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
}