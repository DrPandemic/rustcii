extern crate image;
extern crate imageproc;

use std::env;
use std::io;
use std::error::Error;

use image::{GenericImageView, Rgb, SubImage};
use imageproc::drawing;
pub use rusttype::{Scale, Font};

type BaseImage = image::RgbImage;

fn main() -> Result<(), Box<dyn Error>> {
  let (input_filename, output_filename) = parse_input()?;
  let tile_size = 6;

  let input_image = image::open(input_filename)?.to_rgb();
  let (width, height) = input_image.dimensions();
  let mut canvas = create_canvas(width, height);

  let characters = get_character_images(tile_size)?;

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
  image::ImageBuffer::from_pixel(width, height, image::Rgb([255, 255, 255]))
}

fn get_character_images(tile_size: u32) -> Result<Vec<BaseImage>, io::Error> {
  let font = get_font()?;

  Ok((b'a'..=b'z')
    .chain(b'A'..=b'Z').map(|c| c as char)
    .chain("".chars())
    .map(|c| {
      let mut canvas = create_canvas(tile_size, tile_size);
      drawing::draw_text_mut(
        &mut canvas,
        image::Rgb([0, 0, 0]),
        tile_size / 4,
        0,
        Scale::uniform((tile_size + 1) as f32),
        &font,
        &c.to_string(),
      );
      canvas
    }).collect())
}

fn get_font() -> Result<Font<'static>, io::Error> {
  let font_data: &[u8] = include_bytes!("../LiberationMono-Regular.ttf");
  let font: Font<'static> = Font::from_bytes(font_data)?;
  Ok(font)
}

fn fill_canvas(source: &BaseImage, destination: &mut BaseImage, characters: &Vec<BaseImage>, tile_size: u32) {
  let offset = 1;
  for x in 0..source.dimensions().0 / (tile_size - offset) {
    for y in 0..source.dimensions().1 / (tile_size - offset) {
      copy_image(&characters[0], destination, x * (tile_size - offset), y * (tile_size - offset));
    }
  }
}

fn copy_image(source: &BaseImage, destination: &mut BaseImage, x: u32, y: u32) {
  let offset = 1;
  for xx in (0 + offset)..(source.dimensions().0 - offset) {
    for yy in (0 + offset)..(source.dimensions().1 - offset) {
      destination.put_pixel(x + xx, y + yy, *source.get_pixel(xx, yy))
    }
  }
}
