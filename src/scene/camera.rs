extern crate nalgebra;
extern crate image;

use std::fs::File;
use self::image::png::PNGEncoder;
use self::image::ColorType;

use super::Scene;
use primitives::{Collidable, Ray};
use self::nalgebra::*;

pub trait Camera {
  fn render(&self, scene: &Scene);
}

pub struct OrthographicCamera {
  position: Point3<f64>,
  direction: Vector3<f64>,
  width: f64,
  height: f64,
}

impl OrthographicCamera {
  pub fn new(position: Point3<f64>, direction: Vector3<f64>, width: f64, height: f64) -> OrthographicCamera {
    OrthographicCamera {
      position: position,
      direction: direction.normalize(),
      width: width,
      height: height,
    }
  }

  pub fn render(&self, scene: &Scene, px: i32, py: i32) -> Vec<u8> {
    let left = self.position[0] - self.width;
    let right = self.position[0] + self.width;
    let top = self.position[2] + self.height;
    let bottom = self.position[2] - self.height;
    let px_step = (2f64 * self.width) / (px as f64);
    let py_step = (2f64 * self.height) / (py as f64);
    vec![]
  }
}

pub struct CameraConfig {
  resolution: (u32, u32),
  vertical_pov: f64,
}

impl CameraConfig {
  pub fn new() -> CameraConfig {
    CameraConfig {
      resolution: (1280, 720),
      vertical_pov: 90f64,
    }
  }
}

pub struct PerspectiveCamera {
  pub matrix: Matrix4<f64>,
  config: CameraConfig
}

impl PerspectiveCamera {
  pub fn new(camera: Point3<f64>, target: Point3<f64>, up: Vector3<f64>) -> PerspectiveCamera {
    let d = camera;
    let c = (d - target).normalize();
    let a = up.cross(&c).normalize();
    let b = c.cross(&a);
    println!("A:{}\nB:{}\nC:{}\nD:{}", a, b, c, d);
    PerspectiveCamera {
      matrix: Matrix4::new(
        a.x, b.x, c.x, d.x,
        a.y, b.y, c.y, d.y,
        a.z, b.z, c.z, d.z,
        0f64, 0f64, 0f64, 1f64),
      config: CameraConfig::new(),
    }
  }
}

impl Camera for PerspectiveCamera {
  fn render(&self, scene: &Scene) {
    println!("CamMat: {}", self.matrix);
    let (x_res, y_res) = self.config.resolution;
    let aspect = (x_res as f64) / (y_res as f64);
    let pov = (self.config.vertical_pov.to_radians() / 2f64).tan();

    let f = File::create("foo.png").unwrap();
    let encoder = PNGEncoder::new(f);
    let mut pixel_data: Vec<u8> = vec![];
    // const WIDTH: usize = 1280;
    // const HEIGHT: usize = 720;
    // const RADIUS: u32 = 60;
    // let pixel_data = [0; WIDTH*HEIGHT];
    // let pixel_data = pixel_data
    //     .into_iter()
    //     .enumerate()
    //     .flat_map(|(i, _)| {
    //         let pos = ((i % WIDTH) as f64, (i / WIDTH) as f64);
    //         let middle = ((WIDTH / 2) as f64, (HEIGHT / 2) as f64);
    //         let radius = ((pos.0 - middle.0).powi(2) + (pos.1 - middle.1).powi(2)).sqrt();
    //     }).collect::<Vec<u8>>();
    for y in 0..y_res {
      for x in 0..x_res {
        let x_pixel = (x as f64 + 0.5f64) / x_res as f64;
        let y_pixel = (y as f64 + 0.5f64) / y_res as f64;
        let x_pixel = (2f64 * x_pixel - 1f64) * aspect * pov;
        let y_pixel = (1f64 - 2f64 * y_pixel) * pov;

        let ray_origin = self.matrix * Point4::new(0f64, 0f64, 0f64, 1f64);
        let cam_point = Point4::new(x_pixel, y_pixel, -1f64, 1f64);
        let ray_direction = (self.matrix * cam_point - ray_origin).normalize();
        let ray = Ray::new(ray_origin, ray_direction);
        // println!("Pixel Point: ({}, {})", x_pixel, y_pixel);
        // println!("{}", cam_point);
        // println!("Ray: {} {}", ray.origin, ray.direction);
        let mut color = if scene.intersected_by(&ray) {
            vec![255, 0, 0, 255]
        } else {
            vec![0, 0, 0, 255]
        };
        pixel_data.append(&mut color);
      }
    }

    encoder.encode(&pixel_data[..], x_res, y_res, ColorType::RGBA(8)).unwrap();
  }
}
