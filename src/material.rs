use rand::Rng;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::utils::random_in_unit_sphere;


pub struct ScatterRecord {
    pub scattered: Ray,
    pub attenuation: Vec3
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
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

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
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

pub struct Dielectric {
    ref_ind: f64
}

impl Dielectric {
    pub fn new(ref_ind: f64) -> Dielectric {
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
