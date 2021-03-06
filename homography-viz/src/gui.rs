use crate::{
    components::{Camera, Landmarks2, Plane},
    estimators::EstimationLabel,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, Shape, Slider},
    EguiContext,
};
use homography::HomographyMatrix;
use itertools::Itertools;
use std::{f32::consts::PI, time::Duration};

pub fn render_gui(
    egui_context: ResMut<EguiContext>,
    mut planes: Query<&mut Plane>,
    mut cameras: Query<(&mut Camera, &Landmarks2)>,
    results: Query<(
        &Option<HomographyMatrix>,
        &EstimationLabel,
        Option<&Duration>,
    )>,
) {
    egui::Window::new("Items").show(egui_context.ctx(), |ui| {
        ui.style_mut().spacing.slider_width = 270.0;
        for mut plane in planes.iter_mut() {
            ui.collapsing("plane", |ui| {
                ui.add(Slider::new(&mut plane.x, -100.0..=100.0).text("x"));
                ui.add(Slider::new(&mut plane.y, -100.0..=100.0).text("y"));
                ui.add(Slider::new(&mut plane.z, -100.0..=100.0).text("z"));
                ui.add(Slider::new(&mut plane.rot_x, -PI..=PI).text("rot_x"));
                ui.add(Slider::new(&mut plane.rot_y, -PI..=PI).text("rot_y"));
                ui.add(Slider::new(&mut plane.rot_z, -PI..=PI).text("rot_z"));
                ui.add(Slider::new(&mut plane.points_x, 0..=50).text("points_x"));
                ui.add(Slider::new(&mut plane.points_y, 0..=50).text("points_y"));
            });
        }
        for (camera_id, (mut camera, landmarks)) in cameras.iter_mut().enumerate() {
            ui.collapsing(format!("camera {}", camera_id), |ui| {
                ui.add(Slider::new(&mut camera.width, 1.0..=1000.0).text("cam width"));
                ui.add(Slider::new(&mut camera.height, 1.0..=1000.0).text("cam height"));
                ui.add(Slider::new(&mut camera.fovy, (PI / 8.0)..=PI).text("fovy"));
                ui.add(Slider::new(&mut camera.x, -100.0..=100.0).text("x"));
                ui.add(Slider::new(&mut camera.y, -100.0..=100.0).text("y"));
                ui.add(Slider::new(&mut camera.z, -100.0..=100.0).text("z"));
                ui.add(Slider::new(&mut camera.target_x, -100.0..=100.0).text("target_x"));
                ui.add(Slider::new(&mut camera.target_y, -100.0..=100.0).text("target_y"));
                ui.add(Slider::new(&mut camera.target_z, -100.0..=100.0).text("target_z"));
                ui.add(Slider::new(&mut camera.noise, 0.0..=50.0).text("noise"));
                ui.add(
                    Slider::new(&mut camera.outlier_proportion, 0.0..=1.0)
                        .text("outlier_proportion"),
                );
                ui.add(Slider::new(&mut camera.outlier_noise, 0.0..=50.0).text("outlier_noise"));
            });

            let width = camera.width;
            let height = camera.height;
            egui::Window::new(format!("Camera {} Image", camera_id))
                .default_size((width, height))
                .show(egui_context.ctx(), |ui| {
                    let (response, painter) =
                        ui.allocate_painter(egui::Vec2::new(width, height), egui::Sense::drag());

                    let left_top = response.rect.left_top();
                    ui.expand_to_include_rect(painter.clip_rect());
                    painter.add(Shape::closed_line(
                        vec![(0.0, 0.0), (0.0, height), (width, height), (width, 0.0)]
                            .into_iter()
                            .map(|p| left_top + p.into())
                            .collect::<Vec<_>>(),
                        (4.0, Color32::RED),
                    ));

                    for lm in landmarks {
                        painter.add(Shape::circle_filled(
                            left_top + (lm.point.coords.x, lm.point.coords.y).into(),
                            4.0,
                            Color32::LIGHT_GRAY,
                        ));
                    }
                });

            for (h, label, time) in results.iter() {
                egui::Window::new(label.to_string()).show(egui_context.ctx(), |ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                    ui.style_mut().wrap = Some(false);
                    if let Some(time) = time {
                        ui.label(format!("Time {:?}", time));
                    }
                    if let Some(h) = h {
                        ui.label(pretty_hmat(h));
                    } else {
                        ui.label("no solution");
                    }
                });
            }
        }
    });
}

fn pretty_hmat(h: &HomographyMatrix) -> String {
    let values = h.iter().map(|v| format!("{:>12.4}", v)).collect_vec();
    vec![&values[0..3], &values[3..6], &values[6..9]]
        .iter()
        .map(|l| l.join(", "))
        .collect_vec()
        .join("\n")
}
