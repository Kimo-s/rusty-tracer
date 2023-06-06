use crate::vec3::{Vector3};



#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    p: Vector3,
    v: Vector3,
    t: f32
}

impl Ray {
    pub fn new(p: Vector3, v: Vector3) -> Ray {
        Ray {
            p,
            v,
            t: 1.0
        }
    }

    pub fn origin(&self) -> Vector3{
        self.p
    }

    pub fn dir(&self) -> Vector3{
        self.v
    }

    pub fn at(&self, t: f32) -> Vector3 {
        self.p + self.v * t
    }

    pub fn step(&mut self, dt: f32) -> Vector3 {
        self.t += dt;
        self.cur()
    }

    pub fn cur(&self) -> Vector3 {
        self.p + self.v * self.t
    }
}