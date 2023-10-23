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