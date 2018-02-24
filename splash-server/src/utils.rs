//! A few utility functions

use chrono::prelude::*;

/// Parse a string of a certain format into a timestamp
///
/// The string needs to have the following formatting:
///
/// <year>-<month>-<day>-<hour>-<minute>
pub fn parse_timestamp(time: String) -> Option<DateTime<Utc>> {
    let split = time.split("-").collect::<Vec<&str>>();
    if split.len() != 5 {
        return None;
    }

    let year = split[0].to_string().parse::<i32>().unwrap();
    let month = split[1].to_string().parse::<u32>().unwrap();
    let day = split[2].to_string().parse::<u32>().unwrap();
    let hour = split[3].to_string().parse::<u32>().unwrap();
    let minute = split[4].to_string().parse::<u32>().unwrap();

    return Some(Utc.ymd(year, month, day).and_hms(hour, minute, 0));
}

/// Transcode a Rust internal timestamp into a string of the form
/// 
/// <year>-<month>-<day>-<hour>-<minute>
pub fn generate_timestamp(time: DateTime<Utc>) -> String {
    return time.format("%Y-%m-%d-%H-%M").to_string();
}
