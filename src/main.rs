use std::ops;

struct Vec3 {
    e : [f64; 3]
}

impl Vec3 {

    pub fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 {e: [e1, e2, e3]}
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

    pub fn normalized(&self) -> Vec3 {
        self / self.length()
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3{e: [-self.e[0], -self.e[1], -self.e[2]]}
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3{e: [self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2]]}
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

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, f: f64) -> Vec3 {
        Vec3{e: [self.e[0] / f, self.e[1] / f, self.e[2] / f]}
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    print!("P3\n{} {}\n255\n", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3::new(i as f64 / nx as f64, j as f64 / ny as f64, 0.2) * 255.99;
            println!("{} {} {}", col.r() as u64, col.g() as u64, col.b() as u64);
        }
    }
}
