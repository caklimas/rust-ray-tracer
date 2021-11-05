use crate::{color::Color, matrix::Matrix, shape::Shape, tuple::Tuple};

pub mod gradient;
pub mod stripe;

#[cfg(test)]
mod tests;

pub trait Pattern {
    fn get_transform(&self) -> &Matrix;
    fn color_at(&self, point: &Tuple) -> Color;
    fn color_at_object(&self, object: &dyn Shape, point: &Tuple) -> Color {
        let object_space = &object.get_transform().inverse() * point;
        let pattern_space = &self.get_transform().inverse() * object_space;
        self.color_at(&pattern_space)
    }
}
