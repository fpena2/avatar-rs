use image::{Rgb, RgbImage};
use rand::{Rng, SeedableRng};

// Blocks
pub const NUM_BLOCKS: u32 = 5;
pub const BLOCK_WIDTH: u32 = 70;
pub const BLOCK_HEIGHT: u32 = 70;
pub const PADDING: u32 = 35;
// Image
pub const IMG_HEIGHT: u32 = PADDING + (BLOCK_HEIGHT * NUM_BLOCKS) + PADDING;
pub const IMG_WIDTH: u32 = PADDING + (BLOCK_WIDTH * NUM_BLOCKS) + PADDING;
// Colors
pub const BLUE_COLOR: Rgb<u8> = Rgb([131, 173, 208]);
pub const GRAY_COLOR: Rgb<u8> = Rgb([240, 240, 240]);
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
        for index in 0..NUM_BLOCKS {
            let block_a_type = rng.random_bool(0.5).then_some(BLUE_COLOR);
            let block_b_type = rng.random_bool(0.5).then_some(BLUE_COLOR);
            let block_c_type = rng.random_bool(0.5).then_some(BLUE_COLOR);

            let y_start = PADDING + BLOCK_HEIGHT * index;
            let y_end = y_start + BLOCK_HEIGHT;

            // A
            if let Some(pixel) = block_a_type {
                for y in y_start..y_end {
                    for x in PADDING..PADDING + 70 {
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
                for y in y_start..y_end {
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
                for y in y_start..y_end {
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
    use image::ImageError;

    #[test]
    fn test_icon_creation() {
        let seed = 12345;
        let icon = Icon::new(seed);
        assert_eq!(icon.0.width(), IMG_WIDTH);
        assert_eq!(icon.0.height(), IMG_HEIGHT);
    }

    #[test]
    fn test_icon_save() -> Result<(), ImageError> {
        let seed = 12345;
        let icon = Icon::new(seed);
        let path = "test_icon_save.png";
        icon.save(path)?;
        Ok(())
    }

    #[test]
    fn test_draw_function() {
        let seed = 12345;
        let mut canvas = RgbImage::from_pixel(IMG_WIDTH, IMG_HEIGHT, GRAY_COLOR);

        Icon::draw(seed, &mut canvas);

        let pixel = canvas.get_pixel(PADDING + 70, PADDING + 70);
        assert_eq!(*pixel, BLUE_COLOR);
    }
}
