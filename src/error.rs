use std::fmt;

#[derive(Debug)]
pub enum WatchFaceError {
    DateFormatParseError(String),
    PrintFormatParseError(String),
}

impl fmt::Display for WatchFaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WatchFaceError::DateFormatParseError(s) => {
                f.write_fmt(format_args!("DateFormatParseError: {}", s))
            }
            WatchFaceError::PrintFormatParseError(s) => {
                f.write_fmt(format_args!("PrintFormatParseError: {}", s))
            }
        }
    }
}
