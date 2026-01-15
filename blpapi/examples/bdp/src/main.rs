use blpapi::{
    Error, RefData, overrides,
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
};

#[derive(Debug, Default, RefData)]
struct Data {
    crncy: String,
    id_bb: String,
    ticker: String,
    market_sector: Option<String>,
    px_last: f64,
    eqy_weighted_avg_px: f64,
    ds002: String,
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

    let securities = &[
        // "IBM US Equity",
        // "MSFT US Equity",
        // "3333 HK Equity",
        // "/cusip/912828GM6@BGN",
        "AAPL US Equity",
    ];

    let overrides = None;

    let data = session.bdp::<Data>(securities, overrides)?;
    // Without Override
    println!("{:#?}", data);

    let overrides = overrides!(VWAP_Dt = "20181224");
    let overrides = Some(overrides);
    let data = session.bdp::<Data>(securities, overrides)?;
    // With override
    println!("{:#?}", data);

    Ok(())
}
