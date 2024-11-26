use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::vec::IntoIter;

use super::objects::generics::*;
use super::objects::valarm::VAlarm;
use super::objects::vevent::VEvent;
use super::objects::vtodo::VTodo;
use super::values::base::ICalValue;

fn parse_component(lines: &mut std::vec::IntoIter<ContentLine>, begin_line: ContentLine) -> Result<ICalObject, Box<dyn Error>> {
    Ok(match begin_line.value.as_str() {
        "VTODO" => ICalObject::VTodo(parse_vtodo(lines)?),
        "VEVENT" => ICalObject::VEvent(parse_vevent(lines)?),
        "VALARM" => ICalObject::VAlarm(parse_valarm(lines)?),
        _ => ICalObject::UnknownComponent(parse_unknown_component(lines, begin_line)?)
    })
}

lazy_static! {
    /* RFC5545 3.1: a long line can be split between any two characters by
       inserting a CRLF immediately followed by a single linear white-space
       character (i.e., SPACE or HTAB) */
    /// Finds the location of all ICal folds
    static ref FOLDS: Regex = Regex::new(r"\r?\n[\t ]").unwrap();
}

pub fn parse(ical_str: &str) -> Result<VCalendar, Box<dyn Error>> {
    let unfolded_ical_str = FOLDS.replace_all(ical_str, "");
    let mut lines = unfolded_ical_str
        .lines()
        .map(ContentLine::from_str)
        .collect::<Result<Vec<ContentLine>, String>>()?
        .into_iter();

    let begin_line = lines.next().ok_or("Error: ICal string is empty!".to_string())?;
    if begin_line.value == "VCALENDAR" {
        return Err("Error: ICal string does not start with VCALENDAR component!".into());
    }

    let mut children: Vec<ICalObject> = vec![];
    while let Some(line) = lines.next() {
        match (line.name.as_str(), line.value.as_str()) {
            ("BEGIN", _) => children.push(parse_component(&mut lines, line)?),
            ("END", "VCALENDAR") => break,
            ("END", _) => return Err(format!("Unexpected END in VCALENDAR. Found {}.", line.value).into()),
            (_, _) => children.push(line.to_unknown_prop_obj()),
        }
    }
    Ok(VCalendar { children })
}

fn parse_vtodo(lines: &mut IntoIter<ContentLine>) -> Result<VTodo, Box<dyn Error>> {
    let mut vtodo = VTodo::default();
    while let Some(line) = lines.next() {
        match (line.name.as_str(), line.value.as_str()) {
            ("BEGIN", "VALARM") => vtodo.alarms.push(parse_valarm(lines)?),
            ("BEGIN", _) => vtodo.unknown.push(parse_component(lines, line)?),
            ("END", "VTODO") => break,
            ("END", _) => return Err(format!("Unexpected END in VCALENDAR. Found {}.", line.value).into()),

            ("UID", value) => vtodo.uid.set(value, line.params)?,
            ("CLASS", value) => vtodo.class.set(value, line.params)?,
            ("CLASS", value) => vtodo.class.set(value, line.params)?,
            ("COMPLETED", value) => vtodo.completed.set(value, line.params)?,
            ("CREATED", value) => vtodo.created.set(value, line.params)?,
            ("DESCRIPTION", value) => vtodo.description.set(value, line.params)?,
            ("DTSTART", value) => vtodo.dtstart.set(value, line.params)?,
            ("GEO", value) => vtodo.geo.set(value, line.params)?,
            ("LAST-MODIFIED", value) => vtodo.last_modified.set(value, line.params)?,
            ("LOCATION", value) => vtodo.location.set(value, line.params)?,
            ("ORGANIZER", value) => vtodo.organizer.set(value, line.params)?,
            ("PERCENT-COMPLETE", value) => vtodo.percent_complete.set(value, line.params)?,
            ("PRIORITY", value) => vtodo.priority.set(value, line.params)?,
            ("RECURRENCE-ID", value) => vtodo.recurrence_id.set(value, line.params)?,
            ("SEQUENCE", value) => vtodo.sequence.set(value, line.params)?,
            ("STATUS", value) => vtodo.status.set(value, line.params)?,
            ("SUMMARY", value) => vtodo.summary.set(value, line.params)?,
            ("URL", value) => vtodo.url.set(value, line.params)?,
            ("DUE", value) => vtodo.due.set(value, line.params)?,
            ("DURATION", value) => vtodo.duration.set(value, line.params)?,

            ("ATTACH", value) => vtodo.attach.add(value, line.params)?,
            ("ATTENDEE", value) => vtodo.attendee.add(value, line.params)?,
            ("CATEGORIES", value) => vtodo.categories.add(value, line.params)?,
            ("COMMENT", value) => vtodo.comment.add(value, line.params)?,
            ("CONTACT", value) => vtodo.contact.add(value, line.params)?,
            ("EXDATE", value) => vtodo.exdate.add(value, line.params)?,
            ("REQUEST-STATUS", value) => vtodo.request_status.add(value, line.params)?,
            ("RELATED-TO", value) => vtodo.related_to.add(value, line.params)?,
            ("RESOURCES", value) => vtodo.resources.add(value, line.params)?,
            ("RDATE", value) => vtodo.rdate.add(value, line.params)?,
            ("RRULE", value) => vtodo.rrule.add(value, line.params)?,

            (_, _) => vtodo.unknown.push(line.to_unknown_prop_obj())
        }
    }

    Ok(vtodo)
}

fn parse_vevent(lines: &mut IntoIter<ContentLine>) -> Result<VEvent, String> {
    todo!();
}

fn parse_valarm(lines: &mut IntoIter<ContentLine>) -> Result<VAlarm, String> {
    todo!();
}

fn parse_unknown_component(lines: &mut IntoIter<ContentLine>, begin_line: ContentLine) -> Result<UnknownComponent, Box<dyn Error>> {
    let mut children: Vec<ICalObject> = vec![];
    let name = begin_line.value.as_str();
    while let Some(line) = lines.next() {
        match (line.name.as_str(), line.value.as_str()) {
            ("BEGIN", _) => children.push(parse_component(lines, line)?),
            ("END", end_name) => {
                if end_name == name {
                    break
                }
                return Err(format!("Unexpected END in {}. Found {}.", begin_line.value, line.value).into())
            },
            (_, _) => children.push(line.to_unknown_prop_obj())
        }
    }
    Ok(UnknownComponent {
        name: name.to_string(),
        children,
        params: begin_line.params
    })
}

#[derive(Debug, Clone)]
struct ContentLine {
    name: String,
    params: HashMap<String, String>,
    value: String,
}

impl ContentLine {
    ///RFC5545 3.1: "name *(";" param ) ":" value CRLF"
    fn from_str(line: &str) -> Result<Self, String> {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() < 2 {
            return Err(format!("Error: no colon found on content line! Line: {}", line));
        }

        let value = parts[1];
        let mut left_hand_parts: Vec<&str> = parts[0].split(';').collect();

        let name = left_hand_parts.remove(0);
        if name.is_empty() {
            return Err(format!("Error: no name found on content line! Line: {}", line));
        }

        let mut params: HashMap<String, String> = HashMap::new();
        for param in left_hand_parts {
            let (name, value) = ContentLine::parse_parameter(param)?;
            params.insert(name, value);
        }

        Ok(ContentLine { name: name.to_string(), params, value: value.to_string() })
    }

    fn parse_parameter(param: &str) -> Result<(String, String), String> {
        let parts: Vec<&str> = param.splitn(2, '=').collect();
        if parts.len() < 2 {
            return Err(format!("Error: no equals found in parameter! Parameter: {}", param));
        }
        Ok((parts[0].to_string(), parts[1].to_string()))
    }

    fn to_unknown_prop_obj(self) -> ICalObject {
        ICalObject::UnknownProperty(UnknownProperty {
            name: self.name,
            value: ICalValue {
                value: self.value,
                params: self.params
            }
        })
    }
}
