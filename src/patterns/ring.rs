use crate::{color::Color, matrix::Matrix};

use super::Pattern;

pub struct Ring {
    a: Color,
    b: Color,
    transform: Matrix,
}

impl Ring {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Default::default(),
        }
    }
}

impl Pattern for Ring {
    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn color_at(&self, point: &crate::tuple::Tuple) -> Color {
        let n = (point.x().powi(2) + point.z().powi(2)).sqrt().floor();
        if n % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}
