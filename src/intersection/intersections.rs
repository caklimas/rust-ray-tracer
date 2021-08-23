use super::Intersection;

pub struct Intersections<'a> {
    pub collection: Vec<Intersection<'a>>
}

impl <'a> Intersections<'a> {
    pub fn new(collection: &mut Vec<Intersection<'a>>) -> Self {
        collection.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
        let mut intersections = Vec::new();
        for i in collection.iter() {
            intersections.push(Intersection::new(i.object, i.value));
        }
        Intersections {
            collection: intersections
        }
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