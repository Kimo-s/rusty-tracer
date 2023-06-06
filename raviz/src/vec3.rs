use std::{ops::{self, Index}};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3{
    x: f32,
    y: f32,
    z: f32
}


impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32 ) -> Vector3{
        Vector3 { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.x
    }

    pub fn z(&self) -> f32 {
        self.x
    }

    pub fn norm(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn unit(self) -> Vector3 {
        let norm = self.norm();
        self / norm
    }
}




impl Index<usize> for Vector3 {
    type Output = f32;
    fn index<'a>(&'a self, i: usize) -> &'a f32{
        match i {
            0 => return &self.x,
            1 => return &self.y,
            2 => return &self.z,
            _ => return &0.0
        };
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: f32) -> Vector3{
        Vector3::new(self.x / _rhs, 
             self.y / _rhs, 
            self.z / _rhs)
    }
}


impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: f32) -> Vector3{
        Vector3::new(_rhs * self.x, 
            _rhs * self.y, 
            _rhs *self.z)
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, _rhs: Vector3) -> Vector3{
        Vector3::new(_rhs.x * self, 
            _rhs.y * self, 
            _rhs.z *self)
    }
}



// Dot Product
impl ops::Mul<Vector3> for Vector3 {
    type Output = f32;

    fn mul(self, _rhs: Vector3) -> f32{
        _rhs.x * self.x +  _rhs.y * self.y + _rhs.z * self.z
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3{
        Vector3::new(_rhs.x + self.x, 
            _rhs.y + self.y, 
            _rhs.z + self.z)
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: Vector3) -> Vector3{
        Vector3::new(self.x - _rhs.x, 
            self.y - _rhs.y, 
            self.z - _rhs.z)
    }
}

impl ops::BitXor<Vector3> for Vector3 {
    type Output = Vector3;

    fn bitxor(self, r: Vector3) -> Vector3{
        Vector3::new(
            self.y*r.z - self.z*r.y, 
            self.z*r.x - self.x*r.z, 
            self.x*r.y - self.y*r.x)
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3{
        self*(-1.0)
    }
}