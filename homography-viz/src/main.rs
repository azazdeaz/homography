use bevy::{pbr::AmbientLight, prelude::*};
use bevy_egui::EguiPlugin;

mod components;
mod orbit_camera;
mod utils;
use components::{MatchEvent, Landmark2, Plane, Camera};
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
        .add_startup_system(add_planes.system())
        .add_startup_system(add_cameras.system())
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

fn add_cameras(mut commands: Commands) {
    for i in 0..2 {
        let x = if i == 0 { -10.0 } else { 10.0 };
        let target_x = if i == 0 { -2.0 } else { 4.0 };
        let camera = Camera {
            fovy: 0.65,
            x,
            z: 22.0,
            target_x,
            target_z: 10.0,
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
