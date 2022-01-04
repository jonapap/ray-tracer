use crate::base::*;
use crate::materials::Material;
use crate::ray::Ray;
use cgmath::dot;
use std::sync::Arc;

pub struct HitRecord<'a> {
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f64,
        u: f64,
        v: f64,
        point: Point3,
        material: &'a dyn Material,
        r: &Ray,
        outward_normal: &Vec3,
    ) -> Self {
        let tmp = HitRecord {
            p: point,
            normal: Vec3::new(0.0, 0.0, 0.0), // temporary before we calculate it bellow
            t,
            u,
            v,
            material,
            front_face: false,
        };

        tmp.calculate_face_normal(r, outward_normal)
    }

    fn calculate_face_normal(self, r: &Ray, outward_normal: &Vec3) -> Self {
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
