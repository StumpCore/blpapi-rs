use blpapi::request::{RequestBuilder, RequestTypes};
use blpapi::service::{BlpServiceStatus, BlpServices};
use blpapi::{
    abstract_session::AbstractSession,
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
fn test_session_start() -> Result<(), Error> {
    let session = start_session()?;
    drop(session);
    Ok(())
}

#[test]
fn test_session_operation() -> Result<(), Error> {
    let session = start_session()?;
    drop(session);
    Ok(())
}

#[test]
fn test_session_with_service() -> Result<(), Error> {
    let mut s = start_session()?;
    let serv = &BlpServices::ReferenceData;
    s.open_service(serv)?;
    drop(s);
    Ok(())
}

#[test]
fn test_session_get_service() -> Result<(), Error> {
    let mut s = start_session()?;
    let serv = &BlpServices::ReferenceData;
    let serv_str: &str = serv.into();
    s.open_service(serv)?;
    let get_serv = s.get_service(serv)?.name();
    assert_eq!(get_serv, serv_str.to_string());
    drop(s);
    Ok(())
}

#[test]
fn test_session_create_identity() -> Result<(), Error> {
    let mut s = start_session()?;
    let serv = &BlpServices::ReferenceData;
    s.open_service(serv)?;
    let id = s.create_identity()?;
    assert_eq!(id.seat_type, 0);
    drop(s);
    Ok(())
}

#[test]
fn test_session_with_option_handler() -> Result<(), Error> {
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
    dbg!(&s_async);
    drop(s_async);
    Ok(())
}

#[test]
fn test_session_create_request() -> Result<(), Error> {
    let mut s = start_session()?;
    let service = BlpServices::ReferenceData;
    let request = RequestTypes::ReferenceData;
    let _req_res = s.create_request(service, request)?;
    Ok(())
}

#[test]
fn test_session_create_service() -> Result<(), Error> {
    let mut s = start_session()?;
    let blp_service = BlpServices::ReferenceData;
    let service = s.get_service(&blp_service)?;
    let status = service.status.clone();
    assert_eq!(status, BlpServiceStatus::InActive);
    dbg!(service);
    s.open_service(&blp_service)?;
    let service = s.get_service(&blp_service)?;
    assert_eq!(service.status, BlpServiceStatus::Active);
    dbg!(service);
    Ok(())
}

#[test]
fn test_session_create_service_name() -> Result<(), Error> {
    let mut s = start_session()?;
    let blp_service = BlpServices::ReferenceData;
    s.open_service(&blp_service)?;
    let service = s.get_service(&blp_service)?;
    let name = service.name();
    let serv_name: &str = (&blp_service).into();
    assert_eq!(name, serv_name);
    Ok(())
}

#[test]
fn test_session_create_service_auth_name() -> Result<(), Error> {
    let mut s = start_session()?;
    let blp_service = BlpServices::ReferenceData;
    s.open_service(&blp_service)?;
    let service = s.get_service(&blp_service)?;
    let name = service.authorization_name();
    dbg!(name);
    Ok(())
}
