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
        HistOptions {
            start_date: start_date.into(),
            end_date: end_date.into(),
            ..HistOptions::default()
        }
    }

    /// Set periodicity_adjustment
    pub fn with_periodicity_adjustment(
        mut self,
        periodicity_adjustment: PeriodicityAdjustment,
    ) -> Self {
        self.periodicity_adjustment = Some(periodicity_adjustment);
        self
    }

    /// Set periodicity_adjustment
    pub fn with_periodicity_selection(
        mut self,
        periodicity_selection: PeriodicitySelection,
    ) -> Self {
        self.periodicity_selection = Some(periodicity_selection);
        self
    }

    /// Set max points
    pub fn with_max_points(mut self, max_data_points: i32) -> Self {
        self.max_data_points = Some(max_data_points);
        self
    }

    /// Amends the value from local currency of the security to the desired currency.
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
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
#[derive(Debug, Clone, Copy)]
pub enum PeriodicityAdjustment {
    Actual,
    Calendar,
    Fiscal,
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
#[derive(Debug, Clone, Copy)]
pub enum PeriodicitySelection {
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
