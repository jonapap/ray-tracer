pub mod hit_record;
pub mod sphere;

use crate::hit::hit_record::HitRecord;
use crate::ray::Ray;

// Hittable

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

// HittableList

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { list: Vec::new() }
    }

    pub fn add(&mut self, item: Box<dyn Hittable>) {
        self.list.push(item);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for object in &self.list {
            match object.hit(r, t_min, closest_so_far) {
                None => {}
                Some(obj) => {
                    closest_so_far = obj.t;
                    rec = Some(obj)
                }
            }
        }

        return rec;
    }
}
