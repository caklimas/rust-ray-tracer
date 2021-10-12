use crate::{matrix::Matrix, ray::Ray, tuple::Tuple};

#[cfg(test)]
mod tests;

pub struct Camera {
    horizontal_size: usize,
    vertical_size: usize,
    field_of_view: f64,
    transform: Matrix,
    half_width: f64,
    half_height: f64,
    pixel_size: f64
}

impl Camera {
    pub fn new(horizontal_size: usize,
        vertical_size: usize,
        field_of_view: f64) -> Self {

            let half_view = (field_of_view / 2.0).tan();
        let aspect = (horizontal_size as f64) / (vertical_size as f64);
        let half_width: f64;
        let half_height: f64;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / (aspect as f64);
        } else {
            half_width = half_view * (aspect as f64);
            half_height = half_view;
        }

        Camera {
            horizontal_size,
            vertical_size,
            field_of_view,
            transform: Matrix::identity(4),
            half_width,
            half_height,
            pixel_size: (half_width * 2.0) / (horizontal_size as f64)
        }
    }

    pub fn horizontal_size(&self) -> &usize {
        &self.horizontal_size
    }

    pub fn vertical_size(&self) -> &usize {
        &self.vertical_size
    }

    pub fn field_of_view(&self) -> &f64 {
        &self.field_of_view
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn half_width(&self) -> &f64 {
        &self.half_width
    }

    pub fn half_height(&self) -> &f64 {
        &self.half_height
    }

    pub fn pixel_size(&self) -> &f64 {
        &self.pixel_size
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let x_offset = ((x as f64) + 0.5) * self.pixel_size;
        let y_offset = ((y as f64) + 0.5) * self.pixel_size;
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;
        let pixel = self.transform.inverse() * Tuple::point(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Tuple::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }
}