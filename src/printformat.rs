use super::error::WatchFaceError;
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PrintFormat {
    Ascii,
    Block,
    Segments,
    BlockPix,
    SegmentPix,
}

impl FromStr for PrintFormat {
    type Err = WatchFaceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "ascii" => Ok(PrintFormat::Ascii),
            "block" => Ok(PrintFormat::Block),
            "segments" => Ok(PrintFormat::Segments),
            "blockpix" => Ok(PrintFormat::BlockPix),
            "segmentpix" => Ok(PrintFormat::SegmentPix),
            _ => Err(WatchFaceError::PrintFormatParseError(s.to_string())),
        }
    }
}

impl Display for PrintFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrintFormat::Ascii => "Ascii".fmt(f),
            PrintFormat::Block => "Block".fmt(f),
            PrintFormat::Segments => "Segments".fmt(f),
            PrintFormat::BlockPix => "BlockPix".fmt(f),
            PrintFormat::SegmentPix => "SegmentPix".fmt(f),
        }
    }
}
