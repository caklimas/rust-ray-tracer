use crate::{color::Color, tuple::Tuple};

#[cfg(test)]
mod tests;

#[derive(PartialEq)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple,
}

impl PointLight {
    pub fn new(intensity: Color, position: Tuple) -> Self {
        if !position.is_point() {
            panic!("Position must be a point");
        }

        Self {
            intensity,
            position,
        }
    }
}
