#[macro_use]
extern crate criterion;

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use homography::find_homography;
use test_utils::TestData;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("kernel");
    for matches in [4, 16, 64, 256].iter() {
        group.throughput(Throughput::Elements(*matches as u64));
        group.bench_with_input(BenchmarkId::from_parameter(matches), matches, |b, &matches| {
            b.iter(|| {
                let TestData { matches, .. } = TestData::new(matches);
                find_homography(matches).unwrap()
            });
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
