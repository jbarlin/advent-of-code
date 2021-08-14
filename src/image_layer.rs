const BLOCK: &str = "█";
const DARK_MODE: bool = true;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pixel {
	Black = 0,
	White = 1,
	Transparent = 2,
}

impl Pixel {
	pub fn convert_char(value: char) -> Pixel {
		match value {
			'0' => Pixel::Black,
			'1' => Pixel::White,
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
					}
					_ => unreachable!(),
				}
			}
			output.push_str("\n");
		}
		return output;
	}
}