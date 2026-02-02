use blpapi::{
    Error,
    data_series::FieldTypes,
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
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

    // Block of 100 Fields
    let block = 1;

    // let field_t = FieldTypes::Static;
    // let field_t = FieldTypes::RealTime;
    let field_t = FieldTypes::All;

    // Example
    let data = session.field_list(block, field_t)?;
    for entry in data {
        println!("{:?}", entry);
    }

    Ok(())
}
