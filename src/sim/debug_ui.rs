use bevy_egui::{egui, EguiContexts};
use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct UiState {
    pub is_window_open: bool,
    pub steering_strength: f32,
    pub max_velocity: f32,
    pub water_desire_scaling: f32,
    pub food_desire_scaling: f32,
    pub max_acceleration: f32,
    pub avoid_other_strength: f32,
    pub avoid_same_strength: f32,
    pub show_perception_radius: bool,
    pub show_physics_vectors: bool,
    pub vector_scaling: f32,
}



pub fn debug_menu_ui(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
){
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Debug Menu");

            ui.add(egui::Slider::new(&mut ui_state.max_acceleration, 0.01..=50.0).text("Max Acceleration"));
            // ui.add(egui::Slider::new(&mut ui_state.water_desire_scaling, 0.1..=300.0).text("Water Desire Scaling"));
            // ui.add(egui::Slider::new(&mut ui_state.food_desire_scaling, 0.1..=300.0).text("Food Desire Scaling"));
            ui.add(egui::Slider::new(&mut ui_state.steering_strength, 0.01..=50.0).text("Steering Strength"));
            ui.add(egui::Slider::new(&mut ui_state.avoid_other_strength, 0.01..=50.0).text("Avoid Other Strength"));
            ui.add(egui::Slider::new(&mut ui_state.avoid_same_strength, 0.01..=50.0).text("Avoid Same Strength"));
            ui.add(egui::Checkbox::new(&mut ui_state.show_perception_radius, "Show Perception Radius"));
            ui.add(egui::Checkbox::new(&mut ui_state.show_physics_vectors, "Show Physics Vectors"));
            ui.add(egui::Slider::new(&mut ui_state.vector_scaling, 1.0..=100.).text("Vector Scale"));

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "powered by egui",
                    "https://github.com/emilk/egui/",
                ));
            });
        });

    //egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //// The top panel is often a good place for a menu bar:
        //egui::menu::bar(ui, |ui| {
            //egui::menu::menu_button(ui, "File", |ui| {
                //if ui.button("Quit").clicked() {
                    //std::process::exit(0);
                //}
            //});
        //});
    //});

}
