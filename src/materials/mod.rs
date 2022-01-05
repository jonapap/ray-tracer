pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;
pub mod textures;

use crate::base::*;
use crate::hit::hit_record::HitRecord;
use crate::random::RNG;
use crate::ray::Ray;

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut RNG) -> Option<(Color, Ray)>;
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
