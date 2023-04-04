#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let mut options = NativeOptions::default();
    options.event_loop_builder = Some(Box::new(move |builder| {
        builder.with_android_app(app);
    }));
    main_entry(options);
}

use eframe::egui;
use eframe::{NativeOptions, Renderer};

#[derive(Default)]
struct DemoApp {
    demo_windows: egui_demo_lib::DemoWindows,
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.demo_windows.ui(ctx);
    }
}

pub fn main_entry(mut options: NativeOptions) {
    options.renderer = Renderer::Wgpu;
    let _ = eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(DemoApp::default())),
    );
}
