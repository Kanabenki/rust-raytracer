use std::f64::consts::PI;

use crate::ray::Ray;
use crate::utils::random_in_unit_disk;
use crate::vec3::Vec3;

#[derive(Clone)]
struct CameraVecs {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
}

#[derive(Clone)]
pub struct Camera {
    vfov: f64,
    aperture: f64,
    vup: Vec3,
    look_from: Vec3,
    look_at: Vec3,
    focus_dist: f64,
    aspect: f64,
    vecs: CameraVecs,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        aperture: f64,
        focus_dist: f64,
        aspect: f64,
    ) -> Self {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect;
        let look_dir = (look_from - look_at).normalized();
        let u = vup.cross(&look_dir).normalized();
        let v = look_dir.cross(&u);
        Camera {
            vecs: CameraVecs {
                lower_left_corner: look_from
                    - focus_dist * (half_width * u + half_height * v + look_dir),
                horizontal: 2.0 * half_width * focus_dist * u,
                vertical: 2.0 * half_height * focus_dist * v,
                u,
                v,
            },
            vup,
            aperture,
            aspect,
            vfov,
            focus_dist,
            look_at,
            look_from,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let look_from = Vec3::new(3.0, 3.0, 2.0);
        let look_at = Vec3::new(0.0, 0.0, -1.0);
        Camera::new(
            look_from,
            look_at,
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            0.2,
            (look_from - look_at).length(),
            2.0,
        )
    }
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rand_v = self.aperture * 0.5 * random_in_unit_disk();
        let lens_offset = self.vecs.u * rand_v.x() + self.vecs.v * rand_v.y();
        Ray::new(
            self.look_from + lens_offset,
            self.vecs.lower_left_corner + s * self.vecs.horizontal + t * self.vecs.vertical
                - self.look_from
                - lens_offset,
        )
    }
}
