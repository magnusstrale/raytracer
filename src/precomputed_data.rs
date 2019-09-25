use super::tuple::Tuple;
use super::shape::BoxShape;

pub struct PrecomputedData {
    pub t: f64,
    pub object: BoxShape,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple
}

impl PrecomputedData {
    pub fn new(t: f64, object: BoxShape, point: Tuple, eyev: Tuple, normalv: Tuple, inside: bool, over_point: Tuple) -> Self {
        Self { t, object, point, eyev, normalv, inside, over_point }
    }
}