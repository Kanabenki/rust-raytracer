extern crate rand;

use std::ops;
use std::f64;
use std::f64::consts::PI;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    e : [f64; 3]
}

impl Vec3 {

    pub fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 {e: [e1, e2, e3]}
    }
    pub fn zero() -> Vec3 {
        Vec3 {e: [0.0, 0.0, 0.0]}
    }
    pub fn one() -> Vec3 {
        Vec3 {e: [1.0, 1.0, 1.0]}
    }

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }
    pub fn r(&self) -> f64 { self.e[0] }
    pub fn g(&self) -> f64 { self.e[1] }
    pub fn b(&self) -> f64 { self.e[2] }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 { e: [
            self.e[1] * v.e[2] - self.e[2] * v.e[1],
            - (self.e[0] * v.e[2] - self.e[2] * v.e[0]),
            self.e[0] * v.e[1] - self.e[1] * v.e[0]
            ]}
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3{e: [-self.e[0], -self.e[1], -self.e[2]]}
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, f: f64) -> Vec3 {
        Vec3{e: [self.e[0] + f, self.e[1] + f, self.e[2] + f]}
    }
}

impl ops::Add<Vec3> for f64 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        v + self
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3{e: [self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2]]}
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        self + (-v)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3{e: [self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2]]}
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, f: f64) -> Vec3 {
        Vec3{e: [self.e[0] * f, self.e[1] * f, self.e[2] * f]}
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, f: f64) {
        *self = *self * f;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, f: f64) -> Vec3 {
        Vec3{e: [self.e[0] / f, self.e[1] / f, self.e[2] / f]}
    }
}

#[derive(Debug, Clone, Copy)]
struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {origin, direction}
    }

    pub fn point(self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

struct HitRecord<'a> {
    t: f64,
    p: Vec3,
    normal: Vec3,
    material: &'a Material
}

trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

struct Sphere<T: Material> {
    center: Vec3,
    radius: f64,
    material: T
}

impl<T: Material> Sphere<T> {
    fn new(center: Vec3, radius: f64, material: T) -> Sphere<T> {
        Sphere {center, radius, material}
    }
}

impl<T: Material> Hitable for Sphere<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);
        
        let disc = b*b - 4.0*a*c;
        if disc > 0.0 {
            let t = (-b - disc.sqrt()) / (2.0 * a);
            if t_min < t && t < t_max {
                let p = ray.point(t);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: &self.material
                });
            }
            let t = (-b + disc.sqrt()) / (2.0 * a);
            if t_min < t && t < t_max {
                let p = ray.point(t);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: &self.material
                });
            }
        }
        None
    }
}

enum List<T> {
    Node(T, Box<List<T>>),
    End
}

struct ListIterator<'a, T> {
    current: &'a List<T>
}

impl<T> List<T> {
    fn new() -> List<T> {
        List::End
    }

    fn add(self, e: T) -> List<T> {
        List::Node(e, Box::new(self))
    }

    fn iter(&self) ->ListIterator<T> {
        ListIterator{current: self}
    }
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        match self.current {
            List::Node(elem, next) => {self.current = next; Some(elem)},
            List::End => None
        }
    }
}

impl<'a> Hitable for List<&Hitable> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut rec_option = None;
        for obj in self.iter() {
            match obj.hit(ray, t_min, closest) {
                Some(record) => {closest = record.t; rec_option = Some(record);},
                None => {}
            }
        }
        rec_option
    }
}

struct ScatterRecord {
    scattered: Ray,
    attenuation: Vec3
}

trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}

struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    fn new(albedo: Vec3, fuzz: f64) -> Metal {
        let fuzz = fuzz.max(0.0).min(1.0);
        Metal {albedo, fuzz}
    }
}


impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(r_in.direction.normalized(), hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo;
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some(ScatterRecord{scattered, attenuation})
        } else {
            None
        }
    }   
}

struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    fn new(albedo: Vec3) -> Self {
        Self {albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit_record.p, target - hit_record.p);
        Some(ScatterRecord{scattered, attenuation: self.albedo})
    }  
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0*v.dot(&n) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let v = v.normalized();
    let dt = v.dot(&n);
    let discriminant = 1.0 - (ni_over_nt).powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        Some(ni_over_nt * (v - n*dt) - n*discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, ref_ind: f64) -> f64 {
    let r0 = ((1.0 - ref_ind) / (1.0 + ref_ind)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

struct Dielectric {
    ref_ind: f64
}

impl Dielectric {
    fn new(ref_ind: f64) -> Dielectric {
        Dielectric {ref_ind}
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let outward_normal;
        let ni_over_nt;
        let cosine;
        let reflected = reflect(r_in.direction, hit_record.normal);
        let attenuation = Vec3::one();
        if r_in.direction.dot(&hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.ref_ind;
            cosine = self.ref_ind * r_in.direction.dot(&hit_record.normal) / r_in.direction.length();
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.ref_ind;
            cosine = - r_in.direction.dot(&hit_record.normal) / r_in.direction.length();
        }
        match refract(r_in.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                let reflect_prob = schlick(cosine, self.ref_ind);
                let scatter_dir;
                if rand::thread_rng().gen::<f64>() < reflect_prob {
                    scatter_dir = reflected;
                } else {
                    scatter_dir = refracted;
                }
                Some(ScatterRecord{attenuation, scattered: Ray::new(hit_record.p, scatter_dir)})
                },
            None => Some(ScatterRecord{attenuation, scattered: Ray::new(hit_record.p, reflected)})
        }
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rand = rand::thread_rng();
    loop {
        let p = 2.0*Vec3::new(rand.gen(), rand.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64
}

impl Camera {
    fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect;
        let look_dir = (look_from - look_at).normalized();
        let u = vup.cross(&look_dir).normalized();
        let v = look_dir.cross(&u);
        Camera {
            lower_left_corner: look_from - focus_dist * (half_width*u + half_height*v + look_dir),
            horizontal: 2.0*half_width*focus_dist*u,
            vertical: 2.0*half_height*focus_dist*v,
            origin: look_from,
            u,
            v,
            lens_radius: aperture / 2.0
        }
    }

    fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rand_v = self.lens_radius * random_in_unit_disk();
        let lens_offset = self.u * rand_v.x() + self.v * rand_v.y();
        Ray::new(self.origin + lens_offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - lens_offset)
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut random = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(random.gen(), random.gen(), random.gen()) - Vec3::one();
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

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
    let camera = Camera::new(look_from, look_at, vup, 20.0, nx as f64 / ny as f64, aperture, focus_dist);

    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(Vec3::new(0.8, 0.3, 0.3)));
    let sphere2 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Dielectric::new(1.5));
    let sphere3 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.1));
    let ground =  Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));

    let list : List<&Hitable> = List::new().add(&sphere1 as &Hitable).add(&sphere2 as &Hitable).add(&sphere3 as &Hitable).add(&ground as &Hitable);

    print!("P3\n{} {}\n255\n", nx, ny);

    let mut random = rand::thread_rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();
            for _ in 0..ns {
                let u = (i as f64 + random.gen::<f64>()) / nx as f64;
                let v = (j as f64 + random.gen::<f64>()) / ny as f64;
                let ray = camera.get_ray(u, v);
                col += color(&ray, &list, 0);
            }
            col = gamma_correct(&(col / ns as f64)) * 255.99;
            println!("{} {} {}", col.r() as i64, col.g() as i64, col.b() as i64);
        }
    }
}
