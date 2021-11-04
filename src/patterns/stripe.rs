use crate::{color::Color, floating_point::EPSILON, tuple::Tuple};

use super::Pattern;

pub struct Stripe {
    pub a: Color,
    pub b: Color,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }
}

impl Pattern for Stripe {
    fn color_at(&self, point: Tuple) -> Color {
        if (point.x().floor() % 2.0).abs() < EPSILON {
            self.a
        } else {
            self.b
        }
    }
}
