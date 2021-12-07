use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use itertools::Itertools;
use nalgebra::Point3;

use crate::{
    components::{Camera, Landmark2, Landmarks3, Plane},
    utils,
};

pub struct CamerasAndPlanes3D;

impl Plugin for CamerasAndPlanes3D {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_planes.system())
            .add_startup_system(add_cameras.system())
            .add_system(update_plane_meshes.system())
            .add_system(update_camera_meshes.system());
    }
}

fn add_cameras(mut commands: Commands) {
    for i in 0..2 {
        let x = if i == 0 { -10.0 } else { 10.0 };
        let camera = Camera {
            fovy: 0.65,
            z: 22.0,
            x,
            ..Default::default()
        };
        commands
            .spawn()
            .insert(camera)
            .insert(Vec::<Landmark2>::default());
    }
}

fn add_planes(mut commands: Commands) {
    let plane = Plane {
        id: 1,
        width: 10.0,
        height: 10.0,
        points_x: 5,
        points_y: 5,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        rot_x: 0.0,
        rot_y: 0.0,
        rot_z: 0.0,
    };

    commands.spawn().insert(plane);
}

fn update_camera_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cameras: Query<(&Camera, Option<&Handle<Mesh>>, Entity)>,
) {
    for (camera, mesh, entity) in cameras.iter_mut() {
        let inv_model_view_projection = camera
            .model_view_projection()
            .try_inverse()
            .expect("cant invert projection matrix");

        // cube corners as in https://www.cs.bham.ac.uk/~slb/courses/Graphics/g63.html
        let corners = [
            &Point3::new(-1.0, 1.0, -1.0),
            &Point3::new(1.0, 1.0, -1.0),
            &Point3::new(1.0, -1.0, -1.0),
            &Point3::new(-1.0, -1.0, -1.0),
            &Point3::new(-1.0, 1.0, 1.0),
            &Point3::new(1.0, 1.0, 1.0),
            &Point3::new(1.0, -1.0, 1.0),
            &Point3::new(-1.0, -1.0, 1.0),
        ]
        .iter()
        .map(|p| inv_model_view_projection * p.to_homogeneous())
        .map(|h| Point3::from_homogeneous(h).unwrap())
        .map(|v| [v.coords.x, v.coords.y, v.coords.z])
        .collect_vec();
        let mut vertices = vec![
            corners[0], corners[1], corners[1], corners[2], corners[2], corners[3], corners[3],
            corners[0], corners[4], corners[5], corners[5], corners[6], corners[6], corners[7],
            corners[7], corners[4], corners[0], corners[4], corners[1], corners[5], corners[2],
            corners[6], corners[3], corners[7],
        ];
        vertices.append(&mut utils::cross_lines(&camera.eye(), 1.0));
        vertices.append(&mut utils::cross_lines(&camera.target(), 1.0));

        if let Some(mesh) = mesh {
            let mesh = meshes.get_mut(mesh).unwrap();
            fill_mesh_with_vertices(mesh, vertices);
        } else {
            let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
            fill_mesh_with_vertices(&mut mesh, vertices);
            let pbr = PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                ..Default::default()
            };
            commands.entity(entity).insert_bundle(pbr);
        }
    }
}

fn update_plane_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut landmarks: Query<(
        &Landmarks3,
        Entity,
        Option<&Handle<Mesh>>,
        Option<&Children>,
    )>,
    mut transforms: Query<&mut Transform>,
) {
    for (landmarks, entity, viz, children) in landmarks.iter_mut() {
        if viz.is_none() {
            commands.entity(entity).insert_bundle(PbrBundle {
                ..Default::default()
            });
        }

        let vertices = landmarks
            .iter()
            .map(|lm| [lm.point.coords.x, lm.point.coords.y, lm.point.coords.z])
            .collect_vec();

        let empty_children = Children::default();
        let children = if let Some(children) = children {
            children
        } else {
            &empty_children
        };

        for (i, v) in vertices.into_iter().enumerate() {
            if let Some(child) = children.get(i) {
                let transform = transforms.get_mut(*child);
                if let Ok(mut transform) = transform {
                    transform.translation = v.into();
                }
            } else {
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Icosphere {
                            radius: 0.5,
                            subdivisions: 2,
                        })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::PINK,
                            ..Default::default()
                        }),
                        transform: Transform::from_xyz(v[0], v[1], v[2]),
                        ..Default::default()
                    })
                    .insert(Parent(entity));
            }
        }

        if children.len() > landmarks.len() {
            for unused in &children[landmarks.len()..] {
                commands.entity(*unused).despawn_recursive();
            }
        }
    }
}

fn fill_mesh_with_vertices(mesh: &mut Mesh, vertices: Vec<[f32; 3]>) {
    let mut normals = Vec::new();
    normals.resize(vertices.len(), [0.0, 0.0, 0.0]);
    let mut uvs = Vec::new();
    uvs.resize(vertices.len(), [0.0, 0.0]);
    let indices = (0..vertices.len() as u32).collect_vec();

    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
}
