use crate::aabb::AABB;
use crate::base::*;
use crate::hit::hit_record::HitRecord;
use crate::hit::{Hittable, HittableList};
use crate::ray::Ray;
use itertools::Itertools;
use std::cmp::Ordering;
use std::error::Error;

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

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.aabb)
    }
}

impl BVHNode {
    pub fn new_from_hittable_list(list: HittableList) -> BVHNode {
        BVHNode::build_using_sah(list.list)
    }

    /*
    Build the BVH tree using a simple algorithm. We recursively decide a random axis, sort the items
    along that axis, and split the items into two.
     */
    fn build_simple(mut objects: Vec<Box<dyn Hittable>>) -> BVHNode {
        let main_box = get_aabb_from_list(&objects);

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

                let left: Box<dyn Hittable> = Box::new(BVHNode::build_simple(left_objs));
                let right: Box<dyn Hittable> = Box::new(BVHNode::build_simple(right_objs));
                (Some(left), Some(right))
            }
        };

        BVHNode {
            left,
            right,
            aabb: main_box,
        }
    }

    /*
    Build the BVH tree using a SAH (surface-area heuristic). We recursively call this function. Each
     time, we compute the AABB of the objects, take the longest axis of the AABB, sort the items
     along that axis, consider all possible splits and choose the best one based on SAH.
     */
    fn build_using_sah(mut objects: Vec<Box<dyn Hittable>>) -> BVHNode {
        let main_box = get_aabb_from_list(&objects);

        let axis = main_box.longest_axis();
        let comparator = match axis {
            Axis::X => compare_box_by_x_axis,
            Axis::Y => compare_box_by_y_axis,
            Axis::Z => compare_box_by_z_axis,
        };

        objects.sort_by(|a, b| comparator(a, b));

        let (left, right) = match objects.len() {
            1 => (Some(objects.remove(0)), None),
            2 => (Some(objects.remove(0)), Some(objects.remove(0))),
            _ => {
                let main_box_area = main_box.area();

                // Compute the optimal split based on a SAH
                let min_cost = (1..objects.len())
                    .map(|i| {
                        let (left, right) = objects.split_at(i);

                        let left_area = get_aabb_from_list(left).area();
                        let right_area = get_aabb_from_list(right).area();

                        let cost = (left_area / main_box_area) * (left.len() as f64)
                            + (right_area / main_box_area) * (right.len() as f64);

                        (i, cost)
                    })
                    .reduce(|a, b| if a.1 < b.1 { a } else { b })
                    .unwrap();

                let left_objs: Vec<_> = objects.drain(0..(min_cost.0)).collect();
                let right_objs = objects;

                let left: Box<dyn Hittable> = Box::new(BVHNode::build_using_sah(left_objs));
                let right: Box<dyn Hittable> = Box::new(BVHNode::build_using_sah(right_objs));
                (Some(left), Some(right))
            }
        };

        BVHNode {
            left,
            right,
            aabb: main_box,
        }
    }
}

fn get_boxes(left: &Box<dyn Hittable>, right: &Box<dyn Hittable>) -> (AABB, AABB) {
    let l_box = left.bounding_box();
    let r_box = right.bounding_box();

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

fn get_aabb_from_list(list: &[Box<dyn Hittable>]) -> AABB {
    list.iter()
        .map(|x| x.bounding_box())
        .reduce(|a, b| match (a, b) {
            (Some(a), Some(b)) => Some(AABB::surrounding_box(&a, &b)),
            _ => None,
        })
        .unwrap()
        .unwrap()
}
