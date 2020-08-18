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
) -> String {
    if pf == &printformat::PrintFormat::Ascii {
        match df {
            dateformat::DateFormat::YearMonthDayHourMinuteSecond => {
                now.format("%0Y:%0m:%0d:%0H:%0M:%0S")
            }
            dateformat::DateFormat::YearMonthDayHourMinute => now.format("%0Y:%0m:%0d:%0H:%0M"),
            dateformat::DateFormat::MonthDayHourMinuteSecond => now.format("%0m:%0d:%0H:%0M:%0S"),
            dateformat::DateFormat::MonthDayHourMinute => now.format("%0m:%0d:%0H:%0M"),
            dateformat::DateFormat::HourMinuteSecond => now.format("%0H:%0M:%0S"),
            dateformat::DateFormat::HourMinute => now.format("%0H:%0M"),
        }
    } else {
        "unimplemented".to_string()
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
    if let Ok(mut stdout) = std::io::stdout().into_raw_mode() {
        loop {
            match rx_evt.recv() {
                Err(_) | Ok(TermEvent::Quit) => {
                    break;
                }
                _ => {
                    let now = OffsetDateTime::now_local();
                    let time = update_tv(&now, &dateformat, &printformat);
                    match write!(
                        stdout,
                        "{}{}{}",
                        termion::clear::All,
                        termion::cursor::Goto(1, 1),
                        time
                    ) {
                        Ok(_) => {}
                        Err(_) => {
                            break;
                        }
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
    }

    std::process::exit(0);
}
