use crate::material::*;

pub use ultraviolet::{
    geometry::{Aabb, Ray},
    vec::{Vec3, Wec3},
    wide::{const_f32_as_f32x4, f32x4, ConstUnionHack_f32x4},
};

pub use tobj;

mod model;
mod sphere;

pub use model::*;
pub use sphere::*;

pub struct Hit<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a Box<dyn Material + Send + Sync>,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit>;
}

pub trait Bounded {
    fn bounds(&self) -> Aabb;
}

pub fn wec3s_from_flat(flat: &[f32], missing_fill: f32) -> Vec<Wec3> {
    let mut wecs = vec![];
    let xs: Vec<f32> = flat.iter().skip(0).step_by(3).copied().collect();
    let ys: Vec<f32> = flat.iter().skip(1).step_by(3).copied().collect();
    let zs: Vec<f32> = flat.iter().skip(2).step_by(3).copied().collect();

    let xs = xs.chunks(12);
    let ys = ys.chunks(12);
    let zs = zs.chunks(12);

    let missing_fill = f32x4::from(missing_fill);

    for (x, (y, z)) in xs.zip(ys.zip(zs)) {
        let mut w = [Wec3::broadcast(missing_fill); 3];

        for (block, xx) in x.chunks(3).enumerate() {
            let n = 3.min(xx.len());
            for i in 0..n {
                w[i][0][block] = x[3 * block + i];
                w[i][1][block] = y[3 * block + i];
                w[i][2][block] = z[3 * block + i];
            }
        }

        for i in 0..3 {
            wecs.push(w[i]);
        }
    }

    wecs
}
