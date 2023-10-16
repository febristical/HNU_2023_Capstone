use serde::{
    Serialize,
    Deserialize
};

use std::ops::{
    Add,
    Mul
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vector3 (pub f64, pub f64, pub f64);

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

impl Add<&Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Self) -> Vector3 {
        Vector3(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2
        )
    }
}

impl Mul<&Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        Vector3(
            self * rhs.0,
            self * rhs.1,
            self * rhs.2
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