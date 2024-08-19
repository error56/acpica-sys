use std::{env, path::{Path, PathBuf}};

fn main() {

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&manifest_dir).display();

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rustc-link-lib=static=acpica-x86_64-unknown-gnu");
    println!("cargo:rustc-link-search=native={}", manifest_dir);

    let bindings = bindgen::Builder::default()
        .header("src/acpica.h")
        .layout_tests(true)
        .use_core()
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
