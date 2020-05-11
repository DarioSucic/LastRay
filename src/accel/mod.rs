// TODO: Implement BVH with SAH [0], using either AABB or AABO [1].
// [0]: https://github.com/svenstaro/bvh
// [1]: https://github.com/bryanmcnett/aabo

#[allow(unused)]
use crate::*;

mod slow;
pub use slow::*;

mod bvh;
pub use bvh::*;

/*
pub trait Bounded {
    fn bounds(&self) -> AABB;
}

pub fn hilbert_curve_sort<T: Bounded>(a: &mut [T]) {
    todo!()
}
*/
