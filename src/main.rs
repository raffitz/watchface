extern crate clap;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

pub mod dateformat;
pub mod error;
pub mod printformat;

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

    println!("dateformat: {}", dateformat);
    println!("printformat: {}", printformat);
}
