extern crate nalgebra;
use nalgebra::{Vector4, Point4};

pub struct Ray {
  pub origin: Point4<f64>,
  pub direction: Vector4<f64>
}

impl Ray {
  pub fn new(origin: Point4<f64>, direction: Vector4<f64>) -> Ray {
    Ray {
      origin: origin,
      direction: direction,
    }
  }
}
