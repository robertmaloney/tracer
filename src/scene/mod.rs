mod camera;
pub use self::camera::OrthographicCamera;
use primitives::collidable::Collidable;

use primitives::Sphere;

pub struct Scene<T: Collidable> {
  objects: Vec<T>,
}
