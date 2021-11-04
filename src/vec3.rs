use cgmath::{InnerSpace, Vector3};

pub(crate) type Vec3 = Vector3<f64>;

pub fn unit_direction(vec: &Vec3) -> Vec3 {
    vec / vec.magnitude()
}
