use crate::base::*;
use crate::hit::hit_record::HitRecord;
use crate::materials::textures::{SolidColor, Texture};
use crate::materials::Material;
use crate::random::RNG;
use crate::ray::Ray;

pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Lambertian { albedo }
    }
}

impl Lambertian<SolidColor> {
    pub fn from_color(color: Color) -> Self {
        Lambertian::new(SolidColor::new(color))
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _: &Ray, rec: &HitRecord, rng: &mut RNG) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal + rng.random_unit_vector();

        if scatter_direction.is_near_zero() {
            Some((
                self.albedo.value(rec.u, rec.v, &rec.p),
                Ray::new(rec.p, rec.normal),
            ))
        } else {
            Some((
                self.albedo.value(rec.u, rec.v, &rec.p),
                Ray::new(rec.p, scatter_direction),
            ))
        }
    }
}
