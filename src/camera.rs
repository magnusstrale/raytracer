use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2};
use super::approx_eq;
use super::canvas::Canvas;
use super::tuple::{Tuple, ORIGO};
use super::ray::Ray;
use super::matrix::{Matrix, IDENTITY_MATRIX};


struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub pixel_size: f64,
    pub transform: Matrix,
    half_width: f64,
    half_height: f64,
    canvas: Canvas
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64, transform: Option<Matrix>) -> Self {
        let half_view = (field_of_view / 2.).tan();
        let aspect_ratio = hsize as f64 / vsize as f64;
        let (half_width, half_height) = if aspect_ratio >= 1. {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };
        let pixel_size = half_width * 2. / hsize as f64;
        Camera { 
            hsize, 
            vsize, 
            field_of_view,
            pixel_size,
            transform: transform.unwrap_or(IDENTITY_MATRIX), 
            half_width,
            half_height,
            canvas: Canvas::new(hsize, vsize) }
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        let pixel = self.transform.inverse().unwrap() * Tuple::point(world_x, world_y, -1.);
        let origin = self.transform.inverse().unwrap() * ORIGO;
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_camera() {
        let c = Camera::new(160, 120, FRAC_PI_2, None);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, FRAC_PI_2);
        assert_eq!(c.transform, IDENTITY_MATRIX);
    }

    #[test]
    fn pixel_size_horizontal_canvas() {
        let c = Camera::new(200, 125, FRAC_PI_2, None);
        assert!(approx_eq(c.pixel_size, 0.01));
    }

    #[test]
    fn pixel_size_vertical_canvas() {
        let c = Camera::new(125, 200, FRAC_PI_2, None);
        assert!(approx_eq(c.pixel_size, 0.01));
    }

    #[test]
    fn construct_ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, FRAC_PI_2, None);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, ORIGO);
        assert_eq!(r.direction, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn construct_ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, FRAC_PI_2, None);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, ORIGO);
        assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn construct_ray_when_camera_transformed() {
        let t = Matrix::rotation_y(FRAC_PI_4) * Matrix::translation(0., -2., 5.);
        let c = Camera::new(201, 101, FRAC_PI_2, Some(t));
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0., 2., -5.));
        assert_eq!(r.direction, Tuple::vector(SQRT_2 / 2., 0., -SQRT_2 / 2.));
    }
}