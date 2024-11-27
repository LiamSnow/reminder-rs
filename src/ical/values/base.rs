use crate::ical::objects::generics::ICalParameterMap;
use std::error::Error;

//Base Value
#[derive(Clone)]
pub struct ICalValue<T: ICalValueType> {
    pub value: T,
    pub params: ICalParameterMap,
}

pub trait ICalValueType: Sized {
    fn parse(value: &str, params: &ICalParameterMap) -> Result<Self, Box<dyn Error>>;
    fn serialize(&self) -> String;
}

impl<T: ICalValueType> ICalValue<T> {
    fn new(value: &str, params: ICalParameterMap) -> Result<Self, Box<dyn Error>> {
        Ok(ICalValue {
            value: T::parse(value, &params)?,
            params,
        })
    }

    fn set(&mut self, value: &str, params: ICalParameterMap) -> Result<(), Box<dyn Error>> {
        self.value = T::parse(value, &params)?;
        self.params = params;
        Ok(())
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}

//Optional
pub struct ICalOptional<T: ICalValueType>(pub Option<ICalValue<T>>);

impl<T: ICalValueType> ICalOptional<T> {
    pub fn set(&mut self, value: &str, params: ICalParameterMap) -> Result<(), Box<dyn Error>> {
        self.0 = Some(ICalValue::new(value, params)?);
        Ok(())
    }

    pub fn get(&self) -> Option<&ICalValue<T>> {
        match &self.0 {
            Some(value) => Some(value),
            None => None,
        }
    }

    pub fn get_value(&self) -> Option<&T> {
        self.get().and_then(|v| Some(v.get_value()))
    }
}

impl<T: ICalValueType> Default for ICalOptional<T> {
    fn default() -> Self {
        Self(None)
    }
}

//Multiple
pub struct ICalMultiple<T: ICalValueType>(pub Vec<ICalValue<T>>);

impl<T: ICalValueType> ICalMultiple<T> {
    pub fn add(&mut self, value: &str, params: ICalParameterMap) -> Result<(), Box<dyn Error>> {
        self.0.push(ICalValue::new(value, params)?);
        Ok(())
    }
}

impl<T: ICalValueType> Default for ICalMultiple<T> {
    fn default() -> Self {
        Self(vec![])
    }
}
