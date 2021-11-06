use crate::base::*;
use crate::hit::hit_record::HitRecord;
use crate::materials::Material;
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
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal + random_unit_vector();

        if is_near_zero(&scatter_direction) {
            Some((self.albedo, Ray::new(rec.p, rec.normal)))
        } else {
            Some((self.albedo, Ray::new(rec.p, scatter_direction)))
        }
    }
}
