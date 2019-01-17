use crate::camera::Camera;
use crate::hitable::{Hitable, Sphere};
use crate::list::List;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::vec3::Vec3;
use png::HasParameters;
use rand::Rng;
use std::f64;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

type HitableArc = Arc<Hitable + Send + Sync>;

#[derive(Builder, Clone)]
#[builder(default)]
pub struct Raytracer {
    world: HitableArc,
    #[builder(default = "self.default_camera()")]
    camera: Camera,
    max_depth: u32,
    thread_nb: u32,
    res_x: u32,
    res_y: u32,
    antialiasing_samples: u32,
    #[builder(setter(into))]
    out_file: String,
}

impl RaytracerBuilder {
    fn default_camera(&self) -> Camera {
        match (self.res_x, self.res_y) {
            (Some(x), Some(y)) => {
                let look_from = Vec3::new(3.0, 3.0, 2.0);
                let look_at = Vec3::new(0.0, 0.0, -1.0);
                Camera::new(
                    look_from,
                    look_at,
                    Vec3::new(0.0, 1.0, 0.0),
                    20.0,
                    0.2,
                    (look_from - look_at).length(),
                    x as f64 / y as f64,
                )
            }
            _ => Camera::default(),
        }
    }
}

impl Default for Raytracer {
    fn default() -> Self {
        let res_x = 2000;
        let res_y = 1000;
        Raytracer {
            world: Self::build_sample_scene(),
            camera: Camera::default(),
            max_depth: 50,
            thread_nb: num_cpus::get() as u32,
            res_x,
            res_y,
            antialiasing_samples: 100,
            out_file: String::from("out.png"),
        }
    }
}

impl Raytracer {
    fn build_sample_scene() -> HitableArc {
        let sphere1 = Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Vec3::new(0.8, 0.3, 0.3)),
        );
        let sphere2 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Dielectric::new(1.5));
        let sphere3 = Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.1),
        );
        let ground = Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Vec3::new(0.8, 0.8, 0.0)),
        );

        let list: List<Arc<Hitable + Send + Sync>> = List::new()
            .add(Arc::new(sphere1) as HitableArc)
            .add(Arc::new(sphere2) as HitableArc)
            .add(Arc::new(sphere3) as HitableArc)
            .add(Arc::new(ground) as HitableArc);

        Arc::new(list) as HitableArc
    }

    fn color(&self, ray: &Ray, depth: u32) -> Vec3 {
        match self.world.hit(ray, 0.001, f64::MAX) {
            Some(hit_record) => {
                if depth >= self.max_depth {
                    return Vec3::zero();
                }
                match hit_record.material.scatter(ray, &hit_record) {
                    Some(scatter_record) => {
                        scatter_record.attenuation
                            * self.color(&scatter_record.scattered, depth + 1)
                    }
                    None => Vec3::zero(),
                }
            }
            None => {
                let t = 0.5 * (ray.direction.normalized().y() + 1.0);
                (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
            }
        }
    }

    fn gamma_correct(v: &Vec3) -> Vec3 {
        Vec3::new(v.r().sqrt(), v.g().sqrt(), v.b().sqrt())
    }

    pub fn run(&self) {
        let nx = self.res_x;
        let ny = self.res_y;
        let thread_nb = self.thread_nb;

        println!("Running with {} threads", thread_nb);

        let path = Path::new(&self.out_file);
        let file = File::create(path).expect("Could not create file");
        let w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, nx, ny);
        encoder.set(png::ColorType::RGB);
        let mut writer = encoder.write_header().expect("Could not write png header");

        let data = Arc::new(Mutex::new(vec![0; (nx * ny * 3) as usize]));

        crossbeam::scope(|scope| {
            for thread_ind in 0..thread_nb {
                let data = data.clone();

                let start_y = ny / thread_nb * thread_ind;
                let end_y = ny / thread_nb * (thread_ind + 1);

                scope.spawn(move |_| {
                    let mut random = rand::thread_rng();
                    let mut ind =
                        ((thread_nb - thread_ind - 1) * (nx * ny * 3 / thread_nb)) as usize;
                    for j in (start_y..end_y).rev() {
                        for i in 0..nx {
                            let mut col = Vec3::zero();
                            for _ in 0..self.antialiasing_samples {
                                let u = (i as f64 + random.gen::<f64>()) / nx as f64;
                                let v = (j as f64 + random.gen::<f64>()) / ny as f64;
                                let ray = self.camera.get_ray(u, v);
                                col += self.color(&ray, 0);
                            }
                            col = Self::gamma_correct(&(col / self.antialiasing_samples as f64))
                                * 255.99;
                            let mut data_u = data.lock().expect("Error while locking image buffer");
                            data_u[ind] = col.r() as u8;
                            data_u[ind + 1] = col.g() as u8;
                            data_u[ind + 2] = col.b() as u8;
                            ind += 3;
                        }
                    }
                });
            }
        })
        .expect("Error while joining thread");

        writer
            .write_image_data(&data.lock().unwrap())
            .expect("Error while writing png");
    }
}
