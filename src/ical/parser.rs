use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::vec::IntoIter;

use super::objects::generics::*;
use super::objects::valarm::VAlarm;
use super::objects::vevent::VEvent;
use super::objects::vjournal::VJournal;
use super::objects::vtodo::VTodo;
use super::values::base::ICalValue;

pub trait Parsable
where
    Self: Sized,
{
    fn parse(lines: &mut IntoIter<ContentLine>, begin_line: ContentLine) -> Result<Self, Box<dyn Error>>;
}

lazy_static! {
    /// Finds the location of all ICal folds
    ///   RFC5545 3.1: a long line can be split between any two characters by
    ///   inserting a CRLF immediately followed by a single linear white-space
    ///   character (i.e., SPACE or HTAB)
    static ref FOLDS: Regex = Regex::new(r"\r?\n[\t ]").unwrap();
}

impl VCalendar {
    pub fn parse(ical_str: &str) -> Result<Self, Box<dyn Error>> {
        let unfolded_ical_str = FOLDS.replace_all(ical_str, "");
        let mut lines = unfolded_ical_str
            .lines()
            .map(ContentLine::from_str)
            .collect::<Result<Vec<ContentLine>, String>>()?
            .into_iter();

        let begin_line = lines.next().ok_or("Error: ICal string is empty!".to_string())?;
        if begin_line.name != "BEGIN" || begin_line.value != VCalendar::NAME {
            return Err(format!("Error: ICal started with {}:{} not BEGIN:VCALENDAR!", begin_line.name, begin_line.value).into());
        }

        let mut children: Vec<ICalObject> = vec![];
        while let Some(line) = lines.next() {
            match (line.name.as_str(), line.value.as_str()) {
                ("END", VCalendar::NAME) => break,
                ("END", _) => return Err(format!("Unexpected END in VCALENDAR. Found {}.", line.value).into()),
                (_, _) => children.push(ICalObject::parse(&mut lines, line)?),
            }
        }
        Ok(Self { children })
    }
}

impl Parsable for ICalObject {
    fn parse(lines: &mut IntoIter<ContentLine>, begin_line: ContentLine) -> Result<Self, Box<dyn Error>> {
        // println!("parsing {} {}", begin_line.value, begin_line.name);
        if begin_line.name != "BEGIN" {
            return Ok(begin_line.to_unknown_prop_obj());
        }

        Ok(match begin_line.value.as_str() {
            VTodo::NAME => ICalObject::VTodo(VTodo::parse(lines, begin_line)?),
            VEvent::NAME => ICalObject::VEvent(VEvent::parse(lines, begin_line)?),
            VAlarm::NAME => ICalObject::VAlarm(VAlarm::parse(lines, begin_line)?),
            VJournal::NAME => ICalObject::VJournal(VJournal::parse(lines, begin_line)?),
            _ => ICalObject::UnknownComponent(UnknownComponent::parse(lines, begin_line)?)
        })
    }
}

impl Parsable for UnknownComponent {
    fn parse(lines: &mut IntoIter<ContentLine>, begin_line: ContentLine) -> Result<Self, Box<dyn Error>> {
        let mut children: Vec<ICalObject> = vec![];
        let name = begin_line.value.as_str();
        while let Some(line) = lines.next() {
            match (line.name.as_str(), line.value.as_str()) {
                ("END", end_name) => {
                    if end_name == name {
                        break
                    }
                    return Err(format!("Unexpected END in {}. Found {}.", begin_line.value, line.value).into())
                },
                (_, _) => children.push(ICalObject::parse(lines, line)?),
            }
        }
        Ok(Self {
            name: name.to_string(),
            children,
            params: begin_line.params
        })
    }
}

#[derive(Debug, Clone)]
pub struct ContentLine {
    pub name: String,
    pub params: HashMap<String, String>,
    pub value: String,
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

    pub fn to_unknown_prop_obj(self) -> ICalObject {
        ICalObject::UnknownProperty(UnknownProperty {
            name: self.name,
            value: ICalValue {
                value: self.value,
                params: self.params
            }
        })
    }
}
