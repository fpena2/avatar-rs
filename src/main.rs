use image::ImageBuffer;
use image::{Rgb, RgbImage};
use rand::{Rng, SeedableRng};

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
    fn new() -> Self {
        let color = image::Rgb(BACKGROUND_COLOR);
        Icon(RgbImage::from_pixel(IMG_WIDTH as u32, IMG_HEIGHT as u32, color))
    }

    fn func(&mut self, seed: u64){
        let mask = gen_mask(seed); 
        let mut mask_active: Vec<(std::ops::Range<usize>, std::ops::Range<usize>)> = vec![];
        for (y, row) in mask.iter().enumerate(){
            for (x, val) in row.iter().enumerate()
            {
                if *val == true {
                    let x_start = x * BLOCK_WIDTH;
                    let x_end = x_start + BLOCK_WIDTH;
                    let y_start = y * BLOCK_HEIGHT;
                    let y_end = y_start + BLOCK_HEIGHT;
                    mask_active.push(((x_start..x_end), (y_start..y_end)));
                }
            }
        }
        
        // dbg!(mask_active);
        
        for (x_range, y_range) in mask_active.iter() {
            for x in x_range.clone() {
                for y in y_range.clone(){
                    let pixel = self.0.get_pixel_mut((x + PADDING) as u32, (y + PADDING) as u32);
                    *pixel = image::Rgb(LIGHT_BLUE);
                }
            }            
        }
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
    let mut icon = Icon::new();
    icon.func(4449);
    icon.0.save("test.png").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_mask() {}
}
