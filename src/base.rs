use cgmath::{InnerSpace, Vector3};

pub type Color = Vector3<f64>;

pub(crate) type Vec3 = Vector3<f64>;

pub fn unit_vector(vec: &Vec3) -> Vec3 {
    vec / vec.magnitude()
}

pub fn write_color(color: Color) {
    println!(
        "{} {} {}",
        (255.999 * color.x) as i32,
        (255.999 * color.y) as i32,
        (255.999 * color.z) as i32
    );
}
