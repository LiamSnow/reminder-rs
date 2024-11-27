#![allow(dead_code)]

use crate::ical::values::{base::*, datetime::*, integer::*, string::*, duration::*};
use super::{generics::*, valarm::VAlarm, macros::*};
use crate::ical::serializer::{self, ICSAble, ICSAbleWithName};

use std::vec::IntoIter;
use std::error::Error;
use crate::ical::parser::{Parsable, ContentLine};

make_ical_comp_struct! {
    /// RFC5545 3.6.2 A "VTODO" calendar component is a grouping
    /// of component properties and possibly "VALARM" calendar
    /// components that represent an action-item or assignment.
    VTodo {
        uid Opt String,
        dtstamp Opt DateTime,

        class Opt String,
        completed Opt DateTime,
        created Opt DateTime,
        description Opt String,
        dtstart Opt DateTime,
        geo Opt String,
        last_modified Opt DateTime,
        location Opt String,
        organizer Opt String,
        percent_complete Opt Integer,
        priority Opt Integer,
        recurrence_id Opt String,
        sequence Opt Integer,
        status Opt String,
        summary Opt String,
        url Opt String,
        due Opt DateTime,
        duration Opt Duration,

        attach Mul String,
        attendee Mul String,
        categories Mul String,
        comment Mul String,
        contact Mul String,
        exdate Mul DateTime,
        request_status Mul String,
        related_to Mul String,
        resources Mul String,
        rdate Mul DateTime,
        rrule Mul String,

        alarms Children VAlarm,
    }
}

impl VTodo {
    pub const NAME: &str = "VTODO";

    pub fn get_all_categories(&self) -> Vec<&str> {
        todo!()
    }
}

impl Validadable for VTodo {
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
