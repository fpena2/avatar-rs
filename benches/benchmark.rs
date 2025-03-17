use avatar_rs::Icon;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

use avatar_rs;

fn benchmark_init(c: &mut Criterion) {
    c.bench_function("icon_init", |b| b.iter(|| black_box(Icon::new(12345))));
}

fn benchmark_save(c: &mut Criterion) {
    c.bench_function("icon_save", |b| {
        b.iter(|| Icon::new(12345).save("test.png").unwrap())
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(1000);
    targets = benchmark_init, benchmark_save
}

criterion_main!(benches);
