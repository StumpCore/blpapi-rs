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
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_PAGE_DATA: &str = "//blp/pagedata";
pub const BLPAPI_DEFAULT_SERVICE_IDENTIFIER_STATIC_MKT: &str = "//blp/staticmktdata";
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
pub const BLPAPI_DEFAULT_INSTRUMENT_LIST_REQUEST: &str = "instrumentListRequest";
pub const BLPAPI_DEFAULT_CATEGORIZED_FIELD_SEARCH_DATA_REQUEST: &str =
    "categorizedFieldSearchRequest";
pub const BLPAPI_DEFAULT_CURVED_LIST_DATA_REQUEST: &str = "curveListRequest";
pub const BLPAPI_DEFAULT_GOVT_LIST_DATA_REQUEST: &str = "govtListRequest";
pub const BLPAPI_DEFAULT_FIELD_INFO_REQUEST_DATA_REQUEST: &str = "FieldInfoRequest";
pub const BLPAPI_DEFAULT_FIELD_LIST_REQUEST_DATA_REQUEST: &str = "FieldListRequest";
pub const BLPAPI_DEFAULT_FIELD_SEARCH_REQUEST_DATA_REQUEST: &str = "FieldSearchRequest";
pub const BLPAPI_DEFAULT_STUDY_DATA_REQUEST: &str = "StudyRequest";
pub const BLPAPI_DEFAULT_BEQS_DATA_REQUEST: &str = "BeqsRequest";

// BDIB Function Constants
pub const BLPAPI_DEFAULT_BDIB_TRADE: &str = "TRADE";
pub const BLPAPI_DEFAULT_BDIB_BID: &str = "BID";
pub const BLPAPI_DEFAULT_BDIB_ASK: &str = "ASK";
pub const BLPAPI_DEFAULT_BDIB_BID_BEST: &str = "BID_BEST";
pub const BLPAPI_DEFAULT_BDIB_ASK_BEST: &str = "ASK_BEST";
pub const BLPAPI_DEFAULT_BDIB_BID_YIELD: &str = "BID_YIELD";
pub const BLPAPI_DEFAULT_BDIB_ASK_YIELD: &str = "ASK_YIELD";
pub const BLPAPI_DEFAULT_BDIB_MID_PRICE: &str = "MID_PRICE";
pub const BLPAPI_DEFAULT_BDIB_AT_TRADE: &str = "AT_TRADE";
pub const BLPAPI_DEFAULT_BDIB_BEST_BID: &str = "BEST_BID";
pub const BLPAPI_DEFAULT_BDIB_BEST_ASK: &str = "BEST_ASK";
pub const BLPAPI_DEFAULT_BDIB_SETTLE: &str = "SETTLE";
pub const BLPAPI_DEFAULT_ALL: &str = "All";
pub const BLPAPI_DEFAULT_STATIC: &str = "Static";
pub const BLPAPI_DEFAULT_REALTIME: &str = "RealTime";
pub const BLPAPI_YELLOW_FILTER_NONE: &str = "YK_FILTER_NONE";
pub const BLPAPI_YELLOW_FILTER_CMDT: &str = "YK_FILTER_CMDT";
pub const BLPAPI_YELLOW_FILTER_EQTY: &str = "YK_FILTER_EQTY";
pub const BLPAPI_YELLOW_FILTER_MUNI: &str = "YK_FILTER_MUNI";
pub const BLPAPI_YELLOW_FILTER_PRFD: &str = "YK_FILTER_PRFD";
pub const BLPAPI_YELLOW_FILTER_CLNT: &str = "YK_FILTER_CLNT";
pub const BLPAPI_YELLOW_FILTER_MMKT: &str = "YK_FILTER_MMKT";
pub const BLPAPI_YELLOW_FILTER_GOVT: &str = "YK_FILTER_GOVT";
pub const BLPAPI_YELLOW_FILTER_CORP: &str = "YK_FILTER_CORP";
pub const BLPAPI_YELLOW_FILTER_INDX: &str = "YK_FILTER_INDX";
pub const BLPAPI_YELLOW_FILTER_CURR: &str = "YK_FILTER_CURR";
pub const BLPAPI_YELLOW_FILTER_MTGE: &str = "YK_FILTER_MTGE";
pub const BLPAPI_LNG_OVERRIDE_NONE: &str = "LANG_OVERRIDE_NONE";
pub const BLPAPI_LNG_OVERRIDE_NONE_1: &str = "LANG_OVERRIDE_NONE_1";
pub const BLPAPI_LNG_OVERRIDE_NONE_2: &str = "LANG_OVERRIDE_NONE_2";
pub const BLPAPI_LNG_OVERRIDE_NONE_3: &str = "LANG_OVERRIDE_NONE_3";
pub const BLPAPI_LNG_OVERRIDE_NONE_4: &str = "LANG_OVERRIDE_NONE_4";
pub const BLPAPI_LNG_OVERRIDE_NONE_5: &str = "LANG_OVERRIDE_NONE_5";
pub const BLPAPI_LNG_OVERRIDE_ENGLISH: &str = "LANG_OVERRIDE_ENGLISH";
pub const BLPAPI_LNG_OVERRIDE_KANJI: &str = "LANG_OVERRIDE_KANJI";
pub const BLPAPI_LNG_OVERRIDE_FRENCH: &str = "LANG_OVERRIDE_FRENCH";
pub const BLPAPI_LNG_OVERRIDE_GERMAN: &str = "LANG_OVERRIDE_GERMAN";
pub const BLPAPI_LNG_OVERRIDE_SPANISH: &str = "LANG_OVERRIDE_SPANISH";
pub const BLPAPI_LNG_OVERRIDE_PORTUGUESE: &str = "LANG_OVERRIDE_PORTUGUESE";
pub const BLPAPI_LNG_OVERRIDE_ITALIAN: &str = "LANG_OVERRIDE_ITALIAN";
pub const BLPAPI_LNG_OVERRIDE_CHINESE_TRAD: &str = "LANG_OVERRIDE_CHINESE_TRAD";
pub const BLPAPI_LNG_OVERRIDE_KOREAN: &str = "LANG_OVERRIDE_KOREAN";
pub const BLPAPI_LNG_OVERRIDE_CHINESE_SIMP: &str = "LANG_OVERRIDE_CHINESE_SIMP";
pub const BLPAPI_LNG_OVERRIDE_RUSSIAN: &str = "LANG_OVERRIDE_RUSSIAN";
pub const BLPAPI_SECURITY_TYPE_INVALID: &str = "INVALID";
pub const BLPAPI_SECURITY_TYPE_UNASSIGNED: &str = "UNASSIGNED";
pub const BLPAPI_SECURITY_TYPE_IRS: &str = "IRS";
pub const BLPAPI_SECURITY_TYPE_GOVT: &str = "GOVT";
pub const BLPAPI_SECURITY_TYPE_AGENCY: &str = "AGENCY";
pub const BLPAPI_SECURITY_TYPE_MUNI: &str = "MUNI";
pub const BLPAPI_SECURITY_TYPE_CORP: &str = "CORP";
pub const BLPAPI_SECURITY_TYPE_MTGE: &str = "MTGE";
pub const BLPAPI_SECURITY_TYPE_MMKT: &str = "MMKT";
pub const BLPAPI_SECURITY_TYPE_CURNCY: &str = "CURNCY";
pub const BLPAPI_SECURITY_TYPE_COMDTY: &str = "COMDTY";
pub const BLPAPI_SECURITY_SUBTYPE_INVALID: &str = "INVALID";
pub const BLPAPI_SECURITY_SUBTYPE_UNASSIGNED: &str = "UNASSIGNED";
pub const BLPAPI_SECURITY_SUBTYPE_SENIOR: &str = "SENIOR";
pub const BLPAPI_SECURITY_SUBTYPE_SUBORDINATED: &str = "SUBORDINATED";
pub const BLPAPI_SECURITY_SUBTYPE_ZERO: &str = "ZERO";
pub const BLPAPI_SECURITY_SUBTYPE_OIS: &str = "OIS";
pub const BLPAPI_SECURITY_SUBTYPE_INFLATION: &str = "INFLATION";
pub const BLPAPI_SECURITY_SUBTYPE_SPREAD: &str = "SPREAD";
pub const BLPAPI_SECURITY_SUBTYPE_CDS: &str = "CDS";
pub const BLPAPI_SECURITY_SUBTYPE_RATE: &str = "RATE";
pub const BLPAPI_SECURITY_SUBTYPE_SECTOR: &str = "SECTOR";
pub const BLPAPI_SECURITY_SUBTYPE_ISSUER: &str = "ISSUER";

pub const BDH_DATE_REGEX: &str = r"[0-9]{4}[0-1]{1}[0-9]{1}[0-3]{1}[0-9]{1}";
pub const BDH_DATETIME_REGEX: &str =
    r"^[0-9]{4}[0-1][0-9][0-3][0-9]T[0-2][0-9][0-5][0-9][0-5][0-9]$";

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
