#![allow(dead_code)]

use std::{collections::HashMap, error::Error};

use crate::ical::values::{base::ICalValue, string::ICalString};

use super::{valarm::VAlarm, vevent::VEvent, vtodo::VTodo};

pub enum ICalObject {
    UnknownComponent(UnknownComponent),
    UnknownProperty(UnknownProperty),
    VTodo(VTodo),
    VAlarm(VAlarm),
    VEvent(VEvent),
}

pub type ICalParameterMap = HashMap<String, String>;

pub struct UnknownComponent {
    pub name: String,
    pub params: ICalParameterMap,
    pub children: Vec<ICalObject>,
}

pub struct UnknownProperty {
    pub name: String,
    pub value: ICalValue<ICalString>,
}

pub struct VCalendar {
    pub children: Vec<ICalObject>,
}

impl VCalendar {
    pub const NAME: &str = "VCALENDAR";
}

impl ICalObject {
    pub fn get_name(&self) -> &str {
        match self {
            ICalObject::UnknownComponent(comp) => &comp.name,
            ICalObject::UnknownProperty(prop) => &prop.name,
            ICalObject::VTodo(_) => VTodo::NAME,
            ICalObject::VAlarm(_) => VAlarm::NAME,
            ICalObject::VEvent(_) => VEvent::NAME,
        }
    }
}

pub trait Validadable {
    fn validate(&self) -> Result<(), Box<dyn Error>>;
}
