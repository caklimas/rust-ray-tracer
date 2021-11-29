use crate::{floating_point::EPSILON, ray::Ray, shape::Shape};
use std::ptr;

use self::{
    intersection_computation::{IntersectionComputation, IntersectionComputationConfig},
    intersections::Intersections,
};

pub mod intersection_computation;
pub mod intersections;

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct Intersection<'a> {
    pub object: &'a dyn Shape,
    pub value: f64,
}

impl<'a> Intersection<'a> {
    pub fn new(object: &'a dyn Shape, value: f64) -> Self {
        Intersection { object, value }
    }

    pub fn prepare_computations(
        &self,
        ray: &Ray,
        xs: Option<&Intersections<'a>>,
    ) -> IntersectionComputation<'a> {
        let point = ray.position(self.value);
        let eye_v = ray.direction.negate();
        let mut normal_v = self.object.normal_at(point);
        let mut inside = false;
        if normal_v.dot(&eye_v) < 0.0 {
            normal_v = normal_v.negate();
            inside = true;
        }

        let reflect_v = ray.direction.reflect(&normal_v);

        let over_point = point + normal_v * EPSILON;

        let mut n1 = 0.0;
        let mut n2 = 0.0;
        let mut container: Vec<&dyn Shape> = Vec::new();
        if let Some(x) = xs {
            for i in x.collection.iter() {
                if let Some(h) = x.hit() {
                    if ptr::eq(i, h) {
                        if container.is_empty() {
                            n1 = 1.0;
                        } else if let Some(c) = container.last() {
                            n1 = c.get_material().refractive_index;
                        }
                    }
                }

                let mut found_index = Option::None;
                for (j, k) in container.iter().enumerate() {
                    if ptr::eq(&i.object, k) {
                        found_index = Option::Some(j);
                    }
                }

                if let Some(index) = found_index {
                    container.remove(index);
                } else {
                    container.push(i.object);
                }

                if let Some(h) = x.hit() {
                    if ptr::eq(i, h) {
                        if container.is_empty() {
                            n2 = 1.0;
                        } else if let Some(c) = container.last() {
                            n2 = c.get_material().refractive_index;
                        }
                    }

                    break;
                }
            }
        }

        IntersectionComputation::new(
            self.object,
            self.value,
            IntersectionComputationConfig {
                point,
                eye_v,
                normal_v,
                inside,
                over_point,
                reflect_v,
                n1,
                n2,
            },
        )
    }
}
