use blpapi::{
    Error, RefData,
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
    time_series::{Fill, HistOptions, PeriodicityAdjustment, PeriodicitySelection, TradingDays},
};

#[derive(Debug, Default, RefData)]
struct Data {
    last_price: Option<f64>,
    bid: Option<f64>,
    ask: Option<f64>,
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

    let tickers = vec!["AAPL US Equity", "BAYN GY Equity"];
    let interval = 10;
    let options = None;
    let overrides = None;

    session.subscribe::<Data>(tickers, interval, overrides, options)?;

    Ok(())
}
