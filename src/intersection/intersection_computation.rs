use crate::{shape::Shape, tuple::Tuple};

pub struct IntersectionComputation<'a> {
    pub object: &'a dyn Shape,
    pub value: f64,
    pub point: Tuple,
    pub eye_v: Tuple,
    pub normal_v: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

impl<'a> IntersectionComputation<'a> {
    pub fn new(
        object: &'a dyn Shape,
        value: f64,
        point: Tuple,
        eye_v: Tuple,
        normal_v: Tuple,
        inside: bool,
        over_point: Tuple,
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

        if !over_point.is_point() {
            panic!("over_point must be a point");
        }

        Self {
            object,
            value,
            point,
            eye_v,
            normal_v,
            inside,
            over_point,
        }
    }
}
