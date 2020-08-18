extern crate clap;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use std::io::{stdin, Write};
use std::string::String;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use time::OffsetDateTime;

pub mod bitsplay;
pub mod dateformat;
pub mod error;
pub mod printformat;

enum TermEvent {
    Refresh,
    Quit,
}

fn update_tv(
    now: &OffsetDateTime,
    df: &dateformat::DateFormat,
    pf: &printformat::PrintFormat,
) -> (String, u16, u16) {
    let time = match df {
        dateformat::DateFormat::YearMonthDayHourMinuteSecond => {
            now.format("%0Y-%0m-%0d %0H:%0M:%0S")
        }
        dateformat::DateFormat::YearMonthDayHourMinute => now.format("%0Y-%0m-%0d %0H:%0M"),
        dateformat::DateFormat::MonthDayHourMinuteSecond => now.format("%0m-%0d %0H:%0M:%0S"),
        dateformat::DateFormat::MonthDayHourMinute => now.format("%0m-%0d %0H:%0M"),
        dateformat::DateFormat::HourMinuteSecond => now.format("%0H:%0M:%0S"),
        dateformat::DateFormat::HourMinute => now.format("%0H:%0M"),
    };
    let size = time.chars().count() as u16;
    match pf {
        printformat::PrintFormat::Ascii => (time, size, 1),
        printformat::PrintFormat::BlockPix => {
            let (main, width) = bitsplay::blockpix(time);
            (main, width, 2)
        }
        printformat::PrintFormat::SegmentPix => {
            let (main, width) = bitsplay::segmentpix(time);
            (main, width, 3)
        }
        _ => ("unimplemented".to_string(), 13, 1),
    }
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("dateformat")
                .short("d")
                .long("dateformat")
                .empty_values(false)
                .default_value("hm")
                .possible_values(&["hm", "hms", "mdhm", "mdhms", "ymdhm", "ymdhms"])
                .case_insensitive(true),
        )
        .arg(
            Arg::with_name("printformat")
                .short("p")
                .long("printformat")
                .empty_values(false)
                .default_value("ascii")
                .possible_values(&["ascii", "block", "segments", "blockpix", "segmentpix"])
                .case_insensitive(true),
        )
        .get_matches();

    // Unwrap is used because clap verifies values
    let dateformat = matches
        .value_of("dateformat")
        .unwrap_or("hm")
        .parse::<dateformat::DateFormat>()
        .unwrap();
    // Unwrap is used because clap verifies values
    let printformat = matches
        .value_of("printformat")
        .unwrap_or("ascii")
        .parse::<printformat::PrintFormat>()
        .unwrap();

    // println!("dateformat: {}", dateformat);
    // println!("printformat: {}", printformat);

    let (tx_evt, rx_evt) = mpsc::channel();

    // Stdin event reader loop
    let stdin_evt = tx_evt.clone();
    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.events() {
            let evt = c.unwrap();
            match evt {
                Event::Key(Key::Char('q')) | Event::Key(Key::Char('Q')) | Event::Key(Key::Esc) => {
                    let _ = stdin_evt.send(TermEvent::Quit);
                    break;
                }
                _ => {}
            }
        }
    });

    // Timer loop
    thread::spawn(move || {
        while tx_evt.send(TermEvent::Refresh).is_ok() {
            let now = OffsetDateTime::now_local();

            let duration: u64 = if dateformat.has_second() {
                (1000 - now.millisecond()).into()
            } else {
                (60 - (now.second() as u64)) * 1000 - (now.millisecond() as u64)
            };

            thread::sleep(Duration::from_millis(duration));
        }
    });

    // Display loop
    if let Ok(stdout) = std::io::stdout().into_raw_mode() {
        let mut stdout = termion::cursor::HideCursor::from(stdout);
        loop {
            match rx_evt.recv() {
                Err(_) | Ok(TermEvent::Quit) => {
                    break;
                }
                _ => {
                    let now = OffsetDateTime::now_local();
                    let (time, text_w, text_h) = update_tv(&now, &dateformat, &printformat);

                    let mut y = 1;
                    let mut x = 1;

                    if let Ok((w, h)) = termion::terminal_size() {
                        if text_w <= w && text_h <= h {
                            y += (h - text_h) / 2;
                            x += (w - text_w) / 2;
                        }
                    }

                    let _ = write!(stdout, "{}", termion::clear::All);

                    for (linecounter, line) in time.lines().enumerate() {
                        let _ = write!(
                            stdout,
                            "{}{}",
                            termion::cursor::Goto(x, y + (linecounter as u16)),
                            line
                        );
                    }

                    match stdout.flush() {
                        Ok(_) => {}
                        Err(_) => {
                            break;
                        }
                    }
                }
            }
        }
        let _ = write!(
            stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        );
    }

    std::process::exit(0);
}
