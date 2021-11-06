use bevy::prelude::*;
use bevy_egui::{
    egui::{self, pos2, Color32, Rect, Shape, Slider},
    EguiContext, EguiPlugin,
};
use crate::components::{Camera, Plane};
use nalgebra::{self as na, Isometry3, Point3, Point2, Vector3, Perspective3};

pub fn render_gui(
    egui_context: ResMut<EguiContext>,
    mut planes: Query<&mut Plane>,
    mut cameras: Query<(&mut Camera, &Vec<na::Point2<f32>>)>
) {
    egui::Window::new("Items").show(egui_context.ctx(), |ui| {
        for mut plane in planes.iter_mut() {
            ui.collapsing("plane", |ui| {
                ui.add(Slider::new(&mut plane.x, -100.0..=100.0).text("x"));
                ui.add(Slider::new(&mut plane.y, -100.0..=100.0).text("y"));
                ui.add(Slider::new(&mut plane.z, -100.0..=100.0).text("z"));
                ui.add(Slider::new(&mut plane.rot_x, -3.14..=3.14).text("rot_x"));
                ui.add(Slider::new(&mut plane.rot_y, -3.14..=3.14).text("rot_y"));
                ui.add(Slider::new(&mut plane.rot_z, -3.14..=3.14).text("rot_z"));
            });
        }
        for (camera_id, (mut camera, mut points)) in cameras.iter_mut().enumerate() {
            ui.collapsing(format!("camera {}", camera_id), |ui| {
                ui.add(Slider::new(&mut camera.width, 1.0..=1000.0).text("cam width"));
                ui.add(Slider::new(&mut camera.height, 1.0..=1000.0).text("cam height"));
                ui.add(Slider::new(&mut camera.fovy, (3.14 / 8.0)..=3.14).text("fovy"));
                ui.add(Slider::new(&mut camera.x, -100.0..=100.0).text("x"));
                ui.add(Slider::new(&mut camera.y, -100.0..=100.0).text("y"));
                ui.add(Slider::new(&mut camera.z, -100.0..=100.0).text("z"));
                ui.add(Slider::new(&mut camera.target_x, -100.0..=100.0).text("target_x"));
                ui.add(Slider::new(&mut camera.target_y, -100.0..=100.0).text("target_y"));
                ui.add(Slider::new(&mut camera.target_z, -100.0..=100.0).text("target_z"));
            });


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
    
                        painter.add(Shape::circle_filled(
                            left_top + (point.coords.x, point.coords.y).into(),
                            4.0,
                            Color32::LIGHT_GRAY,
                        ));
                    }
                });
        }
    });
}