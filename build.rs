use std::fs;
use std::path::Path;

/// # Build Script main()
/// Copies fonts from assets/fonts to __OUT_DIR__ (e.g. target/debug/build/..)
///
/// OUT_DIR — the folder in which all output and intermediate artifacts should be placed.
/// This folder is inside the build directory for the package being built, and it is unique
/// for the package in question.
fn main() {
    // Env value
    let out_dir = std::env::var("OUT_DIR").unwrap();

    // The first ancestor (nth(0)) is /home/user/project/target/debug/build/my_project/out.
    // The second ancestor (nth(1)) is /home/user/project/target/debug/build/my_project.
    // The third ancestor (nth(2)) is /home/user/project/target/debug/build.
    // The fourth ancestor (nth(3)) is /home/user/project/target.
    let font_dest_path = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .unwrap()
        .join("assets/fonts");

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
