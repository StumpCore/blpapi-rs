use blpapi_sys::blpapi_AuthToken_t;

#[derive(Debug)]
struct AuthToken {
    pub(crate) token: *mut *mut blpapi_AuthToken_t,
    pub token_name: String,
}
