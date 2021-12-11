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
