use bevy::prelude::*;
use bevy_egui::{egui, EguiPlugin, EguiContext};

use crate::graphics::{DLRCCircuit, CircuitTimer};

///Plugin to add slide bar of sliders
pub struct SideBarPlugin;

impl Plugin for SideBarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EguiPlugin)
            .add_system(left_slider_frame);
    }
}

///Create left side bar with the four desired sliders
fn left_slider_frame(
    mut egui_context: ResMut<EguiContext>,
    mut query_circs: Query<&mut DLRCCircuit>,
    mut time: ResMut<CircuitTimer>,
    ) {
    egui::SidePanel::left("side_panel")
        .show(egui_context.ctx_mut(), |ui| {
            for mut dlcc in query_circs.iter_mut() {
                let r = &mut dlcc.0.circuit.resistance;
                ui.add(egui::Slider::new(r, 0.0..=10.0).text("R"));
                let l = &mut dlcc.0.circuit.inductance;
                ui.add(egui::Slider::new(l, 0.0..=10.0).text("L"));
                let c = &mut dlcc.0.circuit.capacitance;
                ui.add(egui::Slider::new(c, 0.0..=10.0).text("C"));
            }
            ui.add(egui::Slider::new(&mut time.time, 0.0..=100.0).text("Time"));
        });
}
