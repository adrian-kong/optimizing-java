use criterion::{criterion_group, criterion_main, Criterion};

const SIZE: usize = 2 * 1024 * 1024;

fn memory_access(step: usize) {
    let mut array = vec![0; SIZE].into_boxed_slice();
    for i in (0..SIZE).step_by(step) {
        array[i] += 1;
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mem 1", |b| b.iter(|| memory_access(1)));
    c.bench_function("mem 2", |b| b.iter(|| memory_access(2)));
    c.bench_function("mem 16", |b| b.iter(|| memory_access(16)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);