use android_activity::{AndroidApp, MainEvent, PollEvent};
use egui_glow::painter::Painter;
use std::time::{Duration, Instant};

#[no_mangle]
fn android_main(app: AndroidApp) {
    let mut nama = String::new();
    let mut angka: i32 = 0;
    let mut counter = 0;
    
    // Inisialisasi Egui Context
    let egui_ctx = egui::Context::default();
    let mut egui_state = egui_android::State::new(&app);
    let mut painter: Option<Painter> = None;
    let start_time = Instant::now();

    loop {
        app.poll_events(Some(Duration::from_millis(16)), |event| {
            // Berikan event ke egui-android untuk handle sentuhan/keyboard
            egui_state.process_event(&event);

            match event {
                PollEvent::Main(MainEvent::InitWindow { .. }) => {
                    // Inisialisasi renderer saat window tersedia
                    let native_window = app.native_window().unwrap();
                    let (gl, shader_prefix) = unsafe {
                        // Di sini kita membuat context GLES
                        egui_android::opengl::create_context(&native_window).expect("Gagal membuat GL context")
                    };
                    painter = Some(Painter::new(std::sync::Arc::new(gl), shader_prefix, None));
                }
                PollEvent::Main(MainEvent::TerminateWindow { .. }) => {
                    painter = None; // Lepas renderer saat window ditutup
                }
                PollEvent::Main(MainEvent::Destroy) => return,
                _ => {}
            }
        });

        if let Some(ref mut painter) = painter {
            let raw_input = egui_state.take_egui_input(&app);
            
            // Definisikan UI
            let full_output = egui_ctx.run(raw_input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Rust Native UI (Egui)");
                    ui.add_space(10.0);

                    ui.label(format!("Waktu berjalan: {:.1}s", start_time.elapsed().as_secs_f32()));
                    ui.label(format!("Tombol ditekan: {} kali", counter));

                    ui.separator();

                    // Input Teks
                    ui.horizontal(|ui| {
                        ui.label("Nama:");
                        ui.text_edit_singleline(&mut nama);
                    });

                    // Input Angka
                    ui.horizontal(|ui| {
                        ui.label("Angka:");
                        ui.add(egui::DragValue::new(&mut angka));
                    });

                    ui.add_space(10.0);

                    // Button
                    if ui.button("Klik Saya!").clicked() {
                        counter += 1;
                    }

                    if ui.button("Reset").clicked() {
                        counter = 0;
                        nama.clear();
                        angka = 0;
                    }
                });
            });

            // Gambar ke layar
            let window = app.native_window().unwrap();
            let width = window.width() as u32;
            let height = window.height() as u32;
            
            painter.paint_and_update_textures(
                [width, height],
                egui_ctx.pixels_per_point(),
                full_output.shapes,
                &full_output.textures_delta,
            );
        }
    }
}
