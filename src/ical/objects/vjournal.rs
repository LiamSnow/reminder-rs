use crate::ical::values::{base::*, integer::*, string::*, datetime::*};

use super::{generics::*, macros::*};

use crate::ical::serializer::{self, ICSAble, ICSAbleWithName};
use std::vec::IntoIter;
use std::error::Error;
use crate::ical::parser::{Parsable, ContentLine};


make_ical_comp_struct! {
    /// RFC5545 3.6.3
    VJournal {
        uid Opt String,
        dtstamp Opt DateTime,

        class Opt String,
        created Opt DateTime,
        dtstart Opt DateTime,
        last_modified Opt DateTime,
        organizer Opt String,
        recurrence_id Opt String,
        sequence Opt Integer,
        status Opt String,
        summary Opt String,
        url Opt String,

        rrule Mul String,

        attach Mul String,
        attendee Mul String,
        categories Mul String,
        comment Mul String,
        contact Mul String,
        description Mul String,
        exdate Mul DateTime,
        related_to Mul String,
        rdate Mul DateTime,
        request_status Mul String,
    }
}

impl VJournal {
    pub const NAME: &str = "VJournal";
}

impl Validadable for VJournal {
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
