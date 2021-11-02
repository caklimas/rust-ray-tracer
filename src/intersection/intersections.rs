use super::Intersection;

pub struct Intersections<'a> {
    pub collection: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new(collection: Vec<Intersection<'a>>) -> Self {
        Intersections { collection }
    }

    pub fn hit(&self) -> Option<&Intersection<'a>> {
        for intersection in self.collection.iter() {
            if intersection.value >= 0.0 {
                return Option::Some(intersection);
            }
        }

        Option::None
    }
}
