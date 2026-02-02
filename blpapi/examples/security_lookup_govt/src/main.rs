use blpapi::{
    Error,
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

    let query = "bond";
    let max_results = 10;
    let partial_match = true;
    let ticker = None;

    // Example
    let data = session.lookup_security_govt(query, max_results, partial_match, ticker)?;
    println!("{:#?}", data);
    Ok(())
}
