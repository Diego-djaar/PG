#![feature(const_float_classify)]
use camera::Camera;
use point::Point;
use vec::Vector;

mod camera;
mod esfera;
mod point;
mod vec;

fn main() {
    let camera = Camera::new(Point::new(0.0, 0.0, 0.0), Point::new(10.0, 0.0, 0.0), 5.0);
    println!("{:?}", camera);
}
