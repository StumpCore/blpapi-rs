use crate::element::Element;
use regex::Error as RegexErr;
use std::{fmt::write, num::ParseIntError, string::ParseError};

/// Error converted from `c_int`
#[derive(Debug)]
pub enum Error {
    InternalError,
    InvalidUser,
    NotLoggedIn,
    InvalidDisplay,
    EntitlementRefresh,
    InvalidAuthToken,
    InvalidAuthenticationOption,
    InvalidDate,
    InvalidDatetime,
    ExpiredAuthToken,
    TokenInUse,
    /// Generic blpapi error return
    Generic(i32),
    /// Some element were not found
    NotFound(String),
    /// Regex Error
    RegexErr(RegexErr),
    /// String Parse Error
    ParseError(ParseError),
    /// Usize Parse Error
    ParseIntError(ParseIntError),
    /// Constant List Error
    ConstantList,
    /// Constant Error
    Constant,
    /// A securityError element was found
    Security {
        security: String,
        category: String,
        sub_category: Option<String>,
        message: String,
    },
    /// A fieldError element was found
    Field {
        id: String,
        message: String,
    },
    /// Error for a Service
    Service,
    /// Error for a Session
    Session,

    /// Error for SessionOption Setup
    SessionOptionError {
        struct_name: String,
        func_name: String,
        msg: String,
    },
    /// Error for Event Dispatcher
    EventDispatcher,
    /// Error for Identity
    Identity,
    /// Error for Schema
    Schema,
    /// Error for SchemaType
    SchemaType,
    /// Error for SubscriptionStatus
    SubscriptionStatus,
    /// Timeout event
    TimeOut,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::ParseError(msg) => write!(f, "Parse Error: {:?}", msg),
            Error::ParseIntError(msg) => write!(f, "Parse Int Error: {:?}", msg),
            Error::RegexErr(msg) => write!(f, "Regex Error: {:?}", msg),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ParseError(e) => Some(e),
            Error::ParseIntError(e) => Some(e),
            Error::RegexErr(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::ParseError(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseIntError(e)
    }
}

impl From<RegexErr> for Error {
    fn from(e: RegexErr) -> Self {
        Error::RegexErr(e)
    }
}

impl Error {
    /// Check if response is an error(!=0)
    pub fn check(res: i32) -> Result<(), Error> {
        if res == 0 {
            Ok(())
        } else {
            match res {
                100 => Err(Error::InternalError),
                101 => Err(Error::InvalidUser),
                102 => Err(Error::NotLoggedIn),
                103 => Err(Error::InvalidDisplay),
                105 => Err(Error::EntitlementRefresh),
                106 => Err(Error::InvalidAuthToken),
                107 => Err(Error::ExpiredAuthToken),
                108 => Err(Error::TokenInUse),
                109 => Err(Error::InvalidAuthenticationOption),
                110 => Err(Self::struct_error(
                    "Default Error",
                    "Default Function",
                    "Something went wrong",
                )),
                111 => Err(Error::EventDispatcher),
                112 => Err(Error::Identity),
                113 => Err(Error::SubscriptionStatus),
                114 => Err(Error::Session),
                115 => Err(Error::ConstantList),
                116 => Err(Error::Constant),
                117 => Err(Error::Schema),
                118 => Err(Error::Service),
                119 => Err(Error::SchemaType),
                _ => {
                    log::debug!("Unrecognized error code: {}", res);
                    Err(Error::Generic(res))
                }
            }
        }
    }

    /// Create a security error
    pub(crate) fn security(security: String, element: Element) -> Error {
        let category = element
            .get_element("category")
            .and_then(|e| e.get_at(0))
            .unwrap_or_default();
        let sub_category = element.get_element("subcategory").and_then(|e| e.get_at(0));
        let message = element
            .get_element("message")
            .and_then(|e| e.get_at(0))
            .unwrap_or_default();
        Error::Security {
            security,
            category,
            sub_category,
            message,
        }
    }

    /// Create a field error
    pub(crate) fn field(element: Element) -> Error {
        let id = element
            .get_element("id")
            .and_then(|e| e.get_at(0))
            .unwrap_or_default();
        let message = element
            .get_element("message")
            .and_then(|e| e.get_at(0))
            .unwrap_or_default();
        Error::Field { id, message }
    }

    /// Create a struct error
    pub fn struct_error<T: Into<String>>(struct_name: T, func_name: T, msg: T) -> Error {
        let struct_name = struct_name.into();
        let func_name = func_name.into();
        let msg = msg.into();

        Error::SessionOptionError {
            struct_name,
            func_name,
            msg,
        }
    }
}
