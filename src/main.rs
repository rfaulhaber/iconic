extern crate clap;
extern crate image;
extern crate rand;
extern crate rand_xorshift;
extern crate sha2;
use clap::{App, Arg};
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use rand::prelude::*;
use rand::SeedableRng;
use rand_xorshift::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct Options {
    size: u16,
    seed: String,
    monochrome: bool,
    symmetric: bool,
}

fn main() {
    let app = App::new("iconic")
        .version("0.1")
        .about("CLI tool for generating icons based on input")
        .author("Ryan Faulhaber <faulhaberryan@gmail.com>")
        .arg(
            Arg::with_name("size")
                .long("size")
                .takes_value(true)
                .help("Size of image, s x s.")
                .default_value("256"),
        )
        .arg(
            Arg::with_name("monochrome")
                .short("m")
                .long("monochrome")
                .help("Whether or not icons will be black and white"),
        )
        .arg(
            Arg::with_name("symmetric")
                .short("s")
                .long("symmetric")
                .help("Whether or not avatar is symmetric"),
        )
        .arg(Arg::with_name("input").help("Value to hash"));

    let matches = app.get_matches();

    let input = matches.value_of("input").unwrap();

    let mut hasher = DefaultHasher::new();

    hasher.write(input.as_bytes());

    let mut rng = XorShiftRng::seed_from_u64(hasher.finish());

    let size = matches.value_of("size").unwrap().parse::<u32>().unwrap();

    let mut imgbuf: RgbImage = ImageBuffer::new(size, size);

    let primary = [rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()];
    let secondary = [rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()];

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if x > size / 2 {
            let r = primary[0];
            let g = primary[1];
            let b = primary[2];
            *pixel = image::Rgb([r as u8, g as u8, b as u8]);
        } else {
            let r = secondary[0];
            let g = secondary[1];
            let b = secondary[2];
            *pixel = image::Rgb([r as u8, g as u8, b as u8]);
        }
    }

    imgbuf
        .save("/Users/ryanfaulhaber/Desktop/test.png")
        .unwrap();
}
