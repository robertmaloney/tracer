use super::Ray;

pub trait Collidable {
  fn intersected_by(&self, ray: &Ray) -> bool;
}
