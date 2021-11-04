use crate::{color::Color, floating_point::EPSILON, tuple::Tuple};

pub struct Stripe {
    pub a: Color,
    pub b: Color,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn at(&self, point: Tuple) -> Color {
        if (point.x().floor() % 2.0).abs() < EPSILON {
            self.a
        } else {
            self.b
        }
    }
}
