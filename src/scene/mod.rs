use crate::*;

pub struct Config {
    pub resolution: (usize, usize),
    pub samples_per_pixel: usize,
    pub parallel: bool,
}

impl Config {
    #[allow(dead_code)]
    pub fn production() -> Config {
        Config {
            resolution: (1080, 1080),
            samples_per_pixel: 512,
            parallel: true,
        }
    }

    #[allow(dead_code)]
    pub fn testing() -> Config {
        Config {
            resolution: (512, 512),
            samples_per_pixel: 16,
            parallel: true,
        }
    }

    #[allow(dead_code)]
    pub fn bench() -> Config {
        Config {
            resolution: (64, 64),
            samples_per_pixel: 4,
            parallel: false,
        }
    }
}

pub struct Scene<I: Intersectable> {
    pub config: Config,
    pub camera: Camera,
    pub accel: I,
}

impl<I: Intersectable> Scene<I> {
    pub fn new(config: Config, camera: Camera, accel: I) -> Scene<I> {
        Scene {
            config,
            camera,
            accel,
        }
    }

    fn _bench(accel: I) -> Scene<I> {
        let config = Config::bench();
        let (width, height) = config.resolution;

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

        Scene {
            config,
            camera,
            accel,
        }
    }

    #[allow(dead_code)]
    pub fn load_bench(filename: &str) -> Scene<Slow<Model>> {
        let accel = Slow {
            objects: vec![Model::load(Vec3::zero(), filename)],
        };
        Scene::_bench(accel)
    }
}
