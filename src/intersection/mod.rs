use crate::{ray::Ray, sphere::Sphere};

use self::intersection_computation::IntersectionComputation;

pub mod intersection_computation;

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

    pub fn prepare_computations(&self, ray: &Ray) -> IntersectionComputation {
        let point = ray.position(self.value);
        IntersectionComputation::new(
            self.value,
            point,
            ray.direction.negate(),
            self.object.normal_at(point),
        )
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.object == other.object && self.value == other.value
    }
}
