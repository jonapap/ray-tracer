use crate::base::*;
use crate::camera::Camera;
use crate::hit::rectangle::{XYRect, XZRect, YZRect};
use crate::hit::sphere::Sphere;
use crate::hit::HittableList;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::materials::textures::SolidColor;
use crate::random::RNG;
use crate::ray::Ray;
use cgmath::InnerSpace;
use std::sync::Arc;

type Scene = (Camera, HittableList, Background);

fn blue_sky(r: &Ray) -> Color {
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub fn random_scene1(aspect_ratio: f64) -> Scene {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(SolidColor::new(Color::new(0.5, 0.5, 0.5))));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = RNG::new();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random_double();
            let center = Vec3::new(
                (a as f64) + 0.9 * rng.random_double(),
                0.2,
                (b as f64) + 0.9 * rng.random_double(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = rng.random_vector().multiply_with(&rng.random_vector());
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Lambertian::new(SolidColor::new(albedo))),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = rng.random_vector_range(0.5..1.0);
                    let fuzz = rng.random_double_range(0.0..0.5);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    // glass
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                };
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(SolidColor::new(Color::new(0.4, 0.2, 0.1))));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    return (cam, world, blue_sky);
}

pub fn simple_scene1(aspect_ratio: f64) -> Scene {
    // World

    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(SolidColor::new(
        (Color::new(0.8, 0.8, 0.0)),
    )));
    let material_center = Arc::new(Dielectric::new(1.5));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera

    let cam = Camera::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -1.0),
        &Vec3::new(0.0, 1.0, 0.0),
        90.0,
        aspect_ratio,
        0.1,
        1.0,
    );

    return (cam, world, blue_sky);
}

pub fn light_scene(aspect_ratio: f64) -> Scene {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(SolidColor::new(Color::new(0.0, 0.8, 0.3)))),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(SolidColor::new(Color::new(0.9, 0.0, 0.3)))),
    )));

    let difflight = Arc::new(DiffuseLight::from_color(Color::new(14.0, 14.0, 14.0)));
    world.add(Box::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    let cam = Camera::new(
        &Vec3::new(26.0, 3.0, 6.0),
        &Vec3::new(0.0, 2.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.0,
        10.0,
    );

    (cam, world, |_| Color::new(0.0, 0.0, 0.0))
    // (cam, world, blue_sky)
}
