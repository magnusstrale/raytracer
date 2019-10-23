use super::color::Color;
use super::tuple::Tuple;
use super::matrix::Matrix;
use super::shape::{Shape, inverse_transform_parameter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pattern {
    a: Color,
    b: Color,
    transform: Matrix,
    inverse_transform: Matrix
}

impl Pattern {
    pub fn stripe_pattern(a: Color, b: Color, transform: Option<Matrix>) -> Self {
        Pattern { 
            a, 
            b, 
            transform: transform.unwrap_or_default(),
            inverse_transform: inverse_transform_parameter(transform)
        }
    }

    pub fn stripe_at(&self, point: Tuple) -> Color {
        if point.x < 0. {
            if point.x.abs() % 2. <= 1. {
                self.b
            } else { 
                self.a 
            }
        } else if point.x % 2. < 1. { 
            self.a 
        } else {
            self.b
        }
    }

    pub fn stripe_at_object(&self, object: &dyn Shape, world_point: Tuple) -> Color {
        let object_point = object.inverse_transformation() * world_point;
        let pattern_point = self.inverse_transform * object_point;
        self.stripe_at(pattern_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{BLACK, WHITE};
    use crate::sphere::Sphere;

    #[test]
    fn create_stripe_pattern() {
        let pattern = Pattern::stripe_pattern(WHITE, BLACK, None);

        assert_eq!(pattern.a, WHITE);
        assert_eq!(pattern.b, BLACK);
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let pattern = Pattern::stripe_pattern(WHITE, BLACK, None);
        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(Tuple::point(0., 1., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(Tuple::point(0., 2., 0.)), WHITE);
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let pattern = Pattern::stripe_pattern(WHITE, BLACK, None);
        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 1.)), WHITE);
        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 2.)), WHITE);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = Pattern::stripe_pattern(WHITE, BLACK, None);
        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(Tuple::point(0.9, 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(Tuple::point(1., 0., 0.)), BLACK);
        assert_eq!(pattern.stripe_at(Tuple::point(-0.1, 0., 0.)), BLACK);
        assert_eq!(pattern.stripe_at(Tuple::point(-1., 0., 0.)), BLACK);
        assert_eq!(pattern.stripe_at(Tuple::point(-1.1, 0., 0.)), WHITE);
    }

    #[test]
    fn stripes_with_object_transformation() {
        let o = Sphere::new(None, Some(Matrix::scaling(2., 2., 2.)));
        let pattern = Pattern::stripe_pattern(WHITE, BLACK, None);
        let c = pattern.stripe_at_object(&o, Tuple::point(1.5, 0., 0.));
        assert_eq!(WHITE, c);
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let o = Sphere::new(None, None);
        let pattern = Pattern::stripe_pattern(WHITE, BLACK, Some(Matrix::scaling(2., 2., 2.)));
        let c = pattern.stripe_at_object(&o, Tuple::point(1.5, 0., 0.));
        assert_eq!(WHITE, c);
    }

    #[test]
    fn patter_with_object_and_pattern_transformation() {
        let o = Sphere::new(None, Some(Matrix::scaling(2., 2., 2.)));
        let pattern = Pattern::stripe_pattern(WHITE, BLACK, Some(Matrix::translation(0.5, 0., 0.)));
        let c = pattern.stripe_at_object(&o, Tuple::point(2.5, 0., 0.));
        assert_eq!(WHITE, c);
    }
}