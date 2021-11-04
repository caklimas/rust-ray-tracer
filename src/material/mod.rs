use crate::{color::Color, patterns::Pattern, point_light::PointLight, tuple::Tuple};
use std::ops::RangeInclusive;

#[cfg(test)]
mod tests;

pub const REFLECTION_RANGE: RangeInclusive<f64> = 0.0..=1.0;

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        if !REFLECTION_RANGE.contains(&ambient) {
            panic_reflection("ambient");
        } else if !REFLECTION_RANGE.contains(&diffuse) {
            panic_reflection("diffuse");
        } else if !REFLECTION_RANGE.contains(&specular) {
            panic_reflection("specular");
        }

        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn lighting(
        &self,
        light: &PointLight,
        position: &Tuple,
        eye: &Tuple,
        normal: &Tuple,
        in_shadow: bool,
    ) -> Color {
        // Combine the surface color with the light's color/intensity
        let effective_color = self.color * light.intensity;

        // Find the direction to the light source
        let light_v = (&light.position - position).normalize();

        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;

        /*
            light_dot_normal represents the cosine of the angle between the​
            light vector and the normal vector. A negative number means the​ ​
            light is on the other side of the surface.​
        */
        let light_dot_normal = light_v.dot(normal);

        let mut diffuse = Color::black();
        let mut specular = Color::black();
        if light_dot_normal >= 0.0 {
            // Compute diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            /*
                reflect_dot_eye represents the cosine of the angle between the​
                reflection vector and the eye vector. A negative number means the​
                light reflects away from the eye.​
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
        Material::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0)
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
