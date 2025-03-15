use image::{Rgb, RgbImage};
use rand::{Rng, SeedableRng};
use std::ops::Range;

const BLOCK_WIDTH: usize = 70;
const BLOCK_HEIGHT: usize = 70;
const PADDING: usize = 35;

// Colors
const LIGHT_BLUE_COLOR: Rgb<u8> = Rgb([131, 173, 208]);
const BACKGROUND_COLOR: Rgb<u8> = Rgb([240, 240, 240]);

struct Mask {
    matrix: [[bool; Self::MASK_WIDTH]; Self::MASK_HEIGHT],
}

impl Mask {
    const MASK_HEIGHT: usize = 5;
    const MASK_WIDTH: usize = 5;

    fn new(seed: u64) -> Self {
        Mask {
            matrix: Self::generate_matrix(seed),
        }
    }

    fn generate_matrix(seed: u64) -> [[bool; Self::MASK_WIDTH]; Self::MASK_HEIGHT] {
        let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
        let mut mask = [[false; Self::MASK_WIDTH]; Self::MASK_HEIGHT];

        for i in 0..Self::MASK_HEIGHT {
            // `c` and `e` are a reflection of `a` and `b`
            let a = rng.random_bool(0.5);
            let b = rng.random_bool(0.5);
            let c = rng.random_bool(0.5);
            mask[i] = [a, b, c, b, a];
        }
        mask
    }

    fn expanded(&self, new_width: usize, new_height: usize) -> Vec<(Range<usize>, Range<usize>)> {
        let mut active_ranges: Vec<(Range<usize>, Range<usize>)> = vec![];

        for (y, row) in self.matrix.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if *val == true {
                    let x_start = x * new_width;
                    let x_end = x_start + new_width;
                    let y_start = y * new_height;
                    let y_end = y_start + new_height;
                    active_ranges.push(((x_start..x_end), (y_start..y_end)));
                }
            }
        }
        active_ranges
    }
}

pub struct Icon(RgbImage);

impl Icon {
    // An Icon will have 5 blocks
    const IMG_HEIGHT: usize = PADDING + (5 * BLOCK_HEIGHT) + PADDING;
    const IMG_WIDTH: usize = PADDING + (5 * BLOCK_WIDTH) + PADDING;

    pub fn new(seed: u64) -> Self {
        let mut icon = Icon(RgbImage::from_pixel(
            Self::IMG_WIDTH as u32,
            Self::IMG_HEIGHT as u32,
            BACKGROUND_COLOR,
        ));

        let mask = Mask::new(seed);
        let ranges = mask.expanded(BLOCK_WIDTH, BLOCK_HEIGHT);
        icon.draw(ranges);
        icon
    }

    fn draw(&mut self, active_ranges: Vec<(Range<usize>, Range<usize>)>) {
        for (x_range, y_range) in active_ranges.iter() {
            for x in x_range.clone() {
                for y in y_range.clone() {
                    let pixel = self
                        .0
                        .get_pixel_mut((x + PADDING) as u32, (y + PADDING) as u32);
                    *pixel = LIGHT_BLUE_COLOR;
                }
            }
        }
    }

    pub fn save(self, path: &str) -> Result<(), image::ImageError> {
        self.0.save(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_mask() {}
}
