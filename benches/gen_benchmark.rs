use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn gen_mask_1(seed: u64) -> [bool; 3 * 5] {
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    let mask: [bool; 15] = (0..15)
        .map(|_| rng.random_bool(0.5))
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap_or_else(|_| panic!("Failed to convert Vec<bool> to [bool; 15]"));
    mask
}

fn gen_mask_2(seed: u64) -> [bool; 15] {
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    let mut mask = [false; 15];
    for i in 0..15 {
        mask[i] = rng.random_bool(0.5);
    }

    mask
}

fn gen_mask_3(seed: u64) -> [bool; 15] {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
    let mut mask = [false; 15];
    for i in 0..15 {
        mask[i] = rng.random_bool(0.5);
    }
    mask
}

fn benchmark_gen_mask_1(c: &mut Criterion) {
    c.bench_function("gen_mask_1", |b| {
        b.iter(|| {
            // Use black_box to prevent compiler optimizations
            black_box(gen_mask_1(444));
        });
    });
}

fn benchmark_gen_mask_2(c: &mut Criterion) {
    c.bench_function("gen_mask_2", |b| {
        b.iter(|| {
            black_box(gen_mask_2(444));
        });
    });
}

fn benchmark_gen_mask_3(c: &mut Criterion) {
    c.bench_function("gen_mask_3", |b| {
        b.iter(|| {
            black_box(gen_mask_3(444));
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(1000);
    targets = benchmark_gen_mask_1, benchmark_gen_mask_2, benchmark_gen_mask_3
}

criterion_main!(benches);
