use crate::*;
use std::f32::consts::PI;

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
    pub fn with_config_aspect(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        config: &Config,
        focus_dist: f32,
        aperture: f32,
    ) -> Camera {
        let (width, height) = config.resolution;
        let aspect = width as f32 / height as f32;
        Camera::new(look_from, look_at, vup, vfov, aspect, focus_dist, aperture)
    }

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

    fn sample_unit_disk(&self, rng: &mut WyRand) -> (f32, f32) {
        let a: f32 = rng.generate();
        let b: f32 = rng.generate();
        (a.sin(), b.cos())
    }

    pub fn get_ray(&self, rng: &mut WyRand, s: f32, t: f32) -> Ray {
        let (a, b) = self.sample_unit_disk(rng);
        let rd = Vec3::new(a, b, 0.0) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        let origin = self.origin + offset;
        let direction =
            self.lower_left + self.horizontal * s + self.vertical * t - self.origin - offset;

        let inv_direction = Vec3::one() / direction;

        Ray { origin, direction, inv_direction }
    }
}
