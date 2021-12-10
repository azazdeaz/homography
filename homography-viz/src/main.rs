use bevy::{prelude::{Transform, *}, pbr::AmbientLight};
use bevy_egui::EguiPlugin;

mod components;
mod orbit_camera;
mod utils;
use components::MatchEvent;
mod estimators;
mod gui;
mod update_landmarks;
mod update_matches;
mod visuals_3d;

fn main() {
    App::build()
        .add_event::<MatchEvent>()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(visuals_3d::CamerasAndPlanes3D)
        .add_plugin(estimators::Estimators)
        .add_startup_system(setup_3d.system())
        .add_startup_system(orbit_camera::spawn_camera.system())
        .add_system(orbit_camera::pan_orbit_camera.system())
        .add_system(update_landmarks::update_landmarks.system())
        .add_system(
            update_matches::render_landmarks
                .system()
                .chain(update_matches::update_matches.system()),
        )
        .add_system(gui::render_gui.system())
        .add_system(utils::inspect.system())
        .run();
}

/// set up a simple 3D scene
fn setup_3d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        light: Light { fov: std::f32::consts::PI * 2.0, ..Default::default() },
        ..Default::default()
    });
}
