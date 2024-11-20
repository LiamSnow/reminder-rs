use super::parser;


/*  RFC2445 4.6.2
A "VTODO" calendar component is a grouping of component
properties and possibly "VALARM" calendar components that represent
an action-item or assignment.
*/
#[derive(Debug)]
pub struct VTodo {
    //injected (not in ICal file)
    pub etag: String,

    //single mentions
    pub class: Option<String>,
    pub completed: Option<String>,
    pub created: Option<String>,
    pub description: Option<String>,
    pub dtstamp: Option<String>,
    pub dtstart: Option<String>,
    pub geo: Option<String>,
    pub last_mod: Option<String>,
    pub location: Option<String>,
    pub organizer: Option<String>,
    pub percent: Option<String>,
    pub priority: Option<String>,
    pub recurid: Option<String>,
    pub seq: Option<String>,
    pub status: Option<String>,
    pub summary: Option<String>,
    pub uid: Option<String>,
    pub url: Option<String>,

    //one or other mentions
    pub due: Option<String>,
    pub duration: Option<String>,

    //mentioned multiple times
    /*
    attach / attendee / categories / comment / contact /
    exdate / exrule / rstatus / related / resources /
    rdate / rrule / x-prop
    */
}

impl VTodo {
    pub fn deserialize(ical_str: String, etag: String) -> Option<Self> {
        let vcal = parser::parse(&ical_str).unwrap();
        if vcal.find_param("VERSION")?.value != "2.0" {
            return None;
        }
        let vtodo = vcal.find_block("VTODO")?;

        Some(VTodo {
            etag,
            uid: vtodo.get_param_value("UID"),
            completed: vtodo.get_param_value_date("COMPLETED"),
            created: vtodo.get_param_value_date("CREATED"),
            dtstamp: vtodo.get_param_value_date("DTSTAMP"),
            last_mod: vtodo.get_param_value_date("LAST-MODIFIED"),
            percent: vtodo.get_param_value("PERCENT-COMPLETE").and_then(|n| n.into()),
            priority: vtodo.get_param_value("PRIORITY"),
            status: vtodo.get_param_value("STATUS"), //ENUM?
            description: vtodo.get_param_value("DESCRIPTION"),
            summary: vtodo.get_param_value("SUMMARY"),
            class: None,
            dtstart: None,
            geo: None,
            location: None,
            organizer: None,
            recurid: None,
            seq: None,
            url: None,
            due: None,
            duration: None,

            // sequence: vtodo.get_param_value("SEQUENCE"),
        })
    }
}

/*
BEGIN:VCALENDAR
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
END:VCALENDAR
*/
