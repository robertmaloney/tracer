extern crate nalgebra;

use nalgebra::*;

mod primitives;
use primitives::Sphere;

mod scene;
use scene::{Scene, PerspectiveCamera, Camera};

fn main() {
    // let camera = OrthographicCamera::new(Point3::new(0f64, 0f64, 0f64), Vector3::y(), 16f64, 9f64);
    // let mut scene: Scene<Sphere> = Scene::new();
    // scene.add_object(Sphere::new(Point3::new(0f64, 100f64, 0f64), 5f64));
    //
    // camera.render(scene, 1280, 720);
    let camera = PerspectiveCamera::new(
      Point3::new(5f64, 0f64, 0f64),
      Point3::new(0f64, 0f64, 0f64),
      Vector3::y());

    let sphere = Sphere::new(Point4::origin(), 3f64);
    let mut scene = Scene::new();
    scene.add_object(&sphere);

    camera.render(&scene);
}
