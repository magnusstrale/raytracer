use super::tuple::Tuple;
use super::shape::Shape;

pub struct PrecomputedData {
    pub t: f64,
    pub object: Box<dyn Shape>,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple
}

impl PrecomputedData {
    pub fn new(t: f64, object: Box<dyn Shape>, point: Tuple, eyev: Tuple, normalv: Tuple, inside: bool, over_point: Tuple) -> Self {
        Self { t, object, point, eyev, normalv, inside, over_point }
    }
}