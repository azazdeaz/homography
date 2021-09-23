use bevy::prelude::*;
use bevy_egui::{
    egui::{self, pos2, Color32, Painter, Rect, Shape},
    EguiContext, EguiPlugin,
};

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
        ui.label("world");

        let (response, mut painter) =
            ui.allocate_painter(ui.available_size_before_wrap_finite(), egui::Sense::drag());

            
        let left_top = response.rect.left_top();
        ui.expand_to_include_rect(painter.clip_rect());
        painter.add(Shape::closed_line(
            vec![
                (0.0, 0.0),
                (0.0, 90.0),
                (90.0, 90.0),
                (90.0, 0.0),
            ].into_iter().map(|p| left_top + p.into()).collect::<Vec<_>>(),
            (4.0, Color32::RED),
        ));
        painter.debug_rect(
            Rect::from_min_max(pos2(0.0, 10.0), pos2(90.0, 120.0)),
            Color32::GOLD,
            "text",
        );
    });
}
