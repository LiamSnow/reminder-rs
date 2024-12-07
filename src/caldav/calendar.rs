use std::rc::Rc;
use anyhow::anyhow;

use minidom::Element;

use super::{parser::{follow_tree, NS_C, NS_CS, NS_D, NS_I}, todo::CalendarTodo};

pub struct Calendar {
    pub url: String,
    pub name: String,
    pub ctag: String,
    pub color: Option<String>,
    pub description: Option<String>,
    pub supports_todo: bool,
    pub(crate) cache_current_todos: Rc<Vec<CalendarTodo>>,
    pub(crate) cache_past_todos: Rc<Vec<CalendarTodo>>,
}

impl Calendar {
    pub fn parse(el: &Element) -> Option<Self> {
        let url = follow_tree(el, "href", NS_D)?;
        let prop = follow_tree(el, "propstat.prop", NS_D)?;
        let name = prop.get_child("displayname", NS_D)?;
        let ctag = prop.get_child("getctag", NS_CS)?;

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

        if !is_calendar {
            return None;
        }

        Some(Calendar {
            url: url.text(),
            name: name.text(),
            ctag: ctag.text(),
            color,
            description,
            supports_todo,
            cache_current_todos: Rc::new(vec![]),
            cache_past_todos: Rc::new(vec![]),
        })
    }

    pub fn get_color(&self) -> &str {
        match &self.color {
            Some(c) => &c,
            None => "#ffffff",
        }
    }

    pub fn fancy_name(&self) -> String {
        let color = self.get_color();
        format!(
            "\x1B[48;2;{};{};{}m  \x1B[0m {}",
            // Convert hex color to RGB
            u8::from_str_radix(&color[1..3], 16).unwrap_or(255),
            u8::from_str_radix(&color[3..5], 16).unwrap_or(255),
            u8::from_str_radix(&color[5..7], 16).unwrap_or(255),
            self.name
        )
    }
}
