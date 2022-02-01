use tobj::LoadOptions;

use crate::*;
use std::f32;
use std::path::Path;
use std::simd::{f32x8, f32x4};

pub struct Model {
    pub subs: Vec<SubModel>,
    pub materials: Vec<MaterialType>,
}

impl Model {
    pub fn from_tobj(pos: Vec3, models: Vec<tobj::Model>, materials: Vec<tobj::Material>) -> Model {
        let materials = materials.into_iter().map(<dyn Material>::from_tobj).collect();
        let mut subs: Vec<SubModel> = models.into_iter().map(SubModel::from_tobj).collect();
        subs.iter_mut().for_each(|sub| sub.translate(pos));
        Model { materials, subs }
    }

    pub fn load<P: AsRef<Path>>(pos: Vec3, filename: P) -> Model {
        match tobj::load_obj(filename.as_ref(), &LoadOptions { single_index: true, ..Default::default() }) {
            Ok((models, materials)) => match materials {
                Ok(materials) => Model::from_tobj(pos, models, materials),
                Err(e) => panic!("{e:?}")
            },
            Err(e) => panic!("{e:?}"),
        }
    }
}

impl Intersectable for Model {
    fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit> {
        self.subs
            .iter()
            .filter_map(|o| o.intersect(&self, ray, tmin, tmax))
            .min_by(|hit1, hit2| hit1.t.partial_cmp(&hit2.t).unwrap())
    }
}

impl Bounded for Model {
    fn bounds(&self) -> AABB {
        self.subs
            .iter()
            .fold(AABB::degenerate(), |acc, x| acc.union(x.bounds()))
    }
}

pub struct SubModel {
    pub positions: Vec<Wec3x8>,
    pub normals: Vec<Wec3x8>,
    pub material_id: usize,
    pub name: String,
}

impl SubModel {
    pub fn from_tobj(m: tobj::Model) -> SubModel {
        let tobj::Model { mesh, name } = m;
        let tobj::Mesh {
            indices,
            material_id,
            normals,
            positions,
            ..
        } = mesh;

        let material_id = material_id.unwrap();

        let flat_positions = indices
            .iter()
            .map(|&i| {
                let i = 3 * i as usize;
                &positions[i..i + 3]
            })
            .flatten()
            .copied()
            .collect::<Vec<f32>>();

        let flat_normals = indices
            .iter()
            .step_by(3)
            .map(|&i| {
                let i = 3 * i as usize;
                &normals[i..i + 3]
            })
            .flatten()
            .copied()
            .collect::<Vec<f32>>();

        SubModel {
            positions: wec3s_from_flat(&flat_positions, 3, f32::NAN),
            normals: wec3s_from_flat(&flat_normals, 1, f32::NAN),
            name,
            material_id,
        }
    }

    pub fn translate(&mut self, pos: Vec3) {
        let wpos = Wec3x8::splat(pos);
        self.positions.iter_mut().for_each(|block| *block += wpos);
    }
}

impl SubModel {
    fn bounds(&self) -> AABB {
        assert!(self.positions.len() > 0);
        AABB {
            mini: self.positions.iter().fold(Vec3::broadcast(INF), |acc, x| {
                acc.min_by_component(x.componentwise_min())
            }),
            maxi: self.positions.iter().fold(Vec3::broadcast(-INF), |acc, x| {
                acc.max_by_component(x.componentwise_max())
            }),
        }
    }

    fn intersect<'a>(&'a self, parent: &'a Model, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit> {
        let origin = Wec3x8::splat(ray.origin);
        let direction = Wec3x8::splat(ray.direction);
        let tmin = f32x8::splat(tmin);
        let tmax = f32x8::splat(tmax);

        let mut points = Wec3x8::zero();
        let mut indexes = f32x8::splat(0.0);
        let mut t = f32x8::splat(INF);
        let mut i = f32x8::splat(-1.0);

        for triangle in self.positions.chunks_exact(3) {
            i += f32x8::splat(1.0);
            if let Some((_points, _t)) = triangle_intersection_avx(origin, direction, triangle) {
                let mask = t.lanes_lt(_t);
                t = merge_packed(mask, t, _t);
                points = Wec3x8::merge(mask, points, _points);
                indexes = merge_packed(mask, indexes, i);
            }
        }

        let mask = t.lanes_lt(tmax) & t.lanes_gt(tmin);
        let t = merge_packed(mask, t, f32x8::splat(INF));

        let (lane, smallest_t) = (0..8)
            .into_iter()
            .map(|i| t.as_array()[i])
            .enumerate()
            .min_by(|(_, t1), (_, t2)| t1.partial_cmp(&t2).unwrap())
            .unwrap();

        if smallest_t == INF {
            return None;
        }

        let point = Vec3::new(
            points.x.as_array()[lane],
            points.y.as_array()[lane],
            points.z.as_array()[lane],
        );

        let idx = indexes.as_array()[lane] as usize;
        let mut normal = Vec3::new(
            self.normals[idx].x.as_array()[lane],
            self.normals[idx].y.as_array()[lane],
            self.normals[idx].z.as_array()[lane],
        );

        let front_face = ray.direction.dot(normal) < 0.0;
        if !front_face {
            normal = -normal;
        }

        Some(Hit {
            front_face,
            material: &parent.materials[self.material_id],
            normal,
            point,
            t: smallest_t,
        })
    }
}

/*
impl SubModel {
    fn intersect<'a>(&'a self, parent: &'a Model, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit> {
        let origin = Wec3::splat(ray.origin);
        let direction = Wec3::splat(ray.direction);
        let tmin = f32x4::from(tmin);
        let tmax = f32x4::from(tmax);

        let mut points = Wec3::zero();
        let mut indexes = f32x4::ZERO;
        let mut t = f32x4::INFINITY;
        let mut i = f32x4::from(-1.0);

        for triangle in self.positions.chunks_exact(3) {
            i += f32x4::ONE;
            if let Some((_points, _t)) = triangle_intersection_ssse3(origin, direction, triangle) {
                let mask = t.cmp_lt(_t);
                t = mask.merge(t, _t);
                points = Wec3::merge(mask, points, _points);
                indexes = mask.merge(indexes, i);
            }
        }

        let mask = t.cmp_lt(tmax) & t.cmp_gt(tmin);
        let t = mask.merge(t, f32x4::INFINITY);

        let (lane, &smallest_t) = t
            .as_ref()
            .iter()
            .enumerate()
            .min_by(|(_, t1), (_, t2)| t1.partial_cmp(&t2).unwrap())
            .unwrap();

        if smallest_t == INF {
            return None;
        }

        let points: [Vec3; 4] = points.into();
        let point = points[lane];

        let idx = indexes[lane] as usize;
        let normals: [Vec3; 4] = self.normals[idx].into();
        let mut normal = normals[lane];

        let front_face = ray.direction.dot(normal) < 0.0;
        if !front_face {
            normal = -normal;
        }

        Some(Hit {
            front_face,
            material: &parent.materials[self.material_id],
            normal,
            point,
            t: smallest_t,
        })
    }
}
*/

fn triangle_intersection_avx(
    origin: Wec3x8,
    direction: Wec3x8,
    triangle: &[Wec3x8],
) -> Option<(Wec3x8, f32x8)> {
    let EPS = f32x8::splat(1e-7);

    let v0: Wec3x8 = unsafe { *triangle.get_unchecked(0) };
    let v1: Wec3x8 = unsafe { *triangle.get_unchecked(1) };
    let v2: Wec3x8 = unsafe { *triangle.get_unchecked(2) };

    let e1: Wec3x8 = v1 - v0;
    let e2: Wec3x8 = v2 - v0;

    let h: Wec3x8 = direction.cross(e2);
    let a: f32x8 = e1.dot(h);

    let mut mask = a.abs().lanes_gt(EPS);

    let f: f32x8 = f32x8::splat(1.0) / a;
    let s: Wec3x8 = origin - v0;
    let u: f32x8 = f * s.dot(h);

    mask &= u.lanes_ge(f32x8::splat(0.0)) & u.lanes_le(f32x8::splat(1.0));

    /*
    if likely(mask.move_mask() == 0) {
        return None;
    }
    */

    let q: Wec3x8 = s.cross(e1);
    let v: f32x8 = f * direction.dot(q);

    mask &= v.lanes_ge(f32x8::splat(0.0)) & (u + v).lanes_le(f32x8::splat(1.0));

    let t: f32x8 = f * e2.dot(q);

    mask &= t.lanes_gt(EPS);

    let t = merge_packed(mask, t, f32x8::splat(f32::INFINITY));
    let points = origin + direction * t;

    Some((points, t))
}
