pub mod tuple;
pub mod color;
pub mod canvas;
pub mod matrix;
pub mod transform;
pub mod ray;
pub mod shape;
pub mod sphere;
pub mod intersection;
pub mod light;
pub mod material;
pub mod world;
pub mod precomputed_data;
pub mod camera;
pub mod sample;

pub fn approx_eq(a: f64, b: f64) -> bool {
        const EPS: f64 = 0.00001;
        (a - b).abs() < EPS
}