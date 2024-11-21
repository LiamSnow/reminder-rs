use minidom::Element;

use crate::ical::todo::CalendarTodo;

use super::client::Calendar;

pub const NS_D: &str = "DAV:";
pub const NS_C: &str = "urn:ietf:params:xml:ns:caldav";
pub const NS_CS: &str = "http://calendarserver.org/ns/";
pub const NS_I: &str = "http://apple.com/ns/ical/";

pub fn format_ns_attrs() -> String {
    format!("xmlns:d=\"{NS_D}\" xmlns:c=\"{NS_C}\" xmlns:cs=\"{NS_CS}\" xmlns:i=\"{NS_I}\"")
}

pub fn follow_tree(el: &Element, tree: &str, namespace: &str) -> Option<Element> {
    let parts = tree.split(".");
    let mut cur_el = el;
    for part in parts {
        cur_el = cur_el.get_child(part, namespace)?;
    }
    Some(cur_el.clone())
}

pub fn parse_cal_propfind(el: &Element) -> Option<Calendar> {
    let url = follow_tree(el, "href", NS_D)?.text();
    let prop = follow_tree(el, "propstat.prop", NS_D)?;
    let name = prop.get_child("displayname", NS_D)?.text();
    let ctag = prop.get_child("getctag", NS_CS)?.text();
    let color = prop
        .get_child("calendar-color", NS_I)
        .and_then(|c| Some(c.text()));
    let description = prop
        .get_child("calendar-description", NS_C)
        .and_then(|c| Some(c.text()));

    let supports_todo = prop
        .get_child("supported-calendar-component-set", NS_C)?
        .nodes()
        .filter_map(|node| node.as_element()?.attr("name"))
        .any(|name| name.contains("VTODO"));

    let is_calendar = prop
        .get_child("resourcetype", NS_D)?
        .nodes()
        .filter_map(|node| node.as_element())
        .any(|elem| elem.name() == "calendar");

    if !supports_todo || !is_calendar {
        return None;
    }

    Some(Calendar {
        url,
        name,
        ctag,
        color,
        description,
    })
}

pub fn parse_todo_report(el: &Element) -> Option<CalendarTodo> {
    let url = follow_tree(el, "href", NS_D)?.text();
    let prop = follow_tree(el, "propstat.prop", NS_D)?;
    let etag = prop.get_child("getetag", NS_D)?.text();
    let data = prop.get_child("calendar-data", NS_C)?.text();
    CalendarTodo::parse(data, etag, url)
}
