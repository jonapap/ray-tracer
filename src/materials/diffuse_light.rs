use crate::base::{Color, Point3};
use crate::hit::hit_record::HitRecord;
use crate::materials::textures::Texture;
use crate::materials::Material;
use crate::random::RNG;
use crate::ray::Ray;

pub struct DiffuseLight<'a, T: Texture> {
    emit: &'a T,
}

impl<'a, T: Texture> Material for DiffuseLight<'a, T> {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut RNG) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
