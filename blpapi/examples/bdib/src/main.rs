use blpapi::{
    Error,
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
    time_series::{HistIntradayOptions, TickTypes},
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

    let ticker = String::from("BAYN GY Equity");
    let start_dt = "20260123T090000";
    let end_dt = "20260123T170000";

    // Example
    let value = true;
    let options = HistIntradayOptions::new(start_dt, end_dt)
        .cond_codes(value)
        .exch_code(value)
        .non_plottable_events(value)
        .brkr_codes(value)
        .rps_codes(value)
        .trade_time(value)
        .action_codes(value)
        .show_yield(value)
        .spread_price(value)
        .upfront_price(value)
        .indicator_codes(value)
        .return_eids(value)
        .bic_mic_codes(value)
        .eq_ref_price(value)
        .xdf_fields(value)
        .trade_id(value);

    let tick_types = vec![TickTypes::Trade, TickTypes::Ask];

    let data = session.bdib(ticker, tick_types, options)?;
    for entry in data {
        println!("{}: {:?} {:?}", entry.ticker, entry.date, entry.data);
    }

    Ok(())
}
