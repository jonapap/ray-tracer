use crate::aabb::AABB;
use crate::base::*;
use crate::hit::hit_record::HitRecord;
use crate::hit::{Hittable, HittableList};
use crate::random::RNG;
use crate::ray::Ray;
use itertools::Itertools;
use std::cmp::Ordering;
use std::error::Error;

enum LinearBVHNodeChild {
    RightTree(usize),
    Element(Box<dyn Hittable>),
}

struct LinearBVHNode {
    bounds: AABB,
    child: LinearBVHNodeChild,
}

pub struct LinearBVHTree {
    list: Vec<LinearBVHNode>,
}

impl LinearBVHTree {
    pub fn new_from_bvhnode(node: BVHNode) -> LinearBVHTree {
        let mut list = LinearBVHTree { list: Vec::new() };

        let mut offset = 0;
        list.flatten_bvhtree(node, &mut offset);

        list
    }

    fn flatten_bvhtree(&mut self, node: BVHNode, offset: &mut usize) -> usize {
        let myoffset = *offset;

        *offset += 1;

        let node = match node.child {
            BVHNodeChild::Trees(left, right) => {
                self.list.push(LinearBVHNode {
                    bounds: node.bounds,
                    child: LinearBVHNodeChild::RightTree(0),
                });
                self.flatten_bvhtree(*left, offset);

                let right_index = self.flatten_bvhtree(*right, offset);

                self.list.get_mut(myoffset).unwrap().child =
                    LinearBVHNodeChild::RightTree(right_index);
            }
            BVHNodeChild::Element(a) => self.list.push(LinearBVHNode {
                bounds: node.bounds,
                child: LinearBVHNodeChild::Element(a),
            }),
        };

        return myoffset;
    }

    fn hit_aux(&self, r: &Ray, t_min: f64, t_max: f64, index: usize) -> Option<HitRecord> {
        let node = self.list.get(index).unwrap();

        if !node.bounds.hit(r, t_min, t_max) {
            return None;
        }

        match &node.child {
            LinearBVHNodeChild::Element(a) => a.hit(r, t_min, t_max),
            LinearBVHNodeChild::RightTree(right) => {
                let left = index + 1;

                let hit_left = self.hit_aux(r, t_min, t_max, left);

                let hit_right = match &hit_left {
                    Some(a) => self.hit_aux(r, t_min, a.t, *right),
                    _ => self.hit_aux(r, t_min, t_max, *right),
                };

                match (hit_left, hit_right) {
                    (_, Some(r)) => Some(r),
                    (Some(l), _) => Some(l),
                    (_, _) => None,
                }
            }
        }
    }
}

impl Hittable for LinearBVHTree {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.hit_aux(r, t_min, t_max, 0)
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.list.get(0).unwrap().bounds)
    }
}

enum BVHNodeChild {
    Trees(Box<BVHNode>, Box<BVHNode>),
    Element(Box<dyn Hittable>),
    // Element2(Box<dyn Hittable>, Box<dyn Hittable>),
}

pub struct BVHNode {
    child: BVHNodeChild,
    bounds: AABB,
}

impl BVHNode {
    pub fn new_from_hittable_list(list: HittableList) -> BVHNode {
        BVHNode::build_using_sah(list.list)
    }

    /*
    Build the BVH tree using a simple algorithm. We recursively decide a random axis, sort the items
    along that axis, and split the items into two.
     */
    // fn build_simple(mut objects: Vec<Box<dyn Hittable>>) -> BVHNode {
    //     let mut rng = RNG::new();
    //
    //     let main_box = get_aabb_from_list(&objects);
    //
    //     let axis = rng.random_int(0..3);
    //     let comparator = match axis {
    //         0 => compare_box_by_x_axis,
    //         1 => compare_box_by_y_axis,
    //         _ => compare_box_by_z_axis,
    //     };
    //
    //     objects.sort_by(|a, b| comparator(a, b));
    //
    //     let (left, right) = match objects.len() {
    //         1 => (objects.remove(0), None),
    //         2 => (objects.remove(0), Some(objects.remove(0))),
    //         _ => {
    //             let mid = objects.len() / 2;
    //
    //             let left_objs: Vec<_> = objects.drain(0..mid).collect();
    //             let right_objs = objects;
    //
    //             let left: Box<dyn Hittable> = Box::new(BVHNode::build_simple(left_objs));
    //             let right: Box<dyn Hittable> = Box::new(BVHNode::build_simple(right_objs));
    //             (left, Some(right))
    //         }
    //     };
    //
    //     BVHNode {
    //         left,
    //         right,
    //         bounds: main_box,
    //     }
    // }

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

        let child = match objects.len() {
            1 => BVHNodeChild::Element(objects.remove(0)),
            // 2 => BVHNodeChild::Element2(objects.remove(0), objects.remove(0)),
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

                let left = Box::new(BVHNode::build_using_sah(left_objs));
                let right = Box::new(BVHNode::build_using_sah(right_objs));
                BVHNodeChild::Trees(left, right)
            }
        };

        BVHNode {
            child,
            bounds: main_box,
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
