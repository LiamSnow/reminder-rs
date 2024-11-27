use crate::ical::values::{base::*, integer::*, string::*, duration::*};
use super::{generics::*, macros::*};
use crate::ical::serializer::{self, ICSAble, ICSAbleWithName};
use std::vec::IntoIter;
use std::error::Error;
use crate::ical::parser::{Parsable, ContentLine};


make_ical_comp_struct! {
    /// RFC5545 3.6.6
    VAlarm {
        action Opt String,
        description Opt String,
        trigger Opt Duration,
        summary Opt String,
        duration Opt Duration,
        repeat Opt Integer,
        attach Opt String,
        attendee Mul String,
    }
}

pub enum VAlarmType {
    Audio,
    Display,
    Email
}

impl VAlarm {
    pub const NAME: &str = "VALARM";

    pub fn get_type() -> VAlarmType {
        todo!();
    }
}

impl Validadable for VAlarm {
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

impl ICSAbleWithName for Vec<VAlarm> {
    fn to_ics_with_name(&self, _: &str, ics: &mut String) {
        for child in self {
            child.to_ics(ics);
        }
    }
}

