use crate::{color::Color, shape::Shape, tuple::Tuple};

pub mod stripe;

#[cfg(test)]
mod tests;

pub trait Pattern {
    fn color_at(&self, point: &Tuple) -> Color;
    fn color_at_object(&self, object: Box<&dyn Shape>, point: &Tuple) -> Color;
}
