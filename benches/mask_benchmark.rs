use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rayon::prelude::*;
use std::ops::Range;

fn expanded_1(
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

fn expanded_2(
    matrix: [[bool; 5]; 5],
    new_width: usize,
    new_height: usize,
) -> Vec<(Range<usize>, Range<usize>)> {
    matrix
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &val)| val)
                .map(move |(x, _)| {
                    let x_start = x * new_width;
                    let x_end = x_start + new_width;
                    let y_start = y * new_height;
                    let y_end = y_start + new_height;
                    (x_start..x_end, y_start..y_end)
                })
        })
        .collect()
}

fn expanded_3(
    matrix: [[bool; 5]; 5],
    new_width: usize,
    new_height: usize,
) -> Vec<(Range<usize>, Range<usize>)> {
    matrix
        .par_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.par_iter()
                .enumerate()
                .filter(|&(_, &val)| val)
                .map(move |(x, _)| {
                    let x_start = x * new_width;
                    let x_end = x_start + new_width;
                    let y_start = y * new_height;
                    let y_end = y_start + new_height;
                    (x_start..x_end, y_start..y_end)
                })
        })
        .collect()
}

fn benchmark_expanded_1(c: &mut Criterion) {
    c.bench_function("benchmark_expanded_1", |b| {
        b.iter(|| {
            black_box(expanded_1(MATRIX, 70, 70));
        });
    });
}

fn benchmark_expanded_2(c: &mut Criterion) {
    c.bench_function("benchmark_expanded_2", |b| {
        b.iter(|| {
            black_box(expanded_2(MATRIX, 70, 70));
        });
    });
}

fn benchmark_expanded_3(c: &mut Criterion) {
    c.bench_function("benchmark_expanded_2", |b| {
        b.iter(|| {
            black_box(expanded_3(MATRIX, 70, 70));
        });
    });
}

const MATRIX: [[bool; 5]; 5] = [
    [true, false, false, false, true],
    [false, true, true, true, false],
    [false, true, false, true, false],
    [false, false, false, false, false],
    [false, false, false, false, false],
];

criterion_group! {
name = benches;
config = Criterion::default().sample_size(1000);
targets = benchmark_expanded_1, benchmark_expanded_2, benchmark_expanded_3}

criterion_main!(benches);
