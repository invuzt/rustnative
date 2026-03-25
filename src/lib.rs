use android_activity::{AndroidApp, MainEvent, PollEvent};
use std::time::Duration;

#[no_mangle]
fn android_main(app: AndroidApp) {
    loop {
        app.poll_events(Some(Duration::from_millis(16)), |event| {
            match event {
                PollEvent::Main(MainEvent::Destroy) => {
                    return;
                }
                _ => {}
            }
        });

        // Di sini kita tidak menggambar apa-apa dulu agar TIDAK FC.
        // Jika ini jalan (layar hitam), berarti HP kamu butuh 
        // inisialisasi EGL yang sangat spesifik.
    }
}
