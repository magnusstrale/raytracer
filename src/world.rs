use super::sphere::Sphere;
use super::shape::{Shape, BoxShape};
use super::color::{Color, WHITE, BLACK};
use super::tuple::{Tuple, ORIGO};
use super::matrix::Matrix;
use super::ray::Ray;
use super::material::{Material, DEFAULT_AMBIENT, DEFAULT_DIFFUSE, DEFAULT_SPECULAR, DEFAULT_SHININESS};
use super::intersection::{Intersection, Intersections};
use super::precomputed_data::PrecomputedData;

use super::light::PointLight;

pub struct World {
    pub light: Option<PointLight>,
    pub objects: Vec<BoxShape>
}

impl World {
    pub fn new(light: PointLight, objects: Vec<BoxShape>) -> Self {
        World { light: Some(light), objects }
    }

    fn empty() -> Self {
        World { light: None, objects: vec![] }
    }

    fn default_objects() -> Vec<BoxShape> {
        let m = Material::new(Color::new(0.8, 1., 0.6), DEFAULT_AMBIENT, 0.7, 0.2, DEFAULT_SHININESS);
        let s1 = Sphere::new_boxed(Some(m), None);
        let tr = Matrix::scaling(0.5, 0.5, 0.5);
        let s2 = Sphere::new_boxed(None, Some(tr));
        vec![s1, s2]
    }

    pub fn default_world() -> Self {
        let light = PointLight::new(Tuple::point(-10., 10., -10.), WHITE);
        World::new(light, World::default_objects())
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let xs = self.intersect(ray);
        match xs.hit() {
            None => BLACK,
            Some(i) => { 
                let comps = i.prepare_computations(ray);
                self.shade_hit(comps)
            }
        }
    }

    fn intersect(&self, ray: Ray) -> Intersections {
        let mut xs = Intersections::new(vec![]);
        for o in self.objects.iter() {
            xs.extend(o.intersect(ray));
        }
        xs
    }

    fn shade_hit(&self, comps: PrecomputedData) -> Color {
        comps.object.material().lighting(
            &self.light.unwrap(), 
            comps.point, 
            comps.eyev, 
            comps.normalv, 
            self.is_shadowed(comps.over_point))
    }

    fn is_shadowed(&self, point: Tuple) -> bool {
        let v = self.light.unwrap().position - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(point, direction);
        let intersections = self.intersect(r);
        let h = intersections.hit();
        h != None && h.unwrap().t < distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_world()
    {
        let w = World::empty();

        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.light, None);
    }

    #[test]
    fn test_default_world()
    {
        let light = PointLight::new(Tuple::point(-10., 10., -10.), WHITE);
        let w = World::default_world();
        assert_eq!(w.light.unwrap(), light);

        let m = w.objects[0].material();
        assert_eq!(m.color, Color::new(0.8, 1., 0.6));
        assert_eq!(m.diffuse, 0.7);
        assert_eq!(m.specular, 0.2);

        let tr = w.objects[1].transformation();
        assert_eq!(tr, Matrix::scaling(0.5, 0.5, 0.5));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let xs = w.intersect(r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.);
    }

    #[test]
    fn shading_intersection() {
        let w = World::default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = &w.objects[0];
        let i = Intersection::new(4., shape.clone());
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_intersection_from_inside() {
        let light = PointLight::new(Tuple::point(0., 0.25, 0.), WHITE);
        let w = World::new(light, World::default_objects());
        let r = Ray::new(ORIGO, Tuple::vector(0., 0., 1.));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape.clone());
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.));
        let c = w.color_at(r);

        assert_eq!(c, BLACK);
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default_world();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let c = w.color_at(r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        // The setup is a bit messy - basically we want to use default_world() and just tweak the ambient property
        // to 1.0 for both spheres. But due to the (mostly) immutable design I've opted for, this is not really
        // possible. Rather most of the setup code needs to be duplicated here. This is embarrasing enough for me
        // to come back later and fix it.
        let m1 = Material::new(Color::new(0.8, 1., 0.6), 1., 0.7, 0.2, DEFAULT_SHININESS);
        let s1 = Sphere::new_boxed(Some(m1), None);
        let tr = Matrix::scaling(0.5, 0.5, 0.5);
        let color = WHITE;
        let m2 = Material::new(color, 1., DEFAULT_DIFFUSE, DEFAULT_SPECULAR, DEFAULT_SHININESS);
        let s2 = Sphere::new_boxed(Some(m2), Some(tr));
        let light = PointLight::new(Tuple::point(-10., 10., -10.), WHITE);
        let w = World::new(light, vec![s1, s2]);
        let r = Ray::new(Tuple::point(0., 0., 0.75), Tuple::vector(0., 0., -1.));
        let c = w.color_at(r);

        assert_eq!(c, color);
    }

    #[test]
    fn no_shadow_when_nothing_collinear_with_point_and_light() {
        let w = World::default_world();
        let p = Tuple::point(0., 10., 0.);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = World::default_world();
        let p = Tuple::point(10., -10., 10.);

        assert!(w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_point_behind_light() {
        let w = World::default_world();
        let p = Tuple::point(-20., 20., -20.);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = World::default_world();
        let p = Tuple::point(-2., 2., -2.);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shade_hit_given_intersection_in_shadow() {
        let light = PointLight::new(Tuple::point(0., 0., -10.), WHITE);
        let s1 = Sphere::default_boxed();
        let s2_transform = Matrix::translation(0., 0., 10.);
        let s2 = Sphere::new_boxed(None, Some(s2_transform));

        let w = World::new(light, vec![s1, s2.clone()]);

        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let i = Intersection::new(4., s2);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}