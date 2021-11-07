use crate::base::Vec3;
use crate::materials::Material;
use crate::ray::Ray;
use cgmath::dot;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f64,
        point: Vec3,
        normal: Vec3,
        material: &'a dyn Material,
        r: &Ray,
        outward_normal: &Vec3,
    ) -> HitRecord<'a> {
        let tmp = HitRecord {
            p: point,
            normal,
            t,
            material,
            front_face: false,
        };

        tmp.calculate_face_normal(r, outward_normal)
    }

    fn calculate_face_normal(self, r: &Ray, outward_normal: &Vec3) -> HitRecord<'a> {
        let front_face = dot(r.direction, *outward_normal) < 0.0;
        HitRecord {
            front_face,
            normal: if front_face {
                outward_normal.clone()
            } else {
                -outward_normal.clone()
            },
            ..self
        }
    }
}
