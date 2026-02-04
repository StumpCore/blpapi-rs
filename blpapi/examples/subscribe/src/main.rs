use blpapi::{
    Error, RefData,
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
    time_series::{Fill, HistOptions, PeriodicityAdjustment, PeriodicitySelection, TradingDays},
};

#[derive(Debug, Default, RefData)]
struct Data {
    px_last: f64,
    high: f64,
    low: f64,
    open: f64,
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

    let tickers = &["AAPL US"];
    let interval = 10;
    let options = None;
    let overrides = None;

    let data = session.subscribe::<Data>(tickers, interval, overrides, options)?;
    for entry in data {
        println!("{}: {:#?} ", entry.ticker, entry.data);
    }

    Ok(())
}
