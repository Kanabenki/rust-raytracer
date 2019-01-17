use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 { e: [e1, e2, e3] }
    }
    pub fn zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
    pub fn one() -> Vec3 {
        Vec3 { e: [1.0, 1.0, 1.0] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn r(&self) -> f64 {
        self.e[0]
    }
    pub fn g(&self) -> f64 {
        self.e[1]
    }
    pub fn b(&self) -> f64 {
        self.e[2]
    }

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
        Vec3 {
            e: [
                self.e[1] * v.e[2] - self.e[2] * v.e[1],
                -(self.e[0] * v.e[2] - self.e[2] * v.e[0]),
                self.e[0] * v.e[1] - self.e[1] * v.e[0],
            ],
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, f: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] + f, self.e[1] + f, self.e[2] + f],
        }
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
        Vec3 {
            e: [self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2]],
        }
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
        Vec3 {
            e: [self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2]],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, f: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * f, self.e[1] * f, self.e[2] * f],
        }
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
        Vec3 {
            e: [self.e[0] / f, self.e[1] / f, self.e[2] / f],
        }
    }
}
