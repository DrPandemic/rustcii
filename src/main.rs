extern crate image;
extern crate imageproc;
extern crate rusttype;
extern crate rayon;
extern crate itertools;

use std::env;
use std::io;
use std::error::Error;
use std::cmp::Ordering;

use image::{GenericImageView, Rgb, SubImage};
use imageproc::drawing;
pub use rusttype::{Scale, Font};
use itertools::Itertools;
use rayon::prelude::*;

type BaseImage = image::RgbImage;
// const BACKGROUND: [u8; 3] = [0, 0, 0];
const BACKGROUND: [u8; 3] = [255, 255, 255];
// const FOREGROUND: [u8; 3] = [255, 255, 255];
const FOREGROUND: [u8; 3] = [127, 127, 127];

fn main() -> Result<(), Box<dyn Error>> {
  let (input_filename, output_filename) = parse_input()?;
  let original_tile_size = 16;

  let input_dynamic_image = image::open(input_filename)?;
  let input_image_rgb = input_dynamic_image.to_rgb();
  let input_image_gray = input_dynamic_image.grayscale().to_rgb();
  let (width, height) = input_image_gray.dimensions();
  let mut canvas = create_canvas(width, height, Rgb(BACKGROUND));

  let font = get_font()?;
  let characters = get_character_images(original_tile_size, &font)?;
  let tile_size = characters[0].1.dimensions().0;

  fill_canvas(&input_image_rgb, &input_image_gray, &mut canvas, &characters, original_tile_size, tile_size, &font);

  canvas.save(output_filename).unwrap();

  Ok(())
}

fn parse_input() -> Result<(String, String), io::Error> {
  let mut args = env::args();

  if args.len() != 3 {
    Err(io::Error::new(io::ErrorKind::InvalidInput, "You need 2 arguments"))
  } else {
    Ok((args.nth(1).unwrap(), args.nth(0).unwrap()))
  }
}

fn create_canvas(width: u32, height: u32, color: Rgb<u8>) -> BaseImage {
  image::ImageBuffer::from_pixel(width, height, color)
}

fn get_character_images(tile_size: u32, font: &Font<'static>) -> Result<Vec<(char, BaseImage)>, io::Error> {
  Ok((b'a'..=b'z')
    .chain(b'A'..=b'Z').map(|c| c as char)
    .chain("@#$%^&()+=023456789/\\".chars())
     .map(|c| {
       (c, get_character_image(tile_size, &font, &Rgb(BACKGROUND), &Rgb(FOREGROUND), &c))
    }).collect())
}

fn get_character_image(tile_size: u32, font: &Font<'static>, background_color: &Rgb<u8>, foreground_color: &Rgb<u8>, character: &char) -> BaseImage {
  let mut canvas = create_canvas(tile_size, tile_size, *background_color);
  drawing::draw_text_mut(
    &mut canvas,
    *foreground_color,
    tile_size / 4,
    0,
    Scale::uniform(tile_size as f32),
    &font,
    &character.to_string(),
  );
  image::imageops::crop(&mut canvas, 4, 1, tile_size - 5, tile_size - 5).to_image()
}

fn get_font() -> Result<Font<'static>, io::Error> {
  let font_data: &[u8] = include_bytes!("../LiberationMono-Bold.ttf");
  let font: Font<'static> = Font::from_bytes(font_data)?;
  Ok(font)
}

fn fill_canvas(
  source_rgb: &BaseImage,
  source_gray: &BaseImage,
  destination: &mut BaseImage,
  characters: &Vec<(char, BaseImage)>,
  original_tile_size: u32,
  tile_size: u32,
  font: &Font<'static>
) {
  // I shouldn't collect here. I need to find what I need to implement to remove the collect
  (0..source_gray.dimensions().0 / tile_size).cartesian_product(0..source_gray.dimensions().1 / tile_size)
    .par_bridge()
    .map(|(x, y)| {
      let source_rgb_tile = SubImage::new(source_rgb, x * tile_size, y * tile_size, tile_size, tile_size);
      let source_gray_tile = SubImage::new(source_gray, x * tile_size, y * tile_size, tile_size, tile_size);
      let character = best_character(&source_gray_tile, &characters);
      let character_image = get_character_image(original_tile_size, font, &get_average_britghness(&source_rgb_tile), &get_average_color(&source_rgb_tile), character);

      (x, y, character_image)
    })
    .collect::<Vec<(u32, u32, BaseImage)>>().iter()
    .for_each(|(x, y, image)| copy_image(&image, destination, x * tile_size, y * tile_size));
}

fn copy_image(source: &BaseImage, destination: &mut BaseImage, x: u32, y: u32) {
  for xx in 0..source.dimensions().0 {
    for yy in 0..source.dimensions().1 {
      destination.put_pixel(x + xx, y + yy, *source.get_pixel(xx, yy))
    }
  }
}

fn compare_images(a: &BaseImage, b: &SubImage<&BaseImage>) -> f32 {
  if a.dimensions() != b.dimensions() {
    panic!("Images size didn't match")
  }
  (0..a.dimensions().0).cartesian_product(0..a.dimensions().1).fold(0., |acc, (x, y)| {
    let a_p = a.get_pixel(x, y);
    let b_p = b.get_pixel(x, y);

    // Only one channel since it's using grayscale
    acc + (a_p[0] as i16 - b_p[0] as i16).abs() as f32 / 255.
  }) / (a.dimensions().0 as f32 * a.dimensions().1 as f32)
}

fn best_character<'a>(source_tile: &SubImage<&BaseImage>, characters: &'a Vec<(char, BaseImage)>) -> &'a char {
  &characters.iter().max_by(|(_, a), (_, b)| {
    let s0 = compare_images(&a, &source_tile);
    let s1 = compare_images(&b, &source_tile);
    if s0 > s1 {
      Ordering::Greater
    } else if s0 < s1 {
      Ordering::Less
    } else {
      Ordering::Equal
    }
  }).unwrap().0
}

fn get_average_color(image: &SubImage<&BaseImage>) -> Rgb<u8> {
  let size = (image.dimensions().0 * image.dimensions().1) as f64;
  let (r, g, b) = image.pixels().fold((0., 0., 0.), |(r, g, b), (_, _, pixel)| {
    (r + pixel[0] as f64, g + pixel[1] as f64, b + pixel[2] as f64)
  });
  Rgb([(r / size) as u8, (g / size) as u8, (b / size) as u8])
}

fn get_average_britghness(image: &SubImage<&BaseImage>) -> Rgb<u8> {
  let brightness = (image.pixels().fold(0., |acc, (_, _, pixel)| {
    acc + ((pixel[0] * 2 + pixel[1] * 3 + pixel[2]) / 6) as f64
  }) / (image.dimensions().0 * image.dimensions().1) as f64) as u8;
  Rgb([brightness, brightness, brightness])
}
