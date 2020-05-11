use crate::geometry::*;
use crate::*;

pub struct Node {
    aabb: AABB,
    left: Option<usize>,
    right: Option<usize>
}

impl Node {
    pub fn is_leaf(&self) -> bool {
        self.right.is_none()
    }
}

pub struct BVH<T: Intersectable> {
    pub objects: Vec<T>,
    pub nodes: Vec<Node>
}

impl<B: Bounded> BVH<B> {
    fn from_objects(objects: Vec<B>) -> BVH<B> {
        todo!("Build BVH from objects")
    }
}

// Need another model...
impl<T: Intersectable> Intersectable for BVH<T> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut best_hit = None;

        let t =  match self.nodes[0].aabb.intersect(ray, t_min, t_max) {
            Some(hit) => hit,
            None => return None,
        };

        let mut node_stack = Vec::with_capacity(64);
        node_stack.push((0, t));

        while !node_stack.is_empty() {

            let (idx_curr, t_curr) = node_stack.pop().unwrap();

            if t_curr >= t_max {
                continue;
            }

            let curr_node = &self.nodes[idx_curr];
            if curr_node.is_leaf() {
                
            }

        }

        best_hit
    }
}
