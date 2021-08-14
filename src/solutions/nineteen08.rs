use crate::AoCDay;
use super::super::image_layer::*;

pub struct Code;

pub const DAY_8_DATA: &str = include_str!("../../inputs/2019/Day8.txt");

const HEIGHT: usize = 6;
const WIDTH: usize = 25;

struct Image {
	layers: Vec<ImageLayer>,
	width: usize,
	height: usize,
}

impl Image {
	fn read_in_str(input: &str, width: usize, height: usize) -> Image {
		let mut img = Image {
			layers: Vec::new(),
			width,
			height,
		};
		let mut curr_layer: ImageLayer = ImageLayer {
			pixels: Vec::with_capacity(height),
		};
		let indx_splice = width * height;
		for (indx, digit) in input.chars().enumerate() {
			let ix = indx % indx_splice;
			let curr_height = ix / width;
			let curr_width = ix % width;
			if curr_width == 0 {
				curr_layer.pixels.push(Vec::with_capacity(width));
			}
			curr_layer.pixels[curr_height].push(Pixel::convert_char(digit));
			if (indx + 1) % indx_splice == 0 {
				img.layers.push(curr_layer);
				curr_layer = ImageLayer {
					pixels: Vec::with_capacity(height),
				};
			}
		}
		return img;
	}

	fn worlds_worst_checksum(&self) -> usize {
		let least_blacks: (usize, usize, usize) = self
			.layers
			.iter()
			.map(|layer| layer.count_of_pixels())
			.min_by(|a, b| a.0.cmp(&b.0))
			.unwrap_or((0, 0, 0));
		return least_blacks.1 * least_blacks.2;
	}

	fn collapse_layers_to_one(&self) -> ImageLayer {
		let mut final_layer = ImageLayer {
			pixels: Vec::with_capacity(self.height),
		};
		for curr_height in 0..self.height {
			final_layer.pixels.push(Vec::with_capacity(self.width));
			for curr_width in 0..self.width {
				for layer in self.layers.iter() {
					let curr_px = &layer.pixels[curr_height][curr_width];
					if curr_px == &Pixel::Transparent {
						continue;
					} else if curr_px == &Pixel::Black {
						final_layer.pixels[curr_height].push(Pixel::Black);
						break;
					} else {
						final_layer.pixels[curr_height].push(Pixel::White);
						break;
					}
				}
			}
		}
		return final_layer;
	}
}

pub fn part_1_impl(input: &str, width: usize, height: usize) -> String {
	return Image::read_in_str(input, width, height)
		.worlds_worst_checksum()
		.to_string();
}

pub fn part_2_impl(input: &str, width: usize, height: usize) -> String {
	return Image::read_in_str(input, width, height)
		.collapse_layers_to_one()
		.to_string();
}

impl AoCDay for Code {
	fn part1(&self) -> String {
		return part_1_impl(DAY_8_DATA, WIDTH, HEIGHT);
	}
	fn part2(&self) -> String {
		//let memory = Vec::from(DAY_7_DATA);
		return part_2_impl(DAY_8_DATA, WIDTH, HEIGHT);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_example_pt_1() {
		let img: Image = Image::read_in_str("123456789012", 3, 2);
		assert_eq!(img.layers.len(), 2);
	}

	#[test]
	fn test_example_pt_2() {
		let img: Image = Image::read_in_str("0222112222120000", 2, 2);
		assert_eq!(img.layers.len(), 4);
		let collapse = img.collapse_layers_to_one();
		assert_eq!(collapse.pixels[0][0], Pixel::Black);
		assert_eq!(collapse.pixels[0][1], Pixel::White);
		assert_eq!(collapse.pixels[1][0], Pixel::White);
		assert_eq!(collapse.pixels[1][1], Pixel::Black);
	}
}
