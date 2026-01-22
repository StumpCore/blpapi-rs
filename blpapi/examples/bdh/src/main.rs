use blpapi::{
    Error, RefData,
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
    time_series::HistOptions,
};

#[derive(Debug, Default, RefData)]
struct Data {
    px_last: f64,
    volatitlity_30d: f64,
}
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

    let tickers = &[
        "IBM US Equity",
        "MSFT US Equity",
        "3333 HK Equity",
        "/cusip/912828GM6@BGN",
    ];

    let options = HistOptions::new("20191001", "20191010");
    let data = session.bdh::<Data>(tickers, options)?;
    for entry in data {
        println!("{}: {:?} {:?}", entry.ticker, entry.date, entry.data);
    }

    Ok(())
}
