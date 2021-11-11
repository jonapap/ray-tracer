use crate::base::*;
use crate::hit::hit_record::HitRecord;
use crate::materials::Material;
use crate::random::RNG;
use crate::ray::Ray;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord, rng: &mut RNG) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal + rng.random_unit_vector();

        if scatter_direction.is_near_zero() {
            Some((self.albedo, Ray::new(rec.p, rec.normal)))
        } else {
            Some((self.albedo, Ray::new(rec.p, scatter_direction)))
        }
    }
}
