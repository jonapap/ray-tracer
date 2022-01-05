#![allow(dead_code)]

use std::time::Instant;

use clap::Parser;
use indicatif::{ParallelProgressIterator, ProgressBar};
use itertools::Itertools;
use rayon::prelude::*;

use crate::base::*;
use crate::bvh::BVHNode;
use crate::hit::*;
use crate::random::RNG;
use crate::ray::Ray;
use crate::worlds::Worlds;

mod aabb;
mod base;
mod camera;
mod hit;
mod materials;
mod random;
mod ray;
mod worlds;

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

    match world.hit(r, 0.001, f64::INFINITY, rng) {
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

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(arg_enum)]
    world: Worlds,

    #[clap(default_value_t = String::from("out.png"), short, long)]
    output: String,

    #[clap(default_value_t = 600, short = 'w', long)]
    image_width: u32,

    #[clap(default_value_t = 200, short, long)]
    samples_per_pixel: u32,
}

fn main() {
    let args = Args::parse();

    // Scene
    let (cam, world, background) = args.world.get_scene();

    // Image
    let aspect_ratio = cam.get_aspect_ratio();
    let image_width = args.image_width;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = args.samples_per_pixel;
    let max_depth = 50;

    // Progress Bar
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
        &args.output,
    );

    println!("Done! Rendered in {:?}", start.elapsed());
}
