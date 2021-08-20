use super::{Matrix, axis::Axis};

pub fn translate(x: f64, y: f64, z: f64) -> Matrix {
    let mut translation = Matrix::identity(4);
    translation.elements[0][3] = x;
    translation.elements[1][3] = y;
    translation.elements[2][3] = z;

    translation
}

pub fn scale(x: f64, y: f64, z: f64) -> Matrix {
    let mut scaling = Matrix::identity(4);
    scaling.elements[0][0] = x;
    scaling.elements[1][1] = y;
    scaling.elements[2][2] = z;

    scaling
}

pub fn reflect(axis: Axis) -> Matrix {
    match axis {
        Axis::X => scale(-1.0, 1.0, 1.0),
        Axis::Y => scale(1.0, -1.0, 1.0),
        Axis::Z => scale(1.0, 1.0, -1.0)
    }
}

pub fn rotate_x(radians: f64) -> Matrix {
    let mut rotation = Matrix::identity(4);
    let cos = radians.cos();
    let sin = radians.sin();
    rotation.elements[1][1] = cos;
    rotation.elements[1][2] = -sin;
    rotation.elements[2][1] = sin;
    rotation.elements[2][2] = cos;

    rotation

}

pub fn rotate_y(radians: f64) -> Matrix {
    let mut rotation = Matrix::identity(4);
    let cos = radians.cos();
    let sin = radians.sin();
    rotation.elements[0][0] = cos;
    rotation.elements[0][2] = sin;
    rotation.elements[2][0] = -sin;
    rotation.elements[2][2] = cos;

    rotation

}

pub fn rotate_z(radians: f64) -> Matrix {
    let mut rotation = Matrix::identity(4);
    let cos = radians.cos();
    let sin = radians.sin();
    rotation.elements[0][0] = cos;
    rotation.elements[0][1] = -sin;
    rotation.elements[1][0] = sin;
    rotation.elements[1][1] = cos;

    rotation

}

pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut shearing = Matrix::identity(4);
    shearing.elements[0][1] = xy;
    shearing.elements[0][2] = xz;
    shearing.elements[1][0] = yx;
    shearing.elements[1][2] = yz;
    shearing.elements[2][0] = zx;
    shearing.elements[2][1] = zy;

    shearing
}

impl Matrix {
    pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
        translate(x, y, z) * self
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
        scale(x, y, z) * self
    }

    pub fn reflect(self, axis: Axis) -> Self {
        reflect(axis) * self
    }

    pub fn rotate_x(self, radians: f64) -> Self {
        rotate_x(radians) * self
    }

    pub fn rotate_y(self, radians: f64) -> Self {
        rotate_y(radians) * self
    }

    pub fn rotate_z(self, radians: f64) -> Self {
        rotate_z(radians) * self
    }

    pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        shear(xy, xz, yx, yz, zx, zy) * self
    }
}