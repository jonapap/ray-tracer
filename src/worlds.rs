use crate::base::*;
use crate::camera::Camera;
use crate::hit::constant_medium::ConstantMedium;
use crate::hit::rectangle::{Cuboid, XYRect, XZRect, YZRect};
use crate::hit::sphere::Sphere;
use crate::hit::transform::Translate;
use crate::hit::HittableList;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::materials::textures::SolidColor;
use crate::random::RNG;
use crate::ray::Ray;
use crate::transform::RotateY;
use cgmath::InnerSpace;
use std::sync::Arc;

type Scene = (Camera, HittableList, Background);

fn blue_sky(r: &Ray) -> Color {
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub fn random_scene1() -> Scene {
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
        3.0 / 2.0,
        aperture,
        dist_to_focus,
    );

    return (cam, world, blue_sky);
}

pub fn simple_scene1() -> Scene {
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
        3.0 / 2.0,
        0.1,
        1.0,
    );

    return (cam, world, blue_sky);
}

pub fn light_scene() -> Scene {
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
        3.0 / 2.0,
        0.0,
        10.0,
    );

    (cam, world, |_| Color::new(0.0, 0.0, 0.0))
}

pub fn cornell_box() -> Scene {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Box::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.add(Box::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Box::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Box::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let box1 = Cuboid::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));
    let box1 = ConstantMedium::new_from_color(box1, 0.01, Color::new(1.0, 1.0, 1.0));
    world.add(Box::new(box1));

    let box2 = Cuboid::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));
    world.add(Box::new(box2));

    let cam = Camera::new(
        &Vec3::new(278.0, 278.0, -800.0),
        &Vec3::new(278.0, 278.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        40.0,
        1.0,
        0.0,
        10.0,
    );

    (cam, world, |_| Color::new(0.0, 0.0, 0.0))
    // (cam, world, blue_sky)
}
