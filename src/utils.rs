use rand::Rng;

use crate::vec3::Vec3;

pub fn random_in_unit_disk() -> Vec3 {
    let mut rand = rand::thread_rng();
    loop {
        let p = 2.0*Vec3::new(rand.gen(), rand.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut random = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(random.gen(), random.gen(), random.gen()) - Vec3::one();
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}