mod accel;
mod camera;
mod geometry;
mod material;

use accel::*;
use camera::*;
use geometry::*;
use material::*;

use std::f32::{consts::PI, INFINITY as INF};
use std::io;

use rand::{self, prelude::*};
use rayon::prelude::*;
use write_png::{write_png, ColorType};

fn save_image(filename: &str, data: &[f32], res: (usize, usize)) -> io::Result<()> {
    let data: Vec<u8> = data
        .iter()
        .map(|c| (c.powf(1.0 / 2.0) * 255.99) as u8)
        .collect();
    let data = data
        .chunks_exact(3 * res.0)
        .rev()
        .flatten()
        .copied()
        .collect();
    write_png(filename, res.0, res.1, ColorType::RGB, data)
}

pub fn uniform_sample_sphere(samples: &(f32, f32)) -> Vec3 {
    let z = 1.0 - 2.0 * samples.0;
    let r = (1.0 - z * z).sqrt().max(0.0);
    let phi = PI * 2.0 * samples.1;
    let (psin, pcos) = phi.sin_cos();
    Vec3::new(pcos * r, psin * r, z)
}

fn ray_color<T: Intersectable>(
    ray: &Ray,
    accel: &Accel<T>,
    rng: &mut ThreadRng,
    depth: usize,
) -> Vec3 {
    if let Some(hit) = accel.intersect(ray, 1e-2, INF) {
        if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit, rng) {
            if depth == 25 {
                return attenuation;
            }
            return attenuation * ray_color(&scattered, accel, rng, depth + 1);
        }
    }

    Vec3::one()
}

fn main() -> io::Result<()> {
    const RES: (usize, usize) = (512, 512);
    let (width, height) = RES;

    const SAMPLES_PER_PIXEL: usize = 2048;

    // Image buffer
    let mut buf = vec![0.0; width * height * 3];

    // Setup Camera, Materials, and Objects
    // TODO: Create Scene abstraction

    // Setup Camera
    let aspect = width as f32 / height as f32;
    let look_from = Vec3::new(-0.2, 2.5, 9.5);
    let look_at = Vec3::new(-0.2, 2.5, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = (look_at - look_from).mag();
    let aperture = 0.0;
    let fov = 30.0;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        fov,
        aspect,
        focus_distance,
        aperture,
    );

    let model = Model::load(Vec3::zero(), "res/cornell-box.obj");

    let accel = Accel {
        objects: vec![model],
    };

    buf.par_chunks_mut(3 * width)
        .enumerate()
        .for_each(|(y, line)| {
            let mut rng = rand::thread_rng();
            let v = y as f32 / height as f32;

            for (x, pix) in line.chunks_exact_mut(3).enumerate() {
                let u = x as f32 / width as f32;
                let mut color = Vec3::zero();

                for _ in 0..SAMPLES_PER_PIXEL {
                    let (dx, dy) = rng.gen::<(f32, f32)>();

                    let u = u + dx / width as f32;
                    let v = v + dy / height as f32;

                    // println!("(u, v): {:?}", (u, v));
                    let mut ray = camera.get_ray(&mut rng, u, v);
                    ray.direction.normalize();

                    // println!("RAY: {:?}", ray);

                    color += ray_color(&ray, &accel, &mut rng, 0);
                }

                color /= SAMPLES_PER_PIXEL as f32;
                pix.copy_from_slice(color.as_slice());
            }
        });

    let filename = "out.png";
    save_image(&filename, &buf, RES)
}
