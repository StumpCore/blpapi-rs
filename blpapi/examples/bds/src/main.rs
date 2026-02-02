use blpapi::{
    Error, RefData, overrides,
    ref_data::BulkElement,
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
};

#[derive(Debug, Default, RefData)]
struct Data {
    ticker: String,
    dvd_hist_all: BulkElement,
}

fn start_session() -> Result<Session, Error> {
    let s_opt = SessionOptions::default();
    let mut session = SessionBuilder::default().options(s_opt).build();
    session.start()?;
    Ok(session)
}

pub fn main() -> Result<(), Error> {
    env_logger::init();

    println!("creating session");
    let mut session = start_session()?;
    println!("{:#?}", session);

    let tickers = &["AAPL US Equity"];
    let static_mkt = false;
    let overrides = overrides!(dvd_start_dt = "20180101", dvd_end_dt = "20180531",);
    let overrides = Some(overrides);
    let data = session.bdp::<Data>(tickers, overrides, static_mkt)?;
    println!("{:#?}", data);

    Ok(())
}
