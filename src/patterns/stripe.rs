use crate::{color::Color, floating_point::EPSILON, matrix::Matrix, tuple::Tuple};

use super::Pattern;

pub struct Stripe {
    pub a: Color,
    pub b: Color,
    transform: Matrix,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Default::default(),
        }
    }
}

impl Pattern for Stripe {
    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn color_at(&self, point: &Tuple) -> Color {
        if (point.x().floor() % 2.0).abs() < EPSILON {
            self.a
        } else {
            self.b
        }
    }
}