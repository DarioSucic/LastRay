use crate::material::*;

pub use ultraviolet::{
    geometry::Ray,
    vec::{Vec3, Wec3},
    wide::{const_f32_as_f32x4, f32x4, ConstUnionHack_f32x4},
};

pub use tobj;

mod model;
mod sphere;
mod wec3x8;

pub use model::*;
pub use sphere::*;
pub use wec3x8::*;

pub struct Hit<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a MaterialType,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit>;
}

pub fn wec3s_from_flat(flat: &[f32], chunk_size: usize, missing_fill: f32) -> Vec<Wec3x8> {
    let mut wecs = vec![];
    let xs: Vec<f32> = flat.iter().skip(0).step_by(3).copied().collect();
    let ys: Vec<f32> = flat.iter().skip(1).step_by(3).copied().collect();
    let zs: Vec<f32> = flat.iter().skip(2).step_by(3).copied().collect();

    let xs = xs.chunks(8 * chunk_size);
    let ys = ys.chunks(8 * chunk_size);
    let zs = zs.chunks(8 * chunk_size);

    let missing_fill = f32x8::splat(missing_fill);

    let mut w = vec![Wec3x8::broadcast(missing_fill); chunk_size];

    for (x, (y, z)) in xs.zip(ys.zip(zs)) {
        for c in w.iter_mut() {
            *c = Wec3x8::broadcast(missing_fill);
        }

        for (block, xx) in x.chunks(chunk_size).enumerate() {
            for i in 0..xx.len() {
                w[i][0] = w[i][0].replace(block, x[chunk_size * block + i]);
                w[i][1] = w[i][1].replace(block, y[chunk_size * block + i]);
                w[i][2] = w[i][2].replace(block, z[chunk_size * block + i]);
            }
        }

        for i in 0..chunk_size {
            wecs.push(w[i]);
        }
    }

    wecs
}
