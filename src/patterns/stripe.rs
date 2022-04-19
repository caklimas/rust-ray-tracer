use crate::{color::Color, floating_point::EPSILON, tuple::Tuple};

pub struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn color_at(&self, point: &Tuple) -> Color {
        if (point.x().floor() % 2.0).abs() < EPSILON {
            self.a
        } else {
            self.b
        }
    }
}
