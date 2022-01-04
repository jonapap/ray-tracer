use crate::aabb::AABB;
use crate::base::{Point3, Vec3};
use crate::hit::hit_record::HitRecord;
use crate::hit::Hittable;
use crate::materials::Material;
use crate::ray::Ray;

// XYRect

pub struct XYRect<M: Material> {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mp: M,
}

impl<M: Material> XYRect<M> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: M) -> Self {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            mp,
        }
    }
}

impl<M: Material> Hittable for XYRect<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.z) / r.direction.z;
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x + t * r.direction.x;
        let y = r.origin.y + t * r.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
            r.at(t),
            &self.mp,
            &r,
            &Vec3::new(0.0, 0.0, 1.0),
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.001),
            Point3::new(self.x1, self.y1, self.k + 0.001),
        ))
    }
}
