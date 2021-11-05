use crate::{color::Color, matrix::Matrix};

use super::Pattern;

pub struct Checker {
    a: Color,
    b: Color,
    transform: Matrix,
}

impl Checker {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Default::default(),
        }
    }
}

impl Pattern for Checker {
    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn color_at(&self, point: &crate::tuple::Tuple) -> Color {
        let n = point.x().floor() + point.y().floor() + point.z().floor();
        if (n % 2.0 == 0.0) {
            self.a
        } else {
            self.b
        }
    }
}
