#![cfg(target_os = "android")]
mod app_logic;
mod keyboard;

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
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "custom_font".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/font.ttf")),
            );
            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "custom_font".to_owned());
            fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("custom_font".to_owned());
            cc.egui_ctx.set_fonts(fonts);

            Box::new(OdfizApp { state: state_inner }) as Box<dyn eframe::App>
        }),
    );
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut state = self.state.lock().unwrap();

        // Panel Utama
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading("VUZT NATIVE UI");
                ui.add_space(10.0);
                
                ui.label("Klik teks di bawah untuk edit:");
                // Gunakan selectable_label sebagai pemicu keyboard
                let response = ui.add(egui::SelectableLabel::new(state.show_kb, format!("> {}", state.app_name)));
                if response.clicked() {
                    state.show_kb = !state.show_kb;
                }
                
                ui.add_space(20.0);
                ui.label(format!("Status: {}", if state.show_kb { "Keyboard Aktif" } else { "Keyboard Sembunyi" }));
            });
        });

        // Panel Keyboard (Nempel di bawah)
        if state.show_kb {
            egui::TopBottomPanel::bottom("virtual_keyboard")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.add_space(10.0);
                    crate::keyboard::render_keyboard(ui, &mut state.app_name);
                    
                    // Tombol khusus tutup keyboard
                    if ui.button("CLOSE").clicked() {
                        state.show_kb = false;
                    }
                    ui.add_space(10.0);
                });
        }
    }
}
