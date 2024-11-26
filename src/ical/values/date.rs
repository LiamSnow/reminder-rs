use std::error::Error;

use chrono::NaiveDate;

use crate::ical::objects::generics::ICalParameterMap;

use super::base::*;

/// RFC 5545 3.3.4: 19970714 -> July 14, 1997
pub type ICalDate = NaiveDate;

impl ICalValueType for ICalDate {
    fn parse(value: &str, _: &ICalParameterMap) -> Result<Self, Box<dyn Error>> {
        Ok(NaiveDate::parse_from_str(value, "%Y%m%d")?)
    }

    fn serialize(&self) -> String {
        self.format("%Y%m%d").to_string()
    }
}
