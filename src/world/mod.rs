use crate::{
    color::Color, intersection::Intersection, matrix::transformation::scale,
    point_light::PointLight, ray::Ray, sphere::Sphere, tuple::Tuple,
};

#[cfg(test)]
mod tests;

pub struct World {
    pub light: PointLight,
    pub objects: Vec<Sphere>,
}

impl World {
    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        for o in self.objects.iter() {
            intersections.append(&mut o.intersect(&ray));
        }

        intersections.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
        intersections
    }
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Color::white(), Tuple::point(-10.0, -10.0, -10.0));
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.transform = scale(0.5, 0.5, 0.5);

        Self {
            light,
            objects: vec![s1, s2],
        }
    }
}
