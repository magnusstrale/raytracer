use core::ops;
use super::EPSILON;
use super::shape::*;
use super::sphere::Sphere;
use super::ray::Ray;
use super::tuple::Tuple;
use super::precomputed_data::PrecomputedData;
use super::matrix::Matrix;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Box<dyn Shape + 'static>
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t &&
        &self.object == &other.object
    }
}

impl Intersection {
    pub fn new(t: f64, object: Box<dyn Shape>) -> Self {
        Intersection { t, object }
    }

    pub fn prepare_computations(&self, ray: Ray) -> PrecomputedData {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let mut normalv = self.object.normal_at(point);
        let mut inside = false;
        if normalv.dot(&eyev) < 0.0 {
            normalv = -normalv;
            inside = true;
        }
        let over_point = point + normalv * EPSILON;

        PrecomputedData::new(
            self.t,
            self.object.clone(),
            point,
            eyev,
            normalv,
            inside,
            over_point
        )
    }
}

#[derive(Debug)]
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
        for i in xs.inner.iter() {
            if i.t >= 0. { 
                xs.current_hit = Some(i.clone());
                break;
            };
        }
        xs
    }

    pub fn extend(&mut self, range: Intersections) {
        self.inner.extend(range.inner);
        match range.current_hit {
            Some(range_hit) =>
                match &self.current_hit {
                    None => self.current_hit = Some(range_hit.clone()),
                    Some(i) => if i.t > range_hit.t { self.current_hit = Some(range_hit.clone());}
                }
            _ => ()
        }
        self.inner.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }

    fn calculate_hit(&self, i: &Intersection) -> Option<Intersection> {
        if i.t < 0.0 { return None; };
        match &self.current_hit {
            None => Some((*i).clone()),
            Some(h) => if i.t < h.t { Some((*i).clone()) } else { None }
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn hit(&self) -> Option<&Intersection> {
        match &self.current_hit {
            None => None,
            Some(i) => Some(i).clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s: Box<dyn Shape> = Box::new(Sphere::default());
        let i = Intersection::new(3.5, s.clone());

        assert_eq!(i.t, 3.5);
        assert_eq!(&i.object, &s);
    }

    #[test]
    fn aggregate_intersections() {
        let s = Sphere::default();
        let i1 = Intersection::new(1., Box::new(s.clone()));
        let i2 = Intersection::new(2., Box::new(s));
        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(2, xs.len());
        assert_eq!(1., xs[0].t);
        assert_eq!(2., xs[1].t);
    }

    #[test]
    fn aggregate_intersections_with_add() {
        let s = Sphere::default();
        let i1 = Intersection::new(1., Box::new(s.clone()));
        let i2 = Intersection::new(2., Box::new(s.clone()));
        let i3 = Intersection::new(3., Box::new(s.clone()));
        let i4 = Intersection::new(4., Box::new(s));
        let xs = Intersections::new(vec![i1, i2, i3, i4]);

        assert_eq!(4, xs.len());
        assert_eq!(1., xs[0].t);
        assert_eq!(2., xs[1].t);
        assert_eq!(3., xs[2].t);
        assert_eq!(4., xs[3].t);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::default();
        let xs =s.intersect(r);
        let bs = Box::new(s) as Box::<dyn Shape>;

        assert_eq!(2, xs.len());
        assert_eq!(&xs[0].object, &bs);
        assert_eq!(&xs[1].object, &bs);
    }

    #[test]
    fn hit_all_intersections_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(1., Box::new(s.clone()));
        let i2 = Intersection::new(2., Box::new(s));
        let xs = Intersections::new(vec![i2, i1.clone()]);
        let i = xs.hit().unwrap();

        assert_eq!(*i, i1);
    }

    #[test]
    fn hit_some_intersections_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1., Box::new(s.clone()));
        let i2 = Intersection::new(1., Box::new(s));
        let xs = Intersections::new(vec![i2.clone(), i1]);
        let i = xs.hit().unwrap();

        assert_eq!(*i, i2);
    }

    #[test]
    fn hit_all_intersections_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2., Box::new(s.clone()));
        let i2 = Intersection::new(-1., Box::new(s));
        let xs = Intersections::new(vec![i2, i1]);
        let i = xs.hit();

        assert_eq!(i, None);
    }

    #[test]
    fn hit_lowest_non_negative_intersection() {
        let s = Sphere::default();
        let i1 = Intersection::new(5., Box::new(s.clone()));
        let i2 = Intersection::new(7., Box::new(s.clone()));
        let i3 = Intersection::new(-3., Box::new(s.clone()));
        let i4 = Intersection::new(2., Box::new(s));
        let xs = Intersections::new(vec![i1, i2, i3, i4.clone()]);
        let i = xs.hit().unwrap();

        assert_eq!(*i, i4);
    }

    #[test]
    fn extend_intersections_gets_union() {
        let s1 = Sphere::default();
        let i1 = Intersection::new(5., Box::new(s1.clone()));
        let i2 = Intersection::new(7., Box::new(s1.clone()));
        let i3 = Intersection::new(-3., Box::new(s1.clone()));
        let i4 = Intersection::new(2., Box::new(s1));
        let mut xs1 = Intersections::new(vec![i1, i2, i3, i4]);

        let s2 = Sphere::default();
        let i5 = Intersection::new(-1., Box::new(s2.clone()));
        let i6 = Intersection::new(1., Box::new(s2.clone()));
        let i7 = Intersection::new(2., Box::new(s2));
        let xs2 = Intersections::new(vec![i5, i6.clone(), i7]);

        xs1.extend(xs2);    // xs2 is moved

        assert_eq!(xs1.len(), 7);
        assert_eq!(*xs1.hit().unwrap(), i6);
    }

    #[test]
    fn precompute_state_of_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::default();
        let i = Intersection::new(4., Box::new(shape));
        let comps = i.prepare_computations(r);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.point, Tuple::point(0., 0., -1.));
        assert_eq!(comps.eyev, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn hit_when_intersection_on_outside() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::default();
        let i = Intersection::new(4., Box::new(shape));
        let comps = i.prepare_computations(r);

        assert!(!comps.inside);
    }

    #[test]
    fn hit_when_intersection_on_inside() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::default();
        let i = Intersection::new(1., Box::new(shape));
        let comps = i.prepare_computations(r);

        assert_eq!(comps.point, Tuple::point(0., 0., 1.));
        assert_eq!(comps.eyev, Tuple::vector(0., 0., -1.));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let transform = Matrix::translation(0., 0., 1.);
        let shape = Sphere::new(None, Some(transform));
        let i = Intersection::new(5., Box::new(shape));
        let comps = i.prepare_computations(r);
        assert!(comps.over_point.z < - EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }
}
