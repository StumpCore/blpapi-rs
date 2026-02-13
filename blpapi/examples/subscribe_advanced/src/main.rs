use blpapi::{
    Error, RefData,
    event::SubscriptionMsg,
    options,
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
    // let options = options!(interval = 5);
    let bay_sup = Subscription {
        ticker: String::from("BAYN GY Equity"),
        fields: vec!["rt_time_of_trade", "last_price", "bid", "ask"],
        options: None,
    };

    // let options_apl = options!(interval = 5);
    let apple_sub = Subscription {
        ticker: String::from("BAS GR Equity"),
        fields: vec!["rt_time_of_trade", "bid"],
        options: None,
    };

    let all_sub = vec![bay_sup, apple_sub];

    let rx = session.start_subscription::<Data>();
    session.subscribe::<Data>(all_sub)?;
    // 1. Move the printing to a background thread
    std::thread::spawn(move || {
        for msg in rx {
            if let SubscriptionMsg::Data { ticker, data } = msg {
                println!("{}: {:?}", ticker, data.data);
            }
        }
    });

    // 2. The main thread stays free to take commands
    println!("Press Enter to change BAS fields to only 'last_price'...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let change = vec![Subscription {
        ticker: String::from("BAS GR Equity"),
        fields: vec!["rt_time_of_trade", "last_price", "ask"],
        options: None,
    }];

    session.resubscribe::<Data>(change)?;

    println!("Resubscribe sent! Keeping program alive...");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
