#![allow(non_snake_case)]
#[warn(unused_parens)]

use camera::Camera;
use color::Color;
use ray::Ray;
use scene::WorldScene;
use world_object::*;
use image::*;

pub mod scene;
pub mod vec3;
pub mod camera;
pub mod ray;
pub mod world_object;
pub mod color;



pub fn init_check() {
    println!("Print from raviz library.");
}

fn clamp(val: f32, min: f32, max: f32) -> f32{
    if val < min {
        min
    } else if val > max {
        max
   } else {
        val
   }
}

fn castRay(ray: Ray, worldScene: &WorldScene) -> Color{
    let record = &mut HitRecord::default();



    for index in 0..worldScene.objectCount() {
        let obj = worldScene.getVec().get(index).unwrap();
        if obj.hit(ray, 0.0, 20000.0, record) {
            return obj.getColor();
        }
    }

    Color::new(0.0,0.0,0.0,0.0)
}

pub fn rvRenderScene(cam: &mut Camera, worldScene: &WorldScene, width: u32, height: u32, filename: &str) {

    cam.set_aspect_ratio(width as f32 / height as f32);
    //cam.set_aspect_ratio(2.0);

    let mut img = ImageBuffer::new(width, height);


    for (x, y, pixel) in img.enumerate_pixels_mut() {

        let u = (x as f32)/((width-1) as f32);
        let v = (y as f32)/((height-1) as f32);

        let ray = cam.view_ray(u, v);
        let colorRes = castRay(ray, worldScene);

        let r = (clamp(colorRes[0], 0.0, 0.999) * 255.0) as u8;
        let g = (clamp(colorRes[1], 0.0, 0.999) * 255.0) as u8;
        let b = (clamp(colorRes[2], 0.0, 0.999) * 255.0) as u8;
        let a = (clamp(colorRes[3], 0.0, 0.999) * 255.0) as u8;
        *pixel = image::Rgba([r, g, b, a]);
    }

    img.save(filename).unwrap();

}













#[cfg(test)]
mod tests {
    use crate::vec3::Vector3;

    use super::*;

    #[test]
    fn vector3_add_sub_test(){
        let vec1 = Vector3::new(1.0,2.0,0.0);
        let vec2 = Vector3::new(6.0,4.0,0.0);
        let vec3 = vec1 + vec2;
        assert_eq!(vec3[0], 7.0);
        assert_eq!(vec3[1], 6.0);
        assert_eq!(vec3[2], 0.0);

        let vec3 = vec1 - vec2;
        assert_eq!(vec3[0], vec1[0] - vec2[0]);
        assert_eq!(vec3[1], vec1[1] - vec2[1]);
        assert_eq!(vec3[2], vec1[2] - vec2[2]);
    }

    #[test]
    fn vector3_scalar_mult_div_test(){
        let mut vec1 = Vector3::new(1.0,2.0,0.0);
        vec1 = vec1 * 2.0;
        assert_eq!(vec1[0], 2.0);
        assert_eq!(vec1[1], 4.0);
        assert_eq!(vec1[2], 0.0);

        vec1 = Vector3::new(1.0,2.0,0.0);
        vec1 = vec1 / 2.0;
        assert_eq!(vec1[0], 1.0/2.0);
        assert_eq!(vec1[1], 1.0);
        assert_eq!(vec1[2], 0.0);
    }

    #[test]
    fn vector3_normalize_test(){
        let vec1 = Vector3::new(1.0,2.0,0.0);
        let norm = vec1.norm();
        assert_eq!(norm, 5.0_f32.sqrt());
    }

}
