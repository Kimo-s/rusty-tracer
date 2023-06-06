use raviz::{self, init_check, vec3::Vector3, camera::Camera, scene::WorldScene, color::Color, world_object::Sphere};

fn main() {

    init_check();

    let campos = Vector3::new(0.0, 5.0, 0.0);
    let mut cam = Camera::new(campos, 
    Vector3::new(0.0, 0.0, 1.0),
    -campos,
    60.0);

    println!("Test cross {:?}", Vector3::new(1.0, 0.0, 0.0)^Vector3::new(0.0, 1.0, 0.0));

    let mut world = WorldScene::init();
    world.addObject(Sphere::new(Vector3::new(0.0, 0.0, 0.0),
    1.0,
    Color::new(0.6, 0.0, 0.3, 1.0)
    ));

    raviz::rvRenderScene(&mut cam, &world, 800, 600, "res.png");

}
