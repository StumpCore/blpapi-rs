use blpapi::{
    session::{HistOptions, Session, SessionBuilder, SessionSync},
    session_options::SessionOptions,
    Error, RefData,
};

#[derive(Debug, Default, RefData)]
struct Data {
    px_last: f64,
    volatitlity_30d: f64,
}
fn start_session() -> Result<Session, Error> {
    let s_opt = SessionOptions::default();
    let mut session = SessionBuilder::default().options(s_opt).build();
    session.start()?;
    Ok(session)
}

pub fn main() -> Result<(), Error> {
    env_logger::init();

    //let mut args = std::env::args();
    //let host = args.nth(1).unwrap_or("127.0.0.1".to_owned());
    //let port = args.next().unwrap_or("8194".to_owned()).parse().unwrap();

    println!("creating session");
    println!("creating session");
    // let mut session = SessionSync::new()?;
    let mut session = start_session()?;
    println!("{:#?}", session);

    let securities = &[
        "IBM US Equity",
        "MSFT US Equity",
        "3333 HK Equity",
        "/cusip/912828GM6@BGN",
    ];

    let options = HistOptions::new("20190101", "20191010");
    let data = session.hist_data_sync::<Data>(securities, options)?;
    for (sec, timeserie) in data {
        println!("{}: {:?} {:?}", sec, timeserie.dates, timeserie.values);
    }

    Ok(())
}
