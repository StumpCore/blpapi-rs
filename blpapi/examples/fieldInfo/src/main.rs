use blpapi::{
    Error, RefData,
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
};

#[derive(Debug, Default, RefData)]
struct Data {
    // dy886: f64,
    // px_last: f64,
    // ds002: f64,
    // volume: f64,
    dv014: f64,
    // rq005: f64,
    // pr959: f64,
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
    let data = session.field_info::<Data>(None, None)?;
    for entry in data {
        println!(
            "{:#?}: \n 
    mnemonic:{:#?}\n 
    description:{:#?}\n 
    dataType: {:#?}\n
    field_type: {:#?}\n
    field_category: {:#?}\n
    field_default_formatting: {:#?}\n
    field_error: {:#?}\n
    field_property: {:#?}\n
    other: {:#?}\n
    overrides: {:#?}\n
            ",
            entry.id,
            entry.mnemonic,
            entry.desc,
            entry.data_type,
            entry.field_type,
            entry.field_category,
            entry.field_default_formatting,
            entry.field_error,
            entry.field_property,
            entry.other,
            entry.overrides,
            // entry.field_documentation,
        );
    }

    Ok(())
}
