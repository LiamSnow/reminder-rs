#[cfg(test)]
mod tests {
    use crate::ical::{parser::parse, serializer::serialize};

    #[test]
    fn test_parse_serialize() {
        let ics = r#"BEGIN:VCALENDAR
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
END:VCALENDAR"#.replace("\n", "\r\n");
        let p = parse(&ics).expect("Failed to parse ical");
        let s = serialize(&p);

        assert_eq!(ics, s);
    }
}

