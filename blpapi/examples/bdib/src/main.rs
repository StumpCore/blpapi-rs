use blpapi::{
    Error, RefData,
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
    time_series::{
        Fill, HistIntradayOptions, HistOptions, PeriodicityAdjustment, PeriodicitySelection,
        TradingDays,
    },
};

#[derive(Debug, Default, RefData)]
struct Data {
    trade: f64,
    bid: f64,
    bid_best: f64,
    best_bid: f64,
    ask: f64,
    ask_best: f64,
    best_ask: f64,
    mid_price: f64,
    at_trade: f64,
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

    let ticker = String::from("IBM US Equity");

    // Example
    let options = HistIntradayOptions::new("20181017T000000", "20181019T000000");

    let data = session.bdib::<Data>(ticker, options)?;
    for entry in data {
        println!("{}: {:?} {:?}", entry.ticker, entry.date, entry.data);
    }

    Ok(())
}
