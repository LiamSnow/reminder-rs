use std::error::Error;

use chrono::NaiveDate;

use crate::ical::objects::generics::ICalParameterMap;

use super::base::*;

/// RFC 5545 3.3.4: 19970714 -> July 14, 1997
pub type ICalDate = NaiveDate;

const FORMAT: &str = "%Y%m%d";

impl ICalValueType for ICalDate {
    fn parse(value: &str, _: &ICalParameterMap) -> Result<Self, Box<dyn Error>> {
        Ok(NaiveDate::parse_from_str(value, FORMAT)?)
    }

    fn serialize(&self) -> String {
        self.format(FORMAT).to_string()
    }
}

#[cfg(test)]
mod tests {
    use chrono::Datelike;
    use std::collections::HashMap;

    use crate::ical::values::base::*;
    use crate::ical::values::date::*;

    #[test]
    fn test_date() {
        let value = "20140517";
        let date = ICalDate::parse(value, &HashMap::new()).expect("Failed to parse!");
        assert_eq!(date.year(), 2014);
        assert_eq!(date.month(), 5);
        assert_eq!(date.day(), 17);
        let s = ICalValueType::serialize(&date);
        assert_eq!(s, value);
    }
}
