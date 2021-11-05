use crate::base::Vec3;
use crate::hit::hit_record::HitRecord;
use crate::hit::Hittable;
use crate::ray::Ray;
use cgmath::{dot, InnerSpace};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.magnitude2();
        let half_b = dot(oc, r.direction);
        let c = oc.magnitude2() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        Some(HitRecord::new_with_face_normal(
            root,
            r.at(root),
            (r.at(root) - self.center) / self.radius,
            &r,
            &((r.at(root) - self.center) / self.radius),
        ))
    }
}
