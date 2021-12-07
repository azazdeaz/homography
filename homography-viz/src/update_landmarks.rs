use crate::components::{Landmark3, Landmarks3, Plane};
use bevy::prelude::*;
use nalgebra::{Isometry3, Point3, Vector3};

pub fn update_landmarks(
    mut commands: Commands,
    planes: Query<&Plane>,
    mut landmarks: Query<&mut Landmarks3>,
) {
    if let Ok(mut landmarks) = landmarks.single_mut() {
        landmarks.clear();
        for plane in planes.iter() {
            let axisangle = Vector3::y() * plane.rot_y;
            let translation = Vector3::new(plane.x, plane.y, plane.z);
            let transform = Isometry3::new(translation, axisangle);

            for xi in 0..plane.points_x {
                for yi in 0..plane.points_y {
                    let point_id = xi * plane.points_y + yi;
                    let point = Point3::new(
                        -plane.width / 2.0
                            + (plane.width / ((plane.points_x - 1) as f32)) * xi as f32,
                        -plane.height / 2.0
                            + (plane.height / ((plane.points_y - 1) as f32)) * yi as f32,
                        -10.0,
                    );
                    let point = transform * point;
                    let id = format!("{}_{}", plane.id, point_id);
                    landmarks.push(Landmark3 { id, point });
                }
            }
        }
    } else {
        // TODO there must be a way to upsert this
        commands.spawn().insert(Landmarks3::default());
    }
}
