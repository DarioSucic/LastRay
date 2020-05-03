#![feature(test)]
#![feature(core_intrinsics)]

#[allow(unused)]
use std::intrinsics::{likely, unlikely};

extern crate test;
#[allow(unused)]
use test::{black_box, Bencher};

mod accel;
mod camera;
mod geometry;
mod material;
mod scene;

use accel::*;
use camera::*;
use geometry::*;
use material::*;
use scene::*;

use std::f32::{consts::PI, INFINITY as INF};
use std::io;

use rand::{self, prelude::*};
use rayon::prelude::*;
use write_png::{write_png, ColorType};

fn save_image(filename: &str, data: &[f32], res: (usize, usize)) -> io::Result<()> {
    let data: Vec<u8> = data
        .iter()
        .map(|c| (c.powf(1.0 / 2.2) * 255.99) as u8)
        .collect();
    let data = data
        .chunks_exact(3 * res.0)
        .rev()
        .flatten()
        .copied()
        .collect();
    write_png(filename, res.0, res.1, ColorType::RGB, data)
}

fn convolution(data: &mut [f32], dim: (usize, usize), kernel: &[f32], kernel_dim: (usize, usize)) {
    let (width, height) = dim;
    let (kx, ky) = kernel_dim;
    let (kw, kh) = (kx / 2, ky / 2);

    for i in kh..height - kh {
        for j in kw..width - kw {
            for c in 0..3 {
                let mut total = 0.0;
                for y in 0..ky {
                    for x in 0..kx {
                        let k = kernel[y * kx + x];
                        let p = data[3 * ((i + y - kh) * width + (j + x - kw)) + c];
                        total += p * k;
                    }
                }
                data[3 * (i * width + j) + c] = total;
            }
        }
    }
}

fn apply_filters(data: &mut [f32], dim: (usize, usize)) {
    let kernel: Vec<f32> = [
        1, 2, 1,
        2, 4, 2,
        1, 2, 1
    ].iter().map(|&x| x as f32 / 16.0).collect();

    let kernel_dim = (3, 3);
    convolution(data, dim, &kernel, kernel_dim);
}

pub fn uniform_sample_sphere(samples: &(f32, f32)) -> Vec3 {
    let z = 1.0 - 2.0 * samples.0;
    let r = (1.0 - z * z).sqrt().max(0.0);
    let phi = PI * 2.0 * samples.1;
    let (psin, pcos) = phi.sin_cos();
    Vec3::new(pcos * r, psin * r, z)
}

fn ray_color<T: Intersectable>(ray: &Ray, accel: &T, rng: &mut ThreadRng, depth: usize) -> Vec3 {
    if depth < 32 {
        if let Some(hit) = accel.intersect(ray, 1e-3, INF) {
            match hit.material {
                MaterialType::Receiver(material) => {
                    if let Some((attenuation, scattered)) = material.scatter(ray, &hit, rng) {
                        return attenuation * ray_color(&scattered, accel, rng, depth + 1);
                    }
                }
                MaterialType::Emitter(material) => {
                    if let Some((attenuation, scattered)) = material.scatter(ray, &hit, rng) {
                        return attenuation;
                        // return attenuation * ray_color(&scattered, accel, rng, depth + 1);
                    }
                }
            }
        }
    }

    Vec3::zero()
}

fn trace_scene<I: Intersectable + Send + Sync>(scene: &Scene<I>, buf: &mut [f32]) {
    let Scene {
        accel,
        camera,
        config,
    } = scene;
    let (width, height) = config.resolution;

    let f = |(y, line): (usize, &mut [f32])| {
        let mut rng = rand::thread_rng();
        let v = y as f32 / height as f32;

        for (x, pix) in line.chunks_exact_mut(3).enumerate() {
            let u = x as f32 / width as f32;
            let mut color = Vec3::zero();

            for _ in 0..config.samples_per_pixel {
                let (dx, dy) = rng.gen::<(f32, f32)>();

                let u = u + dx / width as f32;
                let v = v + dy / height as f32;

                let mut ray = camera.get_ray(&mut rng, u, v);
                ray.direction.normalize();

                color += ray_color(&ray, accel, &mut rng, 0);
            }

            color /= config.samples_per_pixel as f32;
            pix.copy_from_slice(color.as_slice());
        }
    };

    if scene.config.parallel {
        buf.par_chunks_mut(3 * width).enumerate().for_each(f);
    } else {
        buf.chunks_exact_mut(3 * width).enumerate().for_each(f);
    }
}

fn main() -> io::Result<()> {
    let mut config = Config::testing();

    // Cornell Box
    /*
    config.resolution = (128, 128);
    config.samples_per_pixel = 8;
    config.parallel = true;

    // Setup Camera
    let look_from = Vec3::new(-0.2, 2.5, 9.5);
    let look_at = Vec3::new(-0.2, 2.5, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = (look_at - look_from).mag();
    let aperture = 0.0;
    let fov = 30.0;

    let model_fn = "res/cornell-box.obj";
    */

    // Audi
    config.resolution = (256, 256);
    config.samples_per_pixel = 4;
    config.parallel = true;

    // Setup Camera
    let look_from = Vec3::new(5.5, 2.0, 10.25);
    let look_at = Vec3::new(-0.5, 0.5, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = (look_at - look_from).mag();
    let aperture = 0.0;
    let fov = 35.0;

    let model_fn = "res/Audi_R8.obj";

    let camera = Camera::with_config_aspect(
        look_from,
        look_at,
        vup,
        fov,
        &config,
        focus_distance,
        aperture,
    );
    
    let model = Model::load(Vec3::zero(), model_fn);

    let accel = Slow {
        objects: vec![model],
    };

    let scene = Scene::new(config, camera, accel);
    let (width, height) = scene.config.resolution;
    let mut buf = vec![0.0; width * height * 3];

    use std::time::Instant;

    let st = Instant::now();
    trace_scene(&scene, &mut buf);
    let et = Instant::now();

    println!("Time: {:.3?}", et.duration_since(st));

    // apply_filters(&mut buf, scene.config.resolution);

    let filename = "out.png";
    save_image(&filename, &mut buf, scene.config.resolution)
}

// #[bench]
fn bench_cornell_box(b: &mut Bencher) {
    let scene = Scene::<Slow<Model>>::load_bench("res/cornell-box.obj");
    let (width, height) = scene.config.resolution;
    let mut buf = vec![0.0; width * height * 3];

    b.iter(|| {
        trace_scene(&scene, &mut buf);
    });
}
#[bench]
fn bench_slow_intersection_cornell_box(b: &mut Bencher) {
    let scene = Scene::<Slow<Model>>::load_bench("res/cornell-box.obj");

    let origin = Vec3::zero();
    let direction = Vec3::new(0.0, 1.0, -1.0).normalized();
    let ray = Ray { origin, direction };

    b.iter(|| {
        match scene.accel.intersect(&ray, 0.0, INF) {
            Some(hit) => {
                black_box(hit);
            }
            _ => (),
        };
    });
}

#[bench]
fn bench_slow_intersection_audi_r8(b: &mut Bencher) {
    let scene = Scene::<Slow<Model>>::load_bench("res/Audi_R8.obj");

    let origin = Vec3::zero();
    let direction = Vec3::new(0.0, 1.0, -1.0).normalized();
    let ray = Ray { origin, direction };

    b.iter(|| {
        match scene.accel.intersect(&ray, 0.0, INF) {
            Some(hit) => {
                black_box(hit);
            }
            _ => (),
        };
    });
}

#[bench]
fn bench_slow_intersection_512(b: &mut Bencher) {
    let scene = Scene::<Slow<Model>>::load_bench("res/512_poly.obj");

    let origin = Vec3::zero();
    let direction = Vec3::new(0.0, 1.0, -1.0).normalized();
    let ray = Ray { origin, direction };

    b.iter(|| {
        match scene.accel.intersect(&ray, 0.0, INF) {
            Some(hit) => {
                black_box(hit);
            }
            _ => (),
        };
    });
}