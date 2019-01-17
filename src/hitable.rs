use std::sync::Arc;

use crate::list::List;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere<T: Material> {
    center: Vec3,
    radius: f64,
    material: T,
}

impl<T: Material> Sphere<T> {
    pub fn new(center: Vec3, radius: f64, material: T) -> Sphere<T> {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<T: Material> Hitable for Sphere<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);

        let disc = b * b - 4.0 * a * c;
        if disc > 0.0 {
            let t = (-b - disc.sqrt()) / (2.0 * a);
            if t_min < t && t < t_max {
                let p = ray.point(t);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: &self.material,
                });
            }
            let t = (-b + disc.sqrt()) / (2.0 * a);
            if t_min < t && t < t_max {
                let p = ray.point(t);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: &self.material,
                });
            }
        }
        None
    }
}

impl<'a> Hitable for Arc<List<Arc<Hitable + Send + Sync>>> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut rec_option = None;
        for obj in self.iter() {
            if let Some(record) = obj.hit(ray, t_min, closest) {
                closest = record.t;
                rec_option = Some(record);
            }
        }
        rec_option
    }
}

impl<'a> Hitable for List<Arc<Hitable + Send + Sync>> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut rec_option = None;
        for obj in self.iter() {
            if let Some(record) = obj.hit(ray, t_min, closest) {
                closest = record.t;
                rec_option = Some(record);
            }
        }
        rec_option
    }
}
