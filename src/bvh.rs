use crate::aabb::*;
use crate::hit::*;
use crate::ray::*;
use crate::scenes::*;

#[allow(dead_code)]
pub enum BvhContents {
    Leaf(Box<dyn Hit>),
    Node {
        left: Box<Bvh>,
        right: Box<Bvh>
    }
}

#[allow(dead_code)]
pub struct Bvh {
    size: usize,
    bb: Aabb,
    contents: BvhContents
}

impl Bvh {
    #[allow(dead_code)]
    pub fn new(mut world: Objects, t_min: f64, t_max: f64) -> Bvh {
        fn range_on_axis(world: &[Box<dyn Hit>], t_min: f64, t_max: f64, axis: usize) -> f64 {
            assert!(axis <= 2);

            let (min, max) = world.iter().fold((std::f64::MIN, std::f64::MAX), |range, obj| {
                if let Some(bb) = obj.bounding_box(t_min, t_max) {
                    let min = bb.min[axis].min(bb.max[axis]);
                    let max = bb.min[axis].max(bb.max[axis]);
                    (min, max)
                } else {
                    range
                }
            });

            max - min
        }

        let best_axis = {
            let mut axes = [
                (0, range_on_axis(&world, t_min, t_max, 0)),
                (1, range_on_axis(&world, t_min, t_max, 1)),
                (2, range_on_axis(&world, t_min, t_max, 2))
            ];

            axes.sort_unstable_by(|a, b| {
                b.1.partial_cmp(&a.1).unwrap()
            });
            axes[0].0
        };

        world.sort_unstable_by(|a, b| {
            let a_bb = a.bounding_box(t_min, t_max);
            let b_bb = b.bounding_box(t_min, t_max);
            match (a_bb, b_bb) {
                (Some(a_bb), Some(b_bb)) => {
                    let a_cent = a_bb.min[best_axis] + a_bb.max[best_axis];
                    let b_cent = b_bb.min[best_axis] + b_bb.max[best_axis];
                    a_cent.partial_cmp(&b_cent).unwrap()
                },
                _ => std::cmp::Ordering::Greater
            }
        });

        match world.len() {
            0 => panic!("Can't create a BVH-object from an empty world."),
            1 => Bvh {
                bb: world[0].bounding_box(t_min, t_max).expect("Can't create a BVH-object from objects without a bounding-box"),
                contents: BvhContents::Leaf(world.pop().unwrap()),
                size: 1,
            },
            _ => {
                let right = Box::new(
                    Bvh::new(
                        world.drain((world.len() / 2)..).collect(),
                        t_min,
                        t_max
                    )
                );
                let left = Box::new(
                    Bvh::new(
                        world.drain(..).collect(),
                        t_min,
                        t_max
                    )
                );

                Bvh {
                    size: left.size + right.size,
                    bb: Aabb::surround(left.bounding_box(t_min, t_max), right.bounding_box(t_min, t_max)).expect("Can't create a BVH-object from objects without a bounding-box"),
                    contents: BvhContents::Node {
                        left,
                        right
                    },
                }
            }
        }
    }
}

impl Hit for Bvh {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bb.hit(ray, t_min, t_max) {
            match &self.contents {
                BvhContents::Leaf(obj) => {
                    obj.hit(ray, t_min, t_max)
                },
                BvhContents::Node { left, right } => {
                    let mut new_max = t_max;
                    let left_hit = left.hit(ray, t_min, t_max);
                    if let Some(hit) = left_hit {
                        new_max = hit.t;
                    }
    
                    let right_hit = right.hit(ray, t_min, new_max);
    
                    match (left_hit, right_hit) {
                        (h, None) | (None, h) => h,
                        (Some(left), Some(right)) => {
                            if left.t < right.t  {
                                Some(left)
                            } else {
                                Some(right)
                            }
                        }
                    }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(self.bb)
    }
}
