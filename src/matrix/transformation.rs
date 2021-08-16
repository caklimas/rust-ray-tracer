use super::{Matrix, axis::Axis};

impl Matrix {
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut translation = Matrix::identity(4);
        translation.elements[0][3] = x;
        translation.elements[1][3] = y;
        translation.elements[2][3] = z;

        translation
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mut scaling = Matrix::identity(4);
        scaling.elements[0][0] = x;
        scaling.elements[1][1] = y;
        scaling.elements[2][2] = z;

        scaling
    }

    pub fn reflect(axis: Axis) -> Self {
        return match axis {
            Axis::X => Matrix::scaling(-1.0, 1.0, 1.0),
            Axis::Y => Matrix::scaling(1.0, -1.0, 1.0),
            Axis::Z => Matrix::scaling(1.0, 1.0, -1.0)
        };
    }

    pub fn rotate_x(radians: f64) -> Self {
        let mut rotation = Matrix::identity(4);
        let cos = radians.cos();
        let sin = radians.sin();
        rotation.elements[1][1] = cos;
        rotation.elements[1][2] = -sin;
        rotation.elements[2][1] = sin;
        rotation.elements[2][2] = cos;

        rotation

    }
}