use blpapi::{
    Error, RefData,
    auth_options::{AuthOptionsBuilder, AuthUserBuilder},
    correlation_id::{CorrelationId, CorrelationIdBuilder},
    event::SubscriptionMsg,
    options,
    session::{Session, SessionBuilder},
    session_options::{SessionOptions, SessionOptionsBuilder},
    subscription_list::Subscription,
};
use chrono::NaiveDateTime;

#[derive(Debug, Default, RefData)]
struct Data {
    rt_time_of_trade: Option<NaiveDateTime>,
    last_price: Option<f64>,
    bid: Option<f64>,
    ask: Option<f64>,
    evt_trade_time_rt: Option<f64>,
    last_trade_price_time_today_rt: Option<f64>,
    bid_update_stamp_rt: Option<f64>,
    ask_update_stamp_rt: Option<f64>,
    bloomberg_event_time_rt: Option<f64>,
    bloomberg_send_time_rt: Option<f64>,
}

fn start_session() -> Result<Session, Error> {
    let host_name = "EX1234";

    // Create new AuthUser
    let auth_user = AuthUserBuilder::default().build();
    let auth_opt = AuthOptionsBuilder::default()
        .set_auth_user(auth_user)
        .build();
    let id = CorrelationIdBuilder::default()
        .set_value_type(blpapi::correlation_id::OwnValueType::IntValue(0))
        .build();

    // Creation SessionOptionsBuilder
    let ses_op_builder = SessionOptionsBuilder::default()
        .set_server_host(host_name)
        .set_auth_options(auth_opt)
        .set_correlation_id(id);
    let s_opt = ses_op_builder.build();
    let session = SessionBuilder::default().options(s_opt).build();
    Ok(session)
}

pub fn main() -> Result<(), Error> {
    env_logger::init();

    println!("creating session");
    let mut session = start_session()?;
    session.start()?;

    // Creating a subscription
    let options = options!(interval = 5);
    let bay_sup = Subscription {
        ticker: String::from("BAYN GY Equity"),
        fields: vec!["rt_time_of_trade", "last_price", "bid", "ask"],
        options: Some(options),
    };

    let all_sub = vec![bay_sup];

    let rx = session.start_subscription::<Data>();
    session.subscribe::<Data>(all_sub)?;

    for msg in rx {
        if let SubscriptionMsg::Data { ticker, data } = msg {
            println!("{}:{:?}", ticker, data.data);
        }
    }

    Ok(())
}
