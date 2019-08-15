extern crate clap;
extern crate image;
extern crate imageproc;
extern crate rand;
extern crate rand_xorshift;
use clap::{App, Arg};
use image::{GenericImage, ImageFormat, Pixel, Rgb, RgbImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::{Rect, RectPosition};
use rand::prelude::*;
use rand::SeedableRng;
use rand_xorshift::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;

enum Format {
    JPG,
    PNG,
}

impl Format {
    fn from_str(s: &str) -> Option<Format> {
        match s {
            "jpg" => Some(Format::JPG),
            "png" => Some(Format::PNG),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Format::JPG => "jpg",
            Format::PNG => "png",
        }
    }
}

fn main() {
    let app = App::new("iconic")
        .version("0.1.0")
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
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .default_value(".")
                .help("Directory to output"),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .default_value("png")
                .help("Image format to output. Possible values are: [png, jpg]"),
        )
        .arg(Arg::with_name("input").help("Value to hash"));

    let matches = app.get_matches();

    let input = matches.value_of("input").unwrap();
    let path = Path::new(matches.value_of("out").unwrap());
    let format = matches.value_of("format").unwrap();

    let mut hasher = DefaultHasher::new();

    hasher.write(input.as_bytes());

    let mut rng = XorShiftRng::seed_from_u64(hasher.finish());

    let size = matches.value_of("size").unwrap().parse::<u32>().unwrap();

    let mut img = RgbImage::new(size, size);

    let primary = [rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()];
    let secondary = [rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()];
    let background = [rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()];

    if matches.is_present("symmetric") {
        for _ in 0..rng.gen_range(5, size / 3) {
            let (x, y) = (
                rng.gen_range(-(size as i32) / 2, size as i32 / 2),
                rng.gen_range(-(size as i32) / 2, size as i32 / 2),
            );
            let (width, height) = (rng.gen_range(10, size / 2), rng.gen_range(10, size / 2));
            let rect = Rect::at(x, y).of_size(width, height);

            let flip = rng.gen_ratio(1, 3);

            if flip {
                draw_filled_rect_mut(&mut img, rect, image::Rgb(primary));
            } else {
                draw_filled_rect_mut(&mut img, rect, image::Rgb(secondary));
            }
        }

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            if pixel.to_rgb() == image::Rgb([0, 0, 0]) {
                *pixel = image::Rgb(background);
            }
        }

        let img_copy = img.clone();
        let half = size / 2;

        // TODO this isn't right!

        for x in half..img.width() {
            for y in 0..img.height() {
                *img.get_pixel_mut(x, y) = *img_copy.get_pixel(x, y);
            }
        }
    } else {
        for _ in 0..rng.gen_range(5, size / 3) {
            let (x, y) = (
                rng.gen_range(-(size as i32), size as i32),
                rng.gen_range(-(size as i32), size as i32),
            );
            let (width, height) = (rng.gen_range(10, size / 2), rng.gen_range(10, size / 2));
            let rect = Rect::at(x, y).of_size(width, height);

            let flip = rng.gen_ratio(1, 3);

            if flip {
                draw_filled_rect_mut(&mut img, rect, image::Rgb(primary));
            } else {
                draw_filled_rect_mut(&mut img, rect, image::Rgb(secondary));
            }
        }

        for (_, _, pixel) in img.enumerate_pixels_mut() {
            if pixel.to_rgb() == image::Rgb([0, 0, 0]) {
                *pixel = image::Rgb(background);
            }
        }
    }

    let name = format!("{}/{}-{}.{}", path.display(), input, size, format);

    img.save(name).unwrap();
}
