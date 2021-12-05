#[macro_use]
extern crate criterion;

use criterion::Criterion;
use homography::find_homography;
use test_utils::TestData;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("homography_kernel", |b| b.iter(|| {
        let TestData { matches, .. } = TestData::new();
        find_homography(matches).unwrap()
    }));  
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
