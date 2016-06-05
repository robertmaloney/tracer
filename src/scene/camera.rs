extern crate nalgebra;

use super::Scene;
use primitives::collidable::Collidable;
use self::nalgebra::{Vector3, Norm};

pub struct OrthographicCamera {
  position: Vector3<f64>,
  direction: Vector3<f64>,
  width: f64,
  height: f64,
}

impl OrthographicCamera {
  pub fn new(position: Vector3<f64>, direction: Vector3<f64>, width: f64, height: f64) -> OrthographicCamera {
    OrthographicCamera {
      position: position,
      direction: direction.normalize(),
      width: width,
      height: height,
    }
  }

  pub fn render<T: Collidable>(&self, scene: Scene<T>, px: i32, py: i32) -> Vec<u8> {
    let left = self.position[0] - self.width;
    let right = self.position[0] + self.width;
    let top = self.position[2] + self.height;
    let bottom = self.position[2] - self.height;
    let px_step = (2f64 * self.width) / f64::from(px);
    let py_step = (2f64 * self.height) / f64::from(py);
    vec![]
  }
}
