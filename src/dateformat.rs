use super::error::WatchFaceError;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
pub enum DateFormat {
    HourMinute,
    HourMinuteSecond,
    MonthDayHourMinute,
    MonthDayHourMinuteSecond,
    YearMonthDayHourMinute,
    YearMonthDayHourMinuteSecond,
}

impl FromStr for DateFormat {
    type Err = WatchFaceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "hm" => Ok(DateFormat::HourMinute),
            "hms" => Ok(DateFormat::HourMinuteSecond),
            "mdhm" => Ok(DateFormat::MonthDayHourMinute),
            "mdhms" => Ok(DateFormat::MonthDayHourMinuteSecond),
            "ymdhm" => Ok(DateFormat::YearMonthDayHourMinute),
            "ymdhms" => Ok(DateFormat::YearMonthDayHourMinuteSecond),
            _ => Err(WatchFaceError::DateFormatParseError(s.to_string())),
        }
    }
}

impl Display for DateFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DateFormat::HourMinute => "HourMinute".fmt(f),
            DateFormat::HourMinuteSecond => "HourMinuteSecond".fmt(f),
            DateFormat::MonthDayHourMinute => "MonthDayHourMinute".fmt(f),
            DateFormat::MonthDayHourMinuteSecond => "MonthDayHourMinuteSecond".fmt(f),
            DateFormat::YearMonthDayHourMinute => "YearMonthDayHourMinute".fmt(f),
            DateFormat::YearMonthDayHourMinuteSecond => "YearMonthDayHourMinuteSecond".fmt(f),
        }
    }
}
