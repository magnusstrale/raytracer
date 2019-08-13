use super::matrix::Matrix;
use super::tuple::Tuple;

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

}