use crate::base::*;
use crate::ray::Ray;

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
            let t0 = f64::min(
                (self.minimum[a] - r.origin[a]) / r.direction[a],
                (self.maximum[a] - r.origin[a]) / r.direction[a],
            );
            let t1 = f64::max(
                (self.minimum[a] - r.origin[a]) / r.direction[a],
                (self.maximum[a] - r.origin[a]) / r.direction[a],
            );
            let t_min = f64::max(t0, t_min);
            let t_max = f64::min(t1, t_max);
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
            f64::min(box0.max().x, box1.max().x),
            f64::min(box0.max().y, box1.max().y),
            f64::min(box0.max().z, box1.max().z),
        );

        AABB::new(small, big)
    }
}
