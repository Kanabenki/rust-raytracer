#[macro_use]
extern crate derive_builder;

use crate::raytracer::RaytracerBuilder;

mod camera;
mod hitable;
mod list;
mod material;
mod ray;
mod raytracer;
mod utils;
mod vec3;

fn main() {
    let raytracer = RaytracerBuilder::default()
        .build()
        .expect("Error while creating raytracer");
    raytracer.run();
}
