use super::tuple::Tuple;
use super::ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
}

impl Sphere {
    fn new() -> Self {
        Sphere {}
    }

    pub fn intersect(&self, ray: Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 { return vec![] }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        if t1 <= t2 { return vec![t1, t2] }
        vec![t2, t1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_intersect_sphere_at_two_points()
    {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }    

    #[test]
    fn ray_intersect_sphere_at_tangent()
    {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }    

    #[test]
    fn ray_miss_sphere()
    {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_insider_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}
