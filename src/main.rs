mod base;
mod camera;
mod hit;
mod materials;
mod ray;
mod worlds;

use crate::base::*;
use crate::hit::*;
use crate::ray::Ray;
use crate::worlds::random_scene1;
use indicatif::{ParallelProgressIterator, ProgressBar};
use itertools::Itertools;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.hit(r, 0.001, f64::INFINITY) {
        Some(rec) => match rec.material.scatter(r, &rec) {
            Some(scatter) => multiply_colors(&scatter.0, &ray_color(&scatter.1, world, depth - 1)),
            None => Color::new(0.0, 0.0, 0.0),
        },
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
    let samples_per_pixel = 100;
    let max_depth = 50;

    let (cam, world) = random_scene1(aspect_ratio);

    let bar = ProgressBar::new((image_height * image_width) as u64);
    bar.set_style(indicatif::ProgressStyle::default_bar().progress_chars("=> "));
    bar.set_draw_delta((image_height * image_width / 1000) as u64);

    let pixels: Vec<Color> = (0..(image_height - 1))
        .rev()
        .cartesian_product(0..image_width)
        .collect::<Vec<_>>()
        .into_par_iter()
        .progress_with(bar)
        .map(|(j, i)| {
            let mut rng = SmallRng::from_rng(rand::thread_rng()).unwrap();

            (0..samples_per_pixel)
                // .into_iter()
                .map(|_| {
                    let u = ((i as f64) + rng.gen::<f64>()) / (image_width - 1) as f64;
                    let v = ((j as f64) + rng.gen::<f64>()) / (image_height - 1) as f64;

                    let r = cam.get_ray(u, v);
                    ray_color(&r, &world, max_depth)
                })
                .sum()
        })
        .collect();

    write_color(
        &pixels,
        samples_per_pixel,
        image_width as u32,
        (image_height - 1) as u32,
    );

    println!("Done!");
}
