use minidom::Element;

use crate::ical::vtodo::{self, VTodo};

use super::client::Calendar;

pub const NS_D: &str = "DAV:";
pub const NS_C: &str = "urn:ietf:params:xml:ns:caldav";
pub const NS_CS: &str = "http://calendarserver.org/ns/";

pub fn format_ns_attrs() -> String {
    format!("xmlns:d=\"{NS_D}\" xmlns:c=\"{NS_C}\" xmlns:cs=\"{NS_CS}\"")
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
    let supports_todo = prop.get_child("supported-calendar-component-set", NS_C)?.nodes()
        .filter_map(|node| node.as_element())
        .any(|elem| elem.attr("name").map_or(false, |name| name.contains("VTODO")));
    match supports_todo {
        true => Some(Calendar { url, name, ctag }),
        false => None,
    }
}

pub fn parse_todo_report(el: &Element) -> Option<VTodo> {
    let url = follow_tree(el, "href", NS_D)?.text();
    let prop = follow_tree(el, "propstat.prop", NS_D)?;
    let etag = prop.get_child("getetag", NS_D)?.text();
    let data = prop.get_child("calendar-data", NS_C)?.text();
    // println!("\n***\n{data}***\n");
    let a = VTodo::deserialize(data, etag);

    // if let Some(ref aa) = a {
    //     if let Some(ref s) = aa.summary {
    //         println!("|||{s}|||");
    //     }
    // }

    a
}
