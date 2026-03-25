use android_activity::{AndroidApp, MainEvent, PollEvent, InputEvent};
use log::info;
use std::time::Duration;

#[no_mangle]
fn android_main(app: AndroidApp) {
    // Inisialisasi logger agar kita bisa lihat pesan di adb logcat
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    info!("Aplikasi Odfiz Native Dimulai!");

    loop {
        app.poll_events(Some(Duration::from_millis(16)), |event| {
            match event {
                PollEvent::Main(MainEvent::Destroy) => {
                    info!("Aplikasi ditutup oleh user.");
                }
                PollEvent::Input(InputEvent::MotionEvent(motion_event)) => {
                    let action = motion_event.action();
                    info!("Layar disentuh! Action: {:?}", action);
                }
                _ => {}
            }
        });
    }
}
