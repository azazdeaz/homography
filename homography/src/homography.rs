use eyre::{eyre, Result};
use na::Const;
use nalgebra::{self as na, Matrix3, SMatrix};
type Point2 = na::Point2<f64>;

pub fn run_homography_kernel(m1: Vec<Point2>, m2: Vec<Point2>) -> Result<Matrix3<f64>> {
    assert!(m1.len() == m2.len());

    println!("\n\n=============================test==");

    println!("m1 {:?}", m1);
    println!("m2 {:?}", m2);

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

    println!("cm {:?}", cm);
    println!("cM {:?}", cM);

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

    println!("sm {:?}", sm);
    println!("sM {:?}", sM);

    // double invHnorm[9] = { 1./sm.x, 0, cm.x, 0, 1./sm.y, cm.y, 0, 0, 1 };
    // double Hnorm2[9] = { sM.x, 0, -cM.x*sM.x, 0, sM.y, -cM.y*sM.y, 0, 0, 1 };
    // Mat _invHnorm( 3, 3, CV_64FC1, invHnorm );
    // Mat _Hnorm2( 3, 3, CV_64FC1, Hnorm2 );
    let invHnorm = Matrix3::new(1. / sm.x, 0., cm.x, 0., 1. / sm.y, cm.y, 0., 0., 1.);
    let Hnorm2 = Matrix3::new(sM.x, 0., -cM.x * sM.x, 0., sM.y, -cM.y * sM.y, 0., 0., 1.);

    println!("invHnorm {}", invHnorm);
    println!("Hnorm2 {}", Hnorm2);

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
    println!("LtL {}", LtL);

    LtL.fill_lower_triangle_with_upper_triangle();
    println!("LtL lowup {}", LtL);
    let eigen = LtL.symmetric_eigen();

    let (eigen_vector_idx, _) = eigen.eigenvalues.argmin();
    println!("eigen_vector_idx: {}", eigen_vector_idx);
    let H0 = eigen.eigenvectors.column(eigen_vector_idx);
    let H0 = H0
        .clone_owned()
        .reshape_generic(Const::<3>, Const::<3>)
        .transpose();

        println!("eigen.eigenvectors {}", eigen.eigenvectors);
        println!("eigen.eigenvalues {}", eigen.eigenvalues);
    println!("H0 m- {}", H0);
    // let H0 = H0.resize(3, 3, 0.0);
    // let H0 = Matrix3::new(
    //     eigen.eigenvectors[(0, 2)],
    //     eigen.eigenvectors[(1, 2)],
    //     eigen.eigenvectors[(2, 2)],
    //     eigen.eigenvectors[(3, 2)],
    //     eigen.eigenvectors[(4, 2)],
    //     eigen.eigenvectors[(5, 2)],
    //     eigen.eigenvectors[(6, 2)],
    //     eigen.eigenvectors[(7, 2)],
    //     eigen.eigenvectors[(8, 2)],
    // );
    println!("H0 m {}", H0);
    let Htemp = invHnorm * H0;
    println!("Htemp {}", Htemp);
    let res = Htemp * Hnorm2;
    println!("Htemp * Hnorm2 {}", res);
    let res = res * (1.0 / res[(2, 2)]);

    println!("scaled {}", res);
    Ok(res)
}
