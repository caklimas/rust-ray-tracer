use super::{Matrix, axis::Axis};

impl Matrix {
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut translation = Self::identity(4);
        translation.elements[0][3] = x;
        translation.elements[1][3] = y;
        translation.elements[2][3] = z;

        translation
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mut scaling = Self::identity(4);
        scaling.elements[0][0] = x;
        scaling.elements[1][1] = y;
        scaling.elements[2][2] = z;

        scaling
    }

    pub fn reflect(axis: Axis) -> Self {
        return match axis {
            Axis::X => Self::scaling(-1.0, 1.0, 1.0),
            Axis::Y => Self::scaling(1.0, -1.0, 1.0),
            Axis::Z => Self::scaling(1.0, 1.0, -1.0)
        };
    }

    pub fn rotate_x(radians: f64) -> Self {
        let mut rotation = Self::identity(4);
        let cos = radians.cos();
        let sin = radians.sin();
        rotation.elements[1][1] = cos;
        rotation.elements[1][2] = -sin;
        rotation.elements[2][1] = sin;
        rotation.elements[2][2] = cos;

        rotation

    }

    pub fn rotate_y(radians: f64) -> Self {
        let mut rotation = Self::identity(4);
        let cos = radians.cos();
        let sin = radians.sin();
        rotation.elements[0][0] = cos;
        rotation.elements[0][2] = sin;
        rotation.elements[2][0] = -sin;
        rotation.elements[2][2] = cos;

        rotation

    }

    pub fn rotate_z(radians: f64) -> Self {
        let mut rotation = Self::identity(4);
        let cos = radians.cos();
        let sin = radians.sin();
        rotation.elements[0][0] = cos;
        rotation.elements[0][1] = -sin;
        rotation.elements[1][0] = sin;
        rotation.elements[1][1] = cos;

        rotation

    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let mut shearing = Self::identity(4);
        shearing.elements[0][1] = xy;
        shearing.elements[0][2] = xz;
        shearing.elements[1][0] = yx;
        shearing.elements[1][2] = yz;
        shearing.elements[2][0] = zx;
        shearing.elements[2][1] = zy;

        shearing
    }
}