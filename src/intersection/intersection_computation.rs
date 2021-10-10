use crate::{sphere::Sphere, tuple::Tuple};

pub struct IntersectionComputation<'a> {
    pub object: &'a Sphere,
    pub value: f64,
    pub point: Tuple,
    pub eye_v: Tuple,
    pub normal_v: Tuple,
    pub inside: bool,
}

impl<'a> IntersectionComputation<'a> {
    pub fn new(
        object: &'a Sphere,
        value: f64,
        point: Tuple,
        eye_v: Tuple,
        normal_v: Tuple,
        inside: bool,
    ) -> Self {
        if !point.is_point() {
            panic!("point must be a point");
        }

        if !eye_v.is_vector() {
            panic!("eye_v must be a vector");
        }

        if !normal_v.is_vector() {
            panic!("normal_v must be a vector");
        }

        Self {
            object,
            value,
            point,
            eye_v,
            normal_v,
            inside,
        }
    }
}
