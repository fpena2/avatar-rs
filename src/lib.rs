use image::{ImageBuffer, Rgb, RgbImage};
use rand::{Rng, RngCore, SeedableRng, rngs::SmallRng};

// Colors
const BLUE_COLOR: Rgb<u8> = Rgb([131, 173, 208]);
const GRAY_COLOR: Rgb<u8> = Rgb([240, 240, 240]);

// Blocks
const NUM_BLOCKS: u32 = 5;
const BLOCK_WIDTH: u32 = 70;
const BLOCK_HEIGHT: u32 = 70;

// Avatar
const AVATAR_WIDTH: u32 = BLOCK_WIDTH * NUM_BLOCKS;
const AVATAR_HEIGHT: u32 = BLOCK_HEIGHT * NUM_BLOCKS;

pub struct Avatar {
    data: RgbImage,
}

impl Avatar {
    pub fn new(seed: u64) -> Self {
        let rng = SmallRng::seed_from_u64(seed);
        let logical_map = generate_logical_map(rng);
        let data = ImageBuffer::from_fn(AVATAR_WIDTH, AVATAR_HEIGHT, |x, y| {
            let block_row = y / BLOCK_HEIGHT;
            let block_col = x / BLOCK_WIDTH;
            if logical_map[block_row as usize][block_col as usize] {
                BLUE_COLOR
            } else {
                GRAY_COLOR
            }
        });

        Avatar { data }
    }

    pub fn save(&self, path: &str) -> Result<(), image::ImageError> {
        self.data.save(path)
    }
}

fn generate_logical_map(mut rng: impl RngCore) -> [[bool; 5]; 5] {
    std::array::from_fn(|_| {
        let (c1, c2) = (rng.random_bool(0.5), rng.random_bool(0.5));
        [c1, c2, rng.random_bool(0.5), c2, c1]
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageError;

    #[test]
    fn logical_map_should_mirror() {
        let rng = SmallRng::seed_from_u64(1);
        let map = generate_logical_map(rng);
        for row in map {
            assert_eq!((row[0], row[1]), (row[4], row[3]));
        }
    }

    #[test]
    fn image_data_should_extand_from_map() -> Result<(), ImageError> {
        let seed = 12345;
        let icon = Avatar::new(seed);

        assert_eq!(icon.data.width(), BLOCK_WIDTH * NUM_BLOCKS);
        assert_eq!(icon.data.height(), BLOCK_HEIGHT * NUM_BLOCKS);

        let c1 = icon.data.get_pixel(0, 0); // 0 - 69
        let c2 = icon.data.get_pixel(70, 0); // 70 - 139
        let _c3 = icon.data.get_pixel(140, 0); // 140 - 209
        let c4 = icon.data.get_pixel(210, 0); // 210 - 279
        let c5 = icon.data.get_pixel(280, 0); // 280 - 349

        assert_eq!((c1, c2), (c5, c4));

        Ok(())
    }
}
