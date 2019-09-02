use core::ops;
use std::iter;
use super::sphere::Sphere;
use super::ray::Ray;
use super::tuple::Tuple;

#[derive(Debug, Copy, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere
}

impl Intersection {
    pub fn new(t: f64, object: Sphere) -> Self {
        Intersection {t, object}
    }

    fn intersections(i1: Intersection, i2: Intersection) -> Vec<Intersection> {
        vec![i1, i2]
    }
}

pub struct Intersections {
    inner: Vec<Intersection>
}

impl ops::Index<usize> for Intersections {
    type Output = Intersection;
    fn index(&self, i: usize) -> &Self::Output {
        &self.inner[i]
    }
}

impl Intersections {
    fn new(i1: Intersection, i2: Intersection) -> Intersections {
        Intersections { inner: vec![i1, i2]}
    }

    fn add(&mut self, i: Intersection) -> &mut Intersections {
        &mut self.inner.push(i);
        self
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregate_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(i1, i2);

        assert_eq!(2, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert_eq!(2.0, xs[1].t);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs =s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }
}
