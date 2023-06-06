use crate::{vec3::{Vector3}, ray::Ray, color::Color};

#[derive(Debug, Clone, Copy, Default)]
pub struct HitRecord {
    normal: Vector3,
    position: Vector3,
    t: f32
}




pub trait HittableObject {
    fn hit(&self, ray: Ray, tmin: f32, tmax: f32, record: &mut HitRecord) -> bool;
    fn getColor(&self) -> Color;

}




// primitive shapes

pub struct Sphere {
    center: Vector3,
    radius: f32,
    color: Color
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32, color: Color) -> Sphere{
        Sphere{
            center,
            radius,
            color
        }
    }
}

impl HittableObject for Sphere{
    fn getColor(&self) -> Color {
        self.color
    }

    fn hit(&self, ray: Ray, tmin: f32, tmax: f32, record: &mut HitRecord) -> bool {
        let n = ray.origin() - self.center;
        let v = ray.dir();
        let a = v*v;
        let c = n*n - self.radius.powi(2);
        let b = 2.0*v*n;
        
        let discriminant =  b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return false;
        }

        let firsthit = (-b-discriminant.sqrt())/(2.0*a);
        let secondhit = (-b+discriminant.sqrt())/(2.0*a);
        if firsthit < tmin || tmax < firsthit {
            if secondhit < tmin || tmax < secondhit {
                return false;
            }
        }

        let hitpos = ray.at(firsthit);
        record.normal = hitpos - self.center;
        record.t = firsthit;
        record.position = hitpos;

        return true;
    }
}