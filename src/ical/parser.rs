use lazy_static::lazy_static;
use regex::Regex;
use std::str::Lines;

use super::generics::{CalendarComponent, CalendarObject, CalendarProperty};

lazy_static! {
    /// Joins ICal multi-line strings
    /* RFC5545 3.1: a long line can be split between any two characters by
       inserting a CRLF immediately followed by a single linear white-space
       character (i.e., SPACE or HTAB) */
    static ref RE: Regex = Regex::new(r"\r?\n[\t ]").unwrap();
}

pub fn parse(ical_str: &str) -> Result<CalendarComponent, String> {
    let processed = RE.replace_all(ical_str, "");
    let mut lines = processed.lines();
    let first_line = lines
        .next()
        .ok_or("Error: ICal string is empty!".to_string())?;
    let (name, value) = parse_name_value(first_line)?;
    if name != "BEGIN" && value != "VCALENDAR" {
        return Err("Error ICal string does not start with VCALENDAR component!".to_string());
    }
    let vcal = parse_component(&mut lines, value);
    vcal
}

fn parse_component(lines: &mut Lines, name: &str) -> Result<CalendarComponent, String> {
    let mut children: Vec<CalendarObject> = vec![];
    while let Some(line) = lines.next() {
        let (k, v) = parse_name_value(line)?;
        match k {
            "BEGIN" => {
                children.push(CalendarObject::Component(parse_component(lines, v)?));
            }
            "END" => {
                if v != name {
                    return Err(format!("Unexpected END component! Expected {name} saw {v}."));
                }
                break;
            }
            _ => {
                children.push(CalendarObject::Property(CalendarProperty {
                    name: k.to_string(),
                    value: v.to_string(),
                }));
            }
        }
    }
    Ok(CalendarComponent {
        name: name.to_string(),
        children,
    })
}

fn parse_name_value(line: &str) -> Result<(&str, &str), String> {
    let parts: Vec<&str> = line.splitn(2, ':').collect();
    if parts.len() < 2 {
        return Err(format!("Error: no colon found on line! Line: {}", line));
    }
    Ok((parts[0], parts[1]))
}

