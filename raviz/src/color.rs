use std::{ops::{self, Index}};


#[derive(Debug, Clone, Copy, Default)]
pub struct Color{
    r: f32,
    g: f32,
    b: f32,
    a: f32
}


impl Color {
    pub fn new(r: f32, g: f32, b: f32 , a: f32) -> Color{
        Color { r, g, b, a}
    }

    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn g(&self) -> f32 {
        self.g
    }

    pub fn b(&self) -> f32 {
        self.b
    }

}




impl Index<usize> for Color {
    type Output = f32;
    fn index<'a>(&'a self, i: usize) -> &'a f32{
        match i {
            0 => return &self.r,
            1 => return &self.g,
            2 => return &self.b,
            3 => return &self.a,
            _ => return &0.0
        };
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, _rhs: f32) -> Color{
        Color::new(self.r * _rhs, 
            self.g * _rhs, 
            self.b * _rhs,
            self.a * _rhs)
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color{
        Color::new(self * _rhs.r, 
            self * _rhs.g, 
            self * _rhs.b,
            self * _rhs.a)
    }
}


impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color{
        Color::new(self.r * _rhs.r, 
            self.g * _rhs.g, 
            self.b * _rhs.b,
            self.a * _rhs.a)
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, _rhs: Color) -> Color{
        Color::new(self.r + _rhs.r, 
            self.g + _rhs.g, 
            self.b + _rhs.b,
            self.a + _rhs.a)
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, _rhs: Color) -> Color{
        Color::new(self.r - _rhs.r, 
            self.g - _rhs.g, 
            self.b - _rhs.b,
            self.a - _rhs.a)
    }
}
