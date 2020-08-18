use super::error::WatchFaceError;
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DateFormat {
    HourMinute,
    HourMinuteSecond,
    MonthDayHourMinute,
    MonthDayHourMinuteSecond,
    YearMonthDayHourMinute,
    YearMonthDayHourMinuteSecond,
}

impl DateFormat {
    pub fn has_year(&self) -> bool {
        match self {
            DateFormat::YearMonthDayHourMinute => true,
            DateFormat::YearMonthDayHourMinuteSecond => true,
            _ => false,
        }
    }

    pub fn has_month_day(&self) -> bool {
        match self {
            DateFormat::YearMonthDayHourMinute => true,
            DateFormat::YearMonthDayHourMinuteSecond => true,
            DateFormat::MonthDayHourMinute => true,
            DateFormat::MonthDayHourMinuteSecond => true,
            _ => false,
        }
    }

    pub fn has_second(&self) -> bool {
        match self {
            DateFormat::HourMinuteSecond => true,
            DateFormat::YearMonthDayHourMinuteSecond => true,
            DateFormat::MonthDayHourMinuteSecond => true,
            _ => false,
        }
    }
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
