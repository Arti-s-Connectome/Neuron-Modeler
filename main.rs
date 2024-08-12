#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod neuron_drawing;
mod draw;

use crate::app::NeuronModelerApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 900.0])
            .with_min_inner_size([800.0, 600.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icons/icons8-neuron-64.png")[..])
                    .expect("Failed to load icon")
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Neuron Modeler",
        native_options,
        Box::new(|cc| Ok(Box::new(NeuronModelerApp::new(cc)))),
    )
}
