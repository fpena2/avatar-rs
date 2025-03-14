use image::ImageBuffer;
use image::{Rgb, RgbImage};
use rand::{Rng, SeedableRng};

// NOTE: Add a 20px padding to the final image
const IMG_HEIGHT: u32 = 400;
const IMG_WIDTH: u32 = 400;
const MASK_HEIGHT: usize = (IMG_HEIGHT / 80) as usize;
const MASK_WIDTH: usize = (IMG_WIDTH / 80) as usize;

struct Icon(RgbImage);

impl Icon {
    fn new(seed: u64) -> Self {
        let rgb_image = RgbImage::new(IMG_WIDTH, IMG_HEIGHT);
        Icon(rgb_image)
    }
}

fn gen_mask(seed: u64) -> [[bool; MASK_WIDTH]; MASK_HEIGHT] {
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

fn main() {
    println!("Hello, world!");
    let icon = Icon::new(444);
    icon.0.save("test.png").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_mask() {}
}
