use regex::Regex;

use crate::{core::BDH_DATE_REGEX, datetime::DatetimeBuilder, request::Request, Error};

#[cfg(feature = "dates")]
pub type DateType = chrono::NaiveDate;
#[cfg(not(feature = "dates"))]
pub type DateType = crate::datetime::Datetime;

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

fn validate_date_string<S: Into<String>>(x: S) -> Result<bool, Error> {
    let mut date = false;
    if let Ok(re) = Regex::new(BDH_DATE_REGEX) {
        let haystack = x.into();
        match re.is_match(&haystack) {
            false => return Ok(false),
            _ => true,
        };
        let year = &haystack[..4];
        let year_usize = year.parse::<usize>()?;

        let month = &haystack[4..6];
        let month_usize = month.parse::<usize>()?;

        let day = &haystack[6..];
        let day_usize = day.parse::<usize>()?;

        date = DatetimeBuilder::default()
            .set_year(year_usize)
            .set_month(month_usize)
            .set_day(day_usize)
            .is_valid_date();
    };
    Ok(date)
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
        // Check if provided dates are correct
        let start_valid = validate_date_string(&self.start_date)?;
        let end_valid = validate_date_string(&self.end_date)?;

        if !(start_valid && end_valid) {
            return Err(Error::InvalidDate);
        };

        let mut element = request.element();
        element.set("startDate", &self.start_date[..])?;
        element.set("endDate", &self.end_date[..])?;
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

#[derive(Debug, Clone, PartialEq)]
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
            .zip(self.values.into_iter())
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
