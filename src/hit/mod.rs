pub mod bvh;
pub mod hit_record;
pub mod rectangle;
pub mod sphere;
pub mod transform;

use crate::aabb::AABB;
use crate::base::Point3;
use crate::hit::hit_record::HitRecord;
use crate::ray::Ray;
use itertools::Itertools;

// Hittable

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
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

    fn bounding_box(&self) -> Option<AABB> {
        if self.list.is_empty() {
            return None;
        }

        self.list
            .iter()
            .map(|x| x.bounding_box())
            .reduce(|a, b| match (a, b) {
                (Some(box0), Some(box1)) => Some(AABB::surrounding_box(&box0, &box1)),
                (_, _) => None,
            })
            .unwrap_or(None)
    }
}
