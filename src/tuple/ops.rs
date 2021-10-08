use crate::floating_point::FloatingPoint;
use std::{ops::{Add, Div, Mul, Sub}};
use super::Tuple;

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        FloatingPoint::equals(self.x(), other.x()) &&
        FloatingPoint::equals(self.y(), other.y()) &&
        FloatingPoint::equals(self.z(), other.z()) &&
        FloatingPoint::equals(self.w(), other.w())
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_point() && rhs.is_point() {
            panic!("Can't add two points together");
        }

        Self {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
            z: self.z() + rhs.z(),
            w: self.w() + rhs.w()
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        if self.is_vector() && rhs.is_point() {
            panic!("Can't subtract a point from a vector");
        }

        Self {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
            z: self.z() - rhs.z(),
            w: self.w() - rhs.w()
        }
    }
}

impl Sub for &Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Tuple {
        if self.is_vector() && rhs.is_point() {
            panic!("Can't subtract a point from a vector");
        }

        Tuple {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
            z: self.z() - rhs.z(),
            w: self.w() - rhs.w()
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x() * rhs,
            y: self.y() * rhs,
            z: self.z() * rhs,
            w: self.w() * rhs
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x() / rhs,
            y: self.y() / rhs,
            z: self.z() / rhs,
            w: self.w() / rhs
        }
    }
}