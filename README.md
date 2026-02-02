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
#### Requesting BDP Data without overrides.
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
...
{
    "AAPL US Equity": Data {
        crncy: "USD",
        id_bb: "037833100",
        ticker: "AAPL",
        market_sector: Some(
            "2",
        ),
        px_last: 258.21,
        crncy_adj_px_last: 258.21,
        ds002: "APPLE INC",
    },
}

```

#### Requesting BDP Data with overrides.
```rust
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
    crncy_adj_px_last: f64,
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
        "AAPL US Equity",
    ];

    let data = session.bdp::<Data>(securities, None)?;
    // Without Override
    println!("{:#?}", data);

    let overrides = overrides!(EQY_FUND_CRNCY = "EUR");
    let overrides = Some(overrides);
    let data = session.bdp::<Data>(securities, overrides)?;
    // With Overrides
    println!("{:#?}", data);

    Ok(())
}
```

```Shell
...

{
    "AAPL US Equity": Data {
        crncy: "USD",
        id_bb: "037833100",
        ticker: "AAPL",
        market_sector: Some(
            "2",
        ),
        px_last: 258.21,
        crncy_adj_px_last: 258.21,
        ds002: "APPLE INC",
    },
}
...
{
    "AAPL US Equity": Data {
        crncy: "USD",
        id_bb: "037833100",
        ticker: "AAPL",
        market_sector: Some(
            "2",
        ),
        px_last: 258.21,
        crncy_adj_px_last: 222.3265,
        ds002: "APPLE INC",
    },
}

```


### Historical data

```rust
use blpapi::{
    Error, RefData,
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
    time_series::HistOptions,
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

    let tickers = &[
        "IBM US Equity",
        "MSFT US Equity",
        "3333 HK Equity",
        "/cusip/912828GM6@BGN",
    ];

    let options = HistOptions::new("20191001", "20191010");
    let data = session.bdh::<Data>(tickers, options)?;
    for entry in data {
        println!("{}: {:?} {:?}", entry.ticker, entry.date, entry.data);
    }

    Ok(())
}
```

```Shell
IBM US Equity: 2019-10-01 Data { px_last: 137.2189, volatitlity_30d: 0.0 }
IBM US Equity: 2019-10-02 Data { px_last: 135.3372, volatitlity_30d: 0.0 }
IBM US Equity: 2019-10-03 Data { px_last: 135.6524, volatitlity_30d: 0.0 }
IBM US Equity: 2019-10-04 Data { px_last: 136.5789, volatitlity_30d: 0.0 }
IBM US Equity: 2019-10-07 Data { px_last: 134.9456, volatitlity_30d: 0.0 }
IBM US Equity: 2019-10-08 Data { px_last: 132.1756, volatitlity_30d: 0.0 }
IBM US Equity: 2019-10-09 Data { px_last: 133.4078, volatitlity_30d: 0.0 }
IBM US Equity: 2019-10-10 Data { px_last: 134.8023, volatitlity_30d: 0.0 }
MSFT US Equity: 2019-10-01 Data { px_last: 137.07, volatitlity_30d: 0.0 }
MSFT US Equity: 2019-10-02 Data { px_last: 134.65, volatitlity_30d: 0.0 }
MSFT US Equity: 2019-10-03 Data { px_last: 136.28, volatitlity_30d: 0.0 }
MSFT US Equity: 2019-10-04 Data { px_last: 138.12, volatitlity_30d: 0.0 }
MSFT US Equity: 2019-10-07 Data { px_last: 137.12, volatitlity_30d: 0.0 }
MSFT US Equity: 2019-10-08 Data { px_last: 135.67, volatitlity_30d: 0.0 }
MSFT US Equity: 2019-10-09 Data { px_last: 138.24, volatitlity_30d: 0.0 }
MSFT US Equity: 2019-10-10 Data { px_last: 139.1, volatitlity_30d: 0.0 }
3333 HK Equity: 2019-10-02 Data { px_last: 16.88, volatitlity_30d: 0.0 }
3333 HK Equity: 2019-10-03 Data { px_last: 16.7, volatitlity_30d: 0.0 }
3333 HK Equity: 2019-10-04 Data { px_last: 17.58, volatitlity_30d: 0.0 }
3333 HK Equity: 2019-10-08 Data { px_last: 17.58, volatitlity_30d: 0.0 }
3333 HK Equity: 2019-10-09 Data { px_last: 17.5, volatitlity_30d: 0.0 }
3333 HK Equity: 2019-10-10 Data { px_last: 17.14, volatitlity_30d: 0.0 }
```


### Historical Intraday Tick Ref  

```rust
use blpapi::{
    Error,
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
    time_series::{HistIntradayOptions, TickTypes},
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

    let ticker = String::from("BAYN GY Equity");
    let start_dt = "20260123T090000";
    let end_dt = "20260123T170000";

    // Example
    let value = true;
    let options = HistIntradayOptions::new(start_dt, end_dt)
        .cond_codes(value)
        .exch_code(value)
        .non_plottable_events(value)
        .brkr_codes(value)
        .rps_codes(value)
        .trade_time(value)
        .action_codes(value)
        .show_yield(value)
        .spread_price(value)
        .upfront_price(value)
        .indicator_codes(value)
        .return_eids(value)
        .bic_mic_codes(value)
        .eq_ref_price(value)
        .xdf_fields(value)
        .trade_id(value);

    let tick_types = vec![TickTypes::Trade, TickTypes::Ask];

    let data = session.bdib(ticker, tick_types, options)?;
    for entry in data {
        println!("{}: {:?} {:?}", entry.ticker, entry.date, entry.data);
    }

    Ok(())
}
```



### Field Information 

```rust
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
    let data = session.field_info::<Data>(None, None)?;
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
```

```Shell
BAYN GY Equity: 2026-01-23T11:44:06.000100 TickData { tick_type: "ASK", size: 695, value: 44.63, conditional_codes: "", exchange_code: "", eids: [] }
BAYN GY Equity: 2026-01-23T11:44:06.000166 TickData { tick_type: "ASK", size: 521, value: 44.63, conditional_codes: "", exchange_code: "", eids: [] }
BAYN GY Equity: 2026-01-23T11:44:06.000166 TickData { tick_type: "ASK", size: 390, value: 44.63, conditional_codes: "", exchange_code: "", eids: [] }
BAYN GY Equity: 2026-01-23T11:44:06.000166 TickData { tick_type: "ASK", size: 216, value: 44.63, conditional_codes: "", exchange_code: "", eids: [] }
BAYN GY Equity: 2026-01-23T11:44:06.000822 TickData { tick_type: "TRADE", size: 1741, value: 44.625, conditional_codes: "XR", exchange_code: "", eids: [] }
BAYN GY Equity: 2026-01-23T11:44:06.000822 TickData { tick_type: "TRADE", size: 84, value: 44.63, conditional_codes: "", exchange_code: "", eids: [] }
BAYN GY Equity: 2026-01-23T11:44:06.000822 TickData { tick_type: "TRADE", size: 1369, value: 44.63, conditional_codes: "XR", exchange_code: "", eids: [] }
BAYN GY Equity: 2026-01-23T11:44:06.000822 TickData { tick_type: "TRADE", size: 132, value: 44.63, conditional_codes: "", exchange_code: "", eids: [] }
BAYN GY Equity: 2026-01-23T11:44:06.000822 TickData { tick_type: "TRADE", size: 5274, value: 44.63, conditional_codes: "XR", exchange_code: "", eids: [] }
BAYN GY Equity: 2026-01-23T11:44:06.000822 TickData { tick_type: "ASK", size: 552, value: 44.635, conditional_codes: "", exchange_code: "", eids: [] }
```


### Field Info

```rust

use blpapi::{
    Error, RefData,
    session::{Session, SessionBuilder},
    session_options::SessionOptions,
};

#[derive(Debug, Default, RefData)]
struct Data {
    dy886: f64,
    px_last: f64,
    ds002: f64,
    volume: f64,
    dv014: f64,
    rq005: f64,
    pr959: f64,
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

```

```Shell
...
"RQ008": "" "" Some("Double") Some("Price") None None Some("Lowest price the security reached during the current trading day. If the market is closed then it is the lowest price the security reached on the last day the market was open. Field updates 
in realtime.") []
"RQ007": "" "" Some("Double") Some("Price") None None Some("Highest price the security reached during the current trading day. If the market is closed then it is the highest price the security reached on the last day the market was open. Field update
s in realtime.") []

```

### Field Search
```rust

use blpapi::{
    Error, 
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

    // Search Pattern
    let search = vec!["last price"];

    // Example
    let data = session.field_search(search, None)?;
    for entry in data {
        println!("{:#?}", entry);
    }

    Ok(())
}

```

```shell
...
FieldSeries {
    id: "RX097",
    mnemonic: "TRAIL_12M_STK_COMP_AMT_PER_SH",
    desc: "Trail 12M Stock Based Compensation Amt Per Share",
    data_type: Some(
        "Double",
    ),
    field_type: Some(
        "Real",
    ),
    field_category: Some(
        "Fundamentals/Bloomberg Fundamentals/Estimate Comparable/Trailing",
    ),
    field_documentation: None,
    field_property: {},
    field_default_formatting: {},
    field_error: {},
    other: {},
    overrides: [
        "DY892",
        "DS215",
        "DT582",
        "DT085",
        "DX243",
        "DT581",
        "DY891",
        "DT084",
        "DT096",
        "DS323",
        "DY771",
        "DT081",
        "DT089",
        "DT086",
        "DT092",
        "DT095",
        "DS324",
        "DT097",
        "DS276",
        "DX242",
        "DT082",
        "DT083",
        "DT093",
    ],
}
FieldSeries {
    id: "Q2361",
    mnemonic: "EVT_TRADE_BLOOMBERG_STD_CC_RT",
    desc: "Event Trade Bloomberg Standard CC - Realtime",
    data_type: Some(
        "String",
    ),
    field_type: Some(
        "Character",
    ),
    field_category: Some(
        "Market Activity/Last",
    ),
    field_documentation: None,
    field_property: {},
    field_default_formatting: {},
    field_error: {},
    other: {},
    overrides: [],
}

```

### Field List  
```rust

use blpapi::{
    Error,
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

```

```shell
...
FieldSeries {
    id: "RX097",
    mnemonic: "TRAIL_12M_STK_COMP_AMT_PER_SH",
    desc: "Trail 12M Stock Based Compensation Amt Per Share",
    data_type: Some(
        "Double",
    ),
    field_type: Some(
        "Real",
    ),
    field_category: Some(
        "Fundamentals/Bloomberg Fundamentals/Estimate Comparable/Trailing",
    ),
    field_documentation: None,
    field_property: {},
    field_default_formatting: {},
    field_error: {},
    other: {},
    overrides: [
        "DY892",
        "DS215",
        "DT582",
        "DT085",
        "DX243",
        "DT581",
        "DY891",
        "DT084",
        "DT096",
        "DS323",
        "DY771",
        "DT081",
        "DT089",
        "DT086",
        "DT092",
        "DT095",
        "DS324",
        "DT097",
        "DS276",
        "DX242",
        "DT082",
        "DT083",
        "DT093",
    ],
}
FieldSeries {
    id: "Q2361",
    mnemonic: "EVT_TRADE_BLOOMBERG_STD_CC_RT",
    desc: "Event Trade Bloomberg Standard CC - Realtime",
    data_type: Some(
        "String",
    ),
    field_type: Some(
        "Character",
    ),
    field_category: Some(
        "Market Activity/Last",
    ),
    field_documentation: None,
    field_property: {},
    field_default_formatting: {},
    field_error: {},
    other: {},
    overrides: [],
}

```


### Security Look Up
```rust
use blpapi::{
    Error,
    data_series::{Language, YellowKey},
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

    let name = "Apple";
    let max_results = 10;

    // Example
    let data = session.lookup_security(name, max_results, None, None)?;
    println!("{:#?}", data);

    let name = "Apple";
    let max_results = 10;
    let yellow_key = Some(YellowKey::Eqty);
    let lng_override = Some(Language::Kanji);

    // Example
    let data = session.lookup_security(name, max_results, yellow_key, lng_override)?;
    println!("{:#?}", data);

    Ok(())
}

```

```shell
...Without Overrides
SecurityLookUp {
    query: "Apple",
    total_results: 10,
    results: [
        Security {
            id: "",
            yellow_key: None,
            security: "AAPL CB USD SR 5Y<corp>",
            parse_key: "",
            ticker: "",
            country_code: None,
            market_sector: None,
            instrument_type: None,
            description: Some(
                "Apple Inc Generic Benchmark 5Y Corporate",
            ),
            currency: None,
            curve_id: None,
            security_type: None,
            security_subtype: None,
            publisher: None,
            bbg_id: None,
        },
        Security {
            id: "",
            yellow_key: None,
            security: "AAPL CB USD SR 5Y<corp>",
            parse_key: "",
            ticker: "",
            country_code: None,
            market_sector: None,
            instrument_type: None,
            description: Some(
                "Apple Inc Generic Benchmark 5Y Corporate",
            ),
            currency: None,
            curve_id: None,
            security_type: None,
            security_subtype: None,
            publisher: None,
            bbg_id: None,
        },
        ...


...With Overrides
SecurityLookUp {
    query: "Apple",
    total_results: 10,
    results: [
        Security {
            id: "",
            yellow_key: None,
            security: "APC GR<equity>",
            parse_key: "",
            ticker: "",
            country_code: None,
            market_sector: None,
            instrument_type: None,
            description: Some(
                "ｱｯﾌ\u{ff9f}ﾙ (ﾄ\u{ff9e}ｲﾂ)",
            ),
            currency: None,
            curve_id: None,
            security_type: None,
            security_subtype: None,
            publisher: None,
            bbg_id: None,
        },
        Security {
            id: "",
            yellow_key: None,
            security: "APC GR<equity>",
            parse_key: "",
            ticker: "",
            country_code: None,
            market_sector: None,
            instrument_type: None,
            description: Some(
                "ｱｯﾌ\u{ff9f}ﾙ (ﾄ\u{ff9e}ｲﾂ)",
            ),
            currency: None,
            curve_id: None,
            security_type: None,
            security_subtype: None,
            publisher: None,
            bbg_id: None,
        },
        ...

```

### Curve Look Up 
```rust
use blpapi::{
    Error,
    data_series::{CurveOptions, Language, SecuritySubType, SecurityType, YellowKey},
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

    let max_results = 10;
    let options = CurveOptions::new("Gold");
    // let options = CurveOptions::new("Gold")
    //     .bbg_id("YCCD1016")
    //     .country("US")
    //     .currency("USD")
    //     .curve_id("CD1016")
    //     .security_type(SecurityType::Corp)
    //     .security_subtype(SecuritySubType::Cds);

    // Example
    let data = session.lookup_security_curved(max_results, options)?;
    println!("{:#?}", data);

    Ok(())
}

```
```shell
SecurityLookUp {
    query: "Gold",
    total_results: 10,
    results: [
        Security {
            id: "",
            yellow_key: None,
            security: "",
            parse_key: "",
            ticker: "",
            country_code: Some(
                "US",
            ),
            market_sector: None,
            instrument_type: None,
            description: Some(
                "Goldman Sachs Group Inc/The",
            ),
            currency: Some(
                "USD",
            ),
            curve_id: Some(
                "CD1017",
            ),
            security_type: Some(
                "CORP",
            ),
            security_subtype: Some(
                "CDS",
            ),
            publisher: Some(
                "Bloomberg",
            ),
            bbg_id: Some(
                "",
            ),
        },
        Security {
            id: "",
            yellow_key: None,
            security: "",
            ...

```


### Security Look Up Govt
```rust
use blpapi::{
    Error,
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

    let query = "bond";
    let max_results = 10;
    let partial_match = true;
    let ticker = None;

    // Example
    let data = session.lookup_security_govt(query, max_results, partial_match, ticker)?;
    println!("{:#?}", data);
    Ok(())
}

```

```shell

SecurityLookUp {
    query: "bond",
    total_results: 10,
    results: [
        Security {
            id: "",
            yellow_key: None,
            security: "",
            parse_key: "YL748982 Corp",
            ticker: "FRTR",
            country_code: None,
            name: Some(
                "French Republic Government Bond OAT",
            ),
            market_sector: None,
            instrument_type: None,
            description: None,
            currency: None,
            curve_id: None,
            security_type: None,
            security_subtype: None,
            publisher: None,
            bbg_id: None,
            isin: None,
            sedol: None,
        },
        Security {
            id: "",
            yellow_key: None,
            security: "",
            parse_key: "YL748982 Corp",
            ticker: "FRTR",
            country_code: None,
            name: Some(
                "French Republic Government Bond OAT",
            ),
            ...

```

