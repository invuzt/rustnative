use android_activity::{AndroidApp, MainEvent, PollEvent};

#[no_mangle]
fn android_main(app: AndroidApp) {
    loop {
        app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
            if let PollEvent::Main(MainEvent::Destroy) = event {
                return;
            }
        });
    }
}
