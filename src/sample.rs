use std::f64::consts::*;
use super::color::*;
use super::canvas::*;
use super::tuple::*;
use super::ray::*;
use super::sphere::*;
use super::matrix::*;
use super::pointlight::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_to_file()
    {
        let mut c = Canvas::new(100, 100);
        c.write_pixel(1, 1, RED);
        c.write_pixel(99, 0, GREEN);
        c.write_pixel(99, 99, WHITE);
        c.save("black.png").expect("Failed to save file");
    }

    #[test]
    fn circle_shadow()
    {
        const CANVAS_PIXELS: usize = 1000;
        const WALL_SIZE:usize = 7;
        let ray_origin = Tuple::point(0.0, 0.0, -5.0);
        let wall_z = 10.0;
        let pixel_size = WALL_SIZE as f64 / CANVAS_PIXELS as f64;
        let half = WALL_SIZE as f64 / 2.0;

        let mut canvas = Canvas::new(CANVAS_PIXELS, CANVAS_PIXELS);
        let color = Color::new(1.0, 0.0, 0.0);
        let mut shape = Sphere::new();
        shape.set_transform(Matrix::shearing(1.0, 0.0, 0.5, 0.0, 0.0, 0.0) * Matrix::scaling(1.0, 0.5, 1.0));
        for y in 0..CANVAS_PIXELS {
            let world_y = half - pixel_size * (y as f64);
            for x in 0..CANVAS_PIXELS {
                let world_x = -half + pixel_size * (x as f64);
                let position = Tuple::point(world_x, world_y, wall_z);
                let r = Ray::new(ray_origin, (position - ray_origin).normalize());
                let xs = shape.intersect(r);
                match xs.hit() {
                    Some(t) => canvas.write_pixel(x, y, color),
                    None => ()
                }
            }
        }
        canvas.save("shadow.png").unwrap();
    }

    #[test]
    fn rendered_sphere()
    {
        const CANVAS_PIXELS: usize = 1000;
        const WALL_SIZE:usize = 7;
        let ray_origin = Tuple::point(0.0, 0.0, -5.0);
        let wall_z = 10.0;
        let pixel_size = WALL_SIZE as f64 / CANVAS_PIXELS as f64;
        let half = WALL_SIZE as f64 / 2.0;

        let mut canvas = Canvas::new(CANVAS_PIXELS, CANVAS_PIXELS);
        let mut shape = Sphere::new();
        shape.material.color = Color::new(1.0, 0.2, 1.0);
        let light_position = Tuple::point(-10.0, 10.0, -10.0);
        let light_color = WHITE;
        let light = PointLight::new(light_position, light_color);

        shape.set_transform(Matrix::shearing(1.0, 0.0, 0.5, 0.0, 0.0, 0.0) * Matrix::scaling(1.0, 0.5, 1.0));
        for y in 0..CANVAS_PIXELS {
            let world_y = half - pixel_size * (y as f64);
            for x in 0..CANVAS_PIXELS {
                let world_x = -half + pixel_size * (x as f64);
                let position = Tuple::point(world_x, world_y, wall_z);
                let r = Ray::new(ray_origin, (position - ray_origin).normalize());
                let xs = shape.intersect(r);
                match xs.hit() {
                    Some(h) => {
                        let point = r.position(h.t);
                        let normal = h.object.normal_at(point);
                        let eye = - r.direction;
                        let color = h.object.material.lighting(&light, point, eye, normal);
                        canvas.write_pixel(x, y, color);
                    },
                    _ => ()
                }
            }
        }
        canvas.save("sphere.png").unwrap();
    }
}
