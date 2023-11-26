use std::ops::{
    Neg,
    Add,
    Sub,
    Mul,
    Div
};

#[derive(Debug, Clone, Copy)]
pub struct Vector3 (
    pub f32,
    pub f32,
    pub f32
);

impl Vector3 {
    pub fn cross(&self, rhs: Self) -> Self {
        Vector3(
            (self.1 * rhs.2) - (self.2 * rhs.1),
            (self.2 * rhs.0) - (self.0 * rhs.2),
            (self.0 * rhs.1) - (self.1 * rhs.0)
        )
    }

    pub fn norm(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3(
            - self.0,
            - self.1,
            - self.2
        )
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

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2
        )
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3(
            self * rhs.0,
            self * rhs.1,
            self * rhs.2
        )
    }
}

impl Div for Vector3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vector3(
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2
        )
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3(
            self.0 / rhs,
            self.1 / rhs,
            self.2 / rhs
        )
    }
}