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

fn create_header_folder() -> Result<(), Error> {
    let lib_dir = env::var("BLPAPI_LIB").expect(ENV_WARNING);
    let mut lib_dir = lib_dir
        .replace("\\Linux", "")
        .replace("\\lib", "")
        .replace("/Linux", "");
    let header_path = format!("{lib_dir}\\header\\new_header");
    let _res = fs::create_dir(&header_path);

    lib_dir.push_str("\\include");
    for entry in WalkDir::new(lib_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let file_name = entry.file_name().to_str().unwrap();

        if path.is_file() && is_c_or_h_file(path) {
            let new_header = format!("{header_path}\\{file_name}");
            println!("{new_header:?}");
            fs::copy(path, new_header)?;
        }
    }
    Ok(())
}

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
    let _ = create_header_folder();
}
