use std::error::Error;

use crate::ical::objects::generics::ICalParameterMap;
use super::base::*;

pub type ICalString = String;

impl ICalValueType for ICalString {
    fn parse(value: &str, _: &ICalParameterMap) -> Result<Self, Box<dyn Error>> {
        Ok(value.to_string())
    }

    fn serialize(&self) -> String {
        self.to_string()
    }
}
