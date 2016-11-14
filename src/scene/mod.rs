mod camera;
pub use self::camera::{OrthographicCamera, PerspectiveCamera, Camera};
use primitives::{Collidable, Ray};

use primitives::Sphere;

pub struct Scene<'a> {
  objects: Vec<&'a Collidable>,
}

impl<'a> Scene<'a> {
  pub fn new() -> Scene<'a> {
    Scene {
      objects: vec![]
    }
  }

  pub fn add_object(&mut self, obj: &'a Collidable) {
    self.objects.push(obj);
  }
}

impl<'a> Collidable for Scene<'a> {
  fn intersected_by(&self, ray: &Ray) -> bool {
    self.objects.iter().any(|&x| x.intersected_by(&ray))
  }
}
