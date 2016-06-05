extern crate image;

use std::fs::File;
use image::png::PNGEncoder;
use image::ColorType;

mod primitives;
use primitives::Sphere;

mod scene;
use scene::{Scene, OrthographicCamera};

mod util;
use util::TwoDimensionalIterator;

fn main() {
    let one = 0..50;
    let two = vec!["a", "b", "c", "d", "e"];

    let it = TwoDimensionalIterator::new(one, two.iter());
    for i in it {
        println!("{} {}", i.0, i.1);
    }
    // let f = File::create("foo.png").unwrap();
    // let encoder = PNGEncoder::new(f);
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
    //         if radius < RADIUS as f64 {
    //             vec![255, 255, 255, 255]
    //         } else {
    //             vec![0, 0, 0, 0]
    //         }
    //     }).collect::<Vec<u8>>();
    // encoder.encode(&pixel_data[..], 1280, 720, ColorType::RGBA(8)).unwrap();
}
