use avatar_rs::Icon;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn benchmark_icon_new(c: &mut Criterion) {
    c.bench_function("Icon::new", |b| b.iter(|| Icon::new(black_box(1253))));
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(1000);
    targets = benchmark_icon_new
}

criterion_main!(benches);
