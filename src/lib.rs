use image::{Rgb, RgbImage};
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

// Blocks
const NUM_BLOCKS: u32 = 5;
const BLOCK_WIDTH: u32 = 70;
const BLOCK_HEIGHT: u32 = 70;
const PADDING: u32 = 35;
// Image
const IMG_HEIGHT: u32 = PADDING + (BLOCK_HEIGHT * NUM_BLOCKS) + PADDING;
const IMG_WIDTH: u32 = PADDING + (BLOCK_WIDTH * NUM_BLOCKS) + PADDING;
// Colors
const BLUE_COLOR: Rgb<u8> = Rgb([131, 173, 208]);
const GRAY_COLOR: Rgb<u8> = Rgb([240, 240, 240]);
// Compile-time assertions
const _: () = assert!(BLOCK_WIDTH == BLOCK_HEIGHT, "Blocks must be squares");

pub struct Icon(RgbImage);

impl Icon {
    pub fn new(seed: u64) -> Self {
        let mut canvas = RgbImage::from_pixel(IMG_WIDTH, IMG_HEIGHT, GRAY_COLOR);
        Self::draw(seed, &mut canvas);
        Icon(canvas)
    }

    fn draw(seed: u64, canvas: &mut RgbImage) {
        let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
        for block_index in (0..NUM_BLOCKS).map(|v| v * 70) {
            let base = PADDING + block_index;
            let block_a_type = rng.random_bool(0.5).then(|| BLUE_COLOR);
            let block_b_type = rng.random_bool(0.5).then(|| BLUE_COLOR);
            let block_c_type = rng.random_bool(0.5).then(|| BLUE_COLOR);

            // A
            if let Some(pixel) = block_a_type {
                for y in base..base + 70 {
                    for x in PADDING + 0..PADDING + 70 {
                        canvas.put_pixel(x, y, pixel);
                    }
                    // A's reflection
                    for x in PADDING + 280..PADDING + 350 {
                        canvas.put_pixel(x, y, pixel);
                    }
                }
            }

            // B
            if let Some(pixel) = block_b_type {
                for y in base..base + 70 {
                    for x in PADDING + 70..PADDING + 140 {
                        canvas.put_pixel(x, y, pixel);
                    }
                    // B's reflection
                    for x in PADDING + 210..PADDING + 280 {
                        canvas.put_pixel(x, y, pixel);
                    }
                }
            }

            // C
            if let Some(pixel) = block_c_type {
                for y in base..base + 70 {
                    for x in PADDING + 140..PADDING + 210 {
                        canvas.put_pixel(x, y, pixel);
                    }
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
