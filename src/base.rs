use cgmath::num_traits::clamp;
use cgmath::{InnerSpace, Vector3};
use png::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cell::RefCell;
use std::fs::File;
use std::io::BufWriter;
use std::ops::Range;
use std::path::Path;

thread_local! {
    // Share the random number generator in a thread for performance reasons and prevent issues
    // where the same vector will be returned
    pub static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(123123));
}

pub type Color = Vector3<f64>;

pub type Vec3 = Vector3<f64>;

pub fn unit_vector(vec: &Vec3) -> Vec3 {
    vec / vec.magnitude()
}

pub fn write_color(pixels_color: &Vec<Color>, samples_per_pixel: u32, width: u32, height: u32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let pixels: Vec<_> = pixels_color
        .iter()
        .map(|pix| {
            // Divide the color by the number of samples and gamma-correct for gamma=2.0.
            let r = (pix.x * scale).sqrt();
            let g = (pix.y * scale).sqrt();
            let b = (pix.z * scale).sqrt();

            vec![
                (256.0 * clamp(r, 0.0, 0.999)) as u8,
                (256.0 * clamp(g, 0.0, 0.999)) as u8,
                (256.0 * clamp(b, 0.0, 0.999)) as u8,
            ]
        })
        .flatten()
        .collect();

    let file = File::create(Path::new("out.png")).unwrap();
    let mut encoder = Encoder::new(BufWriter::new(file), width as u32, height as u32);
    encoder.set_color(ColorType::Rgb);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels[..]).unwrap();
}

fn random_vector() -> Vec3 {
    RNG.with(|rng| {
        let mut rng = rng.borrow_mut();

        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    })
}

fn random_vector_range(r: Range<f64>) -> Vec3 {
    RNG.with(|rng| {
        let mut rng = rng.borrow_mut();
        Vec3::new(
            rng.gen_range(r.clone()),
            rng.gen_range(r.clone()),
            rng.gen_range(r.clone()),
        )
    })
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
