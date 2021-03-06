use crate::{
    color::Color, patterns::Pattern, point_light::PointLight, shapes::Shape, tuple::Tuple,
};
use std::ops::RangeInclusive;

#[cfg(test)]
mod tests;

pub const REFLECTION_RANGE: RangeInclusive<f64> = 0.0..=1.0;

pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub pattern: Option<Pattern>,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl Material {
    pub fn new(
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        reflective: f64,
    ) -> Self {
        if !REFLECTION_RANGE.contains(&ambient) {
            panic_reflection("ambient");
        } else if !REFLECTION_RANGE.contains(&diffuse) {
            panic_reflection("diffuse");
        } else if !REFLECTION_RANGE.contains(&specular) {
            panic_reflection("specular");
        } else if !REFLECTION_RANGE.contains(&reflective) {
            panic_reflection("reflective");
        }

        Self {
            color,
            ambient,
            diffuse,
            pattern: Option::None,
            specular,
            shininess,
            reflective,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }

    pub fn lighting(
        &self,
        object: &Shape,
        light: &PointLight,
        position: &Tuple,
        eye: &Tuple,
        normal: &Tuple,
        in_shadow: bool,
    ) -> Color {
        // Combine the surface color with the light's color/intensity
        let color = if let Some(pattern) = &self.pattern {
            pattern.color_at_object(object, position)
        } else {
            self.color
        };
        let effective_color = color * light.intensity;

        // Find the direction to the light source
        let light_v = (&light.position - position).normalize();

        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;

        /*
            light_dot_normal represents the cosine of the angle between the???
            light vector and the normal vector. A negative number means the??? ???
            light is on the other side of the surface.???
        */
        let light_dot_normal = light_v.dot(normal);

        let mut diffuse = Color::black();
        let mut specular = Color::black();
        if light_dot_normal >= 0.0 {
            // Compute diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            /*
                reflect_dot_eye represents the cosine of the angle between the???
                reflection vector and the eye vector. A negative number means the???
                light reflects away from the eye.???
            */
            let reflectv = light_v.negate().reflect(normal);
            let reflect_dot_eye = reflectv.dot(eye);
            if reflect_dot_eye > 0.0 {
                // Comput specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        // Three contribution give final shading
        if in_shadow {
            return ambient;
        }

        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0, 0.0)
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.specular == other.specular
            && self.shininess == other.shininess
    }
}

fn panic_reflection(name: &str) {
    panic!(
        "{} must be between {} and {}",
        name,
        REFLECTION_RANGE.start(),
        REFLECTION_RANGE.end()
    );
}
