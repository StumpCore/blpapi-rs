use blpapi::{
    Error,
    data_series::{Language, YellowKey},
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
};

fn start_session() -> Result<Session, Error> {
    let s_opt = SessionOptions::default();
    let session = SessionBuilder::default().options(s_opt).build();
    Ok(session)
}

pub fn main() -> Result<(), Error> {
    env_logger::init();

    println!("creating session");
    let mut session = start_session()?;
    session.start()?;
    println!("{:#?}", session);

    let name = "Apple";
    let max_results = 10;

    // Example
    let data = session.lookup_security(name, max_results, None, None)?;
    println!("{:#?}", data);

    let name = "Apple";
    let max_results = 10;
    let yellow_key = Some(YellowKey::Eqty);
    let lng_override = Some(Language::Kanji);

    // Example
    let data = session.lookup_security(name, max_results, yellow_key, lng_override)?;
    println!("{:#?}", data);

    Ok(())
}
