use chrono::{NaiveDate, NaiveDateTime};
use std::collections::HashMap;

use crate::{
    core::{
        BLPAPI_DEFAULT_BDIB_ASK, BLPAPI_DEFAULT_BDIB_ASK_BEST, BLPAPI_DEFAULT_BDIB_ASK_YIELD,
        BLPAPI_DEFAULT_BDIB_AT_TRADE, BLPAPI_DEFAULT_BDIB_BEST_ASK, BLPAPI_DEFAULT_BDIB_BEST_BID,
        BLPAPI_DEFAULT_BDIB_BID, BLPAPI_DEFAULT_BDIB_BID_BEST, BLPAPI_DEFAULT_BDIB_BID_YIELD,
        BLPAPI_DEFAULT_BDIB_MID_PRICE, BLPAPI_DEFAULT_BDIB_SETTLE, BLPAPI_DEFAULT_BDIB_TRADE,
    },
    datetime::{DateFormats, Datetime, DatetimeBuilder},
    names::{END_DATE_TIME, START_DATE_TIME},
    request::Request,
    Error,
};

pub type DateType = chrono::NaiveDate;
pub type IntradayDateType = chrono::NaiveDateTime;

#[derive(Debug)]
pub enum XdfFields {
    ClientDomicile,
    ClientSegment,
    ClientSubsegment,
    ClientIdentifier,
    Direction,
    TradeId,
}

pub fn is_valid_datetime(input: &str) -> Result<Datetime, Error> {
    let datetime_formats = [
        // 20250101T000000
        "%Y%m%dT%H%M%S",
        // 20221009 T101833
        "%Y-%m-%d T%H%M%S",
        "%Y-%m-%d %H:%M:%S",
    ];

    for fmt in datetime_formats {
        if NaiveDateTime::parse_from_str(input, fmt).is_ok() {
            let new_dt = convert_datetime(input)?;
            return Ok(new_dt);
        }
    }

    let date_formats = ["%Y%m%d", "%Y-%m-%d"];

    for fmt in date_formats {
        if NaiveDate::parse_from_str(input, fmt).is_ok() {
            let new_dt = convert_date(input)?;
            return Ok(new_dt);
        }
    }
    Err(Error::InvalidDatetime)
}

fn convert_date<S: Into<String>>(x: S) -> Result<Datetime, Error> {
    let haystack = x.into();
    let year = &haystack[..4];
    let year_usize = year.parse::<usize>()?;

    let month = &haystack[4..6];
    let month_usize = month.parse::<usize>()?;

    let day = &haystack[6..];
    let day_usize = day.parse::<usize>()?;

    let dt = DatetimeBuilder::default()
        .set_year(year_usize)
        .set_month(month_usize)
        .set_day(day_usize)
        .set_offset(-240)
        .build();
    Ok(dt)
}

fn convert_datetime<S: Into<String>>(x: S) -> Result<Datetime, Error> {
    let haystack = x.into();
    let year = &haystack[..4];
    let year_usize = year.parse::<usize>()?;

    let month = &haystack[4..6];
    let month_usize = month.parse::<usize>()?;

    let day = &haystack[6..8];
    let day_usize = day.parse::<usize>()?;

    let hour = &haystack[9..11];
    let hour_usize = hour.parse::<usize>()?;

    let min = &haystack[11..13];
    let min_usize = min.parse::<usize>()?;

    let sec = &haystack[13..];
    let sec_usize = sec.parse::<usize>()?;

    let dt_b = DatetimeBuilder::default()
        .set_year(year_usize)
        .set_month(month_usize)
        .set_day(day_usize)
        .set_hours(hour_usize)
        .set_minutes(min_usize)
        .set_seconds(sec_usize)
        .build();

    Ok(dt_b)
}

/// Options for historical data request
#[derive(Debug, Default)]
pub struct HistOptions {
    /// Start date yyyyMMdd
    start_date: String,
    /// end date yyyyMMdd
    end_date: String,
    /// periodicity_adjustment (ACTUAL...)
    periodicity_adjustment: Option<PeriodicityAdjustment>,
    /// periodicity_selection (DAILY, MONTHLY, QUARTERLY, SEMIANNUALLY, ANNUALLY)
    periodicity_selection: Option<PeriodicitySelection>,
    /// max_data_points
    max_data_points: Option<i32>,
    /// Amends the value from local currency of the security to the desired currency.
    currency: Option<String>,
    /// Argument of two-character calendar code which aligns the data accordingly to the calendar
    calender_codes: Option<String>,
    date_format_as_relative: Option<bool>,
    include_non_trading_days: Option<TradingDays>,
    fill: Option<Fill>,
    quote_type: Option<QuoteType>,
    quote_price: Option<QuotePrice>,
    use_dpdf: Option<bool>,
    cash_adjst_abnormal: Option<bool>,
    cash_adjst_normal: Option<bool>,
    cap_chg: Option<bool>,
}

/// Options for historical Intraday Tick Requests
#[derive(Debug, Default)]
pub struct HistIntradayOptions {
    /// Start date yyyyMMddThhmmss
    start_dt: String,
    /// end date yyyyMMddThhmmss
    end_dt: String,
    /// Include non plottable Events
    non_plottable_events: Option<bool>,
    /// Shows the condition codes of a trade
    cond_codes: Option<bool>,
    /// Returns the exchange code of the trade
    exch_code: Option<bool>,
    /// Broker code for canadian, finnish, mexican, philippine and swdish equities only
    brkr_codes: Option<bool>,
    /// Reporting Party side
    rps_codes: Option<bool>,
    /// Trade time
    trade_time: Option<bool>,
    /// Display additional information about the action associated with the trade
    action_codes: Option<bool>,
    /// Display yield for the trade
    show_yield: Option<bool>,
    /// Display spread price
    spread_price: Option<bool>,
    /// Display the pricing for credit default swaps
    upfront_price: Option<bool>,
    /// Display additional indicator information about the trade
    indicator_codes: Option<bool>,
    /// Include Exchange IDs
    return_eids: Option<bool>,
    /// Include Bic Mic Codes for venues
    bic_mic_codes: Option<bool>,
    ///  Add artificial delay
    force_delay: Option<bool>,
    /// Include equity ref price if available
    eq_ref_price: Option<bool>,
    /// Include Client speicifc fields (new XDF)
    xdf_fields: Option<bool>,
    /// Include the Trade Id
    trade_id: Option<bool>,
    /// max_data_points
    max_data_points: Option<i32>,
    /// Max Data Pöints Origin
    max_data_origin: Option<bool>,
}

impl HistOptions {
    /// Crate a new historical options
    pub fn new<S: Into<String>, E: Into<String>>(start_date: S, end_date: E) -> Self {
        let start_date = start_date.into();
        let end_date = end_date.into();
        HistOptions {
            start_date,
            end_date,
            ..HistOptions::default()
        }
    }

    /// Set periodicity_adjustment
    pub fn periodicity_adjustment(mut self, periodicity_adjustment: PeriodicityAdjustment) -> Self {
        self.periodicity_adjustment = Some(periodicity_adjustment);
        self
    }

    /// Set periodicity_adjustment
    pub fn periodicity_selection(mut self, periodicity_selection: PeriodicitySelection) -> Self {
        self.periodicity_selection = Some(periodicity_selection);
        self
    }

    /// Set max points
    pub fn max_points(mut self, max_data_points: i32) -> Self {
        self.max_data_points = Some(max_data_points);
        self
    }

    /// Amends the value from local currency of the security to the desired currency.
    pub fn currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Set the calendar Code
    pub fn calendar_code(mut self, code: String) -> Self {
        self.calender_codes = Some(code);
        self
    }

    /// Set relative date
    pub fn relative_date(mut self, relative_date: bool) -> Self {
        self.date_format_as_relative = Some(relative_date);
        self
    }

    /// Set non-trading days
    pub fn days(mut self, non_trade_days: TradingDays) -> Self {
        self.include_non_trading_days = Some(non_trade_days);
        self
    }

    /// Set fill
    pub fn fill(mut self, fill: Fill) -> Self {
        self.fill = Some(fill);
        self
    }

    /// Set Quote Type
    pub fn quote_type(mut self, qtype: QuoteType) -> Self {
        self.quote_type = Some(qtype);
        self
    }

    /// Set Quote Price
    pub fn quote_price(mut self, qprice: QuotePrice) -> Self {
        self.quote_price = Some(qprice);
        self
    }

    /// Set DPDF
    pub fn dpdf(mut self, dpdf: bool) -> Self {
        self.use_dpdf = Some(dpdf);
        self
    }

    /// Set Cash Changes
    pub fn cash_adj_abnormal(mut self, include: bool) -> Self {
        self.cash_adjst_abnormal = Some(include);
        self
    }

    /// Set Capital Change
    pub fn cap_chg(mut self, include: bool) -> Self {
        self.cap_chg = Some(include);
        self
    }

    /// Set Capital Adjustment
    pub fn cash_adj_normal(mut self, include: bool) -> Self {
        self.cash_adjst_normal = Some(include);
        self
    }

    pub fn apply(&self, request: &mut Request) -> Result<(), Error> {
        let fmt = DateFormats::Bdh;
        // Check if provided dates are correct
        let start_valid = is_valid_datetime(&self.start_date)?;
        let end_valid = is_valid_datetime(&self.end_date)?;

        let start_date = start_valid.get_fmt(&fmt);
        let end_date = end_valid.get_fmt(&fmt);

        let mut element = request.element();
        element.set("startDate", start_date.as_ref())?;
        element.set("endDate", end_date.as_ref())?;
        if let Some(periodicity_selection) = self.periodicity_selection {
            element.set("periodicitySelection", periodicity_selection.as_str())?;
        }
        if let Some(periodicity_adjustment) = self.periodicity_adjustment {
            element.set("periodicityAdjustment", periodicity_adjustment.as_str())?;
        }
        if let Some(max_data_points) = self.max_data_points {
            element.set("maxDataPoints", max_data_points)?;
        }
        if let Some(currency) = self.currency.as_ref() {
            element.set("currency", &**currency)?;
        }
        if let Some(calendar_code) = self.calender_codes.as_ref() {
            element.set("calendarCodeOverride", &**calendar_code)?;
        }
        if let Some(relative_date) = self.date_format_as_relative {
            element.set("returnRelativeDate", relative_date)?;
        }
        if let Some(non_trade_days) = self.include_non_trading_days {
            element.set("nonTradingDayFillOption", non_trade_days.as_str())?;
        }
        if let Some(fill) = self.fill {
            element.set("nonTradingDayFillMethod", fill.as_str())?;
        }
        if let Some(quote_type) = self.quote_type {
            element.set("pricingOption", quote_type.as_str())?;
        }
        if let Some(quote_price) = self.quote_price {
            element.set("overrideOption", quote_price.as_str())?;
        }
        if let Some(dpdf) = self.use_dpdf {
            element.set("adjustmentFollowDPDF", dpdf)?;
        }
        if let Some(cash_adj) = self.cash_adjst_abnormal {
            element.set("adjustmentAbnormal", cash_adj)?;
        }
        if let Some(capital_adj) = self.cash_adjst_normal {
            element.set("adjustmentNormal", capital_adj)?;
        }
        if let Some(capital_chng) = self.cap_chg {
            element.set("adjustmentSplit", capital_chng)?;
        }

        Ok(())
    }
}

impl HistIntradayOptions {
    /// Crate a new historical options
    pub fn new<S: Into<String>, E: Into<String>>(start_dt: S, end_dt: E) -> Self {
        let start_dt = start_dt.into();
        let end_dt = end_dt.into();
        HistIntradayOptions {
            start_dt,
            end_dt,
            ..HistIntradayOptions::default()
        }
    }

    /// Set conditional codes
    pub fn cond_codes(mut self, value: bool) -> Self {
        self.cond_codes = Some(value);
        self
    }

    /// Set non plottable events
    pub fn non_plottable_events(mut self, value: bool) -> Self {
        self.non_plottable_events = Some(value);
        self
    }

    /// Set exch_code
    pub fn exch_code(mut self, value: bool) -> Self {
        self.exch_code = Some(value);
        self
    }

    /// Set broker codes
    pub fn brkr_codes(mut self, value: bool) -> Self {
        self.brkr_codes = Some(value);
        self
    }
    /// Set rps codes
    pub fn rps_codes(mut self, value: bool) -> Self {
        self.rps_codes = Some(value);
        self
    }
    /// Set bic mics
    pub fn bic_mic_codes(mut self, value: bool) -> Self {
        self.bic_mic_codes = Some(value);
        self
    }

    /// Set trade_time
    pub fn trade_time(mut self, value: bool) -> Self {
        self.trade_time = Some(value);
        self
    }

    /// Set action codes
    pub fn action_codes(mut self, value: bool) -> Self {
        self.action_codes = Some(value);
        self
    }

    /// Set show yield
    pub fn show_yield(mut self, value: bool) -> Self {
        self.show_yield = Some(value);
        self
    }

    /// Set spread price
    pub fn spread_price(mut self, value: bool) -> Self {
        self.spread_price = Some(value);
        self
    }

    /// Set upfront_price
    pub fn upfront_price(mut self, value: bool) -> Self {
        self.upfront_price = Some(value);
        self
    }

    /// Set indicator codes
    pub fn indicator_codes(mut self, value: bool) -> Self {
        self.indicator_codes = Some(value);
        self
    }

    /// Set return eids codes
    pub fn return_eids(mut self, value: bool) -> Self {
        self.return_eids = Some(value);
        self
    }

    /// Set force delay
    pub fn force_delay(mut self, value: bool) -> Self {
        self.force_delay = Some(value);
        self
    }

    /// Set Equity Reference Price
    pub fn eq_ref_price(mut self, value: bool) -> Self {
        self.eq_ref_price = Some(value);
        self
    }

    /// Set xdf fields
    pub fn xdf_fields(mut self, value: bool) -> Self {
        self.xdf_fields = Some(value);
        self
    }

    /// trade id
    pub fn trade_id(mut self, value: bool) -> Self {
        self.trade_id = Some(value);
        self
    }
    /// max data points
    pub fn max_data_points(mut self, value: i32) -> Self {
        self.max_data_points = Some(value);
        self
    }

    /// max data origin
    pub fn max_data_origin(mut self, value: bool) -> Self {
        self.max_data_origin = Some(value);
        self
    }

    pub fn apply(&self, request: &mut Request) -> Result<(), Error> {
        // Check if provided dates are correct
        let start_valid = is_valid_datetime(&self.start_dt)?;
        let end_valid = is_valid_datetime(&self.end_dt)?;

        let mut element = request.element();

        element.set_named(&START_DATE_TIME, start_valid)?;
        element.set_named(&END_DATE_TIME, end_valid)?;

        if let Some(cond_codes) = self.cond_codes {
            element.set("includeConditionCodes", cond_codes)?;
        }
        if let Some(val) = self.exch_code {
            element.set("includeExchangeCodes", val)?;
        }
        if let Some(val) = self.non_plottable_events {
            element.set("includeNonPlottableEvents", val)?;
        }
        if let Some(val) = self.brkr_codes {
            element.set("includeBrokerCodes", val)?;
        }
        if let Some(val) = self.rps_codes {
            element.set("includeRpsCodes", val)?;
        }
        if let Some(val) = self.trade_time {
            element.set("includeTradeTime", val)?;
        }
        if let Some(val) = self.action_codes {
            element.set("includeActionCodes", val)?;
        }
        if let Some(val) = self.show_yield {
            element.set("includeYield", val)?;
        }
        if let Some(val) = self.spread_price {
            element.set("includeSpreadPrice", val)?;
        }
        if let Some(val) = self.upfront_price {
            element.set("includeUpfrontPrice", val)?;
        }
        if let Some(val) = self.indicator_codes {
            element.set("includeIndicatorCodes", val)?;
        }
        if let Some(val) = self.return_eids {
            element.set("returnEids", val)?;
        }
        if let Some(val) = self.bic_mic_codes {
            element.set("includeBicMicCodes", val)?;
        }
        if let Some(val) = self.force_delay {
            element.set("forcedDelay", val)?;
        }
        if let Some(val) = self.eq_ref_price {
            element.set("includeEqRefPrice", val)?;
        }
        if let Some(val) = self.xdf_fields {
            element.set("includeClientSpecificFields", val)?;
        }
        if let Some(val) = self.trade_id {
            element.set("includeTradeId", val)?;
        }
        if let Some(val) = self.max_data_points {
            element.set("maxDataPoints", val)?;
        }
        if let Some(val) = self.max_data_origin {
            element.set("maxDataPointsOrigin", val)?;
        }

        Ok(())
    }
}
#[derive(Debug, Default)]
pub enum TickTypes {
    #[default]
    Trade,
    Ask,
    Bid,
    BidBest,
    BidYield,
    BestBid,
    AskBest,
    BestAsk,
    AskYield,
    MidPrice,
    AtTrade,
    Settle,
}

impl From<&TickTypes> for &str {
    fn from(arg: &TickTypes) -> Self {
        match arg {
            TickTypes::Trade => BLPAPI_DEFAULT_BDIB_TRADE,
            TickTypes::Ask => BLPAPI_DEFAULT_BDIB_ASK,
            TickTypes::Bid => BLPAPI_DEFAULT_BDIB_BID,
            TickTypes::BidBest => BLPAPI_DEFAULT_BDIB_BID_BEST,
            TickTypes::BidYield => BLPAPI_DEFAULT_BDIB_BID_YIELD,
            TickTypes::BestBid => BLPAPI_DEFAULT_BDIB_BEST_BID,
            TickTypes::AskBest => BLPAPI_DEFAULT_BDIB_ASK_BEST,
            TickTypes::BestAsk => BLPAPI_DEFAULT_BDIB_BEST_ASK,
            TickTypes::AskYield => BLPAPI_DEFAULT_BDIB_ASK_YIELD,
            TickTypes::MidPrice => BLPAPI_DEFAULT_BDIB_MID_PRICE,
            TickTypes::AtTrade => BLPAPI_DEFAULT_BDIB_AT_TRADE,
            TickTypes::Settle => BLPAPI_DEFAULT_BDIB_SETTLE,
        }
    }
}

#[derive(Debug, Default)]
pub struct TickDataBuilder {
    pub tick_type: Option<String>,
    pub size: Option<i32>,
    pub value: Option<f64>,
    pub conditional_codes: Option<String>,
    pub exchange_code: Option<String>,
    pub eids: Vec<i32>,
    pub other: HashMap<String, String>,
}

impl TickDataBuilder {
    pub fn tick_type(&mut self, tick_type: String) -> &mut Self {
        self.tick_type = Some(tick_type);
        self
    }
    pub fn size(&mut self, size: i32) -> &mut Self {
        self.size = Some(size);
        self
    }
    pub fn value(&mut self, value: f64) -> &mut Self {
        self.value = Some(value);
        self
    }
    pub fn conditional_codes(&mut self, codes: String) -> &mut Self {
        self.conditional_codes = Some(codes);
        self
    }
    pub fn exchange_code(&mut self, codes: String) -> &mut Self {
        self.exchange_code = Some(codes);
        self
    }
    pub fn eids(&mut self, eids: i32) -> &mut Self {
        self.eids.push(eids);
        self
    }
    pub fn other(&mut self, k: String, v: String) -> &mut Self {
        self.other.insert(k, v);
        self
    }

    pub fn build(self) -> TickData {
        let tick_type = self.tick_type.expect("Expected Tick Type");
        let size = self.size.expect("Expected Size");
        let value = self.value.expect("Expected Value");
        let conditional_codes = self.conditional_codes.unwrap_or_default();
        let exchange_code = self.exchange_code.unwrap_or_default();
        let eids = self.eids;
        TickData {
            tick_type,
            size,
            value,
            conditional_codes,
            exchange_code,
            eids,
        }
    }
}

#[derive(Debug, Default)]
pub struct TickData {
    pub tick_type: String,
    pub size: i32,
    pub value: f64,
    pub conditional_codes: String,
    pub exchange_code: String,
    pub eids: Vec<i32>,
}

#[derive(Debug)]
pub struct TimeSeries<R, T> {
    pub date: T,
    pub ticker: String,
    pub data: R,
}

#[derive(Default, Debug)]
pub struct TimeSerieBuilder<R, T> {
    pub ticker: String,
    pub dates: Vec<T>,
    pub values: Vec<R>,
}

impl<R, T> TimeSerieBuilder<R, T> {
    /// Create a new timeseries with given capacity
    pub fn with_capacity(capacity: usize, ticker: String) -> Self {
        TimeSerieBuilder {
            ticker,
            dates: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }

    fn iter_entries(self, ticker: String) -> impl Iterator<Item = TimeSeries<R, T>> {
        self.dates
            .into_iter()
            .zip(self.values)
            .map(move |(date, data)| TimeSeries {
                date,
                data,
                ticker: ticker.to_string(),
            })
    }

    pub fn to_rows(self) -> Vec<TimeSeries<R, T>> {
        let ticker = self.ticker.clone();
        self.iter_entries(ticker).collect()
    }
}

/// Periodicity Adjustment
#[derive(Default, Debug, Clone, Copy)]
pub enum PeriodicityAdjustment {
    Actual,
    Fiscal,
    #[default]
    Calendar,
}

impl PeriodicityAdjustment {
    /// Get str value for periodicity selection
    pub fn as_str(self) -> &'static str {
        match self {
            PeriodicityAdjustment::Actual => "ACTUAL",
            PeriodicityAdjustment::Calendar => "CALENDAR",
            PeriodicityAdjustment::Fiscal => "FISCAL",
        }
    }
}

/// Periodicity Selection
#[derive(Default, Debug, Clone, Copy)]
pub enum PeriodicitySelection {
    #[default]
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    SemiAnnually,
    Yearly,
}

impl PeriodicitySelection {
    /// Get str value for periodicity selection
    pub fn as_str(self) -> &'static str {
        match self {
            PeriodicitySelection::Daily => "DAILY",
            PeriodicitySelection::Weekly => "WEEKLY",
            PeriodicitySelection::Monthly => "MONTHLY",
            PeriodicitySelection::Quarterly => "QUARTERLY",
            PeriodicitySelection::SemiAnnually => "SEMIANNUALLY",
            PeriodicitySelection::Yearly => "YEARLY",
        }
    }
}

/// Trading Days
#[derive(Default, Debug, Clone, Copy)]
pub enum TradingDays {
    AllCalendarDays,
    ActiveDaysOnly,
    #[default]
    NonTradingWeekdays,
}

impl TradingDays {
    /// Get str value for periodicity selection
    pub fn as_str(self) -> &'static str {
        match self {
            TradingDays::NonTradingWeekdays => "NON_TRADING_WEEKDAYS",
            TradingDays::AllCalendarDays => "ALL_CALENDAR_DAYS",
            TradingDays::ActiveDaysOnly => "ACTIVE_DAYS_ONLY",
        }
    }
}

/// Fill Value  
#[derive(Default, Debug, Clone, Copy)]
pub enum Fill {
    NoValue,
    #[default]
    PreviousValue,
}

impl Fill {
    /// Get str value for periodicity selection
    pub fn as_str(self) -> &'static str {
        match self {
            Fill::PreviousValue => "PREVIOUS_VALUE",
            Fill::NoValue => "NIL_VALUE",
        }
    }
}

/// Quote Type
#[derive(Default, Debug, Clone, Copy)]
pub enum QuoteType {
    OptionPrice,
    #[default]
    OptionYield,
}

impl QuoteType {
    /// Get str value for periodicity selection
    pub fn as_str(self) -> &'static str {
        match self {
            QuoteType::OptionPrice => "PRICING_OPTION_PRICE",
            QuoteType::OptionYield => "PRICING_OPTION_YIELD",
        }
    }
}

/// Quote Price
#[derive(Default, Debug, Clone, Copy)]
pub enum QuotePrice {
    #[default]
    OptionClose,
    OptionGPA,
}

impl QuotePrice {
    /// Get str value for periodicity selection
    pub fn as_str(self) -> &'static str {
        match self {
            QuotePrice::OptionClose => "OVERRIDE_OPTION_CLOSE",
            QuotePrice::OptionGPA => "OVERRIDE_OPTION_GPA",
        }
    }
}
