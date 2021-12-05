use cv_core::FeatureMatch;
use itertools::{zip, Itertools};
use nalgebra::{Matrix3, Point2};
use rand::Rng;
use std::f64::consts::PI;

pub struct TestData {
    pub matches: Vec<FeatureMatch<Point2<f64>>>,
    pub h: Matrix3<f64>,
}

impl TestData {
    pub fn new(match_count: usize) -> Self {
        let img_size = 100.0;
        let mut rng = rand::thread_rng();
        let src = (0..match_count)
            .map(|_| {
                Point2::new(rng.gen_range(0.0..img_size), rng.gen_range(0.0..img_size))
                    .to_homogeneous()
            })
            .collect_vec();
        let fi = rng.gen_range(0.0..PI * 2.0);
        let tx = rng.gen_range(0.0..f64::sqrt(img_size));
        let ty = rng.gen_range(0.0..f64::sqrt(img_size));
        #[rustfmt::skip]
        let h = Matrix3::new(
            f64::cos(fi), -f64::sin(fi), tx,
            f64::sin(fi), f64::cos(fi), ty,
            0.0, 0.0, 1.0
        );
        let dst = src.iter().map(|p| h * p).collect_vec();
        let matches = zip(src, dst)
            .map(|(a, b)| {
                FeatureMatch(
                    Point2::from_homogeneous(a).unwrap(),
                    Point2::from_homogeneous(b).unwrap(),
                )
            })
            .collect_vec();
        Self { matches, h }
    }
}
