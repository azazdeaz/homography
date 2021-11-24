use cv_core::FeatureMatch;
use eyre::{eyre, Result};
use itertools::Itertools;
use na::Const;
use nalgebra::{self as na, Matrix3, SMatrix};
type Point2 = na::Point2<f64>;
use arrsac::Arrsac;
use derive_more::{AsMut, AsRef, Deref, DerefMut, Display, From, Into};
use rand::SeedableRng;
use rand_pcg::Pcg64;
use sample_consensus::{Consensus, Estimator, Model};

pub fn find_homography(matches: &Vec<FeatureMatch<Point2>>) -> Option<HomographyMatrix> {
    let mut arrsac = Arrsac::new(0.1, Pcg64::from_seed([1; 32]));
    let estimator = HomographyEstimator {};
    // TODO shuffle matches?
    arrsac.model(&estimator, matches.iter().cloned())
}

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
        if let Ok(homography_matrix) = run_homography_kernel(matches) {
            Some(HomographyMatrix(homography_matrix))
        } else {
            None
        }
    }
}

pub fn run_homography_kernel(matches: Vec<FeatureMatch<Point2>>) -> Result<Matrix3<f64>> {
    // TODO detect degenerate cases
    let (m1, m2): (Vec<_>, Vec<_>) = matches.iter().map(|m| (m.0, m.1)).unzip();

    let count = m1.len();
    let mut cm = Point2::origin();
    let mut cM = Point2::origin();

    for i in 0..count {
        cm.x += m2[i].coords.x;
        cm.y += m2[i].coords.y;
        cM.x += m1[i].coords.x;
        cM.y += m1[i].coords.y;
    }

    cm.x /= count as f64;
    cm.y /= count as f64;
    cM.x /= count as f64;
    cM.y /= count as f64;

    // println!("cm {:?}", cm);
    // println!("cM {:?}", cM);

    let mut sm = Point2::origin();
    let mut sM = Point2::origin();

    for i in 0..count {
        sm.x += (cm.coords.x - m2[i].coords.x).abs();
        sm.y += (cm.coords.y - m2[i].coords.y).abs();
        sM.x += (cM.coords.x - m1[i].coords.x).abs();
        sM.y += (cM.coords.y - m1[i].coords.y).abs();
    }

    if sm.x.abs() < f64::EPSILON
        || sm.y.abs() < f64::EPSILON
        || sM.x.abs() < f64::EPSILON
        || sM.y.abs() < f64::EPSILON
    {
        return Err(eyre!("Points are too close to each other"));
    }

    sm.x = count as f64 / sm.x;
    sm.y = count as f64 / sm.y;
    sM.x = count as f64 / sM.x;
    sM.y = count as f64 / sM.y;

    // println!("sm {:?}", sm);
    // println!("sM {:?}", sM);

    // double invHnorm[9] = { 1./sm.x, 0, cm.x, 0, 1./sm.y, cm.y, 0, 0, 1 };
    // double Hnorm2[9] = { sM.x, 0, -cM.x*sM.x, 0, sM.y, -cM.y*sM.y, 0, 0, 1 };
    // Mat _invHnorm( 3, 3, CV_64FC1, invHnorm );
    // Mat _Hnorm2( 3, 3, CV_64FC1, Hnorm2 );
    let invHnorm = Matrix3::new(1. / sm.x, 0., cm.x, 0., 1. / sm.y, cm.y, 0., 0., 1.);
    let Hnorm2 = Matrix3::new(sM.x, 0., -cM.x * sM.x, 0., sM.y, -cM.y * sM.y, 0., 0., 1.);

    // println!("invHnorm {}", invHnorm);
    // println!("Hnorm2 {}", Hnorm2);

    let mut LtL: SMatrix<f64, 9, 9> = SMatrix::zeros();
    for i in 0..count {
        let x = (m2[i].coords.x - cm.coords.x) * sm.coords.x;
        let y = (m2[i].coords.y - cm.coords.y) * sm.coords.y;
        let X = (m1[i].coords.x - cM.coords.x) * sM.coords.x;
        let Y = (m1[i].coords.y - cM.coords.y) * sM.coords.y;
        let Lx = [X, Y, 1., 0., 0., 0., -x * X, -x * Y, -x];
        let Ly = [0., 0., 0., X, Y, 1., -y * X, -y * Y, -y];
        // println!("{} Lx {:?} Ly {:?}", i, Lx, Ly);
        for j in 0..9 {
            for k in 0..9 {
                LtL[(j, k)] += Lx[j] * Lx[k] + Ly[j] * Ly[k];
            }
        }
    }
    // println!("LtL {}", LtL);

    LtL.fill_lower_triangle_with_upper_triangle();
    // println!("LtL lowup {}", LtL);
    let eigen = LtL.symmetric_eigen();

    let (eigen_vector_idx, _) = eigen.eigenvalues.argmin();
    // println!("eigen_vector_idx: {}", eigen_vector_idx);
    let H0 = eigen.eigenvectors.column(eigen_vector_idx);
    let H0 = H0
        .clone_owned()
        .reshape_generic(Const::<3>, Const::<3>)
        .transpose();

    // println!("H0 m {}", H0);
    let Htemp = invHnorm * H0;
    // println!("Htemp {}", Htemp);
    let res = Htemp * Hnorm2;
    // println!("Htemp * Hnorm2 {}", res);
    let res = res * (1.0 / res[(2, 2)]);

    // println!("scaled {}", res);
    Ok(res)
}
// TODO reimplement tests from https://github.com/opencv/opencv/blob/4.x/modules/calib3d/test/test_homography.cpp

#[cfg(test)]
mod tests {
    use crate::{HomographyMatrix, homography::Point2, run_homography_kernel};
    use approx::AbsDiffEq;
    use cv_core::FeatureMatch;
    use itertools::{zip, Itertools};
    use nalgebra::Matrix3;
    use rand::Rng;
    use std::f64::consts::PI;

    #[test]
    fn it_works() {
        let img_size = 100.0;
        let mut rng = rand::thread_rng();
        let src = (0..100)
            .map(|_| {
                Point2::new(rng.gen_range(0.0..img_size), rng.gen_range(0.0..img_size))
                    .to_homogeneous()
            })
            .collect_vec();
        let fi = rng.gen_range(0.0..PI * 2.0);
        let tx = rng.gen_range(0.0..f64::sqrt(img_size));
        let ty = rng.gen_range(0.0..f64::sqrt(img_size));
        #[rustfmt::skip]
        let h_src = Matrix3::new(
            f64::cos(fi), -f64::sin(fi), tx,
            f64::sin(fi), f64::cos(fi), ty,
            0.0, 0.0, 1.0
        );
        let dst = src.iter().map(|p| h_src * p).collect_vec();
        let matches = zip(src, dst)
            .map(|(a, b)| {
                FeatureMatch(
                    Point2::from_homogeneous(a).unwrap(),
                    Point2::from_homogeneous(b).unwrap(),
                )
            })
            .collect_vec();
        let h= run_homography_kernel(matches).unwrap();
        // TODO implement cvtest::norm from opencv modules/ts/src/ts_func.cpp
        
        let max_diff = 0.000001;
        assert!(h_src.abs_diff_eq(&h, max_diff), "absolute difference is too large");
        assert!((h_src - h).norm() < max_diff, "L2 norm is too large");
    }
}
