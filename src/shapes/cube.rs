use std::f64::INFINITY;

use crate::{floating_point::EPSILON, material::Material, matrix::Matrix, ray::Ray, tuple::Tuple};

#[derive(Default)]
pub struct Cube {
    pub material: Material,
    pub transform: Matrix,
}

impl Cube {
    pub fn new() -> Self {
        Cube::default()
    }

    pub fn local_intersect(&self, ray: &Ray) -> Option<Vec<f64>> {
        let (xt_min, xt_max) = self.check_axis(ray.origin.x(), ray.direction.x());
        let (yt_min, yt_max) = self.check_axis(ray.origin.y(), ray.direction.y());
        let (zt_min, zt_max) = self.check_axis(ray.origin.z(), ray.direction.z());

        let t_min = xt_min.max(yt_min.max(zt_min));
        let t_max = xt_max.min(yt_max.min(zt_max));

        if t_min > t_max {
            Option::None
        } else {
            Option::Some(vec![t_min, t_max])
        }
    }

    pub fn local_normal(&self, object_point: Tuple) -> Tuple {
        let max_c = object_point
            .x()
            .abs()
            .max(object_point.y().abs().max(object_point.z().abs()));

        if max_c == object_point.x().abs() {
            Tuple::vector(object_point.x(), 0.0, 0.0)
        } else if max_c == object_point.y().abs() {
            Tuple::vector(0.0, object_point.y(), 0.0)
        } else {
            Tuple::vector(0.0, 0.0, object_point.z())
        }
    }

    fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let (tmin, tmax) = if direction.abs() >= EPSILON {
            (tmin_numerator / direction, tmax_numerator / direction)
        } else {
            (tmin_numerator * INFINITY, tmax_numerator * INFINITY)
        };

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }
}
