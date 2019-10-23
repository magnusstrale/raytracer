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
        Self { 
            a, 
            b, 
            transform: transform.unwrap_or_default(),
            inverse_transform: inverse_transform_parameter(transform)
        }
    }

    pub fn new_boxed(a: Color, b: Color, transform: Option<Matrix>) -> BoxPattern {
        Box::new(Self::new(a, b, transform))
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GradientPattern {
    a: Color,
    b: Color,
    transform: Matrix,
    inverse_transform: Matrix
}

impl GradientPattern {
    pub fn new(a: Color, b: Color, transform: Option<Matrix>) -> Self {
        Self { 
            a, 
            b, 
            transform: transform.unwrap_or_default(),
            inverse_transform: inverse_transform_parameter(transform)
        }
    }

    pub fn new_boxed(a: Color, b: Color, transform: Option<Matrix>) -> BoxPattern {
        Box::new(Self::new(a, b, transform))
    }
}

impl Pattern for GradientPattern {
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
        let distance = self.b - self.a;
        let fraction = pattern_point.x.fract();
        self.a + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{BLACK, WHITE};
    use crate::sphere::Sphere;
    use crate::matrix::IDENTITY_MATRIX;
    use crate::EPSILON;

    #[derive(Debug, Clone, PartialEq)]
    struct TestPattern {
        transform: Matrix,
        inverse_transform: Matrix
    }

    impl TestPattern {
        fn new(transform: Option<Matrix>) -> Self {
            TestPattern {
                transform: transform.unwrap_or_default(),
                inverse_transform: inverse_transform_parameter(transform)
            }
        }

        fn new_boxed(transform: Option<Matrix>) -> BoxPattern {
            Box::new(TestPattern::new(transform))
        }
    }

    impl Pattern for TestPattern {
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
            Color::new(pattern_point.x, pattern_point.y, pattern_point.z)
        }
    }

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
    fn stripes_with_object_and_pattern_transformation() {
        let o = Sphere::new(None, Some(Matrix::scaling(2., 2., 2.)));
        let pattern = StripePattern::new_boxed(WHITE, BLACK, Some(Matrix::translation(0.5, 0., 0.)));
        let c = pattern.pattern_at_shape(&o, Tuple::point(2.5, 0., 0.));
        assert_eq!(WHITE, c);
    }

    #[test]
    fn default_pattern_transformation() {
        let tp = TestPattern::new_boxed(None);
        assert_eq!(tp.transformation(), IDENTITY_MATRIX);
    }

    #[test]
    fn assign_transformation_to_pattern() {
        let tp = TestPattern::new_boxed(Some(Matrix::translation(1., 2., 3.)));
        assert_eq!(tp.transformation(), Matrix::translation(1., 2., 3.));
    }

    #[test]
    fn pattern_with_object_transformation() {
        let shape = Sphere::new_boxed(None, Some(Matrix::scaling(2., 2., 2.)));
        let tp = TestPattern::new_boxed(None);
        let c = tp.pattern_at_shape(&*shape, Tuple::point(2., 3., 4.));
        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn pattern_with_pattern_transformation() {
        let shape = Sphere::new_boxed(None, None);
        let tp = TestPattern::new_boxed(Some(Matrix::scaling(2., 2., 2.)));
        let c = tp.pattern_at_shape(&*shape, Tuple::point(2., 3., 4.));
        assert_eq!(c, Color::new(1., 1.5, 2.));
    }

    #[test]
    fn pattern_with_object_and_pattern_transformation() {
        let shape = Sphere::new_boxed(None, Some(Matrix::scaling(2., 2., 2.)));
        let tp = TestPattern::new_boxed(Some(Matrix::translation(0.5, 1., 1.5)));
        let c = tp.pattern_at_shape(&*shape, Tuple::point(2.5, 3., 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = GradientPattern::new(WHITE, BLACK, None);
        assert_eq!(pattern.inner_pattern_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.inner_pattern_at(Tuple::point(0.25, 0., 0.)), Color::new(0.75, 0.75, 0.75));
        assert_eq!(pattern.inner_pattern_at(Tuple::point(0.5, 0., 0.)), Color::new(0.5, 0.5, 0.5));
        assert_eq!(pattern.inner_pattern_at(Tuple::point(0.75, 0., 0.)), Color::new(0.25, 0.25, 0.25));
        assert_eq!(pattern.inner_pattern_at(Tuple::point(1. - EPSILON, 0., 0.)), BLACK);
    }
}
