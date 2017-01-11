extern crate image;

use std::path::Path;
use image::GenericImage;

fn main() {

    let img = image::open(&Path::new("img/screen1.png")).unwrap();

    println!("dimensions {:?}", img.dimensions());

    println!("{:?}", img.color());

	let ref mut new_image = image::imageops::resize(&img, 300, 300, image::FilterType::Gaussian);

	image::save_buffer(&Path::new("test3.png"), new_image, 300, 300, img.color()).unwrap();
}