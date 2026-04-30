#![cfg(target_os = "android")]
mod app_logic;
mod keyboard;
mod css; // Import CSS module

use eframe::egui;
use std::sync::{Arc, Mutex};
use android_activity::AndroidApp;

struct OdfizApp {
    state: Arc<Mutex<app_logic::AppState>>,
}

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));

    let mut options = eframe::NativeOptions::default();
    let app_clone = app.clone();
    
    options.event_loop_builder = Some(Box::new(move |builder| {
        use winit::platform::android::EventLoopBuilderExtAndroid;
        builder.with_android_app(app_clone);
    }));

    let state = Arc::new(Mutex::new(app_logic::AppState::new()));
    let state_inner = state.clone();

    let _ = eframe::run_native(
        "Vuzt",
        options,
        Box::new(move |cc| {
            // Panggil fungsi CSS di sini
            crate::css::apply_custom_style(&cc.egui_ctx);

            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "custom_font".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/font.ttf")),
            );
            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "custom_font".to_owned());
            cc.egui_ctx.set_fonts(fonts);

            Box::new(OdfizApp { state: state_inner }) as Box<dyn eframe::App>
        }),
    );
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut state = self.state.lock().unwrap();

        egui::TopBottomPanel::top("status_bar_spacer")
            .frame(egui::Frame::none().inner_margin(egui::Margin::symmetric(0.0, 15.0)))
            .show(ctx, |_| {});

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading("VUZT NATIVE UI"); // Ini pakai Heading style (24.0)
                
                ui.label("Input Teks:");
                let response = ui.add(egui::SelectableLabel::new(
                    state.show_kb, 
                    format!(" > {} ", state.app_name)
                ));
                if response.clicked() {
                    state.show_kb = !state.show_kb;
                }

                ui.add_space(10.0);
                ui.add(egui::Slider::new(&mut 50, 0..=100).text("Slider Test"));
            });
        });

        if state.show_kb {
            egui::TopBottomPanel::bottom("virtual_keyboard")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.add_space(10.0);
                    crate::keyboard::render_keyboard(ui, &mut state.app_name);
                    if ui.button("CLOSE").clicked() {
                        state.show_kb = false;
                    }
                    ui.add_space(10.0);
                });
        }
    }
}
