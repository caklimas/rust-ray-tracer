use crate::{color::Color, floating_point::EPSILON, matrix::Matrix, tuple::Tuple};

use super::Pattern;

pub struct Stripe {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
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

    fn color_at(&self, point: &Tuple) -> Color {
        if (point.x().floor() % 2.0).abs() < EPSILON {
            self.a
        } else {
            self.b
        }
    }
}
