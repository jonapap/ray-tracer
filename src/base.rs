use cgmath::num_traits::clamp;
use cgmath::{InnerSpace, Vector3};

pub type Color = Vector3<f64>;

pub type Vec3 = Vector3<f64>;

pub fn unit_vector(vec: &Vec3) -> Vec3 {
    vec / vec.magnitude()
}

pub fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = pixel_color.x * scale;
    let g = pixel_color.y * scale;
    let b = pixel_color.z * scale;

    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32
    );
}
