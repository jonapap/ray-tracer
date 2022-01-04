use crate::aabb::AABB;
use crate::base::*;
use crate::hit::hit_record::HitRecord;
use crate::hit::Hittable;
use crate::materials::Material;
use crate::ray::Ray;
use cgmath::{dot, InnerSpace};

pub struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, material: M) -> Sphere<M> {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
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

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let (u, v) = calc_sphere_uv(&outward_normal);

        Some(HitRecord::new(
            root,
            u,
            v,
            p,
            &self.material,
            &r,
            &outward_normal,
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

fn calc_sphere_uv(p: &Point3) -> (f64, f64) {
    let pi = std::f64::consts::PI;
    let theta = (-p.y).acos();
    let phi = f64::atan2(-p.z, p.x) + pi;

    (phi / (2.0 * pi), theta / pi)
}
