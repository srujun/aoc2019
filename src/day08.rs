use std::collections::HashMap;

use crate::problem::Problem;

const SHOULD_PRINT: bool = false;
const WIDTH: usize = 25;
const HEIGHT: usize = 6;

mod helpers {
  pub type Layer = [u8];

  pub struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
  }

  impl Image {
    pub fn new(width: usize, height: usize, data: Vec<u8>) -> Self {
      Self {
        width,
        height,
        data,
      }
    }

    pub fn num_layers(&self) -> usize {
      self.data.len() / (self.width * self.height)
    }

    pub fn get_layer(&self, layer: usize) -> &Layer {
      let start = (self.width * self.height) * layer;
      let end = start + (self.width * self.height);
      self.data.get(start..end).expect("Invalid layer size!")
    }

    pub fn iter(&self) -> Layers {
      Layers {
        image: &self,
        next_layer: 0,
      }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
      for layer in self.iter() {
        let color = layer[self.width * y + x];
        if color != 2 {
          return color;
        }
      }
      panic!("No color found at ({}, {})!", x, y);
    }
  }

  pub struct Layers<'a> {
    image: &'a Image,
    next_layer: usize,
  }

  impl<'a> Iterator for Layers<'a> {
    type Item = &'a Layer;

    fn next(&mut self) -> Option<Self::Item> {
      if self.next_layer >= self.image.num_layers() {
        None
      } else {
        let layer = self.image.get_layer(self.next_layer);
        self.next_layer += 1;
        Some(layer)
      }
    }
  }
}

#[derive(Default)]
pub struct DayEight {}

impl DayEight {
  pub fn new() -> Self {
    Self {}
  }
}

use helpers::{Image, Layer};

impl Problem for DayEight {
  fn soln_one(&self) -> Option<String> {
    Some("2562".to_string())
  }

  #[allow(clippy::naive_bytecount)]
  fn part_one(&self, input: &str) -> Option<String> {
    let pixels: Vec<u8> = input
      .chars()
      .map(|c| c.to_digit(10).unwrap() as u8)
      .collect();
    let image = Image::new(WIDTH, HEIGHT, pixels);

    let mut layer_zeros: HashMap<&Layer, usize> = HashMap::new();
    for layer in image.iter() {
      layer_zeros.insert(layer, layer.iter().filter(|&&p| p == 0).count());
    }
    let (&min_layer, _) = layer_zeros.iter().min_by_key(|(_, &zeros)| zeros).unwrap();

    let ones = min_layer.iter().filter(|&&p| p == 1).count();
    let twos = min_layer.iter().filter(|&&p| p == 2).count();

    Some((ones * twos).to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    if SHOULD_PRINT {
      let pixels: Vec<u8> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
      let image = Image::new(WIDTH, HEIGHT, pixels);

      for y in 0..HEIGHT {
        for x in 0..WIDTH {
          let out = match image.get_pixel(x, y) {
            0 => ' ',
            1 => '*',
            _ => '?',
          };
          print!("{}", out);
        }
        println!();
      }
    }
    Some("Done".to_string())
  }
}

#[cfg(test)]
mod tests {
  // use super::DayEight;
  // use crate::problem::Problem;
}
