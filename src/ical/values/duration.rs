use std::{error::Error, str::Chars};

use chrono::TimeDelta;

use crate::ical::objects::generics::ICalParameterMap;

use super::base::*;

/// RFC 5545 3.3.6 Duration
/// Syntax: ["+" / "-"] "P" (date / time / week)
///  date = day "D" time
///  time = "T" [hours "H"] [minutes "M"] [seconds "S"]
///  week = weeks "W"
/// Examples:
///  "P15DT5H0M20S" = 15 days, 5 hours, 20 seconds
///  "P7W" = 7 weeks
///  "-P1D" = Negative 1 day
pub type ICalDuration = TimeDelta;

impl ICalValueType for ICalDuration {
    ///The RFC is strict on either being date (day + time), time, or week
    ///but this system is more relaxed
    ///This also does not require time to include a T
    fn parse(value: &str, _: &ICalParameterMap) -> Result<Self, Box<dyn Error>> {
        let mut chars = value.chars();
        let inverted = find_sign(&mut chars)?;
        let duration = parse_duration(&mut chars)?;
        Ok(if inverted { -duration } else { duration })
    }

    fn serialize(&self) -> String {
        todo!()
    }
}

fn parse_duration(chars: &mut Chars<'_>) -> Result<TimeDelta, Box<dyn Error>> {
    let (mut weeks, mut days, mut hours, mut minutes, mut seconds) = (0, 0, 0, 0, 0);
    let mut num_buffer = String::new();

    for char in chars {
        if char.is_numeric() {
            num_buffer.push(char);
            continue;
        }

        let num: i64 = num_buffer.parse()?;
        num_buffer.clear();

        match char {
            'T' => continue,
            'W' => weeks = num,
            'D' => days = num,
            'H' => hours = num,
            'M' => minutes = num,
            'S' => seconds = num,
            _ => return Err(format!("Unexpected character {} in duration string", char).into()),
        }
    }

    Ok(TimeDelta::weeks(weeks)
        + TimeDelta::days(days)
        + TimeDelta::hours(hours)
        + TimeDelta::minutes(minutes)
        + TimeDelta::seconds(seconds))
}

fn find_sign(chars: &mut Chars<'_>) -> Result<bool, String> {
    let sign;
    let first_char = chars.next().ok_or("Empty Duration String".to_string())?;
    match first_char {
        '-' => sign = true,
        '+' => sign = false,
        'P' => return Ok(false),
        _ => {
            return Err(format!(
                "Duration string should start with P but found {}",
                first_char
            ))
        }
    }

    let second_char = chars
        .next()
        .ok_or("Duration String too Short".to_string())?;
    if second_char != 'P' {
        return Err(format!(
            "Duration string should have P but found {}",
            second_char
        ));
    }
    Ok(sign)
}
