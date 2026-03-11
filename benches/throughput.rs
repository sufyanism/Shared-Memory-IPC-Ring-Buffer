use criterion::{criterion_group, criterion_main, Criterion};

fn bench_ring(c: &mut Criterion) {
    c.bench_function("shm_ring_dummy", |b| {
        b.iter(|| {
            let mut x = 0;
            x += 1;
        })
    });
}

criterion_group!(benches, bench_ring);
criterion_main!(benches);
