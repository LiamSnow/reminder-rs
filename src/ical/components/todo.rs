#![allow(dead_code)]

use crate::ical::tree::onceler;
use crate::ical::tree::types::*;
use crate::ical::tree::lorax;

///represents the entire VTODO REPORT
pub struct CalendarTodo {
    pub etag: String,
    pub url: String,
    ///parsed ICAL file without VTODO
    pub vcal: TreeComponent,
    pub vtodo: VTodo
}

/* RFC2445 4.6.2 A "VTODO" calendar component is a grouping
 of component properties and possibly "VALARM" calendar
 components that represent an action-item or assignment. */
pub struct VTodo {
    //single mentions
    pub class: String,
    pub completed: String,
    pub created: String,
    pub description: String,
    pub dtstamp: String,
    pub dtstart: String,
    pub geo: String,
    pub last_modified: String,
    pub location: String,
    pub organizer: String,
    pub percent_complete: String,
    pub priority: String,
    pub recurid: String,
    pub sequence: String,
    pub status: String,
    pub summary: String,
    pub uid: String,

    //mutally exclusive mentions
    pub due: String,
    pub duration: String,

    //mentioned multiple times
    /* attach / attendee / categories / comment / contact /
    exdate / exrule / rstatus / related / resources /
    rdate / rrule / x-prop */

    ///unknown objects inside VTODO
    pub unknown: Vec<TreeObject>,
}

macro_rules! to_property_name {
    ($field:expr) => {
        stringify!($field).to_uppercase().replace("_", "-")
    };
}

macro_rules! parse_vtodo {
    ($vtodo_comp:expr, $($field:ident),+) => {
        VTodo {
            $(
                $field: $vtodo_comp.pop_property_value_or_empty(&to_property_name!($field)),
            )+
            unknown: $vtodo_comp.children
        }
    };
}

macro_rules! unparse_vtodo {
    ($vtodo:expr, $($field:ident),+) => {
        vec![
            $(
                TreeObject::Property(TreeProperty {
                    name: to_property_name!($field),
                    value: $vtodo.$field.clone()
                }),
            )+
        ]
    };
}

impl CalendarTodo {
    pub fn parse(ical_str: String, etag: String, url: String) -> Option<Self> {
        let mut vcal = lorax::parse(&ical_str).ok()?;
        let mut vtodo_comp = vcal.pop_component("VTODO")?;
        let vtodo = parse_vtodo!(vtodo_comp,
            class, completed, created, description,
            dtstamp, dtstart, geo, last_modified,
            location, organizer, percent_complete,
            priority, recurid, seq, status, summary,
            uid, due, duration
        );
        Some(CalendarTodo { etag, url, vcal, vtodo })
    }

    pub fn serialize(&self) -> String {
        let mut vcal = self.vcal.clone();
        let mut vtodo_comp = unparse_vtodo!(self.vtodo,
            class, completed, created, description,
            dtstamp, dtstart, geo, last_modified,
            location, organizer, percent_complete,
            priority, recurid, seq, status, summary,
            uid, due, duration
        );
        vtodo_comp.extend(self.vtodo.unknown.clone());
        vcal.children.append(&mut vtodo_comp);
        onceler::serialize(&vcal)
    }
}
