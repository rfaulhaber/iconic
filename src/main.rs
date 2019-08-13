extern crate clap;
extern crate image;
extern crate rand;
extern crate rand_xorshift;
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
                .default_value("50"),
        )
        .arg(
            Arg::with_name("scale")
                .long("scale")
                .takes_value(true)
                .help("Scale of image")
                .default_value("10"),
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
    let scale = matches.value_of("scale").unwrap().parse::<u32>().unwrap();

    let mut imgbuf: RgbImage = ImageBuffer::new(size * scale, size * scale);

    let primary = [rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()];
    let secondary = [rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()];
    let background = [rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()];

    let width = rng.gen_range(10, (size * scale) / 2);

    let run = rng.gen_ratio(1, 2);
    for (x, pixel) in imgbuf.enumerate_rows_mut() {
        // if run {
        //     pixel = image::Rgb(primary);
        // } else {
        //     pixel = image::Rgb(secondary);
        // }
    }

    imgbuf
        .save("/Users/ryanfaulhaber/Desktop/test.png")
        .unwrap();
}
