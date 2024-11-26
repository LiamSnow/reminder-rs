use std::{error::Error, num::ParseIntError};

use crate::ical::objects::generics::ICalParameterMap;

use super::base::*;

pub type ICalInteger = i32;

impl ICalValueType for ICalInteger {
    fn parse(value: &str, _: &ICalParameterMap) -> Result<Self, Box<dyn Error>> {
        Ok(value.parse()?)
    }

    fn serialize(&self) -> String {
        self.to_string()
    }
}
