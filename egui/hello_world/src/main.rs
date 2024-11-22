#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use std::time::Duration;

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([280.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Hey, Hello!",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    name: String,
    counter: i32,
    paint: i64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            counter: 0,
            paint: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hey, Hello!");
            ui.heading("This program is");
            ui.heading("used to say hello to you.");
            ui.heading(format!("frame: {}", self.paint));
            ui.add_space(15.0);

            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add_space(15.0);

            if ui.button("Say It").clicked() {
                if self.counter == 0{
                    self.counter = 180;
                }
            }

            if self.name.to_string() == "" {
                ui.label(format!("Type Your Name First"));
            } else {
                ui.label(format!("Hey! {}", self.name));
            }

            if self.counter < 180 && self.counter > 0 {
                ui.label(format!("Hello! {}", self.name));
            }
            if self.counter < 120 && self.counter > 0 {
                ui.label(format!("Super Hello!! {}", self.name));
            }
            if self.counter < 80 && self.counter > 0 {
                ui.label(format!("Giao Giao Hello!!! {}", self.name));
            }
            if self.counter > 1 {
                self.counter -= 1;
            }

            if self.counter == 1 {
                if ui.button("Clean It").clicked() {
                    self.counter = 0;
                }
            }

            ctx.request_repaint_after(Duration::from_millis(5));
            self.paint += 1;

        });
    }
}
