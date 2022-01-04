mod aabb;
mod base;
mod camera;
mod hit;
mod materials;
mod random;
mod ray;
mod worlds;

use crate::base::*;
use crate::bvh::BVHNode;
use crate::hit::*;
use crate::random::RNG;
use crate::ray::Ray;
use crate::worlds::random_scene1;
use clap::{App, Arg};
use indicatif::{ParallelProgressIterator, ProgressBar};
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::time::Instant;

fn ray_color<T: Hittable>(
    r: &Ray,
    world: &T,
    depth: i32,
    background: Background,
    rng: &mut RNG,
) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.hit(r, 0.001, f64::INFINITY) {
        Some(rec) => {
            let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
            match rec.material.scatter(r, &rec, rng) {
                Some((color, ray)) => {
                    emitted
                        + color.multiply_with(&ray_color(&ray, world, depth - 1, background, rng))
                }
                None => emitted,
            }
        }
        None => background(r),
    }
}

fn main() {
    let matches = App::new("Ray-Tracer Engine")
        .version("1.0")
        .author("Jonathan Papineau <hello@jontech.app>")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Output file")
                .takes_value(true)
                .default_value("out.png"),
        )
        .get_matches();

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let (cam, world, background) = random_scene1(aspect_ratio);

    let bar = ProgressBar::new((image_height * image_width) as u64);
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed}] {bar:40.cyan/blue} {pos:>7}/{len:7} eta:{eta}")
            .progress_chars("=> "),
    );
    bar.set_draw_delta((image_height * image_width / 1000) as u64);

    println!("Starting to build BVH tree...");
    let world = BVHNode::new_from_hittable_list(world);
    println!("Done building the tree");

    println!("Starting to render...");
    let start = Instant::now();
    let pixels: Vec<Color> = (0..(image_height - 1))
        .rev()
        .cartesian_product(0..image_width)
        .collect::<Vec<_>>()
        .into_par_iter()
        .progress_with(bar)
        .map(|(j, i)| {
            let mut rng = RNG::new();

            (0..samples_per_pixel)
                // .into_iter()
                .map(|_| {
                    let u = ((i as f64) + rng.random_double()) / (image_width - 1) as f64;
                    let v = ((j as f64) + rng.random_double()) / (image_height - 1) as f64;

                    let r = cam.get_ray(u, v, &mut rng);
                    ray_color(&r, &world, max_depth, background, &mut rng)
                })
                .sum()
        })
        .collect();

    write_color(
        &pixels,
        samples_per_pixel,
        image_width as u32,
        (image_height - 1) as u32,
        matches.value_of("output").unwrap(),
    );

    println!("Done! Rendered in {:?}", start.elapsed());
}
