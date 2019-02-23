extern crate image;
extern crate imageproc;

use std::env;
use std::io;
use std::error::Error;
use std::cmp::Ordering;

use image::{GenericImageView, Rgb, SubImage};
use imageproc::drawing;
pub use rusttype::{Scale, Font};

type BaseImage = image::RgbImage;
const BACKGROUND: [u8; 3] = [0, 0, 0];
const FOREGROUND: [u8; 3] = [255, 255, 255];

fn main() -> Result<(), Box<dyn Error>> {
  let (input_filename, output_filename) = parse_input()?;
  let mut tile_size = 9;

  let input_image = image::open(input_filename)?.grayscale().to_rgb();
  let (width, height) = input_image.dimensions();
  let mut canvas = create_canvas(width, height);

  let characters = get_character_images(tile_size)?;
  tile_size = characters[0].dimensions().0;

  println!("{:?}", compare_images(&characters[0], &characters[1]) > compare_images(&characters[0], &characters[2]));

  fill_canvas(&input_image, &mut canvas, &characters, tile_size);

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

fn create_canvas(width: u32, height: u32) -> BaseImage {
  image::ImageBuffer::from_pixel(width, height, image::Rgb(BACKGROUND))
}

fn get_character_images(tile_size: u32) -> Result<Vec<BaseImage>, io::Error> {
  let font = get_font()?;

  Ok((b'a'..=b'z')
    .chain(b'A'..=b'Z').map(|c| c as char)
    .chain("@#$%^&*()+=023456789/\\".chars())
    .map(|c| {
      let mut canvas = create_canvas(tile_size, tile_size);
      drawing::draw_text_mut(
        &mut canvas,
        image::Rgb(FOREGROUND),
        tile_size / 4,
        0,
        Scale::uniform(tile_size as f32),
        &font,
        &c.to_string(),
      );
      let offset = tile_size / 3;
      image::imageops::crop(&mut canvas, 0, 0, tile_size - offset, tile_size - offset).to_image()
    }).collect())
}

fn get_font() -> Result<Font<'static>, io::Error> {
  let font_data: &[u8] = include_bytes!("../LiberationMono-Regular.ttf");
  let font: Font<'static> = Font::from_bytes(font_data)?;
  Ok(font)
}

fn fill_canvas(source: &BaseImage, destination: &mut BaseImage, characters: &Vec<BaseImage>, tile_size: u32) {
  for x in 0..source.dimensions().0 / tile_size {
    for y in 0..source.dimensions().1 / tile_size {
      let source_tile = SubImage::new(source, x * tile_size, y * tile_size, tile_size, tile_size).to_image();
      copy_image(best_character(&source_tile, &characters), destination, x * tile_size, y * tile_size);
    }
  }
}

fn copy_image(source: &BaseImage, destination: &mut BaseImage, x: u32, y: u32) {
  for xx in 0..source.dimensions().0 {
    for yy in 0..source.dimensions().1 {
      destination.put_pixel(x + xx, y + yy, *source.get_pixel(xx, yy))
    }
  }
}

fn compare_images(a: &BaseImage, b: &BaseImage) -> f32 {
  if a.dimensions() != b.dimensions() {
    panic!("Images size didn't match")
  }
  let mut diff = 0.;
  for x in 0..a.dimensions().0 {
    for y in 0..b.dimensions().1 {
      let a_p = a.get_pixel(x, y);
      let b_p = b.get_pixel(x, y);

      // Only one channel since it's grayscale
      diff += (a_p[0] as i16 - b_p[0] as i16).abs() as f32 / 255.;
    }
  }

  diff / (a.dimensions().0 as f32 * a.dimensions().1 as f32)
}

fn best_character<'a>(source_tile: &BaseImage, characters: &'a Vec<BaseImage>,) -> &'a BaseImage {
  characters.iter().max_by(|a, b| {
    let s0 = compare_images(&a, &source_tile);
    let s1 = compare_images(&b, &source_tile);
    if s0 > s1 {
      Ordering::Greater
    } else if s0 < s1 {
      Ordering::Less
    } else {
      Ordering::Equal
    }
  }).unwrap()
}
