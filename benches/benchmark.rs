use avatar_rs::{BLOCK_WIDTH, BLUE_COLOR, GRAY_COLOR, IMG_HEIGHT, IMG_WIDTH, NUM_BLOCKS, PADDING};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use image::{Rgb, RgbImage};
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

use avatar_rs;

pub fn draw_2(seed: u64, canvas: &mut RgbImage) {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
    let block_types: Vec<(Option<Rgb<u8>>, Option<Rgb<u8>>, Option<Rgb<u8>>)> = (0..NUM_BLOCKS)
        .map(|_| {
            (
                rng.random_bool(0.5).then(|| BLUE_COLOR),
                rng.random_bool(0.5).then(|| BLUE_COLOR),
                rng.random_bool(0.5).then(|| BLUE_COLOR),
            )
        })
        .collect();

    fn index(i: u32) -> usize {
        match i {
            x if x <= PADDING + 70 => 0,
            x if x >= PADDING + 70 && x <= PADDING + 140 => 1,
            x if x >= PADDING + 140 && x <= PADDING + 210 => 2,
            x if x >= PADDING + 210 && x <= PADDING + 280 => 3,
            x if x >= PADDING + 280 && x <= PADDING + 350 => 4,
            x if x >= PADDING + 350 && x <= PADDING + 420 => 5,
            _ => unimplemented!(),
        }
    }

    canvas
        .par_enumerate_pixels_mut()
        .filter(|&(x, y, _)| {
            x > PADDING && x < IMG_WIDTH - PADDING && y > PADDING && y < IMG_HEIGHT - PADDING
        })
        .for_each(|(x, y, pixel)| {
            if let Some((a, b, c)) = block_types.get(index(y)) {
                if let Some(color) = a {
                    if x > PADDING + BLOCK_WIDTH * 0 && x < PADDING + BLOCK_WIDTH * 1 {
                        *pixel = *color;
                    }
                    if x > PADDING + BLOCK_WIDTH * 4 && x < PADDING + BLOCK_WIDTH * 5 {
                        *pixel = *color;
                    }
                }
                if let Some(color) = b {
                    if x >= PADDING + BLOCK_WIDTH * 1 && x <= PADDING + BLOCK_WIDTH * 2 {
                        *pixel = *color;
                    }
                    if x >= PADDING + BLOCK_WIDTH * 3 && x <= PADDING + BLOCK_WIDTH * 4 {
                        *pixel = *color;
                    }
                }
                if let Some(color) = c {
                    if x >= PADDING + BLOCK_WIDTH * 2 && x <= PADDING + BLOCK_WIDTH * 3 {
                        *pixel = *color;
                    }
                }
            }
        });
}

pub fn draw_1(seed: u64, canvas: &mut RgbImage) {
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

fn benchmark_draw_1(c: &mut Criterion) {
    let mut canvas = RgbImage::from_pixel(IMG_WIDTH, IMG_HEIGHT, GRAY_COLOR);
    c.bench_function("draw_1", |b| b.iter(|| draw_1(black_box(42), &mut canvas)));
}

fn benchmark_draw_2(c: &mut Criterion) {
    let mut canvas = RgbImage::from_pixel(IMG_WIDTH, IMG_HEIGHT, GRAY_COLOR);
    c.bench_function("draw_2", |b| b.iter(|| draw_2(black_box(42), &mut canvas)));
}
criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(1000);
    targets = benchmark_draw_1, benchmark_draw_2
}

criterion_main!(benches);
