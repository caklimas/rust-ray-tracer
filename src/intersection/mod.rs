use crate::{ray::Ray, sphere::Sphere};

use self::intersection_computation::IntersectionComputation;

pub mod intersection_computation;
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

    pub fn prepare_computations(&self, ray: &Ray) -> IntersectionComputation<'a> {
        let point = ray.position(self.value);
        let eye_v = ray.direction.negate();
        let mut normal_v = self.object.normal_at(point);
        let mut inside = false;
        if normal_v.dot(&eye_v) < 0.0 {
            normal_v = normal_v.negate();
            inside = true;
        }

        IntersectionComputation::new(self.object, self.value, point, eye_v, normal_v, inside)
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.object == other.object && self.value == other.value
    }
}
