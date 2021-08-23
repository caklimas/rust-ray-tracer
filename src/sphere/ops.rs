use super::Sphere;

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center
    }
}