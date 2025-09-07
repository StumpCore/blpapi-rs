use std::env;

const ENV_WARNING: &'static str = r#"Error while building blpapi-sys.

    Cannot find 'BLPAPI_LIB' environment variable.

    You can download blpapi binaries from bloomberg at:
    https://www.bloomberg.com/professional/support/api-library/

    Once extracted, the BLPAPI_LIB environment variable should point to the
    corresponding lib dir:

    - windows: <EXTRACT_PATH>\lib
    - linux: <EXTRACT_PATH>/Linux"
"#;

fn main() {
    let lib_dir = env::var("BLPAPI_LIB").expect(ENV_WARNING);

    println!("cargo:rustc-link-search={}", lib_dir);
    println!("cargo:rustc-link-lib=blpapi3_64");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    //let bindings = bindgen::Builder::default()
    //    // The input header we would like to generate
    //    // bindings for.
    //    .header("wrapper.h")
    //    .use_core()
    //    // Tell cargo to invalidate the built crate whenever any of the
    //    // included header files changed.
    //    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //    // Finish the builder and generate the bindings.
    //    .generate()
    //    // Unwrap the Result and panic on failure.
    //    .expect("Unable to generate bindings");

    //// Write the bindings to the $OUT_DIR/bindings.rs file.
    //let out_path = PathBuf::from("src");
    //bindings
    //    .write_to_file(out_path.join("bindings.rs"))
    //    .expect("Couldn't write bindings!");
}
