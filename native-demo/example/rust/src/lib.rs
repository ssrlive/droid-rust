use log::info;
use ndk::trace;

#[cfg_attr(target_os = "android",
    ndk_glue::main(
        backtrace = "on",
        logger(level = "debug", tag = "sample-tag"),
    )
)]
pub fn main() {
    // println!("hello world");
    let _trace;
    if trace::is_trace_enabled() {
        _trace = trace::Section::new("ndk-rs example main").unwrap();
    }
    info!("hello world 你好世界");
}
