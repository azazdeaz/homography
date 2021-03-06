use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use itertools::Itertools;
use nalgebra::Point3;

use crate::components::{Camera, Landmarks3};

pub struct CamerasAndPlanes3D;

impl Plugin for CamerasAndPlanes3D {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_light.system())
            .add_system(update_plane_meshes.system())
            .add_system(update_camera_meshes.system())
            .add_system(update_camera_models.system())
            .add_system(update_arrow_models.system());
    }
}
struct CameraModel {}

fn init_camera_models(mut commands: EntityCommands, asset_server: &Res<AssetServer>) {
    // note that we have to include the `Scene0` label
    let my_gltf = asset_server.load(
        "/home/azazdeaz/repos/test/rust/homography/homography-viz/assets/models/camera.glb#Scene0",
    );

    commands.with_children(|parent| {
        parent
            .spawn_bundle((
                CameraModel {},
                Transform::from_xyz(0.0, 0.0, 0.0),
                GlobalTransform::identity(),
            ))
            .with_children(|parent| {
                parent
                    .spawn_bundle((
                        Transform::from_scale(Vec3::splat(0.8)),
                        GlobalTransform::identity(),
                    ))
                    .with_children(|parent| {
                        parent.spawn_scene(my_gltf);
                    });
            });
    });
}

fn update_camera_models(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cameras: Query<(&Camera, Option<&Children>, Entity)>,
    mut models: Query<(&mut Transform, With<CameraModel>)>,
) {
    for (camera, children, entity) in cameras.iter() {
        let model_entity = if let Some(children) = children {
            children
                .iter()
                .find(|&&child| models.get_mut(child).is_ok())
        } else {
            None
        };
        if let Some(&model_entity) = model_entity {
            let (mut transform, _) = models.get_mut(model_entity).unwrap();
            transform.translation.x = camera.x;
            transform.translation.y = camera.y;
            transform.translation.z = camera.z;
            transform.look_at(
                Vec3::new(camera.target_x, camera.target_y, camera.target_z),
                Vec3::Y,
            );
        } else {
            init_camera_models(commands.entity(entity), &asset_server);
        }
    }
}

struct ArrowModel {}

fn init_arrow_models(mut commands: EntityCommands, asset_server: &Res<AssetServer>) {
    // note that we have to include the `Scene0` label
    let my_gltf = asset_server.load(
        "/home/azazdeaz/repos/test/rust/homography/homography-viz/assets/models/arrow.glb#Scene0",
    );

    commands.with_children(|parent| {
        parent
            .spawn_bundle((
                ArrowModel {},
                Transform::from_xyz(0.0, 0.0, 0.0),
                GlobalTransform::identity(),
            ))
            .with_children(|parent| {
                parent
                    .spawn_bundle((
                        Transform::from_scale(Vec3::splat(0.8))
                            * Transform::from_rotation(Quat::from_rotation_x(
                                std::f32::consts::FRAC_PI_2,
                            )),
                        GlobalTransform::identity(),
                    ))
                    .with_children(|parent| {
                        parent.spawn_scene(my_gltf);
                    });
            });
    });
}

fn update_arrow_models(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cameras: Query<(&Camera, Option<&Children>, Entity)>,
    mut models: Query<(&mut Transform, With<ArrowModel>)>,
) {
    for (camera, children, entity) in cameras.iter() {
        let model_entity = if let Some(children) = children {
            children
                .iter()
                .find(|&&child| models.get_mut(child).is_ok())
        } else {
            None
        };
        if let Some(&model_entity) = model_entity {
            let (mut transform, _) = models.get_mut(model_entity).unwrap();
            transform.translation.x = camera.target_x;
            transform.translation.y = camera.target_y;
            transform.translation.z = camera.target_z;
            transform.look_at(Vec3::new(camera.x, camera.y, camera.z), Vec3::Y);
        } else {
            init_arrow_models(commands.entity(entity), &asset_server);
        }
    }
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
        let vertices = vec![
            corners[0], corners[1], corners[1], corners[2], corners[2], corners[3], corners[3],
            corners[0], corners[4], corners[5], corners[5], corners[6], corners[6], corners[7],
            corners[7], corners[4], corners[0], corners[4], corners[1], corners[5], corners[2],
            corners[6], corners[3], corners[7],
        ];

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

fn add_light(mut commands: Commands) {
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        light: Light {
            fov: std::f32::consts::PI * 2.0,
            ..Default::default()
        },
        ..Default::default()
    });
}
