use crate::base::Vec3;
use crate::ray::Ray;
use cgmath::dot;

pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    front_face: Option<bool>,
}

impl HitRecord {
    pub fn new_with_face_normal(
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
