#![windows_subsystem = "windows"]

#[cfg(not(target_os = "android"))]
fn main() {
    use eframe::NativeOptions;
    use nativedemo::main_entry;

    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .parse_default_env()
        .init();

    main_entry(NativeOptions::default());
}

#[cfg(target_os = "android")]
fn main() {}
