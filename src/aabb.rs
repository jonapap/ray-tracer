use crate::base::*;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> AABB {
        AABB { minimum, maximum }
    }

    pub fn min(&self) -> Point3 {
        self.minimum
    }

    pub fn max(&self) -> Point3 {
        self.maximum
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let t0 = (self.min()[a] - r.origin[a]) * inv_d;
            let t1 = (self.max()[a] - r.origin[a]) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }
        return true;
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Point3::new(
            f64::min(box0.min().x, box1.min().x),
            f64::min(box0.min().y, box1.min().y),
            f64::min(box0.min().z, box1.min().z),
        );

        let big = Point3::new(
            f64::max(box0.max().x, box1.max().x),
            f64::max(box0.max().y, box1.max().y),
            f64::max(box0.max().z, box1.max().z),
        );

        AABB::new(small, big)
    }
}
