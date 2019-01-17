#[macro_use]
extern crate derive_builder;

use crate::raytracer::RaytracerBuilder;
use clap::value_t;
use clap::App;

mod camera;
mod hitable;
mod list;
mod material;
mod ray;
mod raytracer;
mod utils;
mod vec3;

fn main() {
    let matches = App::new("Rust Raytracer")
                    .about("Simple raytracer written in Rust")
                    .args_from_usage(
                        "<FILE> 'Sets the output image file path'
                         -x <x> 'Sets the width of the output image, default to 2000'
                         -y <Y> 'Set the height of the output image, default to 1000' 
                         [-t] 'Sets the number of threads to use, by default the number of cores availables'")
                    .get_matches();

    let mut raytracer = RaytracerBuilder::default();
    let x = value_t!(matches.value_of("x"), u32).expect("x should be a number");
    let y = value_t!(matches.value_of("y"), u32).expect("y should be a number");

    raytracer
        .res_x(x)
        .res_y(y)
        .out_file(matches.value_of("FILE").unwrap())
        .build()
        .expect("Error while creating raytracer")
        .run();
}
