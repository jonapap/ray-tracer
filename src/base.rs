use crate::ray::Ray;
use cgmath::num_traits::clamp;
use cgmath::{dot, InnerSpace, Vector3};
use png::*;
use rand::{Rng, SeedableRng};
use std::cell::RefCell;
use std::fs::File;
use std::io::BufWriter;
use std::ops;
use std::ops::Range;
use std::path::Path;

pub type Background = fn(&Ray) -> Color;

pub type Color = Vector3<f64>;

pub type Vec3 = Vector3<f64>;

pub type Point3 = Vec3;

pub enum Axis {
    X,
    Y,
    Z,
}

pub trait VectorExt {
    fn is_near_zero(&self) -> bool;
    fn reflect(&self, n: &Vec3) -> Vec3;
    fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3;
    fn multiply_with(&self, other: &Vec3) -> Vec3;
    fn get_axis(&self, axis: u32) -> f64;
}

impl VectorExt for Vec3 {
    fn is_near_zero(&self) -> bool {
        let s = 1e-8;

        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    fn reflect(&self, n: &Vec3) -> Vec3 {
        self - 2.0 * dot(*self, *n) * n
    }

    fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(dot(-(*self), *n), 1.0);
        let r_out_prep = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_prep.magnitude2()).sqrt() * n;

        r_out_parallel + r_out_prep
    }

    fn multiply_with(&self, c2: &Vec3) -> Vec3 {
        Vec3::new(self.x * c2.x, self.y * c2.y, self.z * c2.z)
    }

    fn get_axis(&self, axis: u32) -> f64 {
        match axis {
            0 => self.x,
            1 => self.y,
            _ => self.z,
        }
    }
}

pub fn write_color(
    pixels_color: &Vec<Color>,
    samples_per_pixel: u32,
    width: u32,
    height: u32,
    output_name: &str,
) {
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

    let file = File::create(Path::new(output_name)).unwrap();
    let mut encoder = Encoder::new(BufWriter::new(file), width as u32, height as u32);
    encoder.set_color(ColorType::Rgb);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels[..]).unwrap();
}
