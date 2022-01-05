use std::ops::Range;

use cgmath::{dot, InnerSpace};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::base::Vec3;

pub struct RNG {
    rng: SmallRng,
}

impl RNG {
    pub fn new() -> RNG {
        RNG {
            rng: SmallRng::from_entropy(),
        }
    }

    pub fn random_vector(&mut self) -> Vec3 {
        Vec3::new(self.rng.gen(), self.rng.gen(), self.rng.gen())
    }

    pub fn random_vector_range(&mut self, r: Range<f64>) -> Vec3 {
        Vec3::new(
            self.rng.gen_range(r.clone()),
            self.rng.gen_range(r.clone()),
            self.rng.gen_range(r.clone()),
        )
    }

    pub fn random_in_unit_sphere(&mut self) -> Vec3 {
        loop {
            let p = self.random_vector_range(-1.0..1.0);
            if p.magnitude2() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    pub fn random_unit_vector(&mut self) -> Vec3 {
        return self.random_in_unit_sphere().normalize();
    }

    pub fn random_in_hemisphere(&mut self, normal: &Vec3) -> Vec3 {
        let in_unit_sphere = self.random_in_unit_sphere();
        if dot(in_unit_sphere, *normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_int(&mut self, a: Range<i32>) -> i32 {
        self.rng.gen_range(a)
    }

    pub fn random_double(&mut self) -> f64 {
        self.rng.gen()
    }

    pub fn random_double_range(&mut self, a: Range<f64>) -> f64 {
        self.rng.gen_range(a)
    }

    pub fn random_in_unit_disk(&mut self) -> Vec3 {
        loop {
            let p = Vec3::new(
                self.rng.gen_range(-1.0..1.0),
                self.rng.gen_range(-1.0..1.0),
                self.rng.gen_range(-1.0..1.0),
            );
            if p.magnitude2() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}
