use image::RgbImage;
use rand::{Rng, SeedableRng};
use std::ops::Range;

const BLOCK_WIDTH: usize = 70;
const BLOCK_HEIGHT: usize = 70;
const PADDING: usize = 35;

// An Icon will have 5 blocks
const IMG_HEIGHT: usize = PADDING + (5 * BLOCK_HEIGHT) + PADDING;
const IMG_WIDTH: usize = PADDING + (5 * BLOCK_WIDTH) + PADDING;

// The mask is  5 x 5
const MASK_HEIGHT: usize = 5;
const MASK_WIDTH: usize = 5;

// Colors
const LIGHT_BLUE: [u8; 3] = [131, 173, 208];
const BACKGROUND_COLOR: [u8; 3] = [240, 240, 240];

struct Icon(RgbImage);

impl Icon {
    fn new(seed: u64) -> Self {
        let mut icon = Icon(RgbImage::from_pixel(
            IMG_WIDTH as u32,
            IMG_HEIGHT as u32,
            image::Rgb(BACKGROUND_COLOR),
        ));

        let mask = Self::generate_random_mask(seed);
        let active_blocks = Self::active_blocks(mask);
        icon.draw(active_blocks);
        icon
    }

    fn generate_random_mask(seed: u64) -> [[bool; MASK_WIDTH]; MASK_HEIGHT] {
        let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
        let mut mask = [[false; MASK_WIDTH]; MASK_HEIGHT];
        for i in 0..5 {
            // `c` and `e` are a reflection of `a` and `b`
            let a = rng.random_bool(0.5);
            let b = rng.random_bool(0.5);
            let c = rng.random_bool(0.5);
            mask[i] = [a, b, c, b, a];
        }
        mask
    }

    fn active_blocks(mask: [[bool; MASK_WIDTH]; MASK_HEIGHT]) -> Vec<(Range<usize>, Range<usize>)> {
        let mut active: Vec<(Range<usize>, Range<usize>)> = vec![];
        for (y, row) in mask.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                if *val == true {
                    let x_start = x * BLOCK_WIDTH;
                    let x_end = x_start + BLOCK_WIDTH;
                    let y_start = y * BLOCK_HEIGHT;
                    let y_end = y_start + BLOCK_HEIGHT;
                    active.push(((x_start..x_end), (y_start..y_end)));
                }
            }
        }
        active
    }

    fn draw(&mut self, active_blocks: Vec<(Range<usize>, Range<usize>)>) {
        for (x_range, y_range) in active_blocks.iter() {
            for x in x_range.clone() {
                for y in y_range.clone() {
                    let pixel = self
                        .0
                        .get_pixel_mut((x + PADDING) as u32, (y + PADDING) as u32);
                    *pixel = image::Rgb(LIGHT_BLUE);
                }
            }
        }
    }

    fn save(self, path: &str) -> Result<(), image::ImageError> {
        self.0.save(path)
    }
}

fn main() {
    let icon = Icon::new(1253);
    icon.save("test.png").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_mask() {}
}
