use super::matrix::Matrix;
use super::tuple::Tuple;
use std::f64::consts::*;

impl Matrix {
    fn translation(x: f64, y: f64, z: f64) -> Matrix {
        let mut m = Matrix::identity_matrix();
        m.set(0, 3, x);
        m.set(1, 3, y);
        m.set(2, 3, z);
        m
    }

    fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        let mut m = Matrix::identity_matrix();
        m.set(0, 0, x);
        m.set(1, 1, y);
        m.set(2, 2, z);
        m
    }

    fn rotation_x(rad: f64) -> Matrix {
        let mut m = Matrix::identity_matrix();
        m.set(1, 1, rad.cos());
        m.set(1, 2, -rad.sin());
        m.set(2, 1, rad.sin());
        m.set(2, 2, rad.cos());
        m
    }

    fn rotation_y(rad: f64) -> Matrix {
        let mut m = Matrix::identity_matrix();
        m.set(0, 0, rad.cos());
        m.set(0, 2, rad.sin());
        m.set(2, 0, -rad.sin());
        m.set(2, 2, rad.cos());
        m
    }

    fn rotation_z(rad: f64) -> Matrix {
        let mut m = Matrix::identity_matrix();
        m.set(0, 0, rad.cos());
        m.set(0, 1, -rad.sin());
        m.set(1, 0, rad.sin());
        m.set(1, 1, rad.cos());
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_by_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        let actual = transform * p;
        let expected = Tuple::point(2.0, 1.0, 7.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiply_by_inverted_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let p = Tuple::point(-3.0, 4.0, 5.0);
        let actual = inv * p;
        let expected = Tuple::point(-8.0, 7.0, 3.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn translation_does_not_change_vector() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        let actual = transform * v;
        assert_eq!(actual, v);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        let actual = transform * p;
        let expected = Tuple::point(-8.0, 18.0, 32.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        let actual = transform * v;
        let expected = Tuple::vector(-8.0, 18.0, 32.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn inverted_scaling_matrix_applied_to_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        let actual = inv * v;
        let expected = Tuple::vector(-2.0, 2.0, 2.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        let actual = transform * p;
        let expected = Tuple::point(-2.0, 3.0, 4.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn rotate_point_around_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(FRAC_PI_4);
        let full_quarter = Matrix::rotation_x(FRAC_PI_2);
        let actual_half_quarter = half_quarter * p;
        let actual_full_quarter = full_quarter * p;

        assert_eq!(actual_half_quarter, Tuple::point(0.0, 2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0));
        assert_eq!(actual_full_quarter, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_rotate_point_around_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(FRAC_PI_4);
        let inv = half_quarter.inverse().unwrap();
        let actual = inv * p;
        let expected = Tuple::point(0.0, 2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn rotate_point_around_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(FRAC_PI_4);
        let full_quarter = Matrix::rotation_y(FRAC_PI_2);
        let actual_half_quarter = half_quarter * p;
        let actual_full_quarter = full_quarter * p;

        assert_eq!(actual_half_quarter, Tuple::point(2.0_f64.sqrt()/2.0, 0.0, 2.0_f64.sqrt()/2.0));
        assert_eq!(actual_full_quarter, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotate_point_around_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(FRAC_PI_4);
        let full_quarter = Matrix::rotation_z(FRAC_PI_2);
        let actual_half_quarter = half_quarter * p;
        let actual_full_quarter = full_quarter * p;

        assert_eq!(actual_half_quarter, Tuple::point(-2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0, 0.0));
        assert_eq!(actual_full_quarter, Tuple::point(-1.0, 0.0, 0.0));
    }
}