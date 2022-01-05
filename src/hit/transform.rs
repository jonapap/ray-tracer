use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

use crate::aabb::AABB;
use crate::base::*;
use crate::hit::hit_record::HitRecord;
use crate::hit::Hittable;
use crate::random::RNG;
use crate::ray::Ray;

// Translate

pub struct Translate<H: Hittable> {
    obj: H,
    offset: Vec3,
}

impl<H: Hittable> Translate<H> {
    pub fn new(obj: H, offset: Vec3) -> Self {
        Translate { obj, offset }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rng: &mut RNG) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction);

        self.obj.hit(&moved_r, t_min, t_max, rng).map(|hit| {
            HitRecord::new(
                hit.t,
                hit.u,
                hit.v,
                hit.p + self.offset,
                hit.material,
                &moved_r,
                &hit.normal,
            )
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.obj
            .bounding_box()
            .map(|bbox| AABB::new(bbox.min() + self.offset, bbox.max() + self.offset))
    }
}

// RotateY

pub struct RotateY<H: Hittable> {
    obj: H,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
}

impl<H: Hittable> RotateY<H> {
    pub fn new(obj: H, angle: f64) -> Self {
        let sin_theta = angle.to_radians().sin();
        let cos_theta = angle.to_radians().cos();

        let rotate = |p: Vec<&f64>| {
            let new_x = cos_theta * p[0] + sin_theta * p[2];
            let new_z = -sin_theta * p[0] + cos_theta * p[2];
            [new_x, *p[1], new_z]
        };

        let bbox = obj.bounding_box().map(|bbox| {
            let p0 = bbox.min();
            let p1 = bbox.max();
            let min_max_corners = [[p0.x, p1.x], [p0.y, p1.y], [p0.z, p1.z]];
            let corners = min_max_corners.iter().multi_cartesian_product().map(rotate);

            let x = corners.clone().map(|p| p[0]).minmax();
            let y = corners.clone().map(|p| p[1]).minmax();
            let z = corners.clone().map(|p| p[2]).minmax();

            match (x, y, z) {
                (MinMax(min_x, max_x), MinMax(min_y, max_y), MinMax(min_z, max_z)) => AABB::new(
                    Point3::new(min_x, min_y, min_z),
                    Point3::new(max_x, max_y, max_z),
                ),
                _ => unreachable!(),
            }
        });

        RotateY {
            obj,
            sin_theta,
            cos_theta,
            bbox,
        }
    }

    fn rotate_around_y(&self, p: Vec3) -> Vec3 {
        let new_x = self.cos_theta * p.x + self.sin_theta * p.z;
        let new_z = -self.sin_theta * p.x + self.cos_theta * p.z;
        Vec3::new(new_x, p.y, new_z)
    }

    fn rotate_around_y_rev(&self, p: Vec3) -> Vec3 {
        let new_x = self.cos_theta * p.x - self.sin_theta * p.z;
        let new_z = self.sin_theta * p.x + self.cos_theta * p.z;
        Vec3::new(new_x, p.y, new_z)
    }
}

impl<H: Hittable> Hittable for RotateY<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rng: &mut RNG) -> Option<HitRecord> {
        let origin_rotated = self.rotate_around_y_rev(r.origin);
        let direction_rotated = self.rotate_around_y_rev(r.direction);

        let rotated_r = Ray::new(origin_rotated, direction_rotated);

        self.obj.hit(&rotated_r, t_min, t_max, rng).map(|hit| {
            HitRecord::new(
                hit.t,
                hit.u,
                hit.v,
                self.rotate_around_y(hit.p),
                hit.material,
                &rotated_r,
                &self.rotate_around_y(hit.normal),
            )
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.bbox
    }
}
