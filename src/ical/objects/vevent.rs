use crate::ical::values::{base::*, integer::*, string::*, duration::*, datetime::*};

use super::{generics::*, macros::*};

/* RFC5545 3.6.1 */
make_ical_comp_struct! {
    VEvent {
        uid Opt String,
        dtstamp Opt DateTime,

        dtstart Opt DateTime,

        class Opt String,
        created Opt DateTime,
        description Opt String,
        geo Opt String,
        last_modified Opt DateTime,
        location Opt String,
        organizer Opt String,
        priority Opt Integer,
        sequence Opt Integer,
        status Opt String,
        transp Opt String,
        summary Opt String,
        url Opt String,
        recurrence_id Opt String,

        rrule Mul String,

        dtend Opt DateTime,
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

        ///Includes 3.8.8.1 IANA Properties and 3.8.8.2 Non-Standard/X-Props
        unknown Vec ICalObject,
    }
}

impl VEvent {
    pub const NAME: &str = "VEVENT";
}

impl Validadable for VEvent {
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
