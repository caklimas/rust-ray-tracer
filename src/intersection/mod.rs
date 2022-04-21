use crate::{floating_point::EPSILON, ray::Ray, shapes::Shape};
use std::ptr;

use self::{
    intersection_computation::{IntersectionComputation, IntersectionComputationConfig},
    prepare_computation::PrepareComputationConfig,
};

pub mod intersection_computation;
pub mod intersections;
pub mod prepare_computation;

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct Intersection<'a> {
    pub object: &'a Shape,
    pub value: f64,
}

impl<'a> Intersection<'a> {
    pub fn new(object: &'a Shape, value: f64) -> Self {
        Intersection { object, value }
    }

    pub fn prepare_computations(
        &self,
        ray: &Ray,
        config: Option<&PrepareComputationConfig>,
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
        let under_point = point - normal_v * EPSILON;
        let (n1, n2) = self.determine_n1_n2(config);

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
                under_point,
            },
        )
    }

    #[allow(clippy::vtable_address_comparisons)]
    fn determine_n1_n2(&self, config: Option<&PrepareComputationConfig>) -> (f64, f64) {
        let mut n1 = 0.0;
        let mut n2 = 0.0;
        let mut container: Vec<&Shape> = Vec::new();
        if let Some(pc) = config {
            for i in pc.xs.collection.iter() {
                let is_intersection = ptr::eq(self, i);
                if is_intersection {
                    n1 = self.get_refractive_index(&container);
                }

                let mut found_index = Option::None;
                for (j, k) in container.iter().enumerate() {
                    if std::ptr::eq(i.object, *k) {
                        found_index = Option::Some(j);
                        break;
                    }
                }

                if let Some(index) = found_index {
                    // The intersection must be exiting the object
                    container.remove(index);
                } else {
                    // The intersection is entering the object
                    container.push(i.object);
                }

                if is_intersection {
                    n2 = self.get_refractive_index(&container);
                    break;
                }
            }
        }

        (n1, n2)
    }

    fn get_refractive_index(&self, container: &[&Shape]) -> f64 {
        if let Some(c) = container.last() {
            c.get_material().refractive_index
        } else {
            // There is no containing object
            1.0
        }
    }
}
