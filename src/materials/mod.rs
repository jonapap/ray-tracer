pub mod lambertian;
pub mod metal;

use crate::base::*;
use crate::hit::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}
