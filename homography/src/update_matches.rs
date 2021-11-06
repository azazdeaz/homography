use crate::components::{Camera, Landmarks3};
use bevy::prelude::*;
use itertools::Itertools;
use nalgebra::{self as na, Isometry3, Perspective3, Point2, Point3, Vector3};
use rand::Rng;

fn render_points(camera: &Camera, landmarks: &Landmarks3) -> Vec<Point2<f32>> {
    let mut rng = rand::thread_rng();
    let model = Isometry3::new(Vector3::x(), na::zero());

    // Our camera looks toward the point (1.0, 0.0, 0.0).
    // It is located at (0.0, 0.0, 1.0).
    let eye = Point3::new(camera.x, camera.y, camera.z);
    let target = Point3::new(camera.target_x, camera.target_y, camera.target_z);
    let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

    // A perspective projection.
    let projection = Perspective3::new(
        camera.width / camera.height,
        camera.fovy,
        camera.znear,
        camera.zfar,
    );

    // The combination of the model with the view is still an isometry.
    let model_view = view * model;

    // Convert everything to a `Matrix4` so that they can be combined.
    let mat_model_view = model_view.to_homogeneous();

    // Combine everything.
    // let translation = Translation3::new(camera.x, camera.y, camera.z).to_homogeneous();
    let model_view_projection = projection.as_matrix() * mat_model_view;

    landmarks
        .iter()
        .filter_map(|lm| {
            let point = model_view_projection * lm.point.to_homogeneous();
            let point = Point3::from_homogeneous(point);

            if let Some(point) = point {
                // is inside the visibility box
                if point.coords.x.abs() <= 1.0
                    && point.coords.y.abs() <= 1.0
                    && point.coords.z.abs() <= 1.0
                {
                    let x = (point.coords.x + 1.0) * camera.width / 2.0 + (rng.gen::<f32>() * 1.0);
                    let y = (point.coords.y + 1.0) * camera.height / 2.0 + (rng.gen::<f32>() * 1.0);
                    return Some(Point2::new(x, y));
                }
            }
            None
        })
        .collect_vec()
}

pub fn render_landmarks(
    mut commands: Commands,
    mut cameras: Query<(&Camera, Option<&mut Vec<na::Point2<f32>>>, Entity)>,
    mut landmarks: Query<&Landmarks3>,
) {
    if let (Ok(mut landmarks), Some((mut cam1, mut cam2))) =
        (landmarks.single(), cameras.iter_mut().collect_tuple())
    {
        let points1 = render_points(cam1.0, landmarks);
        if let (_, Some(mut points), _) = cam1 {
            *points = points1;
        } else {
            commands.entity(cam1.2).insert(points1);
        }
        let points2 = render_points(cam2.0, landmarks);
        if let (_, Some(mut points), _) = cam2 {
            *points = points2;
        } else {
            commands.entity(cam2.2).insert(points2);
        }
    }
}
