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

    // Example
    let data = session.field_info::<Data>()?;
    for entry in data {
        println!(
            "{:#?}: {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
            entry.id,
            entry.mnemonic,
            entry.desc,
            entry.data_type,
            entry.field_type,
            entry.field_category,
            entry.field_default_formatting,
            entry.field_documentation,
            entry.other,
        );
    }

    Ok(())
}
