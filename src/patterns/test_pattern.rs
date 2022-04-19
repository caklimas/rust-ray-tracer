use crate::{color::Color, tuple::Tuple};

pub struct TestPattern {}

impl TestPattern {
    pub fn color_at(point: &Tuple) -> Color {
        Color::new(point.x(), point.y(), point.z())
    }
}
