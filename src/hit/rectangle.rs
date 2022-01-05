use crate::aabb::AABB;
use crate::base::{Point3, Vec3};
use crate::hit::hit_record::HitRecord;
use crate::hit::{Hittable, HittableList};
use crate::materials::Material;
use crate::random::RNG;
use crate::ray::Ray;
use std::sync::Arc;

// XYRect

pub struct XYRect<M: Material> {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mp: Arc<M>,
}

impl<M: Material> XYRect<M> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Arc<M>) -> Self {
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rng: &mut RNG) -> Option<HitRecord> {
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
            self.mp.as_ref(),
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

// XZRect

pub struct XZRect<M: Material> {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Arc<M>,
}

impl<M: Material> XZRect<M> {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: Arc<M>) -> Self {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            mp,
        }
    }
}

impl<M: Material> Hittable for XZRect<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rng: &mut RNG) -> Option<HitRecord> {
        let t = (self.k - r.origin.y) / r.direction.y;
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
            r.at(t),
            self.mp.as_ref(),
            &r,
            &Vec3::new(0.0, 1.0, 0.0),
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.x0, self.k - 0.001, self.z0),
            Point3::new(self.x1, self.k + 0.001, self.z1),
        ))
    }
}

// YZRect

pub struct YZRect<M: Material> {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Arc<M>,
}

impl<M: Material> YZRect<M> {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: Arc<M>) -> Self {
        YZRect {
            y0,
            y1,
            z0,
            z1,
            k,
            mp,
        }
    }
}

impl<M: Material> Hittable for YZRect<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rng: &mut RNG) -> Option<HitRecord> {
        let t = (self.k - r.origin.x) / r.direction.x;
        if t < t_min || t > t_max {
            return None;
        }

        let y = r.origin.y + t * r.direction.y;
        let z = r.origin.z + t * r.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0),
            r.at(t),
            self.mp.as_ref(),
            &r,
            &Vec3::new(1.0, 0.0, 0.0),
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.k - 0.001, self.y0, self.z0),
            Point3::new(self.k + 0.001, self.y1, self.z1),
        ))
    }
}

// Box

pub struct Cuboid {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl Cuboid {
    pub fn new<M: 'static + Material>(box_min: Point3, box_max: Point3, material: Arc<M>) -> Self {
        let mut sides = HittableList::new();

        sides.add(Box::new(XYRect::new(
            box_min.x,
            box_max.x,
            box_min.y,
            box_max.y,
            box_max.z,
            material.clone(),
        )));
        sides.add(Box::new(XYRect::new(
            box_min.x,
            box_max.x,
            box_min.y,
            box_max.y,
            box_min.z,
            material.clone(),
        )));

        sides.add(Box::new(XZRect::new(
            box_min.x,
            box_max.x,
            box_min.z,
            box_max.z,
            box_max.y,
            material.clone(),
        )));
        sides.add(Box::new(XZRect::new(
            box_min.x,
            box_max.x,
            box_min.z,
            box_max.z,
            box_min.y,
            material.clone(),
        )));

        sides.add(Box::new(YZRect::new(
            box_min.y,
            box_max.y,
            box_min.z,
            box_max.z,
            box_max.x,
            material.clone(),
        )));
        sides.add(Box::new(YZRect::new(
            box_min.y,
            box_max.y,
            box_min.z,
            box_max.z,
            box_min.x,
            material.clone(),
        )));

        Cuboid {
            sides,
            box_min,
            box_max,
        }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rng: &mut RNG) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max, rng)
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }
}
