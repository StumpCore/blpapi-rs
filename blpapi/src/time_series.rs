use chrono::{NaiveDate, NaiveDateTime};

use crate::{
    datetime::{DateFormats, Datetime, DatetimeBuilder},
    request::Request,
    Error,
};

#[cfg(feature = "dates")]
pub type DateType = chrono::NaiveDate;
#[cfg(feature = "dates")]
pub type IntradayTickType = chrono::NaiveDateTime;

#[cfg(not(feature = "dates"))]
pub type DateType = crate::datetime::Datetime;
#[cfg(not(feature = "dates"))]
pub type IntradayTickType = crate::datetime::Datetime;

pub fn is_valid_datetime(input: &str) -> Result<Datetime, Error> {
    // 1. Check for Full DateTime formats first (most specific)
    let datetime_formats = [
        "%Y%m%dT%H%M%S",    // 20250101T000000
        "%Y-%m-%d T%H%M%S", // 20221009 T101833 (from your previous example)
        "%Y-%m-%d %H:%M:%S",
    ];

    for fmt in datetime_formats {
        if NaiveDateTime::parse_from_str(input, fmt).is_ok() {
            let new_dt = convert_datetime(input)?;
            return Ok(new_dt);
        }
    }

    // 2. Check for Date-only formats
    let date_formats = [
        "%Y%m%d",   // 20250101
        "%Y-%m-%d", // 2025-01-01
    ];

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

    let hour = &haystack[10..12];
    let hour_usize = hour.parse::<usize>()?;

    let min = &haystack[12..14];
    let min_usize = min.parse::<usize>()?;

    let sec = &haystack[14..];
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
    /// Shows the condition codes of a trade
    cond_codes: Option<bool>,
    /// Returns the exchange code of the trade
    exch_code: Option<bool>,
    /// Rturns all ticks, including with condition codes
    qrm: Option<bool>,
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

    /// Set exch_code
    pub fn exch_code(mut self, value: bool) -> Self {
        self.exch_code = Some(value);
        self
    }

    /// Set qrm
    pub fn qrm(mut self, value: bool) -> Self {
        self.qrm = Some(value);
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

    pub fn apply(&self, request: &mut Request) -> Result<(), Error> {
        let fmt = DateFormats::IntraTick;
        // Check if provided dates are correct
        let start_valid = is_valid_datetime(&self.start_dt)?;
        let end_valid = is_valid_datetime(&self.end_dt)?;

        let start_date = start_valid.get_fmt(&fmt);
        let end_date = end_valid.get_fmt(&fmt);

        dbg!(&start_date);
        dbg!(&end_date);

        let mut element = request.element();
        element.set("startDateTime", start_date.as_ref())?;
        element.set("endDateTime", end_date.as_ref())?;
        dbg!(&element);
        if let Some(cond_codes) = self.cond_codes {
            element.set("includeConditionCodes", cond_codes)?;
        }
        if let Some(val) = self.exch_code {
            element.set("includeExchangeCodes", val)?;
        }
        if let Some(val) = self.qrm {
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

        Ok(())
    }
}

#[derive(Debug)]
pub struct TimeSeries<R> {
    pub date: DateType,
    pub ticker: String,
    pub data: R,
}

#[derive(Default, Debug)]
pub struct TimeSerieBuilder<R> {
    pub ticker: String,
    pub dates: Vec<DateType>,
    pub values: Vec<R>,
}

impl<R> TimeSerieBuilder<R> {
    /// Create a new timeseries with given capacity
    pub fn with_capacity(capacity: usize, ticker: String) -> Self {
        TimeSerieBuilder {
            ticker,
            dates: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }

    fn iter_entries(self, ticker: String) -> impl Iterator<Item = TimeSeries<R>> {
        self.dates
            .into_iter()
            .zip(self.values)
            .map(move |(date, data)| TimeSeries {
                date,
                data,
                ticker: ticker.to_string(),
            })
    }

    pub fn to_rows(self) -> Vec<TimeSeries<R>> {
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
