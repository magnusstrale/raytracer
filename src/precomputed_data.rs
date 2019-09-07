use super::tuple::Tuple;
use super::sphere::Sphere;

#[derive(Debug)]
pub struct PrecomputedData {
    pub t: f64,
    pub object: Sphere,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple
}

impl PrecomputedData {
    pub fn new(t: f64, object: Sphere, point: Tuple, eyev: Tuple, normalv: Tuple, inside: bool, over_point: Tuple) -> Self {
        Self { t, object, point, eyev, normalv, inside, over_point }
    }
}