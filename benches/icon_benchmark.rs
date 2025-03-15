use criterion::{Criterion, black_box, criterion_group, criterion_main};
use image::{Pixel, Rgb, RgbImage};
use rayon::prelude::*;
use std::ops::Range;

const MATRIX: [[bool; 5]; 5] = [
    [true, false, false, false, true],
    [false, true, true, true, false],
    [false, true, false, true, false],
    [false, false, false, false, false],
    [false, false, false, false, false],
];

fn expanded_dummy_impl(
    matrix: [[bool; 5]; 5],
    new_width: usize,
    new_height: usize,
) -> Vec<(Range<usize>, Range<usize>)> {
    let mut active_ranges: Vec<(Range<usize>, Range<usize>)> = vec![];

    for (y, row) in matrix.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val {
                let x_start = x * new_width;
                let x_end = x_start + new_width;
                let y_start = y * new_height;
                let y_end = y_start + new_height;
                active_ranges.push((x_start..x_end, y_start..y_end));
            }
        }
    }
    active_ranges
}

fn draw_1(mut img: RgbImage, active_ranges: &Vec<(Range<usize>, Range<usize>)>) {
    for (x_range, y_range) in active_ranges {
        for y in y_range.start..y_range.end {
            for x in x_range.start..x_range.end {
                let pixel = img.get_pixel_mut(x as u32, y as u32);
                *pixel = Rgb([240, 240, 240]);
            }
        }
    }
}

fn benchmark_draw_1(c: &mut Criterion) {
    let active_ranges = expanded_dummy_impl(MATRIX, 70, 70);
    let img = RgbImage::new(400, 400);
    c.bench_function("benchmark_draw_1", |b| {
        b.iter(|| {
            black_box(draw_1(img.clone(), &active_ranges));
        });
    });
}

// fn benchmark_icon_new(c: &mut Criterion) {
//     c.bench_function("Icon::new", |b| b.iter(|| Icon::new(black_box(1253))));
// }

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(1000);
    targets = benchmark_draw_1,
}

criterion_main!(benches);
