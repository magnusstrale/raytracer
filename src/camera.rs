use std::f64::consts::FRAC_PI_2;
use super::approx_eq;
use super::canvas::Canvas;
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
}