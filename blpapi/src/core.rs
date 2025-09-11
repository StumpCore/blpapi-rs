use crate::session_options::{Authentication, ClientMode};
use std::ffi::{c_char, c_int, c_void};
use std::io::Write;

/// Const Values
pub const BLPAPI_DEFAULT_HOST: &'static str = "127.0.0.1";
pub const BLPAPI_DEFAULT_SESSION_NAME: &'static str = "localhost";
pub const BLPAPI_DEFAULT_PORT: u16 = 8_194;
pub const BLPAPI_DEFAULT_INDEX: usize = 0;
pub const BLPAPI_DEFAULT_AUTO_RESTART: usize = 0;
pub const BLPAPI_DEFAULT_TIMEOUT: u32 = 5_000;
pub const BLPAPI_DEFAULT_MULTIPLE_CORR_PER_MSG: usize = 0;
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MKTDATA: &'static str = "//blp/mktdata";
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_REFDATA: &'static str = "//blp/refdata";
pub const BLPAPI_DEFAULT_TOPIC_PREFIX: &'static str = "/ticker/";
pub const BLPAPI_AUTHENTICATION_OS_LOGON: &'static str = "OS_LOGON";
pub const BLPAPI_AUTHENTICATION_DIRECTORY_SERVICE: &'static str = "DIRECTORY_SERVICE";
pub const BLPAPI_AUTHENTICATION_APPLICATION_ONLY: &'static str = "APPLICATION_ONLY";
pub const BLPAPI_AUTHENTICATION_APPNAME_AND_KEY: &'static str = "APPNAME_AND_KEY";
pub const BLPAPI_DEFAULT_MAX_PENDING_REQUEST: u16 = 1024;
pub const BLPAPI_DEFAULT_MAX_START_ATTEMPTS: u16 = 3;
pub const BLPAPI_DEFAULT_MAX_EVENT_QUEUE_SIZE: usize = 10_000;
pub const BLPAPI_DEFAULT_HIGH_WATER_MARK: f32 = 0.75;
pub const BLPAPI_DEFAULT_LOW_WATER_MARK: f32 = 0.50;
pub const BLPAPI_DEFAULT_KEEP_ALIVE_INACTIVITY_TIME: isize = 20_000;
pub const BLPAPI_DEFAULT_KEEP_ALIVE_RESPONSE_TIMEOUT: isize = 5_000;
pub const BLPAPI_DEFAULT_KEEP_ALIVE: bool = true;
pub const BLPAPI_DEFAULT_RECORD_SUBSCRIPTION: bool = false;
pub const BLPAPI_DEFAULT_SERVICE_CHECK_TIMEOUT: isize = 120_000;
pub const BLPAPI_DEFAULT_SERVICE_DOWNLOAD_TIMEOUT: isize = 120_000;
pub const BLPAPI_DEFAULT_FLUSH_PUBLISHED_EVENTS_TIMEOUT: isize = 2_000;
pub const BLPAPI_DEFAULT_CLIENT_MODE: ClientMode = ClientMode::Auto;
pub const BLPAPI_DEFAULT_AUTHENTICATION: Authentication = Authentication::OsLogon;
pub const BLPAPI_DEFAULT_BANDWIDTH_SAVE_MODE: bool = true;
pub const BLPAPI_DEFAULT_APPLICATION_IDENTIFICATION_KEY: &'static str = "RUST_BLPAPI_DEFAULT_APPLICATION_ID";


/// StreamWriterContext
/// The StreamWriterContext struct is necessary due to Rust 'Fat Pointer' implementation
/// of pointers. The trait object Write is a fat pointer and contains both, a pointer
/// to the actual data (where the output is stored) and a pointer to a table
/// of function pointers (vtable). The table holds the instruction for the
/// write_all method. Without the struct a Segment Fault error persists.
#[repr(C)]
pub struct StreamWriterContext<'a> {
    pub writer: &'a mut dyn Write,
}
/// Implementing the streaming function for the socks5config print
/// Streaming function is necessary to communicate with the C Api
/// of the bloomberg connection. Only usable in the context of print.
#[no_mangle]
pub unsafe extern "C" fn write_to_stream_cb(
    data: *const c_char,
    len: c_int,
    stream_context: *mut c_void,
) -> c_int {
    if stream_context.is_null() {
        return -1;
    };

    let context = &mut *(stream_context as *mut StreamWriterContext);
    let writer = &mut context.writer;

    let bytes = std::slice::from_raw_parts(data as *const u8, len as usize);
    let result = writer.write_all(bytes);
    if result.is_ok() {
        0
    } else {
        -1
    }
}
