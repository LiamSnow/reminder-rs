use std::{cell::RefCell, rc::Rc};

use minidom::Element;

use crate::ical::objects::{generics::{ICalObject, VCalendar}, vtodo::VTodo};

use super::{calendar::Calendar, client::CalDAVClient, parser::{follow_tree, NS_C, NS_D}};

use anyhow::{anyhow, Context};

///represents the entire VTODO REPORT
pub struct CalendarTodo {
    pub etag: String,
    pub url: String,
    ///has VTODO removed
    pub vcal: VCalendar,
    pub vtodo: VTodo
}

impl CalDAVClient {
    async fn get_todos(&self, cal: &Calendar, filter: &str) -> anyhow::Result<Vec<CalendarTodo>> {
        let body = format!(
            r#"
            <d:prop>
                <d:getetag />
                <c:calendar-data />
            </d:prop>
            <c:filter>
                <c:comp-filter name="VCALENDAR">
                    <c:comp-filter name="VTODO">
                        {filter}
                    </c:comp-filter>
                </c:comp-filter>
            </c:filter>
        "#);
        let root = self.calquery(&cal.url, 1, &body).await
            .context("Get todos")?;
        let mut todos = vec![];
        for child in root.children() {
            todos.push(CalendarTodo::parse(child)?);
        }
        Ok(todos)
    }

    pub async fn get_current_todos(&self, cal_ref: &RefCell<Calendar>) -> anyhow::Result<Rc<Vec<CalendarTodo>>> {
        //have cache & ctag did not change => use cache
        if cal_ref.borrow().cache_current_todos.len() > 0 && !self.refresh_calendar(cal_ref).await? {
            //TODO check ctag
            return Ok(cal_ref.borrow().cache_current_todos.clone());
        }

        let mut todos1 = self.get_todos(&cal_ref.borrow(), r#"
            <c:prop-filter name="PERCENT-COMPLETE">
                <c:text-match collation="i;ascii-numeric" negate-condition="yes">100</c:text-match>
            </c:prop-filter>
        "#).await?;
        let mut todos2 = self.get_todos(&cal_ref.borrow(), r#"
            <c:prop-filter name="PERCENT-COMPLETE">
                <c:is-not-defined/>
            </c:prop-filter>
        "#,).await?;
        todos1.append(&mut todos2);
        cal_ref.borrow_mut().cache_current_todos = todos1.into();
        Ok(cal_ref.borrow().cache_current_todos.clone())
    }

    pub async fn get_past_todos(&self, cal_ref: &RefCell<Calendar>) -> anyhow::Result<Rc<Vec<CalendarTodo>>> {
        //have cache & ctag did not change => use cache
        if cal_ref.borrow().cache_past_todos.len() > 0  && !self.refresh_calendar(cal_ref).await? {
            return Ok(cal_ref.borrow().cache_past_todos.clone());
        }

        let todos = self.get_todos(&cal_ref.borrow(), r#"
            <c:prop-filter name="PERCENT-COMPLETE">
                <c:text-match collation="i;ascii-numeric">100</c:text-match>
            </c:prop-filter>
        "#).await?;

        cal_ref.borrow_mut().cache_past_todos = todos.into();
        Ok(cal_ref.borrow().cache_past_todos.clone())
    }
}

impl CalendarTodo {
    pub fn parse(el: &Element) -> anyhow::Result<CalendarTodo> {
        let url = follow_tree(el, "href", NS_D)
            .ok_or(anyhow!("Todo response did not contain href"))?;
        let prop = follow_tree(el, "propstat.prop", NS_D)
            .ok_or(anyhow!("Todo response did not contain prop"))?;
        let etag = prop.get_child("getetag", NS_D)
            .ok_or(anyhow!("Todo response did not contain getetag"))?;
        let ics = prop.get_child("calendar-data", NS_C)
            .ok_or(anyhow!("Todo response did not contain calendar-data"))?;

        let mut vcal = VCalendar::parse(&ics.text())?;

        //pop vtodo
        let vtodo = vcal
            .children
            .iter()
            .position(|child| matches!(child, ICalObject::VTodo(_)))
            .and_then(|index| {
                if let ICalObject::VTodo(todo) = vcal.children.remove(index) {
                    Some(todo)
                } else {
                    None
                }
            });

        Ok(CalendarTodo {
            etag: etag.text(),
            url: url.text(),
            vcal,
            vtodo: vtodo.ok_or(anyhow!("Todo response did not contain VTODO"))?,
        })
    }
}
