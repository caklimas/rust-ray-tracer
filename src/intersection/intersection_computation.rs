use crate::{shape::Shape, tuple::Tuple};

pub struct IntersectionComputation<'a> {
    pub object: &'a dyn Shape,
    pub value: f64,
    pub point: Tuple,
    pub eye_v: Tuple,
    pub normal_v: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub reflect_v: Tuple,
}

pub struct IntersectionComputationConfig {
    pub point: Tuple,
    pub eye_v: Tuple,
    pub normal_v: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub reflect_v: Tuple,
}

impl<'a> IntersectionComputation<'a> {
    pub fn new(object: &'a dyn Shape, value: f64, config: IntersectionComputationConfig) -> Self {
        if !config.point.is_point() {
            panic!("point must be a point");
        }

        if !config.eye_v.is_vector() {
            panic!("eye_v must be a vector");
        }

        if !config.normal_v.is_vector() {
            panic!("normal_v must be a vector");
        }

        if !config.over_point.is_point() {
            panic!("over_point must be a point");
        }

        if !config.reflect_v.is_vector() {
            panic!("reflect_v must be a vector");
        }

        Self {
            object,
            value,
            point: config.point,
            eye_v: config.eye_v,
            normal_v: config.normal_v,
            inside: config.inside,
            over_point: config.over_point,
            reflect_v: config.reflect_v,
        }
    }
}
