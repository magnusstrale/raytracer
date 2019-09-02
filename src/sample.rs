use super::color::*;
use super::canvas::*;
use super::tuple::*;
use super::ray::*;
use super::sphere::*;


#[cfg(test)]
mod tests {
    use super::*;

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
        let shape = Sphere::new();
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
        canvas.save("circle.png").unwrap();
    }
}
