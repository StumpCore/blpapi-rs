#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(inact)]

#[cfg(target_os = "windows")]
mod bindings_windows;
#[cfg(target_os = "windows")]
pub use bindings_windows::*;

#[cfg(target_os = "linux")]
mod bindings_linux;
#[cfg(target_os = "linux")]
pub use bindings_linux::*;
