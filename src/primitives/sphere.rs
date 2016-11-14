extern crate nalgebra;

use nalgebra::*;
use super::{Ray, Collidable};

pub struct Sphere {
  center: Point4<f64>,
  radius: f64,
}

impl Sphere {
  pub fn new(center: Point4<f64>, radius: f64) -> Sphere {
    Sphere {
      center: center,
      radius: radius,
    }
  }
}

impl Collidable for Sphere {
  fn intersected_by(&self, ray: &Ray) -> bool {
    let t = -((ray.origin - self.center).dot(&ray.direction));
    let q = ray.origin + t * ray.direction;
    // println!("{} : {}", t, q);
    let r_2 = self.radius.powi(2);
    let qc_2 = distance_squared(&q, &self.center);
    return qc_2 <= r_2;
    // if qc_2 > r_2 {
    //   return false;
    // }
    //
    // let a = (r_2 - qc_2).sqrt();
    // // println!("{}", a);
    // return t >= a || t >= -a;
  }
}
