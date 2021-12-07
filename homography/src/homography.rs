use cv_core::FeatureMatch;
use eyre::{eyre, Result};
use itertools::Itertools;
use na::Const;
use nalgebra::{self as na, Matrix3, SMatrix};
type Point2 = na::Point2<f64>;
use derive_more::{AsMut, AsRef, Deref, DerefMut, Display, From, Into};
use sample_consensus::{Estimator, Model};

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, AsMut, AsRef, Deref, DerefMut, Display, From, Into,
)]
pub struct HomographyMatrix(pub Matrix3<f64>);

impl Model<FeatureMatch<Point2>> for HomographyMatrix {
    fn residual(&self, data: &FeatureMatch<Point2>) -> f64 {
        let Self(mat) = *self;
        let FeatureMatch(a, b) = data;
        let b2 = Point2::from_homogeneous(mat * a.to_homogeneous());
        let residual = na::distance_squared(b, &b2.unwrap());
        // println!("residual {}", residual);
        residual
    }
}

pub struct HomographyEstimator {}

impl Estimator<FeatureMatch<Point2>> for HomographyEstimator {
    type Model = HomographyMatrix;
    type ModelIter = Option<HomographyMatrix>;
    const MIN_SAMPLES: usize = 4;

    fn estimate<I>(&self, data: I) -> Self::ModelIter
    where
        I: Iterator<Item = FeatureMatch<Point2>> + Clone,
    {
        let matches = data.take(Self::MIN_SAMPLES).collect_vec();
        if let Ok(homography_matrix) = find_homography(matches) {
            Some(HomographyMatrix(homography_matrix))
        } else {
            None
        }
    }
}

pub fn find_homography(matches: Vec<FeatureMatch<Point2>>) -> Result<Matrix3<f64>> {
    // TODO detect degenerate cases
    let (m1, m2): (Vec<_>, Vec<_>) = matches.iter().map(|m| (m.0, m.1)).unzip();

    let count = m1.len();
    let mut c2 = Point2::origin();
    let mut c1 = Point2::origin();

    for i in 0..count {
        c2.x += m2[i].x;
        c2.y += m2[i].y;
        c1.x += m1[i].x;
        c1.y += m1[i].y;
    }

    c2.x /= count as f64;
    c2.y /= count as f64;
    c1.x /= count as f64;
    c1.y /= count as f64;

    let mut s2 = Point2::origin();
    let mut s1 = Point2::origin();

    for i in 0..count {
        s2.x += (c2.x - m2[i].x).abs();
        s2.y += (c2.y - m2[i].y).abs();
        s1.x += (c1.x - m1[i].x).abs();
        s1.y += (c1.y - m1[i].y).abs();
    }

    if s2.x.abs() < f64::EPSILON
        || s2.y.abs() < f64::EPSILON
        || s1.x.abs() < f64::EPSILON
        || s1.y.abs() < f64::EPSILON
    {
        return Err(eyre!("Points are too close to each other"));
    }

    s2.x = count as f64 / s2.x;
    s2.y = count as f64 / s2.y;
    s1.x = count as f64 / s1.x;
    s1.y = count as f64 / s1.y;

    let inv_h_norm = Matrix3::new(1. / s2.x, 0., c2.x, 0., 1. / s2.y, c2.y, 0., 0., 1.);
    let h_norm2 = Matrix3::new(s1.x, 0., -c1.x * s1.x, 0., s1.y, -c1.y * s1.y, 0., 0., 1.);


    let mut ltl: SMatrix<f64, 9, 9> = SMatrix::zeros();
    for i in 0..count {
        let x2 = (m2[i].x - c2.x) * s2.x;
        let y2 = (m2[i].y - c2.y) * s2.y;
        let x1 = (m1[i].x - c1.x) * s1.x;
        let y1 = (m1[i].y - c1.y) * s1.y;
        let lx = [x1, y1, 1., 0., 0., 0., -x2 * x1, -x2 * y1, -x2];
        let ly = [0., 0., 0., x1, y1, 1., -y2 * x1, -y2 * y1, -y2];
        // println!("{} lx {:?} ly {:?}", i, lx, ly);
        for j in 0..9 {
            for k in 0..9 {
                ltl[(j, k)] += lx[j] * lx[k] + ly[j] * ly[k];
            }
        }
    }

    ltl.fill_lower_triangle_with_upper_triangle();
    let eigen = ltl.symmetric_eigen();

    let (eigen_vector_idx, _) = eigen.eigenvalues.argmin();
    let h0 = eigen.eigenvectors.column(eigen_vector_idx);
    let h0 = h0
        .clone_owned()
        .reshape_generic(Const::<3>, Const::<3>)
        .transpose();

    let res = (inv_h_norm * h0) * h_norm2;
    let res = res * (1.0 / res[(2, 2)]);

    Ok(res)
}

// TODO reimplement all tests from https://github.com/opencv/opencv/blob/4.x/modules/calib3d/test/test_homography.cpp
#[cfg(test)]
pub mod tests {
    use crate::find_homography;
    use approx::AbsDiffEq;
    use test_utils::TestData;

    #[test]
    fn it_works() {
        for _ in 0..24 {
            let TestData { matches, h: h_src } = TestData::new(48);
            let h = find_homography(matches).unwrap();

            let max_diff = 0.000001;
            assert!(
                h_src.abs_diff_eq(&h, max_diff),
                "absolute difference is too large"
            );
            assert!((h_src - h).norm() < max_diff, "L2 norm is too large");
        }
    }
}
