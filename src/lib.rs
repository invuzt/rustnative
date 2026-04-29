#![cfg(target_os = "android")]
use eframe::egui;
use std::sync::Arc;
use std::collections::HashMap;

struct Node {
    pos: egui::Pos2,
    vel: egui::Vec2,
    is_server: bool,
}

struct AppState {
    nodes: HashMap<String, Node>,
    last_update: f64,
}

#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));

    let state = Arc::new(std::sync::Mutex::new(AppState { 
        nodes: HashMap::new(),
        last_update: 0.0,
    }));

    let mut options = eframe::NativeOptions::default();
    
    // OPTIMASI 1: Pasang Event Loop agar bereaksi hanya pada input (Reactive)
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));

    let _ = eframe::run_native(
        "Odfiz Core", 
        options, 
        Box::new(move |_cc| {
            _cc.egui_ctx.set_pixels_per_point(1.2);
            Box::new(OdfizApp { state }) as Box<dyn eframe::App>
        })
    );
}

struct OdfizApp {
    state: Arc<std::sync::Mutex<AppState>>,
}

impl eframe::App for OdfizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(60.0); 

            ui.vertical_centered(|ui| {
                ui.heading("ODFIZ OPTIMIZED UI");
                ui.label("Mode: Reactive (Low Power)");
                
                ui.add_space(20.0);

                ui.group(|ui| {
                    ui.set_max_width(300.0);
                    
                    // OPTIMASI 2: UI hanya update jika ada interaksi tombol
                    if ui.add(egui::Button::new("➕ ADD NODE").min_size(egui::vec2(200.0, 50.0))).clicked() {
                        if let Ok(mut data) = self.state.lock() {
                            let id = data.nodes.len();
                            data.nodes.insert(format!("N-{}", id), Node { 
                                pos: egui::pos2(150.0, 150.0), vel: egui::Vec2::ZERO, is_server: id == 0 
                            });
                        }
                        // Manual trigger repaint hanya saat data berubah
                        ctx.request_repaint();
                    }

                    if ui.add(egui::Button::new("🔄 RESET").min_size(egui::vec2(200.0, 40.0))).clicked() {
                         if let Ok(mut data) = self.state.lock() { data.nodes.clear(); }
                         ctx.request_repaint();
                    }
                });

                ui.add_space(20.0);
                
                // Visualisasi Node sederhana
                let (rect, _) = ui.allocate_at_least(egui::vec2(300.0, 200.0), egui::Sense::hover());
                let painter = ui.painter();
                
                if let Ok(data) = self.state.lock() {
                    for (name, node) in &data.nodes {
                        let color = if node.is_server { egui::Color32::GOLD } else { egui::Color32::LIGHT_BLUE };
                        painter.circle_filled(rect.min + node.pos.to_vec2(), 10.0, color);
                        painter.text(rect.min + node.pos.to_vec2(), egui::Align2::CENTER_CENTER, name, egui::FontId::proportional(10.0), egui::Color32::WHITE);
                    }
                }
            });
        });

        // OPTIMASI 3: Hapus ctx.request_repaint() dari loop utama!
        // Sekarang, Egui hanya akan render ulang jika:
        // 1. Ada input sentuhan (touch)
        // 2. Jendela di-resize atau fokus berubah
        // 3. Kita memanggil ctx.request_repaint() secara manual
    }
}
