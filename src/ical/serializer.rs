use unicode_segmentation::UnicodeSegmentation;

use super::generics::{CalendarComponent, CalendarObject, CalendarProperty};

const CRLF: &str = "\r\n";

pub fn serialize(vcal: &CalendarComponent) -> String {
    let mut ics = String::new();
    serialize_component(&mut ics, vcal);
    ics
}

pub fn serialize_component(ics: &mut String, comp: &CalendarComponent) {
    if ics.len() != 0 {
        ics.push_str(CRLF);
    }
    ics.push_str("BEGIN:");
    ics.push_str(&comp.name);
    for child in &comp.children {
        match child {
            CalendarObject::Component(child) => serialize_component(ics, child),
            CalendarObject::Property(child) => serialize_property(ics, child),
        }
    }
    ics.push_str(CRLF);
    ics.push_str("END:");
    ics.push_str(&comp.name);
}

/* RFC5545 3.1: Lines of text SHOULD NOT be longer than 75 octets,
excluding the line break. Long content lines SHOULD be split into a
multiple line representations using a line "folding" technique. That
is, a long line can be split between any two characters by inserting
a CRLF immediately followed by a single linear white-space character */
pub fn serialize_property(ics: &mut String, prop: &CalendarProperty) {
    let line = prop.name.clone() + ":" + &prop.value;
    let graphemes = line.graphemes(true); //properly handle unicode
    let end = graphemes.clone().count() - 1;
    let (mut start, mut cur_size) = (0, 0);
    for (i, g) in graphemes.enumerate() {
        let num_bytes = g.len();
        let at_start = start == 0;
        let at_end = i == end;
        let max_bytes = if at_start { 75 } else { 74 }; //account for space
        if at_end || cur_size + num_bytes > max_bytes {
            ics.push_str(CRLF);
            if !at_start {
                ics.push_str(" ");
            }
            ics.push_str(if at_end { &line[start..] } else { &line[start..i] });
            (start, cur_size) = (i, 0);
        }
        cur_size += num_bytes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_property_line_fold() {
        let name = "EXAMPLE".to_string();
        let value = "This is a really long line. ".repeat(20);
        let prop = CalendarProperty { name, value };
        let mut ics = String::new();
        serialize_property(&mut ics, &prop);
        for line in ics.lines() {
            let len = line.len();
            if len > 75 {
                panic!("Line is too long ({} > 75). Line: \"{}\"", len, line);
            }
        }
    }
}



