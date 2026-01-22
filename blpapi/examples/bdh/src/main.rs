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

    let tickers = &["SHCOMP Index"];

    // Example
    let options = HistOptions::new("20191001", "20191010");
    let data = session.bdh::<Data>(tickers, options)?;
    for entry in data {
        println!("{}: {:?} {:?}", entry.ticker, entry.date, entry.data);
    }

    // Example for excel compatible output
    let start_date = "20180926";
    let end_date = "20181020";
    let per_sel = PeriodicitySelection::Weekly;
    let fill = Fill::PreviousValue;
    let days = TradingDays::AllCalendarDays;

    let options = HistOptions::new(start_date, end_date)
        .periodicity_selection(per_sel)
        .fill(fill)
        .days(days);

    let data = session.bdh::<Data>(tickers, options)?;
    for entry in data {
        println!("{}: {:?} {:?}", entry.ticker, entry.date, entry.data);
    }

    // Example for adjustement for dividends and splits
    let tickers = &["AAPL US Equity"];
    let start_date = "20140605";
    let end_date = "20140610";
    let cash_adj_norm = false;
    let cash_adj_abnorm = false;
    let capital_chng = false;

    let options = HistOptions::new(start_date, end_date)
        .cash_adj_normal(cash_adj_norm)
        .cash_adj_abnormal(cash_adj_abnorm)
        .cap_chg(capital_chng);

    let data = session.bdh::<Data>(tickers, options)?;
    for entry in data {
        println!("{}: {:?} {:?}", entry.ticker, entry.date, entry.data);
    }

    Ok(())
}
