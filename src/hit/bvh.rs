use crate::aabb::AABB;
use crate::base::*;
use crate::hit::hit_record::HitRecord;
use crate::hit::{Hittable, HittableList};
use crate::ray::Ray;
use itertools::Itertools;
use std::cmp::Ordering;

enum Axis {
    X,
    Y,
    Z,
}

pub struct BVHNode {
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
    aabb: AABB,
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = match &self.left {
            Some(a) => a.hit(r, t_min, t_max),
            None => None,
        };

        let hit_right = match (&hit_left, &self.right) {
            (Some(a), Some(right)) => right.hit(r, t_min, a.t),
            (None, Some(right)) => right.hit(r, t_min, t_max),
            _ => None,
        };

        match (hit_left, hit_right) {
            (_, Some(r)) => Some(r),
            (Some(l), _) => Some(l),
            (_, _) => None,
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(self.aabb)
    }
}

impl BVHNode {
    pub fn new_from_hittable_list(list: HittableList, time0: f64, time1: f64) -> BVHNode {
        BVHNode::new(list.list, time0, time1)
    }

    pub fn new(mut objects: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> BVHNode {
        // let main_box = objects
        //     .iter()
        //     .map(|x| x.bounding_box(time0, time1))
        //     .reduce(|a, b| match (a, b) {
        //         (Some(a), Some(b)) => Some(AABB::surrounding_box(&a, &b)),
        //         _ => None,
        //     })
        //     .unwrap()
        //     .unwrap();

        let axis = random_int(0..3);
        let comparator = match axis {
            0 => compare_box_by_x_axis,
            1 => compare_box_by_y_axis,
            _ => compare_box_by_z_axis,
        };

        objects.sort_by(|a, b| comparator(a, b));

        let (left, right) = match objects.len() {
            1 => (Some(objects.remove(0)), None),
            2 => (Some(objects.remove(0)), Some(objects.remove(0))),
            _ => {
                let mid = objects.len() / 2;

                let left_objs: Vec<_> = objects.drain(0..mid).collect();
                let right_objs = objects;

                let left: Box<dyn Hittable> = Box::new(BVHNode::new(left_objs, time0, time1));
                let right: Box<dyn Hittable> = Box::new(BVHNode::new(right_objs, time0, time1));
                (Some(left), Some(right))
            }
        };

        let aabb = match (&left, &right) {
            (Some(a), Some(b)) => {
                match (a.bounding_box(time0, time1), b.bounding_box(time0, time1)) {
                    (Some(ab), Some(bb)) => AABB::surrounding_box(&ab, &bb),
                    (_, _) => panic!("No bounding box in BVNNode constructor!"),
                }
            }
            (Some(a), None) => match a.bounding_box(time0, time1) {
                Some(ab) => ab,
                _ => panic!("No bounding box in BVNNode constructor!"),
            },
            _ => panic!("Trying to create a bounding box on two None elements!"),
        };

        BVHNode { left, right, aabb }
    }
}

fn get_boxes(left: &Box<dyn Hittable>, right: &Box<dyn Hittable>) -> (AABB, AABB) {
    let l_box = left.bounding_box(0.0, 0.0);
    let r_box = right.bounding_box(0.0, 0.0);

    let l_box = match l_box {
        Some(bounds) => bounds,
        None => panic!("Encountered an object with no bounding box!"),
    };

    let r_box = match r_box {
        Some(bounds) => bounds,
        None => panic!("Encountered an object with no bounding box!"),
    };
    (l_box, r_box)
}

fn compare_box_by_x_axis(left: &Box<dyn Hittable>, right: &Box<dyn Hittable>) -> Ordering {
    let (l_box, r_box) = get_boxes(left, right);

    l_box.min().x.partial_cmp(&r_box.min().x).unwrap()
}

fn compare_box_by_y_axis(left: &Box<dyn Hittable>, right: &Box<dyn Hittable>) -> Ordering {
    let (l_box, r_box) = get_boxes(left, right);

    l_box.min().z.partial_cmp(&r_box.min().z).unwrap()
}

fn compare_box_by_z_axis(left: &Box<dyn Hittable>, right: &Box<dyn Hittable>) -> Ordering {
    let (l_box, r_box) = get_boxes(left, right);

    l_box.min().z.partial_cmp(&r_box.min().z).unwrap()
}
