// TODO: Implement BVH with SAH [0], using either AABB or AABO [1].
// [0]: https://github.com/svenstaro/bvh
// [1]: https://github.com/bryanmcnett/aabo

use crate::geometry::*;

pub struct Accel<T: Intersectable> {
    pub objects: Vec<T>,
}

impl<T: Intersectable> Intersectable for Accel<T> {
    fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit> {
        self.objects
            .iter()
            .filter_map(|o| o.intersect(&ray, tmin, tmax))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}
