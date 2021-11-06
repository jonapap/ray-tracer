use crate::base::*;
use crate::hit::hit_record::HitRecord;
use crate::materials::Material;
use crate::ray::Ray;
use cgmath::dot;

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
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&unit_vector(&ray.direction), &rec.normal);

        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());

        if dot(scattered.direction, rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
