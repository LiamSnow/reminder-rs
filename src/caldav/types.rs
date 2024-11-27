use crate::ical::objects::{generics::VCalendar, vtodo::VTodo};





///represents the entire VTODO REPORT
pub struct CalendarTodo {
    pub etag: String,
    pub url: String,
    ///has VTODO removed
    pub vcal: VCalendar,
    pub vtodo: VTodo
}
