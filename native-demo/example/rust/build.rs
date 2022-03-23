extern crate cmake;

fn _main() {
    // Builds the project in the directory located in `native_app_glue`, installing it
    // into $OUT_DIR
    let dst = cmake::build("native_app_glue");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=native_app_glue");
}

fn main() {
    cc::Build::new()
        .file("native_app_glue/main.c")
        .file("native_app_glue/android_native_app_glue.c")
        .compile("libnative_app_glue.a");
}
