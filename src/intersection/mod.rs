use crate::sphere::Sphere;

pub mod intersections;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct Intersection<'a> {
    pub object: &'a Sphere,
    pub value: f64,
}

impl<'a> Intersection<'a> {
    pub fn new(object: &'a Sphere, value: f64) -> Self {
        Intersection { object, value }
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.object == other.object && self.value == other.value
    }
}
