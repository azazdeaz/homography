use bevy::prelude::*;
use bevy_egui::{EguiContext, EguiPlugin, egui::{self, Color32, Pos2, Rect, Shape, pos2}};
use nalgebra::{self as na, *};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(ui_example.system())
        .run();
}

// Note the usage of `ResMut`. Even though `ctx` method doesn't require
// mutability, accessing the context from different threads will result
// into panic if you don't enable `egui/multi_threaded` feature.
fn ui_example(egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx(), |ui| {
        let width = 300.0;
        let height = 200.0;
        ui.label("world");

        let (response, mut painter) =
            ui.allocate_painter(ui.available_size_before_wrap_finite(), egui::Sense::drag());

            
        let left_top = response.rect.left_top();
        ui.expand_to_include_rect(painter.clip_rect());
        painter.add(Shape::closed_line(
            vec![
                (0.0, 0.0),
                (0.0, height),
                (width, height),
                (width, 0.0),
            ].into_iter().map(|p| left_top + p.into()).collect::<Vec<_>>(),
            (4.0, Color32::RED),
        ));
        let points = render_points(width, height);
        painter.add(Shape::circle_filled(points[0], 4.0, Color32::LIGHT_GRAY));
        painter.debug_rect(
            Rect::from_min_max(pos2(0.0, 10.0), pos2(90.0, 120.0)),
            Color32::GOLD,
            "text",
        );
    });
}

fn render_points(width: f32, height: f32) -> Vec<Pos2> {
    let p = Point3::new(1.0, 2.0, -10.0);
    let model = Isometry3::new(Vector3::x(), na::zero());

    // Our camera looks toward the point (1.0, 0.0, 0.0).
    // It is located at (0.0, 0.0, 1.0).
    let eye    = Point3::new(0.0, 0.0, 1.0);
    let target = Point3::new(1.0, 0.0, 0.0);
    let view   = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

    // A perspective projection.
    let projection = Perspective3::new(width / height, 3.14 / 2.0, 1.0, 1000.0);

    // The combination of the model with the view is still an isometry.
    let model_view = view * model;

    // Convert everything to a `Matrix4` so that they can be combined.
    let mat_model_view = model_view.to_homogeneous();

    // Combine everything.
    let model_view_projection = projection.as_matrix() * mat_model_view;

    let proj = Perspective3::from_matrix_unchecked(model_view_projection);
    let pp = proj.project_point(&p);

    let x = (pp.coords.x + 1.0) * width / 2.0;
    let y = (pp.coords.y + 1.0) * height / 2.0;

    println!("proj {:?}, x: {}, y:{}", pp, x, y);

    vec![pos2(x, y)]
}
