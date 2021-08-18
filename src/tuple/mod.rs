use std::ops::Neg;

use crate::floating_point::FloatingPoint;

#[cfg(test)]
mod tests;

pub const POINT_W: f64 = 1.0;
pub const VECTOR_W: f64 = 0.0;

#[derive(Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64
}

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, POINT_W)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, VECTOR_W)
    }

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple {
            x,
            y,
            z,
            w
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn w(&self) -> f64 {
        self.w
    }

    pub fn is_vector(&self) -> bool {
        self.w == VECTOR_W
    }

    pub fn is_point(&self) -> bool {
        !self.is_vector()
    }

    pub fn add(&self, other: &Self) -> Self {
        if self.is_point() && other.is_point() {
            panic!("Can't add two points together");
        }

        Self {
            x: self.x() + other.x(),
            y: self.y() + other.y(),
            z: self.z() + other.z(),
            w: self.w() + other.w()
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        if self.is_vector() && other.is_point() {
            panic!("Can't subtract a point from a vector");
        }

        Self {
            x: self.x() - other.x(),
            y: self.y() - other.y(),
            z: self.z() - other.z(),
            w: self.w() - other.w()
        }
    }

    pub fn negate(&self) -> Self {
        Self {
            x: self.x().neg(),
            y: self.y().neg(),
            z: self.z().neg(),
            w: self.w().neg()
        }
    }

    pub fn multiply(&self, scalar: f64) -> Self {
        Self {
            x: self.x() * scalar,
            y: self.y() * scalar,
            z: self.z() * scalar,
            w: self.w() * scalar
        }
    }

    pub fn divide(&self, scalar: f64) -> Self {
        Self {
            x: self.x() / scalar,
            y: self.y() / scalar,
            z: self.z() / scalar,
            w: self.w() / scalar
        }
    }

    pub fn magnitude(&self) -> f64 {
        let sum =
            self.x().powi(2) +
            self.y().powi(2) +
            self.z().powi(2) +
            self.w().powi(2);

        sum.sqrt()
    }

    pub fn normalize(&self) -> Self {
        if !self.is_vector() {
            panic!("Normalize can only be done on vectors");
        }

        let magnitude = self.magnitude();
        Self {
            x: self.x() / magnitude,
            y: self.y() / magnitude,
            z: self.z() / magnitude,
            w: self.w() / magnitude
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        (self.x() * other.x()) +
        (self.y() * other.y()) +
        (self.z() * other.z()) +
        (self.w() * other.w())
    }

    pub fn cross(&self, other: &Self) -> Self {
        if !self.is_vector() || !other.is_vector() {
            panic!("Cross product can only be done on vectors");
        }

        Self::vector(
            (self.y() * other.z()) - (self.z() * other.y()),
            (self.z() * other.x()) - (self.x() * other.z()),
            (self.x() * other.y()) - (self.y() * other.x())
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        FloatingPoint::equals(self.x(), other.x()) &&
        FloatingPoint::equals(self.y(), other.y()) &&
        FloatingPoint::equals(self.z(), other.z()) &&
        FloatingPoint::equals(self.w(), other.w())
    }
}