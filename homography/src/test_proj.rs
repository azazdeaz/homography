use nalgebra::{self as na, *};

fn main() {
    let width = 300.0;
    let height = 200.0;
    // let proj = Perspective3::new(1.0 / 1.0, 3.14 / 4.0, 1.0, 10000.0);
    // let proj2 = proj * Translation3::new(1.0, 0.0, 0.0);
    let p = Point3::new(1.0, 2.0, -10.0);
    // println!("proj {:?}", proj.project_point(&p));
    // println!("proj {:?}", proj2.project_point(&p));

    // let projection = Perspective3::new(800.0 / 600.0, 3.14/ 2.0, 1.0, 1000.0);
    // let screen_point = Point2::new(10.0f32, 20.0);
    // println!("proj {:?}", projection.project_point(&screen_point));

    let model = Isometry3::new(Vector3::x(), na::zero());

    // Our camera looks toward the point (1.0, 0.0, 0.0).
    // It is located at (0.0, 0.0, 1.0).
    let eye    = Point3::new(0.0, 0.0, 1.0);
    let target = Point3::new(0.0, 0.0, 0.0);
    let view   = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

    // A perspective projection.
    let projection = Perspective3::new(width/height, 3.14 / 2.0, 1.0, 1000.0);

    // The combination of the model with the view is still an isometry.
    let model_view = view * model;

    // Convert everything to a `Matrix4` so that they can be combined.
    let mat_model_view = model_view.to_homogeneous();

    // Combine everything.
    let model_view_projection = projection.as_matrix() * mat_model_view;

    let proj = Perspective3::from_matrix_unchecked(model_view_projection);
    let pp = proj.project_point(&p);

    let x = (pp.coords.x + 1.0) * width / 2.0;
    let y = (pp.coords.y + 1.0) * height / 2.0;

    println!("proj {:?} x:{} y:{}", pp, x, y);
}