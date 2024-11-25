use std::cell::RefCell;
use std::rc::Rc;

use crate::ical::components::todo::CalendarTodo;

use super::parser::{follow_tree, format_ns_attrs, parse_cal_propfind, parse_todo_report, NS_D};
use minidom::Element;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Method};

pub struct Calendar {
    pub url: String,
    pub name: String,
    pub ctag: String,
    pub color: Option<String>,
    pub description: Option<String>,
    cache_current_todos: Rc<Vec<CalendarTodo>>,
    cache_past_todos: Rc<Vec<CalendarTodo>>,
}

pub struct CalDAVClient {
    client: Client,
    base_url: String,
    username: String,
    password: String,
    pub home: String,
    pub calendars: Vec<RefCell<Calendar>>,
}

const CALENDAR_PROPS: &str = r#"
    <d:prop>
        <d:displayname />
        <c:supported-calendar-component-set />
        <cs:getctag />
        <c:calendar-description />
        <i:calendar-color />
        <d:resourcetype />
    </d:prop>
"#;

impl CalDAVClient {
    pub async fn new(base_url: &str, username: &str, password: &str) -> Option<Self> {
        let mut c = CalDAVClient {
            client: Client::new(),
            base_url: base_url.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            home: "".to_string(),
            calendars: vec![],
        };
        let principal = c.get_principal().await?;
        c.home = c.get_homeset(&principal).await?;
        c.calendars = c.get_calendars().await;
        Some(c)
    }

    async fn request(
        &self,
        method: Method,
        path: &str,
        depth: i32,
        body: String,
    ) -> Option<String> {
        let res = self
            .client
            .request(method.clone(), self.base_url.to_string() + &path)
            .header("Depth", depth)
            .header(CONTENT_TYPE, "application/xml")
            .basic_auth(&self.username, Some(&self.password))
            .body(body.clone())
            .send()
            .await
            .ok()?;
        let text_opt = res.text().await.ok();

        //TODO remove this? (so the body clone can be removed)
        if let Some(text) = &text_opt {
            if text == "Bad Request" {
                panic!(
                    "Bad Request!\nMethod = {}\nPath = {}\nDepth = {}\nBody = {}\n",
                    method, path, depth, body
                );
            }
        }

        text_opt
    }

    async fn propfind(&self, path: &str, depth: i32, body: &str) -> Option<Element> {
        let method = Method::from_bytes(b"PROPFIND").unwrap();
        let body_start = format!("<d:propfind {}>", format_ns_attrs());
        let body = body_start + body + "</d:propfind>";
        let res = self.request(method, path, depth, body).await?;
        Some(res.parse().ok()?)
    }

    async fn calquery(&self, path: &str, depth: i32, body: &str) -> Option<Element> {
        let method = Method::from_bytes(b"REPORT").unwrap();
        let body_start = format!("<d:calendar-query {}>", format_ns_attrs());
        let body = body_start + body + "</d:calendar-query>";
        let res = self.request(method, path, depth, body).await?;
        Some(res.parse().ok()?)
    }

    async fn get_principal(&self) -> Option<String> {
        let root = self
            .propfind("", 0, "<d:prop><d:current-user-principal /></d:prop>")
            .await?;
        Some(
            follow_tree(
                &root,
                "response.propstat.prop.current-user-principal.href",
                NS_D,
            )?
            .text(),
        )
    }

    async fn get_homeset(&self, path: &str) -> Option<String> {
        let root = self
            .propfind(path, 0, "<d:self/><d:prop><d:current-home-set /></d:prop>")
            .await?;
        Some(follow_tree(&root, "response.href", NS_D)?.text())
    }

    async fn get_calendars(&self) -> Vec<RefCell<Calendar>> {
        let root = self.propfind(&self.home, 1, CALENDAR_PROPS).await.unwrap();
        let cals: Vec<RefCell<Calendar>> = root
            .children()
            .filter_map(|response| Some(RefCell::new(parse_cal_propfind(response)?)))
            .collect();
        cals
    }

    async fn refresh_calendar(&self, cal_ref: &RefCell<Calendar>) -> bool {
        let root = self.propfind(&self.home, 1, CALENDAR_PROPS).await.unwrap();
        let new_calendar = parse_cal_propfind(root.children().next().expect("Refresh calendar failed!")).expect("Calendar is gone!");
        if new_calendar.ctag != cal_ref.borrow().ctag {
            *cal_ref.borrow_mut() = new_calendar;
            return true;
        }
        false
    }

    async fn get_todos(&self, cal: &Calendar, filter: &str) -> Vec<CalendarTodo> {
        let body = format!(
            r#"
            <d:prop>
                <d:getetag />
                <c:calendar-data />
            </d:prop>
            <c:filter>
                <c:comp-filter name="VCALENDAR">
                    {filter}
                </c:comp-filter>
            </c:filter>
        "#
        );
        if let Some(root) = self.calquery(&cal.url, 1, &body).await {
            root.children()
                .filter_map(|response| parse_todo_report(response))
                .collect()
        } else {
            vec![]
        }
    }

    pub async fn get_current_todos(&self, cal_ref: &RefCell<Calendar>) -> Rc<Vec<CalendarTodo>> {
        //have cache & ctag did not change => use cache
        if cal_ref.borrow().cache_current_todos.len() > 0 && !self.refresh_calendar(cal_ref).await {
            //TODO check ctag
            return cal_ref.borrow().cache_current_todos.clone();
        }

        let mut todos1 = self.get_todos(&cal_ref.borrow(), r#"
            <c:comp-filter name="VTODO">
                <c:prop-filter name="PERCENT-COMPLETE">
                    <c:text-match collation="i;ascii-numeric" negate-condition="yes">100</c:text-match>
                </c:prop-filter>
            </c:comp-filter>
        "#).await;
        let mut todos2 = self
            .get_todos(
                &cal_ref.borrow(),
                r#"
            <c:comp-filter name="VTODO">
                <c:prop-filter name="PERCENT-COMPLETE">
                    <c:is-not-defined/>
                </c:prop-filter>
            </c:comp-filter>
        "#,
            )
            .await;
        todos1.append(&mut todos2);
        cal_ref.borrow_mut().cache_current_todos = todos1.into();
        cal_ref.borrow().cache_current_todos.clone()
    }

    pub async fn get_past_todos(&self, cal_ref: &RefCell<Calendar>) -> Rc<Vec<CalendarTodo>> {
        //have cache & ctag did not change => use cache
        if cal_ref.borrow().cache_past_todos.len() > 0  && !self.refresh_calendar(cal_ref).await {
            return cal_ref.borrow().cache_past_todos.clone();
        }

        let todos = self.get_todos(
            &cal_ref.borrow(),
            r#"
            <c:comp-filter name="VTODO">
                <c:prop-filter name="PERCENT-COMPLETE">
                    <c:text-match collation="i;ascii-numeric">100</c:text-match>
                </c:prop-filter>
            </c:comp-filter>
        "#,
        )
        .await;

        cal_ref.borrow_mut().cache_past_todos = todos.into();
        cal_ref.borrow().cache_past_todos.clone()
    }

    pub fn get_calendar(&self, name: &str) -> Option<&RefCell<Calendar>> {
        for cal in &self.calendars {
            if cal.borrow().name == name {
                return Some(cal);
            }
        }
        None
    }
}

impl Calendar {
    pub fn new(
        url: String,
        name: String,
        ctag: String,
        color: Option<String>,
        description: Option<String>,
    ) -> Self {
        Calendar {
            url,
            name,
            ctag,
            color,
            description,
            cache_current_todos: Rc::new(vec![]),
            cache_past_todos: Rc::new(vec![]),
        }
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
