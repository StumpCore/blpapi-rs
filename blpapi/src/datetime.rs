use crate::core::{write_to_stream_cb, StreamWriterContext};
use crate::Error;
use blpapi_sys::*;
use std::ffi::{c_short, c_uchar, c_uint, c_ushort, c_void};
use std::io::Write;
use std::os::raw::c_int;

pub const BLPAPI_DATETIME_DEFAULT_PARTS: usize = 0;
pub const BLPAPI_DATETIME_DEFAULT_HOURS: usize = 0;
pub const BLPAPI_DATETIME_DEFAULT_MINUTES: usize = 0;
pub const BLPAPI_DATETIME_DEFAULT_SECONDS: usize = 0;
pub const BLPAPI_DATETIME_DEFAULT_MILLI_SECONDS: usize = 0;
pub const BLPAPI_DATETIME_DEFAULT_PICTO_SECONDS: i32 = 0;
pub const BLPAPI_DATETIME_DEFAULT_MONTH: usize = 0;
pub const BLPAPI_DATETIME_DEFAULT_DAY: usize = 0;
pub const BLPAPI_DATETIME_DEFAULT_YEAR: usize = 0;
pub const BLPAPI_DATETIME_DEFAULT_OFFSET: i16 = 0;
pub const BLPAPI_DEFAULT_LEAP_YEAR: bool = false;
pub const DATETIME_30_MONTH: &[usize] = &[2, 4, 6, 9, 11];

/// TimePointer Builder
#[derive(Debug)]
pub struct TimePointBuilder {
    pub point: i64,
}

impl TimePointBuilder {
    /// New TimePoint
    pub fn new() -> Self {
        Self::default()
    }
    /// setting time point
    pub fn set_time_point(mut self, point: i64) -> Self {
        self.point = point;
        self
    }

    /// Building Time Point
    pub fn build(self) -> TimePoint {
        let point = blpapi_TimePoint_t {
            d_value: self.point,
        };
        TimePoint { point }
    }
}

/// Implementing Default for TimePoint
impl Default for TimePointBuilder {
    fn default() -> Self {
        TimePointBuilder { point: 0 }
    }
}

/// TimePoint
#[derive(Debug)]
pub struct TimePoint {
    pub(crate) point: blpapi_TimePoint_t,
}

impl TimePoint {
    /// Distance between two time points
    pub fn nano_seconds_between(&self, other: &TimePoint) -> i64 {
        let start_ptr: *const blpapi_TimePoint_t = &self.point;
        let end_ptr: *const blpapi_TimePoint_t = &other.point;

        let res = unsafe { blpapi_TimePointUtil_nanosecondsBetween(start_ptr, end_ptr) };
        res as i64
    }

    /// Changin the pointer
    pub fn from_ptr(&mut self, ptr: *mut blpapi_TimePoint_t) -> Result<(), Error> {
        self.point = unsafe { *ptr };
        Ok(())
    }
}

/// Time Fractions
#[derive(Debug)]
pub enum TimeFractions {
    MicroSeconds,
    MilliSeconds,
    NanoSeconds,
    PicoSeconds,
}

impl TimeFractions {
    pub const fn get_value(&self) -> usize {
        match self {
            TimeFractions::PicoSeconds => 1_000_000_000_000,
            TimeFractions::NanoSeconds => 1_000_000_000,
            TimeFractions::MicroSeconds => 1_000_000,
            TimeFractions::MilliSeconds => 1_000,
        }
    }
}
/// Builder of the Datetime struct
#[derive(Debug)]
pub struct DatetimeBuilder {
    pub parts: Option<usize>,
    pub hours: Option<usize>,
    pub minutes: Option<usize>,
    pub seconds: Option<usize>,
    pub milliseconds: Option<usize>,
    pub picoseconds: Option<i32>,
    pub fraction_of_seconds: Option<usize>,
    pub month: Option<usize>,
    pub day: Option<usize>,
    pub year: Option<usize>,
    pub offset: Option<i16>,
    pub leap_year: Option<bool>,
}

/// Default Implementation of DatetimeBuilder
impl Default for DatetimeBuilder {
    fn default() -> Self {
        Self {
            parts: Some(BLPAPI_DATETIME_DATE_PART as usize | BLPAPI_DATETIME_TIME_PART as usize),
            hours: Some(BLPAPI_DATETIME_DEFAULT_HOURS),
            minutes: Some(BLPAPI_DATETIME_DEFAULT_MINUTES),
            seconds: Some(BLPAPI_DATETIME_DEFAULT_SECONDS),
            milliseconds: Some(BLPAPI_DATETIME_DEFAULT_MILLI_SECONDS),
            picoseconds: Some(BLPAPI_DATETIME_DEFAULT_PICTO_SECONDS),
            fraction_of_seconds: Some(BLPAPI_DATETIME_DEFAULT_MILLI_SECONDS),
            month: Some(BLPAPI_DATETIME_DEFAULT_MONTH),
            day: Some(BLPAPI_DATETIME_DEFAULT_DAY),
            year: Some(BLPAPI_DATETIME_DEFAULT_YEAR),
            offset: Some(BLPAPI_DATETIME_DEFAULT_OFFSET),
            leap_year: Some(BLPAPI_DEFAULT_LEAP_YEAR),
        }
    }
}

/// Setting functions of DatetimeBuilder
impl DatetimeBuilder {
    /// Setting parts
    pub fn set_parts(mut self, parts: usize) -> Self {
        self.parts = Some(parts);
        self
    }

    /// Setting hours
    pub fn set_hours(mut self, hour: usize) -> Self {
        if hour <= 23 {
            self.hours = Some(hour);
            self.parts = Some(
                self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS)
                    | BLPAPI_DATETIME_HOURS_PART as usize,
            );
        }
        self
    }

    /// Setting minutes
    pub fn set_minutes(mut self, minutes: usize) -> Self {
        if minutes <= 59 {
            self.minutes = Some(minutes);
            self.parts = Some(
                self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS)
                    | BLPAPI_DATETIME_MINUTES_PART as usize,
            );
        }
        self
    }
    /// Setting seconds
    pub fn set_seconds(mut self, seconds: usize) -> Self {
        if seconds <= 59 {
            self.seconds = Some(seconds);
            self.parts = Some(
                self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS)
                    | BLPAPI_DATETIME_SECONDS_PART as usize,
            );
        }
        self
    }

    /// Setting fractions of seconds (milliseconds to picoseconds)
    pub fn set_fraction_of_seconds(mut self, fs: usize) -> Self {
        if fs > 0 {
            self.picoseconds = Some(0);
            self.offset = Some(0);
            if fs < TimeFractions::MilliSeconds.get_value() {
                self.milliseconds = Some(fs);
            } else if fs < TimeFractions::MicroSeconds.get_value() {
                self.milliseconds = Some(fs / 1_000);
                self.picoseconds = Some(((fs % 1000) * 1000 * 1000) as i32);
            } else if fs < TimeFractions::NanoSeconds.get_value() {
                self.milliseconds = Some(fs / 1_000 / 1_000);
                self.picoseconds = Some(((fs % (1000 * 1000)) * 1000) as i32);
            } else if fs < TimeFractions::PicoSeconds.get_value() {
                self.milliseconds = Some(fs / 1_000 / 1_000 / 1_000);
                self.picoseconds = Some((fs % (1000 * 1000 * 1000)) as i32);
            }
            self.parts = Some(
                BLPAPI_DATETIME_DATE_PART as usize | BLPAPI_DATETIME_TIMEFRACSECONDS_PART as usize,
            );
        }
        self
    }

    /// Setting month
    pub fn set_month(mut self, month: usize) -> Self {
        if month > 0 && month <= 12 {
            self.month = Some(month);
            self.parts = Some(
                self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS)
                    | BLPAPI_DATETIME_MONTH_PART as usize,
            );
        }
        self
    }

    /// Setting day
    pub fn set_day(mut self, day: usize) -> Self {
        if day == 0 || day > 32 {
            return self;
        };

        let month = match self.month {
            Some(month) => month,
            None => panic!("Set month before setting day"),
        };

        if DATETIME_30_MONTH.contains(&month) && day > 30 {
            return self;
        };

        if month == 2 {
            let max_day = match self.leap_year {
                Some(true) => 29,
                _ => 28,
            };

            if day > max_day {
                panic!(
                    "Set day {} is greater than max day {}. Really leap year?",
                    day, max_day
                );
            }
        }

        self.day = Some(day);
        self.parts = Some(
            self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS) | BLPAPI_DATETIME_DAY_PART as usize,
        );
        self
    }

    /// Setting year
    pub fn set_year(mut self, year: usize) -> Self {
        if year > 0 || year < 9999 {
            self.year = Some(year);
            self.leap_year = Some(is_leap_year(year));
            self.parts = Some(
                self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS)
                    | BLPAPI_DATETIME_YEAR_PART as usize,
            );
        }
        self
    }

    /// Setting offset
    pub fn set_offset(mut self, offset: i16) -> Self {
        if (-840..840).contains(&offset) {
            self.offset = Some(offset);
            self.parts = Some(
                self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS)
                    | BLPAPI_DATETIME_OFFSET_PART as usize,
            );
        }
        self
    }

    /// validate date
    pub fn is_valid_date(&self) -> bool {
        if self.year.is_some() && self.month.is_some() && self.day.is_some() {
            let y = self.year.expect("Expected year");
            let m = self.month.expect("Expected month");
            let d = self.day.expect("Expected day");
            if y != 0 && m != 0 && d != 0 {
                return true;
            }
        }
        false
    }

    /// validate time
    pub fn is_valid_time(&self) -> bool {
        if self.hours.is_some() && self.minutes.is_some() && self.seconds.is_some() {
            let h = self.hours.expect("Expected hours");
            let m = self.minutes.expect("Expected minutes");
            let s = self.seconds.expect("Expected seconds");
            if h != 0 && m != 0 && s != 0 {
                return true;
            }
        }
        false
    }
    /// validate all (depreciated)
    pub fn is_valid(&self) -> bool {
        let dt = self.is_valid_date();
        let time = self.is_valid_time();
        if dt || time {
            return true;
        }
        false
    }

    /// Build
    pub fn build(self) -> Datetime {
        let ptr: blpapi_Datetime_t = blpapi_Datetime_t {
            parts: self.parts.expect("Expected Parts set first") as c_uchar,
            hours: self.hours.expect("Expected hours set first") as c_uchar,
            minutes: self.minutes.expect("Expected minutes set first") as c_uchar,
            seconds: self.seconds.expect("Expected seconds set first") as c_uchar,
            milliSeconds: self.milliseconds.expect("Expected milli seconds set first") as c_ushort,
            month: self.month.expect("Expected month set first") as c_uchar,
            day: self.day.expect("Expected day set first") as c_uchar,
            year: self.year.expect("Expected year set first") as c_ushort,
            offset: self.offset.expect("Expected offset set first"),
        };
        Datetime { ptr }
    }
}

/// Datetime Struct
pub struct Datetime {
    pub(crate) ptr: blpapi_Datetime_t,
}

impl Default for Datetime {
    fn default() -> Self {
        let part = BLPAPI_DATETIME_DATE_PART | BLPAPI_DATETIME_TIME_PART;
        Self {
            ptr: blpapi_Datetime_t {
                parts: part as c_uchar,
                hours: BLPAPI_DATETIME_DEFAULT_HOURS as c_uchar,
                minutes: BLPAPI_DATETIME_DEFAULT_MINUTES as c_uchar,
                seconds: BLPAPI_DATETIME_DEFAULT_SECONDS as c_uchar,
                milliSeconds: BLPAPI_DATETIME_DEFAULT_MILLI_SECONDS as c_ushort,
                month: BLPAPI_DATETIME_DEFAULT_MONTH as c_uchar,
                day: BLPAPI_DATETIME_DEFAULT_DAY as c_uchar,
                year: BLPAPI_DATETIME_DEFAULT_YEAR as c_ushort,
                offset: BLPAPI_DATETIME_DEFAULT_OFFSET,
            },
        }
    }
}

/// High Precision Datetime Builder struct
#[derive(Debug)]
pub struct HighPrecisionDateTimeBuilder {
    pub datetime: DatetimeBuilder,
}

impl Default for HighPrecisionDateTimeBuilder {
    fn default() -> Self {
        let pico_sec = 0;
        let dt_time = DatetimeBuilder::default().set_fraction_of_seconds(pico_sec);
        Self { datetime: dt_time }
    }
}

impl HighPrecisionDateTimeBuilder {
    /// new HighPrecisionDateTimeBuilder
    pub fn new() -> Self {
        Self::default()
    }
    /// Setting datetime
    pub fn set_datetime(mut self, datetime: DatetimeBuilder) -> Self {
        self.datetime = datetime;
        self
    }

    /// Build
    pub fn build(self) -> HighPrecisionDateTime {
        let pico_seconds = &self.datetime.picoseconds.expect("Expected picoseconds");
        let dt_ptr = self.datetime.build();

        let ptr = blpapi_HighPrecisionDatetime_tag {
            datetime: dt_ptr.ptr,
            picoseconds: *pico_seconds as c_uint,
        };

        HighPrecisionDateTime { ptr }
    }
}

/// High Precision Datetime Struct
#[derive(Debug)]
pub struct HighPrecisionDateTime {
    pub ptr: blpapi_HighPrecisionDatetime_t,
}

impl Default for HighPrecisionDateTime {
    fn default() -> Self {
        HighPrecisionDateTimeBuilder::new().build()
    }
}

/// Implementing HighPrecisionDateTime Default
impl HighPrecisionDateTime {
    // compare High Precision Datetime
    pub fn compare_datetime(mut self, mut rhs: HighPrecisionDateTime) -> bool {
        let res = unsafe {
            blpapi_HighPrecisionDatetime_compare(
                &mut self.ptr as *const _,
                &mut rhs.ptr as *const _,
            )
        };

        match res == 0 {
            true => true,
            false => false,
        }
    }

    /// High Precision Datetime from TimePoint
    pub fn get_from_time_point(mut self, mut time_point: TimePoint, offset: i64) -> Self {
        let ptr = &mut self.ptr as *mut blpapi_HighPrecisionDatetime_t;
        let time_p = &mut time_point.point as *const _;

        //Work on the self.ptr issue which results in an invalid return of datetime format
        let res =
            unsafe { blpapi_HighPrecisionDatetime_fromTimePoint(ptr, time_p, offset as c_short) }
                as i64;
        if res != 0 {
            panic!("Invalid time point or offset")
        };
        self
    }

    /// Implementing the writer function to return the details about the High Precision Datetime struct
    pub fn print<T: Write>(
        mut self,
        writer: &mut T,
        indent: i32,
        spaces: i32,
    ) -> Result<Self, Error> {
        let mut context = StreamWriterContext { writer };
        unsafe {
            let res = blpapi_HighPrecisionDatetime_print(
                &mut self.ptr as *const blpapi_HighPrecisionDatetime_t,
                Some(write_to_stream_cb),
                &mut context as *mut _ as *mut c_void,
                indent as std::ffi::c_int,
                spaces as std::ffi::c_int,
            );
            if res != 0 {
                return Err(Error::struct_error(
                    "HighPrecisionDateTime",
                    "print",
                    "Error when trying to write to stream writer",
                ));
            };
        };
        Ok(self)
    }
}

pub enum DatetimeParts {
    Year,
    Month,
    Day,
    Offset,
    Hours,
    Minutes,
    Seconds,
    FracSeconds,
    Milliseconds,
    Date,
    Time,
    TimeFracSeconds,
    Unknown,
}

impl From<c_int> for DatetimeParts {
    fn from(e: c_int) -> Self {
        match e as u32 {
            BLPAPI_DATETIME_YEAR_PART => DatetimeParts::Year,
            BLPAPI_DATETIME_MONTH_PART => DatetimeParts::Month,
            BLPAPI_DATETIME_DAY_PART => DatetimeParts::Day,
            BLPAPI_DATETIME_OFFSET_PART => DatetimeParts::Offset,
            BLPAPI_DATETIME_HOURS_PART => DatetimeParts::Hours,
            BLPAPI_DATETIME_MINUTES_PART => DatetimeParts::Minutes,
            BLPAPI_DATETIME_SECONDS_PART => DatetimeParts::Seconds,
            BLPAPI_DATETIME_FRACSECONDS_PART => DatetimeParts::FracSeconds,
            BLPAPI_DATETIME_DATE_PART => DatetimeParts::Date,
            BLPAPI_DATETIME_TIME_PART => DatetimeParts::Time,
            BLPAPI_DATETIME_TIMEFRACSECONDS_PART => DatetimeParts::TimeFracSeconds,
            _ => DatetimeParts::Unknown,
        }
    }
}

/// Is Leap Year
pub fn is_leap_year(y: usize) -> bool {
    y % 4 == 0 && (y <= 1752 || y % 100 != 0 || y % 400 == 0)
}

impl Datetime {
    /// Compare Dates
    /// Returns True if dates are the same date
    pub fn compare_datetime(self, rhs: Datetime) -> bool {
        let res = unsafe { blpapi_Datetime_compare(self.ptr, rhs.ptr) };

        match res == 0 {
            true => true,
            false => false,
        }
    }

    /// Implementing the writer function to return the details about the Datetime struct
    pub fn print<T: Write>(
        mut self,
        writer: &mut T,
        indent: i32,
        spaces: i32,
    ) -> Result<(), Error> {
        let mut context = StreamWriterContext { writer };
        unsafe {
            let res = blpapi_Datetime_print(
                &mut self.ptr as *const blpapi_Datetime_t,
                Some(write_to_stream_cb),
                &mut context as *mut _ as *mut c_void,
                indent as std::ffi::c_int,
                spaces as std::ffi::c_int,
            );
            if res != 0 {
                return Err(Error::struct_error(
                    "Datetime",
                    "print",
                    "Error when trying to write to stream writer",
                ));
            };
        };
        Ok(())
    }
}

impl std::fmt::Debug for Datetime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let d = self.ptr;
        match (
            d.year,
            d.month,
            d.day,
            d.hours,
            d.minutes,
            d.seconds,
            d.milliSeconds,
        ) {
            (y, m, d, 0, 0, 0, 0) => write!(f, "{:04}-{:02}-{:02}", y, m, d),
            (0, 0, 0, h, mm, s, ms) => write!(f, "{:02}:{:02}:{:02}.{:03}", h, mm, s, ms),
            (y, m, d, h, mm, s, ms) => write!(
                f,
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
                y, m, d, h, mm, s, ms
            ),
        }
    }
}
