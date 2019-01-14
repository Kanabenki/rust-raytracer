use std::f64::consts::PI;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::utils::random_in_unit_disk;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
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

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rand_v = self.lens_radius * random_in_unit_disk();
        let lens_offset = self.u * rand_v.x() + self.v * rand_v.y();
        Ray::new(self.origin + lens_offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - lens_offset)
    }
}
