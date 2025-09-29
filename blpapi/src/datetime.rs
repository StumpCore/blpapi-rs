use crate::core::{write_to_stream_cb, StreamWriterContext};
use crate::datetime::TimeFractions::PicoSeconds;
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
pub const DATETIME_30_MONTH: &'static [usize] = &[2, 4, 6, 9, 11];


/// TimePointer Builder
pub struct TimePointBuilder {
    pub point: i64,
}

impl TimePointBuilder {
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
        TimePoint {
            point,
        }
    }
}

/// Implementing Default for TimePoint
impl Default for TimePointBuilder {
    fn default() -> Self {
        TimePointBuilder {
            point: 0,
        }
    }
}

/// TimePoint
pub struct TimePoint {
    pub(crate) point: blpapi_TimePoint_t,
}

impl TimePoint {
    /// Distance between two time points
    pub fn nano_seconds_between(&mut self, mut other: TimePoint) -> i64 {
        let res = unsafe {
            blpapi_TimePointUtil_nanosecondsBetween(
                &mut self.point as *const _,
                &mut other.point,
            )
        };
        res as i64
    }
}

/// Time Fractions
pub enum TimeFractions {
    MicroSeconds,
    MilliSeconds,
    NanoSeconds,
    PicoSeconds,
}
/// Builder of the Datetime struct
pub struct DatetimeBuilder {
    pub parts: Option<usize>,
    pub hours: Option<usize>,
    pub minutes: Option<usize>,
    pub seconds: Option<usize>,
    pub fraction_of_seconds: Option<usize>,
    pub month: Option<usize>,
    pub day: Option<usize>,
    pub year: Option<usize>,
    pub offset: Option<i16>,
}

/// Default Implementation of DatetimeBuilder
impl Default for DatetimeBuilder {
    fn default() -> Self {
        Self {
            parts: Some(BLPAPI_DATETIME_DEFAULT_PARTS),
            hours: Some(BLPAPI_DATETIME_DEFAULT_HOURS),
            minutes: Some(BLPAPI_DATETIME_DEFAULT_MINUTES),
            seconds: Some(BLPAPI_DATETIME_DEFAULT_SECONDS),
            fraction_of_seconds: Some(BLPAPI_DATETIME_DEFAULT_MILLI_SECONDS),
            month: Some(BLPAPI_DATETIME_DEFAULT_MONTH),
            day: Some(BLPAPI_DATETIME_DEFAULT_DAY),
            year: Some(BLPAPI_DATETIME_DEFAULT_YEAR),
            offset: Some(BLPAPI_DATETIME_DEFAULT_OFFSET),
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
            self.parts = Some(self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS) | BLPAPI_DATETIME_HOURS_PART as usize);
        }
        self
    }

    /// Setting minutes
    pub fn set_minutes(mut self, minutes: usize) -> Self {
        if minutes <= 59 {
            self.minutes = Some(minutes);
            self.parts = Some(self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS) | BLPAPI_DATETIME_MINUTES_PART as usize);
        }
        self
    }
    /// Setting seconds
    pub fn set_seconds(mut self, seconds: usize) -> Self {
        if seconds <= 59 {
            self.seconds = Some(seconds);
            self.parts = Some(self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS) | BLPAPI_DATETIME_SECONDS_PART as usize);
        }
        self
    }

    /// Setting milli seconds
    pub fn set_fraction_of_seconds(mut self, fraction: TimeFractions) -> Self {
        let frac = match fraction {
            TimeFractions::PicoSeconds => { 1_000_000_000_000 }
            TimeFractions::NanoSeconds => { 1_000_000_000 }
            TimeFractions::MicroSeconds => { 1_000_000 }
            TimeFractions::MilliSeconds => { 1000 }
        };
        self.fraction_of_seconds = Some(frac);
        self.parts = Some(self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS) | BLPAPI_DATETIME_FRACSECONDS_PART as usize);
        self
    }

    /// Setting month
    pub fn set_month(mut self, month: usize) -> Self {
        if month > 0 && month <= 12 {
            self.month = Some(month);
            self.parts = Some(self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS) | BLPAPI_DATETIME_MONTH_PART as usize);
        }
        self
    }

    /// Setting day
    pub fn set_day(mut self, day: usize) -> Self {
        if day > 0 && day <= 31 {
            match self.month {
                Some(month) => {
                    if DATETIME_30_MONTH.contains(&month) {
                        if day > 30 {
                            panic!("Invalid day for month: {}", month);
                        } else {
                            if month == 2 && day > 28 {
                                let y = self.year.expect("Expected year set first");
                                if is_leap_year(y) {
                                    if day > 29 {
                                        println!("Is leap year");
                                        panic!("Invalid day for month: {}", month);
                                    }
                                } else {
                                    if day > 28 {
                                        panic!("Invalid day for month: {}", month);
                                    }
                                }
                            }
                        }
                    };
                    self.day = Some(day);
                    self.parts = Some(self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS) | BLPAPI_DATETIME_DAY_PART as usize);
                }
                None => panic!("Set month before setting day"),
            }
        }
        self
    }

    /// Setting year
    pub fn set_year(mut self, year: usize) -> Self {
        if year > 0 || year < 9999 {
            self.year = Some(year);
            self.parts = Some(self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS) | BLPAPI_DATETIME_YEAR_PART as usize);
        }
        self
    }

    /// Setting offset
    pub fn set_offset(mut self, offset: i16) -> Self {
        self.offset = Some(offset);
        self.parts = Some(self.parts.unwrap_or(BLPAPI_DATETIME_DEFAULT_PARTS) | BLPAPI_DATETIME_OFFSET_PART as usize);
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
        return false;
    }

    /// validate time
    pub fn is_valid_time(&self) -> bool {
        if self.hours.is_some() && self.minutes.is_some() && self.seconds.is_some() && self.fraction_of_seconds.is_some() {
            let h = self.hours.expect("Expected hours");
            let m = self.minutes.expect("Expected minutes");
            let s = self.seconds.expect("Expected seconds");
            if h != 0 && m != 0 && s != 0 {
                return true;
            }
        }
        return false;
    }
    /// validate all (depreciated)
    pub fn is_valid(&self) -> bool {
        let dt = self.is_valid_date();
        let time = self.is_valid_time();
        if dt || time {
            return true;
        }
        return false;
    }


    /// Build
    pub fn build(self) -> Datetime {
        let ptr: blpapi_Datetime_t = blpapi_Datetime_t {
            parts: self.parts.expect("Expected Parts set first") as c_uchar,
            hours: self.hours.expect("Expected hours set first") as c_uchar,
            minutes: self.minutes.expect("Expected minutes set first") as c_uchar,
            seconds: self.seconds.expect("Expected seconds set first") as c_uchar,
            milliSeconds: self.fraction_of_seconds.expect("Expected milli seconds set first") as c_ushort,
            month: self.month.expect("Expected month set first") as c_uchar,
            day: self.day.expect("Expected day set first") as c_uchar,
            year: self.year.expect("Expected year set first") as c_ushort,
            offset: self.offset.expect("Expected offset set first"),
        };
        Datetime {
            ptr
        }
    }
}
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
            }
        }
    }
}

/// High Precision Datetime Builder struct
pub struct HighPrecisionDateTimeBuilder {
    pub datetime: DatetimeBuilder,
    pub pico_seconds: TimeFractions,
}

impl Default for HighPrecisionDateTimeBuilder {
    fn default() -> Self {
        let pico_sec = PicoSeconds;
        let dt_time = DatetimeBuilder::default().set_fraction_of_seconds(pico_sec);
        Self {
            datetime: dt_time,
            pico_seconds: TimeFractions::PicoSeconds,
        }
    }
}

impl HighPrecisionDateTimeBuilder {
    /// Setting datetime
    pub fn set_datetime(mut self, datetime: DatetimeBuilder) -> Self {
        self.datetime = datetime;
        self
    }

    /// Build
    pub fn build(self) -> HighPrecisionDateTime {
        let dt_ptr = self.datetime.build();

        let ptr = blpapi_HighPrecisionDatetime_t {
            datetime: dt_ptr.ptr,
            picoseconds: self.pico_seconds as c_uint,
        };

        HighPrecisionDateTime {
            ptr,
        }
    }
}


/// High Precision Datetime Struct
pub struct HighPrecisionDateTime {
    pub(crate) ptr: blpapi_HighPrecisionDatetime_t,
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
        let res = unsafe {
            blpapi_HighPrecisionDatetime_fromTimePoint(
                &mut self.ptr,
                &mut time_point.point as *const _,
                offset as c_short,
            )
        } as i64;
        if res != 0 {
            panic!("Invalid time point or offset")
        };
        self
    }

    /// Implementing the writer function to return the details about the High Precision Datetime struct
    pub fn print<T: Write>(mut self, writer: &mut T, indent: i32, spaces: i32) -> Result<Self, Error> {
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
        let res = unsafe {
            blpapi_Datetime_compare(
                self.ptr,
                rhs.ptr,
            )
        };

        match res == 0 {
            true => true,
            false => false,
        }
    }

    /// Implementing the writer function to return the details about the Datetime struct
    pub fn print<T: Write>(mut self, writer: &mut T, indent: i32, spaces: i32) -> Result<(), Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_time_point() {
        let tp = TimePointBuilder::default();
        let _tp = tp.set_time_point(1000).build();
    }

    #[test]
    pub fn test_time_point_diff() {
        let tp = TimePointBuilder::default();
        let tp_two = TimePointBuilder::default();
        let mut tp = tp.set_time_point(100000).build();
        let tp_two = tp_two.set_time_point(10000).build();
        let diff = tp.nano_seconds_between(tp_two);
        println!("{:?}", diff);
    }

    #[test]
    pub fn test_datetime_builder() {
        let dt_time = DatetimeBuilder::default();
        let _dt = dt_time.build();
    }

    #[test]
    pub fn test_datetime_compare() {
        let dt_time = DatetimeBuilder::default().build();
        let dt_time_two = DatetimeBuilder::default().build();
        assert!(dt_time.compare_datetime(dt_time_two));
    }

    #[test]
    pub fn test_datetime_debug() {
        let dt_time = DatetimeBuilder::default().build();
        println!("{:?}", dt_time);
    }


    #[test]
    pub fn test_datetime_set_year() {
        let y = 2024;
        let dt_time = DatetimeBuilder::default().set_year(y);
        assert_eq!(dt_time.year.unwrap(), 2024);
    }

    #[test]
    pub fn test_datetime_set_month() {
        let y = 2024;
        let m = 12;
        let dt_time = DatetimeBuilder::default()
            .set_year(y).set_month(m);
        assert_eq!(dt_time.year.unwrap(), 2024);
        assert_eq!(dt_time.month.unwrap(), 12);
    }

    #[test]
    pub fn test_datetime_set_days() {
        let y = 2024;
        let m = 12;
        let d = 30;
        let dt_time = DatetimeBuilder::default()
            .set_year(y)
            .set_month(m)
            .set_day(d);
        assert_eq!(dt_time.year.unwrap(), 2024);
        assert_eq!(dt_time.month.unwrap(), 12);
        assert_eq!(dt_time.day.unwrap(), 30);

        // invalid day for month unchanged
        let d = 32;
        let dt_time = dt_time.set_day(d);
        assert_eq!(dt_time.day.unwrap(), 30);
    }

    #[test]
    #[should_panic]
    pub fn test_datetime_panic_days() {
        let y = 2024;
        let m = 11;
        let d = 31;
        let dt_time = DatetimeBuilder::default()
            .set_year(y)
            .set_month(m)
            .set_day(d);
        assert_eq!(dt_time.day.unwrap(), 30);
    }

    #[test]
    #[should_panic]
    pub fn test_datetime_panic_days_feb() {
        let y = 2024;
        let m = 2;
        let d = 29;
        let dt_time = DatetimeBuilder::default()
            .set_year(y)
            .set_month(m)
            .set_day(d);
        assert_eq!(dt_time.day.unwrap(), 28);
    }

    #[test]
    #[should_panic]
    pub fn test_datetime_panic_days_feb_one() {
        let y = 2024;
        let m = 2;
        let d = 29;
        let dt_time = DatetimeBuilder::default()
            .set_year(y)
            .set_month(m)
            .set_day(d);
        assert_eq!(dt_time.day.unwrap(), 28);
    }

    #[test]
    #[should_panic]
    pub fn test_datetime_panic_days_feb_two() {
        let y = 2025;
        let m = 2;
        let d = 29;
        let dt_time = DatetimeBuilder::default()
            .set_year(y)
            .set_month(m)
            .set_day(d);
    }

    #[test]
    pub fn test_datetime_print() {
        let mut output_buffer = Vec::new();
        let dt_time = DatetimeBuilder::default()
            .set_hours(1)
            .set_minutes(10)
            .set_seconds(24)
            .set_fraction_of_seconds(TimeFractions::MilliSeconds)
            .set_day(1)
            .set_month(10)
            .set_year(2025)
            .build();
        let res = dt_time.print(
            &mut output_buffer,
            2,
            4,
        );
        assert!(res.is_ok());
        let output_string = String::from_utf8(output_buffer).unwrap();
        println!("{}", output_string);
    }


    #[test]
    pub fn test_highdatetime_print() {
        let mut output_buffer = Vec::new();
        let dt_time = DatetimeBuilder::default()
            .set_hours(1)
            .set_minutes(10)
            .set_seconds(24)
            .set_fraction_of_seconds(TimeFractions::MilliSeconds)
            .set_day(1)
            .set_month(10)
            .set_year(2025);
        let picto_secs = TimeFractions::PicoSeconds;
        let hp_datetime = HighPrecisionDateTimeBuilder::default().set_datetime(dt_time);
        let hp_dt = hp_datetime.build();

        let res = hp_dt.print(
            &mut output_buffer,
            2,
            4,
        );
        assert!(res.is_ok());
        let output_string = String::from_utf8(output_buffer).unwrap();
        println!("{}", output_string);
    }

    #[test]
    pub fn test_highdatetime_from_time_point() {
        // new timepoint set
        let mut output_buffer = Vec::new();
        let dt_time = DatetimeBuilder::default()
            .set_hours(1)
            .set_minutes(10)
            .set_seconds(24)
            .set_fraction_of_seconds(TimeFractions::MilliSeconds)
            .set_day(1)
            .set_month(10)
            .set_year(2025);
        let hp_datetime = HighPrecisionDateTimeBuilder::default().set_datetime(dt_time);
        let hp_dt = hp_datetime.build();
        let tp = TimePointBuilder::default().set_time_point(1000).build();
        let offset = 1000;

        let hp_dt = hp_dt.get_from_time_point(tp, offset);

        let res = hp_dt.print(
            &mut output_buffer,
            2,
            4,
        );
        assert!(res.is_ok());
        let output_string = String::from_utf8(output_buffer).unwrap();
        println!("{}", output_string);
    }
}