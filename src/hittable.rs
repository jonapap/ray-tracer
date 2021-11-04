use crate::ray::Ray;
use crate::vec3::Vec3;
use cgmath::{dot, InnerSpace};
use std::ptr::drop_in_place;

pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    front_face: Option<bool>,
}

impl HitRecord {
    fn new_with_face_normal(
        t: f64,
        point: Vec3,
        normal: Vec3,
        r: &Ray,
        outward_normal: &Vec3,
    ) -> HitRecord {
        let tmp = HitRecord {
            point,
            normal,
            t,
            front_face: None,
        };

        tmp.calculate_face_normal(r, outward_normal)
    }

    fn calculate_face_normal(self, r: &Ray, outward_normal: &Vec3) -> HitRecord {
        let front_face = dot(r.direction, *outward_normal) < 0.0;
        HitRecord {
            front_face: Some(front_face),
            normal: if front_face {
                outward_normal.clone()
            } else {
                -outward_normal.clone()
            },
            ..self
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

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
