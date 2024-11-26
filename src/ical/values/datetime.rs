use std::{borrow::BorrowMut, error::Error};

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use chrono_tz::{Africa::Johannesburg, Tz};

use crate::ical::objects::generics::ICalParameterMap;

use super::base::*;

/// RFC 5545 3.3.5
pub enum ICalDateTime {
    ///FORM #1: DATE WITH LOCAL TIME
    Local(NaiveDateTime),
    ///FORM #2: DATE WITH UTC TIME
    Utc(DateTime<Utc>),
    ///FORM #3: DATE WITH LOCAL TIME AND TIME ZONE REFERENCE
    Zoned(DateTime<Tz>),
}

impl ICalValueType for ICalDateTime {
    fn parse(value: &str, params: &ICalParameterMap) -> Result<Self, Box<dyn Error>> {
        if params.contains_key("TZID") {
            let tzid = params.get("TZID").unwrap();
            //TODO FIXME
            // let taz: Tz = Tz::from(tzid);
            let local_dt = NaiveDateTime::parse_from_str(value, "%Y%m%dT%H%M%S")?;
            // let dt = Johannesburg.from_local_date(&local_dt);
            // let dt = taz.from_local_datetime(&local_dt).unwrap();
            // Ok(Self::Zoned(dt))
            todo!()
        }
        else if value.ends_with('Z') {
            let dt = DateTime::parse_from_str(value, "%Y%m%dT%H%M%SZ")?;
            Ok(Self::Utc(dt.with_timezone(&Utc)))
        }
        else {
            let dt = NaiveDateTime::parse_from_str(value, "%Y%m%dT%H%M%S")?;
            Ok(Self::Local(dt))
        }
    }

    fn serialize(&self) -> String {
        todo!()
    }
}
