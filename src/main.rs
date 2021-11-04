mod color;
mod hittable;
mod ray;
mod vec3;

use crate::color::{write_color, Color};
use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::unit_vector;
use crate::vec3::Vec3;
use cgmath::{InnerSpace, Vector3};

fn ray_color(r: &Ray, world: &Hittable) -> Color {
    match world.hit(r, 0.0, f64::INFINITY) {
        Some(rec) => 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0)),
        None => {
            let unit_direction = unit_vector(&r.direction);
            let t = 0.5 * (unit_direction.y + 1.0);

            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;

    // World

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..(image_height - 1)).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(&r, &world);

            write_color(pixel_color);
        }
    }

    eprintln!("Done!");
}
