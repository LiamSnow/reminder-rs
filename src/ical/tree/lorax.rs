use lazy_static::lazy_static;
use regex::Regex;
use std::str::Lines;
use std::collections::HashMap;

use super::types::*;

#[derive(Debug)]
struct ContentLine {
    name: String,
    params: HashMap<String, String>,
    value: String,
}

lazy_static! {
    /// Joins ICal multi-line strings
    /* RFC5545 3.1: a long line can be split between any two characters by
       inserting a CRLF immediately followed by a single linear white-space
       character (i.e., SPACE or HTAB) */
    static ref RE: Regex = Regex::new(r"\r?\n[\t ]").unwrap();
}

pub fn parse(ical_str: &str) -> Result<TreeComponent, String> {
    let processed = RE.replace_all(ical_str, "");
    let mut lines = processed.lines();
    let first_line = lines
        .next()
        .ok_or("Error: ICal string is empty!".to_string())?;
    let first_cl = parse_content_line(first_line)?;
    if first_cl.name != "BEGIN" && first_cl.value != "VCALENDAR" {
        return Err("Error ICal string does not start with VCALENDAR component!".to_string());
    }
    let vcal = parse_component(&mut lines, first_cl.value, first_cl.params);
    vcal
}

fn parse_component(lines: &mut Lines, name: &str, params: HashMap<&str, &str>) -> Result<TreeComponent, String> {
    let mut children: Vec<TreeObject> = vec![];
    while let Some(line) = lines.next() {
        let cl = parse_content_line(line)?;
        match name {
            "BEGIN" => {
                children.push(TreeObject::Component(parse_component(lines, cl.value, cl.params)?));
            }
            "END" => {
                if cl.value != name {
                    return Err(format!("Unexpected END component! Expected {} saw {}.", name, cl.value));
                }
                break;
            }
            _ => {
                children.push(TreeObject::Property(TreeProperty {
                    name: name.to_string(),
                    value: cl.value.to_string(),
                    params: cl.params
                }));
            }
        }
    }
    Ok(TreeComponent {
        name: name.to_string(),
        children,
        params
    })
}

//RFC 2445 4.1: "name *(";" param ) ":" value CRLF"
fn parse_content_line(line: &str) -> Result<ContentLine, String> {
    let parts: Vec<&str> = line.splitn(2, ':').collect();
    if parts.len() < 2 {
        return Err(format!("Error: no colon found on content line! Line: {}", line));
    }

    let mut left_hand_parts: Vec<&str> = parts[0].split(';').collect();
    let name = left_hand_parts.remove(0);
    if name.is_empty() {
        return Err(format!("Error: no name found on content line! Line: {}", line));
    }

    let params = left_hand_parts
        .into_iter()
        .map(|param| parse_parameter(param)?)
        .collect();

    let value = parts[1];

    Ok(ContentLine { name, params, value })
}

fn parse_parameter(param: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = param.splitn(2, '=').collect();
    if parts.len() < 2 {
        return Err(format!("Error: no equals found in parameter! Parameter: {}", param));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

