use std::fs;
use std::path::Path;

/// # Build Script main()
/// Copies fonts from assets/fonts to __OUT_DIR__ (e.g. target/debug/build/..)
fn main() {
    // Env value
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let font_dest_path = Path::new(&out_dir).join("assets/fonts");

    fs::create_dir_all(&font_dest_path).unwrap();

    let font_files = vec!["Roboto-Bold.ttf", "Roboto-Regular.ttf"];

    let font_source_path = std::env::current_dir().unwrap().join("assets/fonts");

    for file in font_files {
        let source_file = font_source_path.join(file);
        let dest_file = font_dest_path.join(file);
        fs::copy(source_file, dest_file).unwrap();
    }
    println!("cargo::rerun-if-changed=build.rs");
}
