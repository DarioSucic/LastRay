use std::simd::f32x8;

use crate::material::*;

pub use ultraviolet::vec::{Vec3};

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

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub inv_direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub trait Bounded: Intersectable {
    fn bounds(&self) -> AABB;
}
pub struct AABB {
    pub mini: Vec3,
    pub maxi: Vec3,
}

impl AABB {
    pub fn degenerate() -> AABB {
        AABB {
            mini: Vec3::zero(),
            maxi: Vec3::zero(),
        }
    }

    pub fn surface_area(&self) -> f32 {
        let delta = self.maxi - self.mini;
        let half = delta.x * delta.y + delta.x * delta.z + delta.z * delta.y;
        half + half
    }

    pub fn union(self, other: AABB) -> AABB {
        AABB {
            mini: self.mini.min_by_component(other.mini),
            maxi: self.maxi.max_by_component(other.maxi)
        }
    }

    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<f32> {
        for i in 0..3 {
            if ray.direction[i] == 0.0
                && (ray.origin[i] < self.mini[i] || ray.origin[i] > self.maxi[i])
            {
                return None;
            }
        }

        let mut t1: Vec3 = (self.mini - ray.origin) * ray.inv_direction;
        let mut t2: Vec3 = (self.maxi - ray.origin) * ray.inv_direction;

        use std::mem;
        for i in 0..3 {
            if t1[i] > t2[i] {
                mem::swap(&mut t1[i], &mut t2[i]);
            }
        }

        let tend = t2.component_min();
        if tend < t_min {
            return None;
        }

        let tstart = t1.component_max();
        if tstart > tend {
            return None;
        }

        Some(tstart)
    }
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
                w[i][0].as_mut_array()[block] = x[chunk_size * block + i];
                w[i][1].as_mut_array()[block] = y[chunk_size * block + i];
                w[i][2].as_mut_array()[block] = z[chunk_size * block + i];
            }
        }

        for i in 0..chunk_size {
            wecs.push(w[i]);
        }
    }

    wecs
}
