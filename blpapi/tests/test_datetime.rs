use blpapi::datetime::{DatetimeBuilder, HighPrecisionDateTimeBuilder, TimePointBuilder};
#[test]
pub fn test_datetime_time_point() {
    let tp = TimePointBuilder::default();
    let _tp = tp.set_time_point(1000).build();
}

#[test]
pub fn test_datetime_time_point_diff() {
    let tp = TimePointBuilder::default();
    let tp_two = TimePointBuilder::default();
    let tp = tp.set_time_point(100000).build();
    let tp_two = tp_two.set_time_point(10000).build();
    let diff = tp.nano_seconds_between(&tp_two);
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
    let dt_time = DatetimeBuilder::default().set_year(y).set_month(m);
    assert_eq!(dt_time.year.unwrap(), 2024);
    assert_eq!(dt_time.month.unwrap(), 12);
}

#[test]
#[should_panic]
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
    let _dt_time = DatetimeBuilder::default()
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
        .set_day(1)
        .set_month(10)
        .set_year(2025)
        .build();
    let res = dt_time.print(&mut output_buffer, 2, 4);
    assert!(res.is_ok());
    let output_string = String::from_utf8(output_buffer).unwrap();
    println!("{}", output_string);
}

#[test]
pub fn test_datetime_milli_print() {
    let mut output_buffer = Vec::new();
    let dt_time = DatetimeBuilder::default()
        .set_hours(1)
        .set_minutes(10)
        .set_seconds(24)
        .set_fraction_of_seconds(900)
        .set_day(1)
        .set_month(10)
        .set_year(2025)
        .build();
    let res = dt_time.print(&mut output_buffer, 2, 4);
    assert!(res.is_ok());
    let output_string = String::from_utf8(output_buffer).unwrap();
    println!("{}", output_string);
}

#[test]
pub fn test_datetime_micro_print() {
    let mut output_buffer = Vec::new();
    let dt_time = DatetimeBuilder::default()
        .set_hours(1)
        .set_minutes(10)
        .set_seconds(24)
        .set_fraction_of_seconds(9999)
        .set_day(1)
        .set_month(10)
        .set_year(2025)
        .build();
    let res = dt_time.print(&mut output_buffer, 2, 4);
    assert!(res.is_ok());
    let output_string = String::from_utf8(output_buffer).unwrap();
    println!("{}", output_string);
}

#[test]
pub fn test_datetime_date() {
    let day_vec = vec![
        [1, 1, 2025, 1, 20, 10, 100, 1],
        [15, 2, 2025, 2, 30, 20, 900, 10],
        [30, 4, 1788, 3, 40, 30, 2000, 100],
        [30, 4, 1788, 3, 40, 30, 2022, 1000],
        [30, 4, 1788, 3, 40, 30, 0, 60],
    ];
    for date in day_vec {
        let mut output_buffer = Vec::new();
        let d = date[0];
        let m = date[1];
        let y = date[2];

        let h = date[3];
        let min = date[4];
        let sec = date[5];
        let ms = date[6];

        let off_set = date[7] as i16;

        let dt_time = DatetimeBuilder::default()
            .set_day(d)
            .set_month(m)
            .set_year(y)
            .set_hours(h)
            .set_minutes(min)
            .set_seconds(sec)
            .set_offset(off_set)
            .set_fraction_of_seconds(ms);

        // Default time print
        if ms >= 1000 {
            let hp_datetime = HighPrecisionDateTimeBuilder::default().set_datetime(dt_time);
            let hp_dt = hp_datetime.build();
            let res = hp_dt.print(&mut output_buffer, 2, 4);
            assert!(res.is_ok());
        } else {
            let hp_dt = dt_time.build();
            let res = hp_dt.print(&mut output_buffer, 2, 4);
            assert!(res.is_ok());
        }
        let output_string = String::from_utf8(output_buffer).unwrap();
        println!("{}", output_string);
    }
}

#[test]
pub fn test_datetime_highdatetime_print() {
    let mut output_buffer = Vec::new();
    let dt_time = DatetimeBuilder::default()
        .set_hours(1)
        .set_minutes(10)
        .set_seconds(24)
        .set_day(1)
        .set_month(10)
        .set_year(2025);
    let hp_datetime = HighPrecisionDateTimeBuilder::default().set_datetime(dt_time);
    let hp_dt = hp_datetime.build();

    let res = hp_dt.print(&mut output_buffer, 2, 4);
    assert!(res.is_ok());
    let output_string = String::from_utf8(output_buffer).unwrap();
    println!("{}", output_string);
}

#[test]
pub fn test_datetime_highdatetime_from_time_point() {
    // new timepoint set
    let mut output_buffer = Vec::new();
    let dt_time = DatetimeBuilder::default()
        .set_hours(1)
        .set_minutes(10)
        .set_seconds(24)
        .set_day(1)
        .set_month(10)
        .set_year(2025);
    let hp_datetime = HighPrecisionDateTimeBuilder::default().set_datetime(dt_time);
    let hp_dt = hp_datetime.build();
    let tp = TimePointBuilder::default()
        .set_time_point(1_732_867_400_000_000_000)
        .build();
    let offset = 0;

    let hp_dt = hp_dt.get_from_time_point(tp, offset);
    assert_eq!(hp_dt.ptr.datetime.day, 29);
    assert_eq!(hp_dt.ptr.datetime.month, 11);
    assert_eq!(hp_dt.ptr.datetime.year, 2024);

    let res = hp_dt.print(&mut output_buffer, 2, 4);
    assert!(res.is_ok());
    let output_string = String::from_utf8(output_buffer).unwrap();
    println!("{}", output_string);
}
