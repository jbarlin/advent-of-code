use std::cmp;
use std::collections::HashMap;

use crate::coords::Coords;

const BLOCK: &str = "█";
const DARK_MODE: bool = true;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pixel {
	Black = 0,
	White = 1,
	Transparent = 2,
	Star = 3
}

impl Pixel {
	pub fn convert_char(value: char) -> Pixel {
		match value {
			'0' => Pixel::Black,
			'1' => Pixel::White,
			'*' => Pixel::Star,
			_ => Pixel::Transparent,
		}
	}
	pub fn convert_int(value: i64) -> Pixel{
		match value {
			0 => Pixel::Black,
			1 => Pixel::White,
			_ => Pixel::Transparent
		}
	}
}

pub struct ImageLayer {
	pub pixels: Vec<Vec<Pixel>>,
}

impl ImageLayer {

	pub fn from_hashmap(map: HashMap<Coords, Pixel>) -> ImageLayer{
		let mut min_x = 0;
		let mut max_x = 0;
		let mut min_y = 0;
		let mut max_y = 0;
		for key in map.keys() {
			min_x = cmp::min(min_x, key.x);
			max_x = cmp::max(max_x, key.x);
			min_y = cmp::min(min_y, key.y);
			max_y = cmp::max(max_y, key.y);
		}
		let mut pixels: Vec<Vec<Pixel>> = Vec::new();
		for y in min_y..=max_y {
			let mut x_pix: Vec<Pixel> = Vec::new();
			for x in min_x..=max_x {
				let coords = Coords{
					x, y
				};
				let r: Option<&Pixel> = map.get(&coords);
				x_pix.push(match r {
					Some(x) => *x,
					None => Pixel::Black,
				})
			}
			pixels.push(x_pix);
		}
		pixels.reverse();
		let il: ImageLayer = ImageLayer { pixels };
		return il;
	}

	pub fn count_of_pixels(&self) -> (usize, usize, usize) {
		let mut blacks = 0;
		let mut whites = 0;
		let mut transparents = 0;
		for row in self.pixels.iter() {
			for px in row.iter() {
				match px {
					Pixel::Black => {
						blacks += 1;
					}
					Pixel::White => {
						whites += 1;
					}
					_ => {
						transparents += 1;
					}
				}
			}
		}
		return (blacks, whites, transparents);
	}

	pub fn to_string(&self) -> String {
		let mut output: String =
			String::with_capacity(self.pixels.len() * (self.pixels[0].len() + 1) + 1);
		output.push_str("\n");
		for row in self.pixels.iter() {
			for px in row.iter() {
				match px {
					Pixel::Black => {
						if !DARK_MODE {
							output.push_str(BLOCK);
						} else {
							//Non-breaking space
							output.push_str(" ");
						}
					}
					Pixel::White => {
						if DARK_MODE {
							output.push_str(BLOCK);
						} else {
							//Non-breaking space
							output.push_str(" ");
						}
					},
					Pixel::Star => {
						output.push_str("*")
					}
					_ => unreachable!(),
				}
			}
			output.push_str("\n");
		}
		return output;
	}
}