use bevy::{
    prelude::{Transform, *},
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use bevy_egui::{
    egui::{self, pos2, Color32, Rect, Shape},
    EguiContext, EguiPlugin,
};
use itertools::{zip, Itertools};
use nalgebra::{self as na, *};
use opencv::{
    calib3d::find_homography,
    core::{Mat_, Point2f, ToInputArray},
    prelude::{Mat, MatTrait, MatTraitConst},
    types::VectorOfPoint2f,
};

mod orbit_camera;
mod utils;

struct Camera {
    width: f32,
    height: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
    x: f32,
    y: f32,
    z: f32,
    target_x: f32,
    target_y: f32,
    target_z: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            width: 300.0,
            height: 200.0,
            fovy: 1.9,
            znear: 1.0,
            zfar: 100.0,
            x: 0.0,
            y: 0.0,
            z: 1.0,
            target_x: 0.0,
            target_y: 0.0,
            target_z: 0.0,
        }
    }
}

struct Plane {
    points_x: u32,
    points_y: u32,
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(add_points.system())
        .add_startup_system(add_cameras.system())
        .add_startup_system(setup_3d.system())
        .add_startup_system(orbit_camera::spawn_camera.system())
        .add_system(orbit_camera::pan_orbit_camera.system())
        .add_system(ui_example.system())
        .add_system(utils::inspect.system())
        .run();
}

fn add_cameras(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for _ in 0..2 {
        let mesh = Mesh::new(PrimitiveTopology::LineList);
        let pbr = PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        };
        commands
            .spawn()
            .insert_bundle(pbr)
            .insert(Camera::default());
    }
}

fn add_points(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane = Plane {
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

    let mesh = Mesh::new(PrimitiveTopology::LineStrip);
    let pbr = PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    };
    commands.spawn().insert_bundle(pbr).insert(plane);
}

// Note the usage of `ResMut`. Even though `ctx` method doesn't require
// mutability, accessing the context from different threads will result
// into panic if you don't enable `egui/multi_threaded` feature.
fn ui_example(
    egui_context: ResMut<EguiContext>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut planes: Query<(&mut Plane, &Handle<Mesh>)>,
    mut cameras: Query<(&mut Camera, &Handle<Mesh>)>,
) {
    let mut rendered_points = Vec::new();
    egui::Window::new("Items").show(egui_context.ctx(), |ui| {
        ui.label("world");

        let mut transforms = Vec::new();
        for (mut plane, _) in planes.iter_mut() {
            ui.collapsing("plane", |ui| {
                ui.add(egui::Slider::new(&mut plane.x, -100.0..=100.0).text("x"));
                ui.add(egui::Slider::new(&mut plane.y, -100.0..=100.0).text("y"));
                ui.add(egui::Slider::new(&mut plane.z, -100.0..=100.0).text("z"));
                ui.add(egui::Slider::new(&mut plane.rot_x, -3.14..=3.14).text("rot_x"));
                ui.add(egui::Slider::new(&mut plane.rot_y, -3.14..=3.14).text("rot_y"));
                ui.add(egui::Slider::new(&mut plane.rot_z, -3.14..=3.14).text("rot_z"));
            });
            let axisangle = Vector3::y() * plane.rot_y;
            let translation = Vector3::new(plane.x, plane.y, plane.z);
            let transform = Isometry3::new(translation, axisangle);
            transforms.push(transform);
        }
        for (camera_id, (mut camera, mut camera_mesh)) in cameras.iter_mut().enumerate() {
            ui.collapsing(format!("camera {}", camera_id), |ui| {
                ui.add(egui::Slider::new(&mut camera.width, 1.0..=1000.0).text("cam width"));
                ui.add(egui::Slider::new(&mut camera.height, 1.0..=1000.0).text("cam height"));
                ui.add(egui::Slider::new(&mut camera.fovy, (3.14 / 8.0)..=3.14).text("fovy"));
                ui.add(egui::Slider::new(&mut camera.x, -100.0..=100.0).text("x"));
                ui.add(egui::Slider::new(&mut camera.y, -100.0..=100.0).text("y"));
                ui.add(egui::Slider::new(&mut camera.z, -100.0..=100.0).text("z"));
                ui.add(egui::Slider::new(&mut camera.target_x, -100.0..=100.0).text("target_x"));
                ui.add(egui::Slider::new(&mut camera.target_y, -100.0..=100.0).text("target_y"));
                ui.add(egui::Slider::new(&mut camera.target_z, -100.0..=100.0).text("target_z"));
            });

            let points = {
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
                let inv_model_view_projection = model_view_projection
                    .try_inverse()
                    .expect("cant invert projection matrix");

                let mut points = Vec::new();
                for ((plane, mut mesh), transform) in zip(planes.iter_mut(), &transforms) {
                    let mut vertices = Vec::new();
                    for xi in 0..plane.points_x {
                        for yi in 0..plane.points_y {
                            let p = Point3::new(
                                -plane.width / 2.0
                                    + (plane.width / ((plane.points_x - 1) as f32)) * xi as f32,
                                -plane.height / 2.0
                                    + (plane.height / ((plane.points_y - 1) as f32)) * yi as f32,
                                -10.0,
                            );
                            let p = transform * p;
                            vertices.push([p.coords.x, p.coords.y, p.coords.z]);
                            let pp = model_view_projection * p.to_homogeneous();
                            let pp = Point3::from_homogeneous(pp);

                            if let Some(pp) = pp {
                                if pp.coords.x.abs() <= 1.0
                                    && pp.coords.y.abs() <= 1.0
                                    && pp.coords.z.abs() <= 1.0
                                {
                                    points.push(pp);
                                }
                            }
                        }
                    }

                    let mut mesh = meshes.get_mut(mesh).unwrap();
                    fill_mesh_with_vertices(mesh, vertices);
                }

                // cube corners as in https://www.cs.bham.ac.uk/~slb/courses/Graphics/g63.html
                let corners = [
                    &na::Point3::new(-1.0, 1.0, -1.0),
                    &na::Point3::new(1.0, 1.0, -1.0),
                    &na::Point3::new(1.0, -1.0, -1.0),
                    &na::Point3::new(-1.0, -1.0, -1.0),
                    &na::Point3::new(-1.0, 1.0, 1.0),
                    &na::Point3::new(1.0, 1.0, 1.0),
                    &na::Point3::new(1.0, -1.0, 1.0),
                    &na::Point3::new(-1.0, -1.0, 1.0),
                ]
                .iter()
                .map(|p| inv_model_view_projection * p.to_homogeneous())
                .map(|h| Point3::from_homogeneous(h).unwrap())
                .map(|v| [v.coords.x, v.coords.y, v.coords.z])
                .collect_vec();
                let mut vertices = vec![
                    corners[0], corners[1], corners[1], corners[2], corners[2], corners[3],
                    corners[3], corners[0], corners[4], corners[5], corners[5], corners[6],
                    corners[6], corners[7], corners[7], corners[4], corners[0], corners[4],
                    corners[1], corners[5], corners[2], corners[6], corners[3], corners[7],
                ];
                vertices.append(&mut utils::cross_lines(&eye, 1.0));
                vertices.append(&mut utils::cross_lines(&target, 1.0));
                let mut camera_mesh = meshes.get_mut(camera_mesh).unwrap();
                fill_mesh_with_vertices(camera_mesh, vertices);

                rendered_points.push(points);
            };
        }
    });

    let mut p_src = VectorOfPoint2f::new();
    let mut p_dst = VectorOfPoint2f::new();

    for (camera_id, ((camera, _), points)) in zip(cameras.iter_mut(), rendered_points).enumerate() {
        let width = camera.width;
        let height = camera.height;
        egui::Window::new(format!("Camera {} Image", camera_id))
            .default_size((width, height))
            .show(egui_context.ctx(), |ui| {
                let (response, mut painter) = ui
                    .allocate_painter(ui.available_size_before_wrap_finite(), egui::Sense::drag());

                let left_top = response.rect.left_top();
                ui.expand_to_include_rect(painter.clip_rect());
                painter.add(Shape::closed_line(
                    vec![(0.0, 0.0), (0.0, height), (width, height), (width, 0.0)]
                        .into_iter()
                        .map(|p| left_top + p.into())
                        .collect::<Vec<_>>(),
                    (4.0, Color32::RED),
                ));

                for point in points {
                    let x = (point.coords.x + 1.0) * width / 2.0;
                    let y = (point.coords.y + 1.0) * height / 2.0;
                    if camera_id == 0 {
                        p_src.push(Point2f::new(x, y))
                    } else {
                        p_dst.push(Point2f::new(x, y))
                    }
                    painter.add(Shape::circle_filled(
                        left_top + (x, y).into(),
                        4.0,
                        Color32::LIGHT_GRAY,
                    ));
                }
            });
    }
    if p_src.len() > 3 && p_src.len() == p_dst.len() {
        let res = find_homography(
            &p_src.input_array().unwrap(),
            &p_dst.input_array().unwrap(),
            &mut Mat::default(),
            0,
            3.,
        );

        if let Ok(mut res) = res {
            let values = (0..res.total().unwrap() as i32)
                .map(|i| res.at_mut::<f64>(i).unwrap().clone())
                .collect_vec();
            println!("res: {:?}", values);
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

/// set up a simple 3D scene
fn setup_3d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // // cube
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     ..Default::default()
    // });
    // // light
    // commands.spawn_bundle(LightBundle {
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..Default::default()
    // });
    // // camera
    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_xyz(-3.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..Default::default()
    // });

    let vertices = [
        ([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0, 2.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ];

    let indices = bevy::render::mesh::Indices::U32(vec![0, 2, 1, 0, 3, 2]);

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::LineStrip);
    mesh.set_indices(Some(indices));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    // add entities to the world
    commands
        // plane
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
    // camera
    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
    //         .looking_at(Vec3::default(), Vec3::unit_y()),
    //     ..Default::default()
    // });
}
