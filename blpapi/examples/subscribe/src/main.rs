use blpapi::{
    Error, RefData, options,
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
    subscription_list::Subscription,
};
use chrono::NaiveDateTime;

#[derive(Debug, Default, RefData)]
struct Data {
    rt_time_of_trade: Option<NaiveDateTime>,
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

    // Creating a subscription
    let options = options!(interval = 5);
    let bay_sup = Subscription {
        ticker: String::from("BAYN GY Equity"),
        fields: vec!["time", "last_price", "bid", "ask"],
        options: Some(options),
    };

    let options_apl = options!(interval = 1);
    let apple_sub = Subscription {
        ticker: String::from("AAPL US Equity"),
        fields: vec!["time"],
        options: Some(options_apl),
    };

    let all_sub = vec![bay_sup, apple_sub];
    dbg!(&all_sub);

    // let tickers = vec!["BAYN GY Equity", "AAPL US Equity"];
    // let options = options!(delayed = "");
    // let overrides = None;
    // session.subscribe::<Data>(tickers, overrides, Some(options))?;

    session.subscribe_vec::<Data>(all_sub)?;

    Ok(())
}
