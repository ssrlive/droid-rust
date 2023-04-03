#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use android_activity::{InputStatus, MainEvent, PollEvent};

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    loop {
        let timeout = Some(std::time::Duration::from_millis(1000));
        app.poll_events(timeout, |event| {
            match event {
                PollEvent::Wake => {
                    log::info!("Early wake up");
                }
                PollEvent::Timeout => {
                    log::info!("hello world 你好世界.");
                }
                PollEvent::Main(main_event) => {
                    log::info!("Main event: {:?}", main_event);
                    match main_event {
                        MainEvent::Destroy => {
                            return;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }

            app.input_events(|event| {
                log::info!("Input Event: {event:?}");
                InputStatus::Unhandled
            });
        });
    }
}
