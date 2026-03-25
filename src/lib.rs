use android_activity::{AndroidApp, MainEvent, PollEvent};
use log::info;

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    info!("Halo Odfiz! Aplikasi Rust NativeActivity berhasil jalan.");

    loop {
        app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
            match event {
                PollEvent::Main(MainEvent::SaveState { .. }) => {
                    info!("Aplikasi menyimpan state...");
                }
                PollEvent::Main(MainEvent::Terminate) => {
                    info!("Aplikasi ditutup.");
                }
                _ => {}
            }
        });
    }
}
