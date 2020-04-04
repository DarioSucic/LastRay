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

struct Sphere<'a> {
    pos: Vec3,
    radius: f32,
    material: &'a Box<dyn Material + Send + Sync>,
}

impl<'a> Intersectable for Sphere<'a> {
    fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit> {
        let oc = ray.origin - self.pos;

        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_root = discriminant.sqrt();

        let t = (-b - discriminant_root) / a;
        if t < tmax && t > tmin {
            let point = ray.at_distance(t);
            let out_normal = (point - self.pos) / self.radius;

            let front_face = ray.direction.dot(out_normal) < 0.0;
            let normal = if front_face { out_normal } else { -out_normal };

            return Some(Hit {
                point,
                normal,
                t,
                front_face,
                material: self.material,
            });
        }

        let t = (-b + discriminant_root) / a;
        if t < tmax && t > tmin {
            let point = ray.at_distance(t);
            let out_normal = (point - self.pos) / self.radius;

            let front_face = ray.direction.dot(out_normal) < 0.0;
            let normal = if front_face { out_normal } else { -out_normal };

            return Some(Hit {
                point,
                normal,
                t,
                front_face,
                material: self.material,
            });
        }

        None
    }
}

fn ray_color<T: Intersectable>(
    ray: &Ray,
    accel: &Accel<T>,
    rng: &mut ThreadRng,
    depth: usize,
) -> Vec3 {
    if depth == 33 {
        return Vec3::zero();
    }

    if let Some(hit) = accel.intersect(ray, 1e-2, INF) {
        if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit, rng) {
            attenuation * ray_color(&scattered, accel, rng, depth + 1)
        } else {
            Vec3::zero()
        }
    } else {
        let unit_dir = ray.direction.normalized();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() -> io::Result<()> {
    const RES: (usize, usize) = (1080, 1080);
    let (width, height) = RES;

    const SAMPLES_PER_PIXEL: usize = 128;

    // Image buffer
    let mut buf = vec![0.0; width * height * 3];

    // Setup Camera, Materials, and Objects
    // TODO: Create Scene abstraction

    // Setup Camera
    let aspect = width as f32 / height as f32;
    let look_from = Vec3::new(0.0, 0.67, 5.0);
    let look_at = Vec3::new(0.0, 0.25, 0.00);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = (look_at - look_from).mag();
    let aperture = 0.15;
    let fov = 35.0;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        fov,
        aspect,
        focus_distance,
        aperture,
    );

    // Setup Materials
    let materials: Vec<Box<dyn Material + Send + Sync>> = vec![
        Box::new(Lambertian {
            albedo: Vec3::broadcast(0.2),
        }),
        Box::new(Metal {
            albedo: Vec3::broadcast(0.9),
            fuzziness: 0.1,
        }),
        Box::new(Lambertian {
            albedo: Vec3::new(1.0, 0.1, 0.1),
        }),
        Box::new(Dielectric {
            albedo: Vec3::new(0.95, 0.95, 1.0),
            fuzziness: 0.0,
            refractive_index: 1.52,
        }),
    ];

    // Setup Objects
    let accel = Accel {
        objects: vec![
            Sphere {
                pos: Vec3::new(0.0, -500.5, 0.0),
                radius: 500.0,
                material: &materials[0],
            },
            // Shiny metal right
            Sphere {
                pos: Vec3::new(0.75, 0.0, 0.0),
                radius: 0.5,
                material: &materials[3],
            },
            Sphere {
                pos: Vec3::new(0.75, 0.0, -5.0),
                radius: 0.5,
                material: &materials[3],
            },
            Sphere {
                pos: Vec3::new(0.75, 0.0, -10.0),
                radius: 0.5,
                material: &materials[3],
            },
            // Matte left
            Sphere {
                pos: Vec3::new(-0.75, 0.0, 0.0),
                radius: 0.5,
                material: &materials[2],
            },
            Sphere {
                pos: Vec3::new(-0.75, 0.0, -5.0),
                radius: 0.5,
                material: &materials[2],
            },
            Sphere {
                pos: Vec3::new(-0.75, 0.0, -10.0),
                radius: 0.5,
                material: &materials[2],
            },
            // Glass center
            Sphere {
                pos: Vec3::new(0.0, 2.0 - 0.67, -20.0),
                radius: 2.0,
                material: &materials[1],
            },
        ],
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

                    let ray = camera.get_ray(&mut rng, u, v);
                    color += ray_color(&ray, &accel, &mut rng, 0);
                }

                color /= SAMPLES_PER_PIXEL as f32;
                pix.copy_from_slice(color.as_slice());
            }
        });

    let filename = "out.png";
    save_image(&filename, &buf, RES)
}
