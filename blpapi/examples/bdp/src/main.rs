use blpapi::{
    Error, RefData, overrides,
    overrides::BdpOptions,
    ref_data::BulkElement,
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
};

#[derive(Debug, Default, RefData)]
struct Data {
    ticker: String,
    // dvd_hist_all: BulkElement,
    crncy_adj_px_last: f64,
    ds002: String,
    // crncy: String,
    // px_last: f64,
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
    // println!("{:#?}", session);

    let tickers = &[
        // "IBM US Equity",
        // "MSFT US Equity",
        // "3333 HK Equity",
        "AAPL US Equity",
    ];
    // Static Market Data Service (normilzed Data) true or false
    let static_mkt = false;

    let overrides = None;
    let data = session.bdp::<Data>(tickers, overrides, static_mkt, None)?;
    // Without Override
    println!("{:#?}", data);

    let overrides = None;
    let options = BdpOptions::new()
        .use_utc(false)
        .return_eids(true)
        .start_sequence_number(1);
    let data = session.bdp::<Data>(tickers, overrides, static_mkt, Some(options))?;
    // Without Override but options
    println!("{:#?}", data);

    let overrides = overrides!(EQY_FUND_CRNCY = "EUR");
    let overrides = Some(overrides);
    let data = session.bdp::<Data>(tickers, overrides, static_mkt, None)?;
    println!("{:#?}", data);

    Ok(())
}
