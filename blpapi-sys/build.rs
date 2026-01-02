use regex::Regex;
use std::{
    env, fs,
    io::Error,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[cfg(target_os = "windows")]
pub const OS_TYPE: &str = "windows";

#[cfg(target_os = "linux")]
pub const OS_TYPE: &str = "linux";

const ENV_WARNING: &str = r#"Error while building blpapi-sys.

    Cannot find 'BLPAPI_LIB' environment variable.

    You can download blpapi binaries from bloomberg at:
    https://www.bloomberg.com/professional/support/api-library/

    Once extracted, the BLPAPI_LIB environment variable should point to the
    corresponding lib dir:

    - windows: <EXTRACT_PATH>\lib
    - linux: <EXTRACT_PATH>/Linux"
"#;

fn is_c_or_h_file(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => ext == "h" || ext == "c" || ext == "cpp",
        None => false,
    }
}

pub fn transform_header_files() -> Result<(), Error> {
    let src_path = PathBuf::from("header");
    let re = Regex::new(r#"(?m)^#include\s+<([^>]+\.h)>"#).unwrap();
    for entry in WalkDir::new(src_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() && is_c_or_h_file(path) {
            let content = fs::read_to_string(path).expect("Could not read file");

            // Replace <header.h> with "header.h"
            let new_content = re.replace_all(&content, r#"#include "$1""#);

            // Only write back if changes were actually made to save SSD wear/time
            if content != new_content {
                fs::write(path, new_content.as_ref()).expect("Could not write file");
                println!("cargo:warning=Updated includes in {:?}", path);
            }
        }
    }

    Ok(())
}

fn main() {
    let res = transform_header_files();
    match res {
        Ok(_) => print!("Transformation of Header Files worked."),
        Err(e) => eprintln!("Transformation of Header Files failed: {e}"),
    };
    let lib_dir = env::var("BLPAPI_LIB").expect(ENV_WARNING);
    let header = "wrapper.h";
    let bindings_str = format!("bindings_{OS_TYPE}.rs");

    println!("Lib-Dir:{lib_dir}");
    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=blpapi3_64");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header)
        .use_core()
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join(bindings_str))
        .expect("Couldn't write bindings!");
}
