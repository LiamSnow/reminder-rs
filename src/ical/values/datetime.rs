use std::error::Error;
use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::{Africa::Johannesburg, Tz};

use crate::ical::objects::generics::ICalParameterMap;

use super::base::*;

/// RFC 5545 3.3.5
pub enum ICalDateTime {
    ///FORM #1: Local Time
    Local(NaiveDateTime),
    ///FORM #2: UTC Time, FORM #3: Time zone
    Zoned(DateTime<Tz>),
}

const FORMAT: &str = "%Y%m%dT%H%M%S";

impl ICalValueType for ICalDateTime {
    fn parse(value: &str, params: &ICalParameterMap) -> Result<Self, Box<dyn Error>> {
        let is_utc = value.ends_with('Z');
        let value = if is_utc { value.trim_end_matches('Z') } else { value };

        let local = NaiveDateTime::parse_from_str(value, FORMAT)?;

        if params.contains_key("TZID") {
            let timezone_str = params.get("TZID").unwrap();
            let timezone = Tz::from_str(timezone_str)?;
            let dt = timezone.from_local_datetime(&local).single().ok_or("Failed to transfer into timezone")?;
            Ok(Self::Zoned(dt))
        } else if is_utc {
            let dt = Tz::UTC.from_local_datetime(&local).single().ok_or("Failed to tranfer into UTC")?;
            Ok(Self::Zoned(dt))
        } else {
            Ok(Self::Local(local))
        }
    }

    fn serialize(&self) -> String {
        match self {
            ICalDateTime::Local(dt) => {
                dt.format(FORMAT).to_string()
            },
            ICalDateTime::Zoned(dt) => {
                let suffix = if dt.timezone() == Tz::UTC { "Z" } else { "" };
                dt.format(FORMAT).to_string() + suffix
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};
    use std::collections::HashMap;

    use crate::ical::values::base::*;
    use crate::ical::values::datetime::*;

    #[test]
    fn test_datetime_utc() {
        let value = "20140517T123456Z";
        let icaldt = ICalDateTime::parse(value, &HashMap::new()).expect("Failed to parse!");
        let ICalDateTime::Zoned(dt) = icaldt else {
            panic!("Did not get UTC datetime");
        };
        assert_eq!(dt.year(), 2014);
        assert_eq!(dt.month(), 5);
        assert_eq!(dt.day(), 17);
        assert_eq!(dt.hour(), 12);
        assert_eq!(dt.minute(), 34);
        assert_eq!(dt.second(), 56);
        assert_eq!(dt.timezone(), Tz::UTC);
        let s = ICalValueType::serialize(&icaldt);
        assert_eq!(s, value);
    }

    #[test]
    fn test_datetime_tz() {
        let value = "19921217T123456";
        let mut params = HashMap::new();
        params.insert("TZID".to_string(), "America/New_York".to_string());
        let icaldt = ICalDateTime::parse(value, &params).expect("Failed to parse!");
        let ICalDateTime::Zoned(dt) = icaldt else {
            panic!("Did not get Zoned datetime");
        };
        assert_eq!(dt.year(), 1992);
        assert_eq!(dt.month(), 12);
        assert_eq!(dt.day(), 17);
        assert_eq!(dt.hour(), 12);
        assert_eq!(dt.minute(), 34);
        assert_eq!(dt.second(), 56);
        assert_eq!(dt.timezone(), Tz::America__New_York);
        let s = ICalValueType::serialize(&icaldt);
        assert_eq!(s, value);
    }
}
