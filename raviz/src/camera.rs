use crate::{vec3::{Vector3}, ray::Ray};
use std::f64::consts::PI;


#[derive(Debug, Clone, Copy, Default)]
pub struct Camera {
    camera_pos: Vector3,
    axis_up: Vector3,
    axis_right: Vector3,
    axis_view: Vector3,
    fov: f32,
    aspect_ratio: f32,
    htanfov: f32,
    vtanfov: f32
}


impl Camera {
    pub fn new(camera_position: Vector3, up: Vector3, view: Vector3, fov: f32) -> Camera {
        let mut cam = Camera::default();
        cam.set_eye_view_up(camera_position, up, view);
        cam.set_fov(fov);
        cam.set_aspect_ratio(16.0/9.0);
        cam
    }

    pub fn set_eye_view_up(&mut self, camera_position: Vector3, up: Vector3, view: Vector3){
        self.camera_pos = camera_position;
        self.axis_view = view.unit();
        self.axis_up = ( up - (up*self.axis_view) * self.axis_view ).unit();
        self.axis_right = (self.axis_view^self.axis_up).unit();
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.htanfov = (fov*0.5*(PI as f32)/180.0).tan();
        self.vtanfov = self.htanfov/self.aspect_ratio;
    }

    pub fn set_aspect_ratio(&mut self, ratio: f32) {
        self.aspect_ratio = ratio;
        self.vtanfov = self.htanfov/self.aspect_ratio;
    }

    pub fn view(&self, x: f32, y: f32) -> Vector3 {
        let xx = (2.0*x-1.0)*self.htanfov;
        let yy = (2.0*y-1.0)*self.vtanfov;
        (self.axis_up * yy + self.axis_right * xx + self.axis_view).unit()
    }

    pub fn view_ray(&self, x: f32, y: f32) -> Ray {
        Ray::new(self.camera_pos, self.view(x, y))
    }


}