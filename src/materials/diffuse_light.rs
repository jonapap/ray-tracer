use crate::base::{Color, Point3};
use crate::hit::hit_record::HitRecord;
use crate::materials::textures::{SolidColor, Texture};
use crate::materials::Material;
use crate::random::RNG;
use crate::ray::Ray;

pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> Self {
        DiffuseLight { emit }
    }
}

impl DiffuseLight<SolidColor> {
    pub fn from_color(color: Color) -> DiffuseLight<SolidColor> {
        DiffuseLight::<SolidColor>::new(SolidColor::new(color))
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut RNG) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
