use blpapi::{
    Error,
    data_series::{CurveOptions, Language, SecuritySubType, SecurityType, YellowKey},
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

    let max_results = 10;
    let options = CurveOptions::new("Gold");
    // let options = CurveOptions::new("Gold")
    //     .bbg_id("YCCD1016")
    //     .country("US")
    //     .currency("USD")
    //     .curve_id("CD1016")
    //     .security_type(SecurityType::Corp)
    //     .security_subtype(SecuritySubType::Cds);

    // Example
    let data = session.lookup_security_curved(max_results, options)?;
    println!("{:#?}", data);

    Ok(())
}
