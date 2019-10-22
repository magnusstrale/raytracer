use super::color::Color;
use super::tuple::Tuple;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pattern {
    a: Color,
    b: Color
}

impl Pattern {
    pub fn stripe_pattern(a: Color, b: Color) -> Self {
        Pattern { a, b }
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
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{BLACK, WHITE};

    #[test]
    fn create_stripe_pattern() {
        let pattern = Pattern::stripe_pattern(WHITE, BLACK);

        assert_eq!(pattern.a, WHITE);
        assert_eq!(pattern.b, BLACK);
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let pattern = Pattern::stripe_pattern(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(Tuple::point(0., 1., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(Tuple::point(0., 2., 0.)), WHITE);
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let pattern = Pattern::stripe_pattern(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 1.)), WHITE);
        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 2.)), WHITE);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = Pattern::stripe_pattern(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(Tuple::point(0.9, 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(Tuple::point(1., 0., 0.)), BLACK);
        assert_eq!(pattern.stripe_at(Tuple::point(-0.1, 0., 0.)), BLACK);
        assert_eq!(pattern.stripe_at(Tuple::point(-1., 0., 0.)), BLACK);
        assert_eq!(pattern.stripe_at(Tuple::point(-1.1, 0., 0.)), WHITE);
    }
}