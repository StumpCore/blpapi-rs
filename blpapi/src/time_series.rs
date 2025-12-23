use crate::{request::Request, Error};

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

#[derive(Default, Debug)]
pub struct TimeSerie<R> {
    #[cfg(feature = "dates")]
    pub dates: Vec<chrono::NaiveDate>,
    #[cfg(not(feature = "dates"))]
    pub dates: Vec<crate::datetime::Datetime>,
    pub values: Vec<R>,
}

impl<R> TimeSerie<R> {
    /// Create a new timeseries with given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        TimeSerie {
            dates: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
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
