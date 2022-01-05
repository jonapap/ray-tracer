use cgmath::num_traits::Float;
use cgmath::InnerSpace;

use crate::aabb::AABB;
use crate::base::{Color, Vec3};
use crate::hit::hit_record::HitRecord;
use crate::hit::Hittable;
use crate::materials::isotropic::Isotropic;
use crate::materials::textures::{SolidColor, Texture};
use crate::random::RNG;
use crate::ray::Ray;

pub struct ConstantMedium<H: Hittable, T: Texture> {
    boundary: H,
    phase_function: Isotropic<T>,
    neg_inv_density: f64,
}

impl<H: Hittable, T: Texture> ConstantMedium<H, T> {
    pub fn new(boundary: H, d: f64, texture: T) -> Self {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / d,
            phase_function: Isotropic::new(texture),
        }
    }
}

impl<H: Hittable> ConstantMedium<H, SolidColor> {
    pub fn new_from_color(boundary: H, d: f64, color: Color) -> Self {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / d,
            phase_function: Isotropic::from_color(color),
        }
    }
}

impl<H: Hittable, T: Texture> Hittable for ConstantMedium<H, T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rng: &mut RNG) -> Option<HitRecord> {
        let mut rec1 = self
            .boundary
            .hit(&r, f64::neg_infinity(), f64::infinity(), rng)?;

        let mut rec2 = self
            .boundary
            .hit(&r, rec1.t + 0.0001, f64::infinity(), rng)?;

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0
        }

        let ray_length = r.direction.magnitude();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rng.random_double().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);
        Some(HitRecord {
            t,
            p,
            front_face: true,
            u: 0.0,                           // arbitrary
            v: 0.0,                           // arbitrary
            normal: Vec3::new(1.0, 0.0, 0.0), // arbitrary
            material: &self.phase_function,   // arbitrary
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.boundary.bounding_box()
    }
}
