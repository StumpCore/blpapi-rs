use std::hash::BuildHasher;

use blpapi::{
    core::{event_handler, BLPAPI_DEFAULT_SERVICE_IDENTIFIER_REFDATA},
    event_dispatcher::EventDispatcherBuilder,
    session::{EventHandler, Session, SessionBuilder},
    session_options::SessionOptions,
    Error,
};

fn start_session() -> Result<Session, Error> {
    let s_opt = SessionOptions::default();
    let mut session = SessionBuilder::default().options(s_opt).build();
    session.start()?;
    Ok(session)
}

#[test]
fn test_blpapi_session() -> Result<(), Error> {
    let _s = start_session()?;
    Ok(())
}

#[test]
fn test_blpapi_session_with_service() -> Result<(), Error> {
    let mut s = start_session()?;
    let serv = BLPAPI_DEFAULT_SERVICE_IDENTIFIER_REFDATA;
    s.open_service(serv)?;
    Ok(())
}

#[test]
fn test_blpapi_session_get_service() -> Result<(), Error> {
    let mut s = start_session()?;
    let serv = BLPAPI_DEFAULT_SERVICE_IDENTIFIER_REFDATA;
    s.open_service(serv)?;
    let get_serv = s.get_service(serv)?.name();
    assert_eq!(get_serv, serv.to_string());
    Ok(())
}

#[test]
fn test_blpapi_session_create_identity() -> Result<(), Error> {
    let mut s = start_session()?;
    let serv = BLPAPI_DEFAULT_SERVICE_IDENTIFIER_REFDATA;
    s.open_service(serv)?;
    let id = s.create_identity()?;
    assert_eq!(id.seat_type, -1);
    Ok(())
}

#[test]
fn test_blpapi_session_with_option_handler() -> Result<(), Error> {
    let s_opt = SessionOptions::default();
    let disp = EventDispatcherBuilder::new(3).build();
    let handler: EventHandler = Some(event_handler);
    let s_async = SessionBuilder::default();
    let mut s_async = s_async
        .options(s_opt)
        .dispatcher(disp)
        .handler(handler)
        .build();
    s_async.start()?;
    Ok(())
}
