use image::ImageBuffer;
use image::{Rgb, RgbImage};
use rand::{Rng, SeedableRng};

// NOTE: Add a 20px padding to the final image
const IMG_HEIGHT: usize = 400;
const IMG_WIDTH: usize = 400;
const MASK_HEIGHT: usize = IMG_HEIGHT / 80;
const MASK_WIDTH: usize = IMG_WIDTH / 80;
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
                    let x_start = x * 80;
                    let x_end = x_start + 80;

                    let y_start = y * 80;
                    let y_end = y_start + 80;
                    mask_active.push(((x_start..x_end), (y_start..y_end)));
                }
            }
        }
        
        // dbg!(mask_active);
        
        for (x_range, y_range) in mask_active.iter() {
            for x in x_range.clone() {
                for y in y_range.clone(){
                    let pixel = self.0.get_pixel_mut(x as u32, y as u32);
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
