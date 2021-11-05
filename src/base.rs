use cgmath::num_traits::clamp;
use cgmath::{InnerSpace, Vector3};
use rand::Rng;
use std::ops::Range;

pub type Color = Vector3<f64>;

pub type Vec3 = Vector3<f64>;

pub fn unit_vector(vec: &Vec3) -> Vec3 {
    vec / vec.magnitude()
}

pub fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();

    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32
    );
}

fn random_vector() -> Vec3 {
    let mut rng = rand::thread_rng();

    Vec3::new(rng.gen(), rng.gen(), rng.gen())
}

fn random_vector_range(r: Range<f64>) -> Vec3 {
    let mut rng = rand::thread_rng();

    Vec3::new(
        rng.gen_range(r.clone()),
        rng.gen_range(r.clone()),
        rng.gen_range(r.clone()),
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vector_range(-1.0..1.0);
        if p.magnitude2() >= 1.0 {
            continue;
        }

        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    return unit_vector(&random_in_unit_sphere());
}
