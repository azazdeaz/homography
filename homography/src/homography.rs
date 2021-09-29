use eyre::{eyre, Result};
use nalgebra::{self as na, Matrix3, SMatrix};
type Point2 = na::Point2<f64>;

pub fn run_homography_kernel(m1: Vec<Point2>, m2: Vec<Point2>) -> Result<Matrix3<f64>> {
    assert!(m1.len() == m2.len());

    let count = m1.len();
    let mut cm = Point2::origin();
    let mut cM = Point2::origin();

    for i in 0..count {
        cm.x += m1[i].coords.x;
        cm.y += m1[i].coords.y;
        cM.x += m2[i].coords.x;
        cM.y += m2[i].coords.y;
    }

    cm.x /= count as f64;
    cm.y /= count as f64;
    cM.x /= count as f64;
    cM.y /= count as f64;

    let mut sm = Point2::origin();
    let mut sM = Point2::origin();

    for i in 0..count {
        sm.x += (cm.coords.x - m1[i].coords.x).abs();
        sm.y += (cm.coords.y - m1[i].coords.y).abs();
        sM.x += (cM.coords.x - m2[i].coords.x).abs();
        sM.y += (cM.coords.y - m2[i].coords.y).abs();
    }

    if sm.x.abs() < f64::EPSILON
        || sm.y.abs() < f64::EPSILON
        || sM.x.abs() < f64::EPSILON
        || sM.y.abs() < f64::EPSILON
    {
        return Err(eyre!("Points are too close to each other"));
    }

    sm.x /= count as f64;
    sm.y /= count as f64;
    sM.x /= count as f64;
    sM.y /= count as f64;

    // double invHnorm[9] = { 1./sm.x, 0, cm.x, 0, 1./sm.y, cm.y, 0, 0, 1 };
    // double Hnorm2[9] = { sM.x, 0, -cM.x*sM.x, 0, sM.y, -cM.y*sM.y, 0, 0, 1 };
    // Mat _invHnorm( 3, 3, CV_64FC1, invHnorm );
    // Mat _Hnorm2( 3, 3, CV_64FC1, Hnorm2 );
    let invHnorm = Matrix3::new(1. / sm.x, 0., cm.x, 0., 1. / sm.y, cm.y, 0., 0., 1.);
    let Hnorm2 = Matrix3::new(sM.x, 0., -cM.x * sM.x, 0., sM.y, -cM.y * sM.y, 0., 0., 1.);

    let mut LtL: SMatrix<f64, 9, 9> = SMatrix::zeros();
    for i in 0..count {
        let x = (m1[i].coords.x - cm.coords.x) * sm.coords.x;
        let y = (m1[i].coords.y - cm.coords.y) * sm.coords.y;
        let X = (m2[i].coords.x - cM.coords.x) * sM.coords.x;
        let Y = (m2[i].coords.y - cM.coords.y) * sM.coords.y;
        let Lx = [X, Y, 1., 0., 0., 0., -x * X, -x * Y, -x];
        let Ly = [X, Y, 1., 0., 0., 0., -x * X, -x * Y, -x];
        for j in 0..9 {
            for k in 0..9 {
                LtL[(j, k)] += Lx[j] * Lx[k] + Ly[j] * Ly[k];
            }
        }
    }
    
    LtL.fill_lower_triangle_with_upper_triangle();
    let eigen = LtL.symmetric_eigen();
    let H0 = eigen.eigenvectors.fixed_rows::<1>(8);
    let H0 = H0.resize(3, 3, 0.0);
    let Htemp = invHnorm * H0;
    Ok(Htemp * Hnorm2)
}
