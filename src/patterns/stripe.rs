use crate::{color::Color, floating_point::EPSILON, matrix::Matrix, shape::Shape, tuple::Tuple};

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
    fn color_at(&self, point: &Tuple) -> Color {
        if (point.x().floor() % 2.0).abs() < EPSILON {
            self.a
        } else {
            self.b
        }
    }

    fn color_at_object(&self, object: Box<&dyn Shape>, point: &Tuple) -> Color {
        let object_space = &object.get_transform().inverse() * point;
        let pattern_space = &self.transform.inverse() * &object_space;
        self.color_at(&pattern_space)
    }
}
