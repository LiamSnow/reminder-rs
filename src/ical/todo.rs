#![allow(dead_code)]

use super::parser;
use super::generics::*;
use super::serializer;

/*  RFC2445 4.6.2
A "VTODO" calendar component is a grouping of component
properties and possibly "VALARM" calendar components that represent
an action-item or assignment.
*/

///represents the entire VTODO REPORT
pub struct CalendarTodo {
    pub etag: String,
    pub url: String,
    ///parsed ICAL file
    pub vcal: CalendarComponent,
    last_vtodo_index: usize
}

impl CalendarTodo {
    pub fn parse(ical_str: String, etag: String, url: String) -> Option<Self> {
        let vcal = parser::parse(&ical_str).ok()?;
        let last_vtodo_index = vcal.find_component_index("VTODO")?;
        Some(CalendarTodo { vcal, etag, url, last_vtodo_index })
    }

    pub fn serialize(&self) -> String {
        serializer::serialize(&self.vcal)
    }

    pub fn get_vtodo(&self) -> Option<&CalendarComponent> {
        self.vcal.find_component_with_guess("VTODO", self.last_vtodo_index)
    }

    //single mentions
    pub fn get_class(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("CLASS")
    }
    pub fn get_completed(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("COMPLETED")
    }
    pub fn get_created(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("CREATED")
    }
    pub fn get_description(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("DESCRIPTION")
    }
    pub fn get_dtstamp(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("DTSTAMP")
    }
    pub fn get_dtstart(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("DTSTART")
    }
    pub fn get_geo(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("GEO")
    }
    pub fn get_last_modified(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("LAST-MODIFIED")
    }
    pub fn get_location(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("LOCATION")
    }
    pub fn get_organizer(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("ORGANIZER")
    }
    pub fn get_percent(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("PERCENT-COMPLETE")
    }
    pub fn get_priority(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("PRIORITY")
    }
    pub fn get_recurid(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("RECURID")
    }
    pub fn get_seq(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("SEQ")
    }
    pub fn get_status(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("STATUS")
    }
    pub fn get_summary(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("SUMMARY")
    }
    pub fn get_uid(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("UID")
    }
    pub fn get_url(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("URL")
    }

    //one or other mentions
    pub fn get_due(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("DUE")
    }
    pub fn get_duration(&self) -> Option<&str> {
        self.get_vtodo()?.get_property_value("DURATION")
    }

    //mentioned multiple times
    /*
    attach / attendee / categories / comment / contact /
    exdate / exrule / rstatus / related / resources /
    rdate / rrule / x-prop
    */
}
