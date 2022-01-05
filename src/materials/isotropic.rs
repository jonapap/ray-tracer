use crate::base::Color;
use crate::hit::hit_record::HitRecord;
use crate::materials::textures::{SolidColor, Texture};
use crate::materials::Material;
use crate::random::RNG;
use crate::ray::Ray;

pub struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Self {
        Isotropic { albedo }
    }
}

impl Isotropic<SolidColor> {
    pub fn from_color(color: Color) -> Self {
        Isotropic::new(SolidColor::new(color))
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord, rng: &mut RNG) -> Option<(Color, Ray)> {
        Some((
            self.albedo.value(rec.u, rec.v, &rec.p),
            Ray::new(rec.p, rng.random_in_unit_sphere()),
        ))
    }
}
