extern crate rand;

use std::ops;
use std::f64;
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

struct HitRecord {
    t: f64,
    p: Vec3,
    normal: Vec3
}

trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {center, radius}
    }
}

impl Hitable for Sphere {
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
                    normal: (p - self.center) / self.radius
                });
            }
            let t = (-b + disc.sqrt()) / (2.0 * a);
            if t_min < t && t < t_max {
                let p = ray.point(t);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) / self.radius
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

impl Hitable for List<&Hitable> {
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

struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

impl Camera {
    fn new() -> Camera {
        Camera {..Default::default()}
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
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

fn color(ray: &Ray, world: &Hitable) -> Vec3 {
    match world.hit(ray, 0.001, f64::MAX) {
        Some(hit_record) => {
            let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
            0.5 * color(&Ray::new(hit_record.p, target - hit_record.p), world)},
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
    let nx = 2000;
    let ny = 1000;
    let ns = 100;
    let camera = Camera::new();

    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);

    let list : List<&Hitable> = List::new().add(&sphere1 as &Hitable).add(&ground as &Hitable);

    print!("P3\n{} {}\n255\n", nx, ny);

    let mut random = rand::thread_rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();
            for _ in 0..ns {
                let u = (i as f64 + random.gen::<f64>()) / nx as f64;
                let v = (j as f64 + random.gen::<f64>()) / ny as f64;
                let ray = camera.get_ray(u, v);
                col += color(&ray, &list);
            }
            col = gamma_correct(&(col / ns as f64)) * 255.99;
            println!("{} {} {}", col.r() as i64, col.g() as i64, col.b() as i64);
        }
    }
}
