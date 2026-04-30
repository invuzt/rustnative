use eframe::egui;

pub struct Node {
    pub pos: egui::Pos2,
    pub is_server: bool,
}

pub struct AppState {
    pub app_name: String,
    pub show_kb: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            app_name: "VUZT".to_string(),
            show_kb: false,
        }
    }
}
