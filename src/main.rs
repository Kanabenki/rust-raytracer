mod vec3;
mod ray;
mod camera;
mod utils;
mod list;
mod hitable;
mod material;

use std::f64;
use rand::Rng;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png::HasParameters;
use crate::list::List;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::camera::Camera;
use crate::hitable::{Hitable, Sphere};
use crate::material::{Lambertian, Metal, Dielectric};


fn color(ray: &Ray, world: &Hitable, depth: i32) -> Vec3 {
    match world.hit(ray, 0.001, f64::MAX) {
        Some(hit_record) => {
            if depth >= 50 {
                return Vec3::zero();
            }
            match hit_record.material.scatter(ray, &hit_record) {
                Some (scatter_record) => scatter_record.attenuation * color(&scatter_record.scattered, world, depth + 1),
                None => Vec3::zero()
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

fn main() {
    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let nx = 2000;
    let ny = 1000;
    let ns = 100;
    let aperture = 0.2;
    let focus_dist = (look_from - look_at).length();
    let camera_arc = Arc::new(Camera::new(look_from, look_at, vup, 20.0, nx as f64 / ny as f64, aperture, focus_dist));

    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(Vec3::new(0.8, 0.3, 0.3)));
    let sphere2 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Dielectric::new(1.5));
    let sphere3 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.1));
    let ground =  Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));


    type HitableArc = Arc<Hitable+Send+Sync>;
    let list_arc: List<Arc<Hitable+Send+Sync>> = List::new()
        .add(Arc::new(sphere1) as HitableArc)
        .add(Arc::new(sphere2) as HitableArc)
        .add(Arc::new(sphere3) as HitableArc)
        .add(Arc::new(ground) as HitableArc);

    let list_arc = Arc::new(list_arc);

    let thread_nb = 4;
    let mut threads = vec![];

    let path = Path::new("./test.png");
    let file = File::create(path).expect("Could not create file");
    let w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, nx, ny);
    encoder.set(png::ColorType::RGB);
    let mut writer = encoder.write_header().expect("Could not write png header");

    let data_arc = Arc::new(Mutex::new(vec![0; (nx * ny * 3) as usize]));
    for thread_ind in 0..thread_nb {
        let data = data_arc.clone();
        let camera = camera_arc.clone();
        let list = list_arc.clone();

        let start_y = ny / thread_nb * thread_ind;
        let end_y = ny / thread_nb * (thread_ind + 1);

        let t = thread::spawn(move || {
            let mut random = rand::thread_rng();
            let mut ind = ((thread_nb - thread_ind - 1) * (nx * ny * 3 / thread_nb)) as usize;
            for j in (start_y..end_y).rev() {
                for i in 0..nx {
                    let mut col = Vec3::zero();
                    for _ in 0..ns {
                        let u = (i as f64 + random.gen::<f64>()) / nx as f64;
                        let v = (j as f64 + random.gen::<f64>()) / ny as f64;
                        let ray = camera.get_ray(u, v);
                        col += color(&ray, &list, 0);
                    }
                    col = gamma_correct(&(col / ns as f64)) * 255.99;
                    let mut data_u = data.lock().unwrap();
                    data_u[ind] = col.r() as u8;
                    data_u[ind + 1] = col.g() as u8;
                    data_u[ind + 2] = col.b() as u8;
                    ind += 3;
                }
        }
        });
        threads.push(t);
    }
    for thread in threads {
        thread.join().expect("Error while joining thread");
    }
    writer.write_image_data(&data_arc.lock().unwrap()).expect("Error while writing png");
}
