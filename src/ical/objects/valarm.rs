
use crate::ical::values::{base::*, integer::*, string::*, duration::*};

use super::{generics::*, macros::*};

/* RFC5545 3.6.6 */
make_ical_comp_struct! {
    VAlarm {
        action Opt String,
        description Opt String,
        trigger Opt Duration,
        summary Opt String,
        duration Opt Duration,
        repeat Opt Integer,
        attach Opt String,
        attendee Mul String,

        ///Includes 3.8.8.1 IANA Properties and 3.8.8.2 Non-Standard/X-Props
        unknown Vec ICalObject,
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
