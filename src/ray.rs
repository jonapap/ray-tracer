use crate::base::*;

pub struct Ray {
    pub origin: Point3,
    pub direction: Point3,
}

impl Ray {
    pub(crate) fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    pub(crate) fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
