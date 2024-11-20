use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt::Display, str::Lines};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Element {
    Block(Block),
    Parameter(Parameter),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    pub name: String,
    pub children: Vec<Element>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub value: String,
}

lazy_static! {
    /// ical multi-line strings
    static ref RE: Regex = Regex::new(r"\n ").unwrap();
}

pub fn parse(ical_str: &str) -> Result<Block, String> {
    let processed = RE.replace_all(ical_str, "");
    let mut lines = processed.lines();
    let first_line = lines
        .next()
        .ok_or("Error: ICal string is empty!".to_string())?;
    let (name, value) = parse_name_value(first_line)?;
    if name != "BEGIN" && value != "VCALENDAR" {
        return Err("Error ICal string does not start with VCALENDAR block!".to_string());
    }
    parse_block(&mut lines, value)
}

fn parse_block(lines: &mut Lines, name: &str) -> Result<Block, String> {
    let mut children: Vec<Element> = vec![];
    while let Some(line) = lines.next() {
        let (k, v) = parse_name_value(line)?;
        match k {
            "BEGIN" => {
                children.push(Element::Block(parse_block(lines, v)?));
            }
            "END" => {
                if v != name {
                    return Err("Invalid closing block!".to_string());
                }
                break;
            }
            _ => {
                children.push(Element::Parameter(Parameter {
                    name: k.to_string(),
                    value: v.to_string(),
                }));
            }
        }
    }
    Ok(Block {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let str = r#"BEGIN:VCALENDAR
VERSION:2.0
CALSCALE:GREGORIAN
PRODID:-//Apple Inc.//iOS 18.0.1//EN
BEGIN:VTODO
COMPLETED:20241016T020342Z
CREATED:20241014T211812Z
DTSTAMP:20241027T214435Z
LAST-MODIFIED:20241016T020342Z
PERCENT-COMPLETE:100
STATUS:COMPLETED
DESCRIPTION:
SUMMARY:Chop Saw
UID:F87D9736-8ADE-47E4-AC46-638B5C86E7D0
X-APPLE-SORT-ORDER:740793996
END:VTODO
END:VCALENDAR"#;
        let res = Element::Block(parse(str).expect("Failed to parse ical"));
        let f = format!("{res}");
        assert_eq!(
            f,
            r#"VCALENDAR [
  VERSION = "2.0",
  CALSCALE = "GREGORIAN",
  PRODID = "-//Apple Inc.//iOS 18.0.1//EN",
  VTODO [
    COMPLETED = "20241016T020342Z",
    CREATED = "20241014T211812Z",
    DTSTAMP = "20241027T214435Z",
    LAST-MODIFIED = "20241016T020342Z",
    PERCENT-COMPLETE = "100",
    STATUS = "COMPLETED",
    DESCRIPTION = "",
    SUMMARY = "Chop Saw",
    UID = "F87D9736-8ADE-47E4-AC46-638B5C86E7D0",
    X-APPLE-SORT-ORDER = "740793996",
  ]
]"#
        );
    }
}

impl Block {
    pub fn find_block(&self, name: &str) -> Option<&Block> {
        self.children.iter().find_map(|element| {
            if let Element::Block(block) = element {
                if block.name == name {
                    return Some(block);
                }
            }
            None
        })
    }

    pub fn find_param(&self, name: &str) -> Option<&Parameter> {
        self.children.iter().find_map(|element| {
            if let Element::Parameter(param) = element {
                if param.name == name {
                    return Some(param);
                }
            }
            None
        })
    }

    pub fn get_param_value(&self, name: &str) -> Option<String> {
        let p = self.find_param(name)?;
        Some(p.value.clone())
    }

    pub fn get_param_value_date(&self, name: &str) -> Option<String> {
        self.get_param_value(name)
        // DateTime::parse_from_rfc3339(datetime_str)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_level(f, 0)
    }
}

impl Element {
    fn fmt_level(&self, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
        let t = "  ".repeat(level);

        match self {
            Element::Block(b) => {
                writeln!(f, "{t}{} [", b.name)?;
                for child in &b.children {
                    child.fmt_level(f, level + 1)?;
                    writeln!(f)?;
                }
                write!(f, "{t}]")
            }
            Element::Parameter(p) => write!(f, "{t}{p}"),
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = \"{}\",", self.name, self.value)
    }
}
