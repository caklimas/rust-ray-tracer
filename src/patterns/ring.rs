use crate::{color::Color, tuple::Tuple};

pub struct RingPattern {
    a: Color,
    b: Color,
}

impl RingPattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn color_at(&self, point: &Tuple) -> Color {
        let n = (point.x().powi(2) + point.z().powi(2)).sqrt().floor();
        if n % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}
