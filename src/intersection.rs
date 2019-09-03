use core::ops;
use super::sphere::Sphere;
use super::ray::Ray;
use super::tuple::Tuple;

#[derive(Debug, Copy, Clone, PartialEq)]
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
    inner: Vec<Intersection>,
    current_hit: Option<Intersection>
}

impl ops::Index<usize> for Intersections {
    type Output = Intersection;
    fn index(&self, i: usize) -> &Self::Output {
        &self.inner[i]
    }
}

impl Intersections {

    pub fn new(range: Vec<Intersection>) -> Intersections {
        let mut xs = Intersections { inner: range, current_hit: None };
        xs.inner.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        (0..xs.inner.len()).for_each(|i| xs.calculate_hit(xs.inner[i]));
        xs
    }

    fn calculate_hit(&mut self, i: Intersection) {
        if i.t < 0.0 { return };
        match self.current_hit {
            None => self.current_hit = Some(i),
            Some(h) => if i.t < h.t { self.current_hit = Some(i) }
        };
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn hit(&self) -> Option<Intersection> {
        self.current_hit
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
        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(2, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert_eq!(2.0, xs[1].t);
    }

    #[test]
    fn aggregate_intersections_with_add() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let i3 = Intersection::new(3.0, s);
        let i4 = Intersection::new(4.0, s);
        let xs = Intersections::new(vec![i1, i2, i3, i4]);

        assert_eq!(4, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert_eq!(2.0, xs[1].t);
        assert_eq!(3.0, xs[2].t);
        assert_eq!(4.0, xs[3].t);
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

    #[test]
    fn hit_all_intersections_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i2, i1]);
        let i = xs.hit().unwrap();

        assert_eq!(i, i1);
    }

    #[test]
    fn hit_some_intersections_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let xs = Intersections::new(vec![i2, i1]);
        let i = xs.hit().unwrap();

        assert_eq!(i, i2);
    }

    #[test]
    fn hit_all_intersections_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let xs = Intersections::new(vec![i2, i1]);
        let i = xs.hit();

        assert_eq!(i, None);
    }

    #[test]
    fn hit_lowest_non_negative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i1, i2, i3, i4]);
        let i = xs.hit().unwrap();

        assert_eq!(i, i4);
    }
}
