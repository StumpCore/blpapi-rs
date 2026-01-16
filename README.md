# blpapi

A rust wrapper for Bloomberg blpapi (based on the tafia/blpapi-rs crate by tafia).
This is work in progress and plans to get on parity with the C++ API. 
Currently, the library is not implementing the async features.

Tested on Windows only (DesktopApi). 
Compiles on Linux and Windows.
Tested version: 3.25.11

## Installation

1. Install C/C++ BLPAPI. (Download and extract the file from https://www.bloomberg.com/professional/support/api-library/)
2. Set `BLPAPI_LIB` environment variable
    a. On windows: *<Extract path>\lib*
    b. On linux: *<Extract path>/Linux*
3. Example: 
    a. *C:\blp\DAPI\blpapi_cpp_3.25.7.1*
    b. *C:\blp\DAPI\blpapi_cpp_3.25.7.1\Linux*

## Examples

```sh
# Cargo.toml
[dependencies]
blpapi = { version = "0.0.1", features = [ "derive", "dates" ] }
```

### Reference data

```rust
use blpapi::{
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
    Error, RefData,
};

#[derive(Debug, Default, RefData)]
struct Data {
    crncy: String,
    id_bb: String,
    ticker: String,
    market_sector: Option<String>,
    px_last: f64,
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
        "IBM US Equity",
        "MSFT US Equity",
        "3333 HK Equity",
        "/cusip/912828GM6@BGN",
    ];

    let data = session.ref_data_sync::<Data>(securities)?;
    println!("{:#?}", data);

    Ok(())
}
```

```Shell
{
    "AAPL US Equity": Data {
        crncy: "USD",
        id_bb: "037833100",
        ticker: "AAPL",
        market_sector: Some(
            "2",
        ),
        px_last: 258.21,
        crncy_adj_px_last: 222.3648,
        ds002: "APPLE INC",
    },
}

```



### Historical data

```rust
use blpapi::{
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
    time_series::HistOptions,
    Error, RefData,
};

#[derive(Debug, Default, RefData)]
struct Data {
    px_last: f64,
    volatitlity_30d: f64,
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

    let securities = &[
        "IBM US Equity",
        "MSFT US Equity",
        "3333 HK Equity",
        "/cusip/912828GM6@BGN",
    ];

    let options = HistOptions::new("20191001", "20191010");
    let data = session.hist_data_sync::<Data>(securities, options)?;
    for (sec, timeserie) in data {
        println!("{}: {:?} {:?}", sec, timeserie.dates, timeserie.values);
    }

    Ok(())
}
```
