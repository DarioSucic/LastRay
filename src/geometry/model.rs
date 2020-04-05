use crate::*;
use std::f32;
use std::path::Path;

pub struct Model {
    pub pos: Vec3,
    pub subs: Vec<SubModel>,
    pub materials: Vec<Box<dyn Material + Send + Sync>>,
}

impl Model {
    pub fn from_tobj(pos: Vec3, models: Vec<tobj::Model>, materials: Vec<tobj::Material>) -> Model {
        let materials = materials
            .into_iter()
            .map(Material::from_tobj)
            .collect::<Vec<Box<dyn Material + Send + Sync>>>();

        let subs = models.into_iter().map(SubModel::from_tobj).collect();

        Model {
            pos,
            materials,
            subs,
        }
    }

    pub fn load<P: AsRef<Path>>(pos: Vec3, filename: P) -> Model {
        match tobj::load_obj(filename.as_ref()) {
            Ok((models, materials)) => Model::from_tobj(pos, models, materials),
            Err(e) => panic!("{:?}", e),
        }
    }
}

impl Intersectable for Model {
    fn intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit> {
        self.subs
            .iter()
            .filter_map(|o| o.intersect(&self, ray, tmin, tmax))
            .min_by(|(_, t1), (_, t2)| t1.partial_cmp(&t2).unwrap())
            .map(|(hit, _)| hit)
    }
}

pub struct SubModel {
    pub positions: Vec<Wec3>,
    pub normals: Vec<Wec3>,
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
            texcoords,
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
            .map(|&i| {
                let i = 3 * i as usize;
                &normals[i..i + 3]
            })
            .flatten()
            .copied()
            .collect::<Vec<f32>>();

        SubModel {
            positions: wec3s_from_flat(&flat_positions, f32::NAN),
            normals: wec3s_from_flat(&flat_normals, f32::NAN),
            name,
            material_id,
        }
    }
}

impl SubModel {
    fn intersect<'a>(
        &'a self,
        parent: &'a Model,
        ray: &Ray,
        tmin: f32,
        tmax: f32,
    ) -> Option<(Hit, f32)> {
        let origin = Wec3::splat(ray.origin);
        let direction = Wec3::splat(ray.direction);
        let tmin = f32x4::from(tmin);
        let tmax = f32x4::from(tmax);

        let initial = (
            Wec3::zero(),
            f32x4::new(0.0, 1.0, 2.0, 3.0),
            f32x4::INFINITY,
            f32x4::new(0.0, 1.0, 2.0, 3.0),
        );
        const_f32_as_f32x4!(IDX_DELTA, 4.0);

        let (points, indexes, t, _) = self.positions.chunks_exact(3).fold(
            initial,
            |(curr_points, curr_indexes, curr_t, i), triangle| {
                let (points, t) = triangle_intersection_ssse3(origin, direction, triangle);
                let mask = curr_t.cmp_lt(t);

                let t = mask.merge(curr_t, t);
                let points = Wec3::merge(mask, curr_points, points);
                let indexes = mask.merge(curr_indexes, i);

                (points, indexes, t, i + IDX_DELTA)
            },
        );

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
        let idx = idx - idx % 4;

        let normals: [Vec3; 4] = self.normals[idx].into();
        let mut normal = normals[lane];

        let front_face = ray.direction.dot(normal) < 0.0;
        if !front_face {
            normal = -normal;
        }

        Some((
            Hit {
                front_face,
                material: &parent.materials[self.material_id],
                normal,
                point,
                t: smallest_t,
            },
            smallest_t,
        ))
    }
}

fn triangle_intersection_ssse3(origin: Wec3, direction: Wec3, triangle: &[Wec3]) -> (Wec3, f32x4) {
    const_f32_as_f32x4!(EPS, 1e-7);

    let v0: Wec3 = unsafe { *triangle.get_unchecked(0) };
    let v1: Wec3 = unsafe { *triangle.get_unchecked(1) };
    let v2: Wec3 = unsafe { *triangle.get_unchecked(2) };

    let e1: Wec3 = v1 - v0;
    let e2: Wec3 = v2 - v0;

    let h: Wec3 = direction.cross(e2);
    let a: f32x4 = e1.dot(h);

    let mut mask = a.abs().cmp_gt(EPS);
    // PERF: Early exit if mask == 0?

    let f: f32x4 = f32x4::ONE / a;
    let s: Wec3 = origin - v0;
    let u: f32x4 = f * s.dot(h);

    mask &= u.cmp_ge(f32x4::ZERO) & u.cmp_le(f32x4::ONE);

    let q: Wec3 = s.cross(e1);
    let v: f32x4 = f * direction.dot(q);

    mask &= v.cmp_ge(f32x4::ZERO) & (u + v).cmp_le(f32x4::ONE);

    let t: f32x4 = f * e2.dot(q);

    mask &= t.cmp_gt(EPS);

    let t = mask.merge(t, f32x4::INFINITY);
    let points = origin + direction * t;

    (points, t)
}
