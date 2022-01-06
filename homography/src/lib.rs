//! Homography matrix estimator based on the [OpenCV implementation](https://github.com/opencv/opencv/blob/68d15fc62edad980f1ffa15ee478438335f39cc3/modules/calib3d/src/fundam.cpp#L104-L183)
//!
//! # Usage:
//!
//! ```
//! # use nalgebra::{ Point2, Matrix3 };
//! # use cv_core::FeatureMatch;
//! # use approx::AbsDiffEq;
//! # use crate::homography::find_homography;
//! // Four point pairs, representing a shift to the right
//! let matches = vec![
//!     FeatureMatch(Point2::new(0.0, 0.0), Point2::new(0.0, 2.0)),
//!     FeatureMatch(Point2::new(1.0, 1.0), Point2::new(1.0, 3.0)),
//!     FeatureMatch(Point2::new(2.0, 4.0), Point2::new(2.0, 6.0)),
//!     FeatureMatch(Point2::new(7.0, 3.0), Point2::new(7.0, 5.0)),
//! ];
//! 
//! // Estimate the homography
//! let result = find_homography(matches).unwrap();
//! 
//! let expected = Matrix3::new(1.0, 0.0, 0.0,
//!                             0.0, 1.0, 2.0,
//!                             0.0, 0.0, 1.0);
//! 
//! // The result is close to the expected homography
//! assert!( result.abs_diff_eq(&expected, 0.0001));
//! ```

mod homography;

pub use crate::homography::*;

#[cfg(feature = "arrsac-sc")]
mod homography_with_arrsac;
#[cfg(feature = "arrsac-sc")]
pub use crate::homography_with_arrsac::*;
