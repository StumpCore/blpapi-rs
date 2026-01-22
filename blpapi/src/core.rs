use blpapi_sys::{blpapi_Event_t, blpapi_Session_t, BLPAPI_SEATTYPE_BPS, BLPAPI_SEATTYPE_NONBPS};

use crate::session_options::{Authentication, ClientMode};
use std::ffi::{c_char, c_int, c_uint, c_void};
use std::io::Write;

#[cfg(target_os = "windows")]
pub type OsInt = std::os::raw::c_int;

#[cfg(target_os = "linux")]
pub type OsInt = std::os::raw::c_uint;

/// Const Values
pub const BLPAPI_DEFAULT_HOST: &str = "127.0.0.1";
pub const BLPAPI_DEFAULT_SESSION_NAME: &str = "localhost";
pub const BLPAPI_DEFAULT_PORT: u16 = 8_194;
pub const BLPAPI_DEFAULT_INDEX: usize = 0;
pub const BLPAPI_DEFAULT_AUTO_RESTART: usize = 0;
pub const BLPAPI_DEFAULT_TIMEOUT: u32 = 5_000;
pub const BLPAPI_DEFAULT_MULTIPLE_CORR_PER_MSG: usize = 0;
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MKTDATA: &str = "//blp/mktdata";
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_REFDATA: &str = "//blp/refdata";
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_SOURCE_REF: &str = "//blp/srcref";
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_VWAP: &str = "//blp/mktvwap";
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MARKET_DEPTH: &str = "//blp/mktdepthdata";
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MARKET_BAR: &str = "//blp/mktbar";
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_MARKET_LIST: &str = "//blp/mktlist";
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_API_FIELDS: &str = "//blp/apiflds";
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_INSTRUMENTS: &str = "//blp/instruments";
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_PAGE_DATA: &str = "//blp/ppagedata";
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_TECHNICAL_ANALYSIS: &str = "//blp/tasvc";
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_CURVES_TOOLKIT: &str = "//blp/irdctk3";
pub const BLPAPI_DEFAULT_TOPIC_PREFIX: &str = "/ticker/";
pub const BLPAPI_AUTHENTICATION_OS_LOGON: &str = "OS_LOGON";
pub const BLPAPI_AUTHENTICATION_DIRECTORY_SERVICE: &str = "DIRECTORY_SERVICE";
pub const BLPAPI_AUTHENTICATION_APPLICATION_ONLY: &str = "APPLICATION_ONLY";
pub const BLPAPI_AUTHENTICATION_APPNAME_AND_KEY: &str = "APPNAME_AND_KEY";
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
pub const BLPAPI_DEFAULT_APPLICATION_IDENTIFICATION_KEY: &str =
    "RUST_BLPAPI_DEFAULT_APPLICATION_ID";
pub const BLPAPI_DEFAULT_CORRELATION_CLASS_ID: u32 = 0;
pub const BLPAPI_DEFAULT_CORRELATION_INT_VALUE: u64 = 1;
pub const BLPAPI_DEFAULT_CORRELATION_ID: u64 = 31122025;
pub const BLPAPI_DEFAULT_DIRECTORY_SERVICE: &str = "DIRECTORY_SERVICE";
pub const BLPAPI_DEFAULT_SEATTYPE_NONBPS: i32 = BLPAPI_SEATTYPE_NONBPS as i32;
pub const BLPAPI_DEFAULT_SEATTYPE_BPS: i32 = BLPAPI_SEATTYPE_BPS as i32;
pub const BLPAPI_DEFAULT_HISTORICAL_DATA_REQUEST: &str = "HistoricalDataRequest";
pub const BLPAPI_DEFAULT_INTRADAY_BAR_DATA_REQUEST: &str = "IntradayBarRequest";
pub const BLPAPI_DEFAULT_INTRADAY_TICK_DATA_REQUEST: &str = "IntradayTickRequest";
pub const BLPAPI_DEFAULT_REFERENCE_DATA_REQUEST: &str = "ReferenceDataRequest";
pub const BLPAPI_DEFAULT_CATEGORIZED_FIELD_SEARCH_DATA_REQUEST: &str =
    "categorizedFieldSearchRequest";
pub const BLPAPI_DEFAULT_FIELD_INFO_REQUEST_DATA_REQUEST: &str = "fieldInfoRequest";
pub const BLPAPI_DEFAULT_FIELD_LIST_REQUEST_DATA_REQUEST: &str = "fieldListRequest";
pub const BLPAPI_DEFAULT_FIELD_SEARCH_REQUEST_DATA_REQUEST: &str = "fieldSearchRequest";
pub const BLPAPI_DEFAULT_STUDY_DATA_REQUEST: &str = "StudyRequest";
pub const BLPAPI_DEFAULT_BEQS_DATA_REQUEST: &str = "BeqsRequest";
pub const BDH_DATE_REGEX: &str = r"[0-9]{4}[0-1]{1}[0-9]{1}[0-3]{1}[0-9]{1}";

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

/// Handler Function for the EventHandler
/// The structer can be used to implement own function calls on the eventhandler
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn event_handler(
    event: *mut blpapi_Event_t,
    session: *mut blpapi_Session_t,
    userData: *mut c_void,
) {
    print!("{:?}", event);
    print!("{:?}", session);
    print!("{:?}", userData);
}
