use crate::{shapes::Shape, tuple::Tuple};

pub struct IntersectionComputation<'a> {
    pub object: &'a Shape,
    pub value: f64,
    pub point: Tuple,
    pub eye_v: Tuple,
    pub normal_v: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub reflect_v: Tuple,
    pub n1: f64,
    pub n2: f64,
    pub under_point: Tuple,
}

pub struct IntersectionComputationConfig {
    pub point: Tuple,
    pub eye_v: Tuple,
    pub normal_v: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub reflect_v: Tuple,
    pub n1: f64,
    pub n2: f64,
    pub under_point: Tuple,
}

impl<'a> IntersectionComputation<'a> {
    pub fn new(object: &'a Shape, value: f64, config: IntersectionComputationConfig) -> Self {
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

        if !config.under_point.is_point() {
            panic!("under_point must be a point");
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
            n1: config.n1,
            n2: config.n2,
            under_point: config.under_point,
        }
    }

    pub fn schlick(&self) -> f64 {
        // find the cosine of the angle between the eye and normal vectors​
        let mut cos = self.eye_v.dot(&self.normal_v);

        // total internal reflection can only occur if n1 > n2​
        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }

            // compute cosine of theta_t using trig identity​
            cos = (1.0 - sin2_t).sqrt();
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}
