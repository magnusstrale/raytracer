use super::tuple::{Tuple, ORIGO};
use super::ray::Ray;
use super::intersection::{Intersection, Intersections};
use super::matrix::{Matrix, IDENTITY_MATRIX};
use super::material::Material;
use super::shape::Shape;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::f64::consts::*;

static SPHERE_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub struct Sphere {
    index: usize,
    inverse_transform: Matrix,
    transform: Matrix,
    material: Material
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Default for Sphere {
    fn default() -> Self {
        let im = Matrix::identity_matrix();
        Sphere { index: SPHERE_COUNT.fetch_add(1, Ordering::SeqCst), transform: im, inverse_transform: im, material: Material::default() }
    }
}


impl Shape for Sphere {
    fn inner_intersect(&self, object_ray: Ray) -> Intersections {
        let sphere_to_ray = object_ray.origin - ORIGO;
        let a = object_ray.direction.dot(&object_ray.direction);
        let b = 2.0 * object_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 { return Intersections::new(vec![]) }

        let i1 = Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), self.clone());
        let i2 = Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), self.clone());
        Intersections::new(vec![i2, i1])
    }

    fn inner_normal_at(&self, object_point: Tuple) -> Tuple {
        object_point - ORIGO
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn transformation(&self) -> Matrix {
        self.transform
    }

    fn inverse_transformation(&self) -> Matrix {
        self.inverse_transform
    }
}

impl Sphere {
    pub fn new(material: Option<Material>, transform: Option<Matrix>) -> Self {
        Sphere { 
            index: SPHERE_COUNT.fetch_add(1, Ordering::SeqCst), 
            transform: transform.unwrap_or(IDENTITY_MATRIX), 
            inverse_transform: match transform {
                None => IDENTITY_MATRIX,
                Some(t) => t.inverse().unwrap()
            },
            material: material.unwrap_or(Material::default()) 
        }
    }

    // pub fn intersect(&self, ray: Ray) -> Intersections {
    //     let ray2 = ray.transform(self.inverse_transform);
    //     let sphere_to_ray = ray2.origin - ORIGO;
    //     let a = ray2.direction.dot(&ray2.direction);
    //     let b = 2.0 * ray2.direction.dot(&sphere_to_ray);
    //     let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
    //     let discriminant = b * b - 4.0 * a * c;

    //     if discriminant < 0.0 { return Intersections::new(vec![]) }

    //     let i1 = Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), self.clone());
    //     let i2 = Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), self.clone());
    //     Intersections::new(vec![i2, i1])
    // }
    
    // pub fn normal_at(&self, world_point: Tuple) -> Tuple {
    //     if !world_point.is_point() { panic!("Tuple must be a point"); }
    //     let object_point = self.inverse_transform * world_point;
    //     let object_normal = object_point - ORIGO;
    //     let mut world_normal = self.inverse_transform.transpose() * object_normal;
    //     world_normal.w = 0.0;   // a bit of a hack...
    //     world_normal.normalize()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_intersect_sphere_at_two_points()
    {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 6.);
    }    

    #[test]
    fn ray_intersect_sphere_at_tangent()
    {
        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.);
        assert_eq!(xs[1].t, 5.);
    }    

    #[test]
    fn ray_miss_sphere()
    {
        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_insider_sphere() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.);
        assert_eq!(xs[1].t, 1.);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::default();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.);
        assert_eq!(xs[1].t, -4.);
    }

    #[test]
    fn sphere_default_transform() {
        let s = Sphere::default();

        assert_eq!(s.transformation(), Matrix::identity_matrix());
    }

    #[test]
    fn change_sphere_transform() {
        let tr = Matrix::translation(2., 3., 4.);
        let s = Sphere::new(None, Some(tr));

        assert_eq!(s.transformation(), tr);
    }

    #[test]
    fn intersect_scaled_sphere_with_ray()
    {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let tr = Matrix::scaling(2., 2., 2.);
        let s = Sphere::new(None, Some(tr));
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.);
        assert_eq!(xs[1].t, 7.);
    }

    #[test]
    fn intersect_translated_sphere_with_ray()
    {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let tr = Matrix::translation(5., 0., 0.);
        let s = Sphere::new(None, Some(tr));
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_on_sphere_on_x_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Tuple::point(1., 0., 0.));

        assert_eq!(n, Tuple::vector(1., 0., 0.));
    }

    #[test]
    fn normal_on_sphere_on_y_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Tuple::point(0., 1., 0.));

        assert_eq!(n, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn normal_on_sphere_on_z_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Tuple::point(0., 0., 1.));

        assert_eq!(n, Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn normal_on_sphere_on_non_axial_point() {
        let s = Sphere::default();
        let pv = 3.0f64.sqrt() / 3.0;
        let n = s.normal_at(Tuple::point(pv, pv, pv));

        assert_eq!(n, Tuple::vector(pv, pv, pv));
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Sphere::default();
        let pv = 3.0f64.sqrt() / 3.0;
        let n = s.normal_at(Tuple::point(pv, pv, pv));

        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let tr = Matrix::translation(0., 1., 0.);
        let s = Sphere::new(None, Some(tr));
        let n = s.normal_at(Tuple::point(0., 1.70711, -0.70711));

        assert_eq!(n, Tuple::vector(0., 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let tr = Matrix::scaling(1., 0.5, 1.) * Matrix::rotation_z(PI / 5.);
        let s = Sphere::new(None, Some(tr));
        let pv = 2.0f64.sqrt() / 2.0;
        let n = s.normal_at(Tuple::point(0., pv, -pv));

        assert_eq!(n, Tuple::vector(0., 0.97014, -0.24254));
    }

    #[test]
    fn sphere_has_default_material()
    {
        let s = Sphere::default();

        assert_eq!(*s.material(), Material::default())
    }

    #[test]
    fn sphere_can_be_assigned_material()
    {
        let mut m = Material::default();
        m.ambient = 1.0;
        let s = Sphere::new(Some(m.clone()), None);
        
        assert_eq!(*s.material(), m);
    }
}
