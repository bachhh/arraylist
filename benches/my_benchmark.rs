use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::time::Duration;

fn bench_array(c: &mut Criterion) {
    let mut group = c.benchmark_group("array");

    let total = 27;
    for n in 1..total {
        let i = 2i32.pow(n);
        group.bench_function(BenchmarkId::new("array", n), move |b| {
            b.iter_batched(
                || {
                    let v: Vec<i64> = vec![0; i as usize];
                    v
                },
                |mut vec: Vec<i64>| {
                    for _ in 1..2i32.pow(total - n) {
                        for v in vec.iter_mut() {
                            *v += 1;
                        }
                    }
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group! {
  name = benches;
  config = Criterion::default().measurement_time(Duration::from_secs(30)).sample_size(20);
  targets = bench_array
}
// criterion_group!(benches, bench_array);
criterion_main!(benches);
