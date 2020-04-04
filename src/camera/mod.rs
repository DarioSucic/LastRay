use crate::geometry::*;
use std::f32::consts::PI;

use rand::{self, prelude::*};
#[derive(Clone)]
pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        focus_dist: f32,
        aperture: f32,
    ) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = look_from;
        let w = (look_from - look_at).normalized();
        let u = vup.cross(w).normalized();
        let v = w.cross(u);

        let lower_left = origin - focus_dist * (u * half_width + v * half_height + w);
        let horizontal = u * 2.0 * focus_dist * half_width;
        let vertical = v * 2.0 * focus_dist * half_height;

        Camera {
            origin,
            lower_left,
            horizontal,
            vertical,
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    fn sample_unit_disk(&self, rng: &mut ThreadRng) -> (f32, f32) {
        let (a, b) = rng.gen::<(f32, f32)>();
        (a.sin(), b.cos())
    }

    pub fn get_ray(&self, rng: &mut ThreadRng, s: f32, t: f32) -> Ray {
        let (a, b) = self.sample_unit_disk(rng);
        let rd = Vec3::new(a, b, 0.0) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        let origin = self.origin + offset;
        let direction =
            self.lower_left + self.horizontal * s + self.vertical * t - self.origin - offset;

        Ray { origin, direction }
    }
}
