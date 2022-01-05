use cgmath::{dot, InnerSpace};

use crate::base::*;
use crate::hit::hit_record::HitRecord;
use crate::materials::Material;
use crate::random::RNG;
use crate::ray::Ray;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut RNG) -> Option<(Color, Ray)> {
        let reflected = ray.direction.normalize().reflect(&rec.normal);

        let scattered = Ray::new(rec.p, reflected + self.fuzz * rng.random_in_unit_sphere());

        if dot(scattered.direction, rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
