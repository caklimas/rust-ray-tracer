use super::intersections::Intersections;

pub struct PrepareComputationConfig<'a> {
    pub xs: &'a Intersections<'a>,
}

impl<'a> PrepareComputationConfig<'a> {
    pub fn new(xs: &'a Intersections<'a>) -> Self {
        PrepareComputationConfig { xs }
    }
}
