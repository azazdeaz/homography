use arrsac::Arrsac;
use cv_core::FeatureMatch;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use sample_consensus::Consensus;

use crate::{HomographyEstimator, HomographyMatrix};

type Point2 = nalgebra::Point2<f64>;

/// Find homography with the [ARRSAC](https://docs.rs/arrsac/latest/arrsac/) sample consensus algorithm.  
/// *This is supported on **crate feature `arrsac-sc`** only.*
pub fn find_homography_with_arrsac(matches: &[FeatureMatch<Point2>]) -> Option<HomographyMatrix> {
    let mut arrsac = Arrsac::new(0.1, Pcg64::from_seed([1; 32]));
    let estimator = HomographyEstimator {};
    // TODO shuffle matches?
    arrsac.model(&estimator, matches.iter().cloned())
}
