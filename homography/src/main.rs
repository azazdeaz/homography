use bevy::prelude::*;
use bevy_egui::{
    egui::{self, pos2, Color32, Rect, Shape},
    EguiContext, EguiPlugin,
};
use nalgebra::{self as na, *};

struct CameraState {
    width: f32,
    height: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            width: 300.0,
            height: 200.0,
            fovy: 3.14 / 2.0,
            znear: 1.0,
            zfar: 1000.0,
        }
    }
}

struct Plane {
    points_x: u32,
    points_y: u32,
    width: f32,
    height: f32,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(add_points.system())
        .init_resource::<CameraState>()
        .add_system(ui_example.system())
        .run();
}

fn add_points(mut commands: Commands) {
    commands.spawn().insert(Plane {
        width: 100.0,
        height: 100.0,
        points_x: 5,
        points_y: 5,
    });
}

// Note the usage of `ResMut`. Even though `ctx` method doesn't require
// mutability, accessing the context from different threads will result
// into panic if you don't enable `egui/multi_threaded` feature.
fn ui_example(egui_context: ResMut<EguiContext>, mut camera_state: ResMut<CameraState>, query: Query<&Plane>) {
    egui::Window::new("Hello").show(egui_context.ctx(), |ui| {
        ui.label("world");
        ui.add(egui::Slider::new(&mut camera_state.width, 1.0..=1000.0).text("cam width"));
        ui.add(egui::Slider::new(&mut camera_state.height, 1.0..=1000.0).text("cam height"));
        ui.add(egui::Slider::new(&mut camera_state.fovy, (3.14/8.0)..=3.14).text("fovy"));


        let width = camera_state.width;
        let height = camera_state.height;

        let (response, mut painter) =
            ui.allocate_painter(ui.available_size_before_wrap_finite(), egui::Sense::drag());

        let left_top = response.rect.left_top();
        ui.expand_to_include_rect(painter.clip_rect());
        painter.add(Shape::closed_line(
            vec![(0.0, 0.0), (0.0, height), (width, height), (width, 0.0)]
                .into_iter()
                .map(|p| left_top + p.into())
                .collect::<Vec<_>>(),
            (4.0, Color32::RED),
        ));

        let points = {
            let p = Point3::new(1.0, 2.0, -10.0);
            let model = Isometry3::new(Vector3::x(), na::zero());

            // Our camera looks toward the point (1.0, 0.0, 0.0).
            // It is located at (0.0, 0.0, 1.0).
            let eye = Point3::new(0.0, 0.0, 1.0);
            let target = Point3::new(1.0, 0.0, 0.0);
            let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

            // A perspective projection.
            let projection = Perspective3::new(
                width / height,
                camera_state.fovy,
                camera_state.znear,
                camera_state.zfar,
            );

            // The combination of the model with the view is still an isometry.
            let model_view = view * model;

            // Convert everything to a `Matrix4` so that they can be combined.
            let mat_model_view = model_view.to_homogeneous();

            // Combine everything.
            let model_view_projection = projection.as_matrix() * mat_model_view;

            let proj = Perspective3::from_matrix_unchecked(model_view_projection);
            
            let mut points = Vec::new();
            for plane in query.iter() {
                for xi in 0..plane.points_x {
                    for yi in 0..plane.points_y {
                        let p = Point3::new(
                            -plane.width / 2.0 + (plane.width / ((plane.points_x-1) as f32)) * xi as f32,
                            -plane.height / 2.0 + (plane.height / ((plane.points_y-1) as f32)) * yi as f32,
                            -10.0,
                        );
                        let pp = proj.project_point(&p);

                        let x = (pp.coords.x + 1.0) * width / 2.0;
                        let y = (pp.coords.y + 1.0) * height / 2.0;
                        points.push(left_top + (x, y).into());
                    }
                }
            }

            points
        };

        for point in points {
            painter.add(Shape::circle_filled(point, 4.0, Color32::LIGHT_GRAY));
        }

        painter.debug_rect(
            Rect::from_min_max(pos2(0.0, 10.0), pos2(90.0, 120.0)),
            Color32::GOLD,
            "text",
        );
    });
}
