use std::fmt;
use std::any::Any;

use super::color::Color;
use super::tuple::Tuple;
use super::matrix::Matrix;
use super::shape::{Shape, inverse_transform_parameter};

pub trait Pattern: Any + fmt::Debug {
    fn box_clone(&self) -> BoxPattern;
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn transformation(&self) -> Matrix;
    fn inverse_transformation(&self) -> Matrix;
    fn inner_pattern_at(&self, pattern_point: Tuple) -> Color;
    fn pattern_at_shape(&self, object: &dyn Shape, world_point: Tuple) -> Color {
        let object_point = object.inverse_transformation() * world_point;
        let pattern_point = self.inverse_transformation() * object_point;
        self.inner_pattern_at(pattern_point)
    }
}

pub type BoxPattern = Box<dyn Pattern>;

impl Clone for BoxPattern {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl PartialEq for BoxPattern {
    fn eq(&self, other: &Self) -> bool {
        self.box_eq(other.as_any())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StripePattern {
    a: Color,
    b: Color,
    transform: Matrix,
    inverse_transform: Matrix
}

impl StripePattern {
    pub fn new(a: Color, b: Color, transform: Option<Matrix>) -> Self {
        StripePattern { 
            a, 
            b, 
            transform: transform.unwrap_or_default(),
            inverse_transform: inverse_transform_parameter(transform)
        }
    }

    pub fn new_boxed(a: Color, b: Color, transform: Option<Matrix>) -> BoxPattern {
        Box::new(StripePattern::new(a, b, transform))
    }
}

impl Pattern for StripePattern {
    fn box_clone(&self) -> BoxPattern {
        Box::new((*self).clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn transformation(&self) -> Matrix {
        self.transform
    }

    fn inverse_transformation(&self) -> Matrix {
        self.inverse_transform
    }

    fn inner_pattern_at(&self, pattern_point: Tuple) -> Color {
        if pattern_point.x < 0. {
            if pattern_point.x.abs() % 2. <= 1. {
                self.b
            } else { 
                self.a 
            }
        } else if pattern_point.x % 2. < 1. { 
            self.a 
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{BLACK, WHITE};
    use crate::sphere::Sphere;

    #[test]
    fn create_stripe_pattern() {
        let pattern = StripePattern::new(WHITE, BLACK, None);

        assert_eq!(pattern.a, WHITE);
        assert_eq!(pattern.b, BLACK);
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let pattern = StripePattern::new_boxed(WHITE, BLACK, None);
        assert_eq!(pattern.inner_pattern_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.inner_pattern_at(Tuple::point(0., 1., 0.)), WHITE);
        assert_eq!(pattern.inner_pattern_at(Tuple::point(0., 2., 0.)), WHITE);
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let pattern = StripePattern::new_boxed(WHITE, BLACK, None);
        assert_eq!(pattern.inner_pattern_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.inner_pattern_at(Tuple::point(0., 0., 1.)), WHITE);
        assert_eq!(pattern.inner_pattern_at(Tuple::point(0., 0., 2.)), WHITE);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new_boxed(WHITE, BLACK, None);
        assert_eq!(pattern.inner_pattern_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.inner_pattern_at(Tuple::point(0.9, 0., 0.)), WHITE);
        assert_eq!(pattern.inner_pattern_at(Tuple::point(1., 0., 0.)), BLACK);
        assert_eq!(pattern.inner_pattern_at(Tuple::point(-0.1, 0., 0.)), BLACK);
        assert_eq!(pattern.inner_pattern_at(Tuple::point(-1., 0., 0.)), BLACK);
        assert_eq!(pattern.inner_pattern_at(Tuple::point(-1.1, 0., 0.)), WHITE);
    }

    #[test]
    fn stripes_with_object_transformation() {
        let o = Sphere::new(None, Some(Matrix::scaling(2., 2., 2.)));
        let pattern = StripePattern::new_boxed(WHITE, BLACK, None);
        let c = pattern.pattern_at_shape(&o, Tuple::point(1.5, 0., 0.));
        assert_eq!(WHITE, c);
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let o = Sphere::new(None, None);
        let pattern = StripePattern::new_boxed(WHITE, BLACK, Some(Matrix::scaling(2., 2., 2.)));
        let c = pattern.pattern_at_shape(&o, Tuple::point(1.5, 0., 0.));
        assert_eq!(WHITE, c);
    }

    #[test]
    fn patter_with_object_and_pattern_transformation() {
        let o = Sphere::new(None, Some(Matrix::scaling(2., 2., 2.)));
        let pattern = StripePattern::new_boxed(WHITE, BLACK, Some(Matrix::translation(0.5, 0., 0.)));
        let c = pattern.pattern_at_shape(&o, Tuple::point(2.5, 0., 0.));
        assert_eq!(WHITE, c);
    }
}