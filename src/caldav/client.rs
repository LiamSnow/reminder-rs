use crate::ical::todo::CalendarTodo;

use super::parser::{follow_tree, format_ns_attrs, parse_cal_propfind, parse_todo_report, NS_D};
use minidom::Element;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Method};

pub struct Calendar {
    pub url: String,
    pub name: String,
    pub ctag: String,
    pub description: Option<String>,
    pub color: Option<String>
}

pub struct CalDAVClient {
    client: Client,
    base_url: String,
    username: String,
    password: String,
    pub home: String,
    pub calendars: Vec<Calendar>
}

impl CalDAVClient {
    pub async fn new(base_url: &str, username: &str, password: &str) -> Option<Self> {
        let mut c = CalDAVClient {
            client: Client::new(),
            base_url: base_url.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            home: "".to_string(),
            calendars: vec![]
        };
        let principal = c.get_principal().await?;
        c.home = c.get_homeset(&principal).await?;
        c.refresh_calendars().await;
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
                panic!("Bad Request!\nMethod = {}\nPath = {}\nDepth = {}\nBody = {}\n",
                    method, path, depth, body);
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

    pub async fn refresh_calendars(&mut self) {
        let body = r#"
            <d:prop>
                <d:displayname />
                <c:supported-calendar-component-set />
                <cs:getctag />
                <c:calendar-description />
                <i:calendar-color />
                <d:resourcetype />
            </d:prop>
        "#;
        let root = self.propfind(&self.home, 1, body).await.unwrap();
        let cals: Vec<Calendar> = root
            .children()
            .filter_map(|response| parse_cal_propfind(response))
            .collect();
        self.calendars = cals;
    }

    async fn get_todos(&self, cal: &Calendar, filter: &str) -> Vec<CalendarTodo> {
        let body = format!(r#"
            <d:prop>
                <d:getetag />
                <c:calendar-data />
            </d:prop>
            <c:filter>
                <c:comp-filter name="VCALENDAR">
                    {filter}
                </c:comp-filter>
            </c:filter>
        "#);
        if let Some(root) = self.calquery(&cal.url, 1, &body).await {
            root.children()
                .filter_map(|response| parse_todo_report(response))
                .collect()
        }
        else {
            vec![]
        }
    }

    pub async fn get_current_todos(&self, cal: &Calendar) -> Vec<CalendarTodo> {
        let mut g1 = self.get_todos(cal, r#"
            <c:comp-filter name="VTODO">
                <c:prop-filter name="PERCENT-COMPLETE">
                    <c:text-match collation="i;ascii-numeric" negate-condition="yes">100</c:text-match>
                </c:prop-filter>
            </c:comp-filter>
        "#).await;
        let mut g2 = self.get_todos(cal, r#"
            <c:comp-filter name="VTODO">
                <c:prop-filter name="PERCENT-COMPLETE">
                    <c:is-not-defined/>
                </c:prop-filter>
            </c:comp-filter>
        "#).await;
        g1.append(&mut g2);
        g1
    }

    pub async fn get_past_todos(&self, cal: &Calendar) -> Vec<CalendarTodo> {
        self.get_todos(cal, r#"
            <c:comp-filter name="VTODO">
                <c:prop-filter name="PERCENT-COMPLETE">
                    <c:text-match collation="i;ascii-numeric">100</c:text-match>
                </c:prop-filter>
            </c:comp-filter>
        "#).await
    }

    pub async fn get_all_todos(&self, cal: &Calendar) -> Vec<CalendarTodo> {
        self.get_todos(cal, r#"
            <c:comp-filter name="VTODO" />
        "#).await
    }

    pub fn get_calendar(&self, calendar_name: &str) -> Option<&Calendar> {
        for cal in &self.calendars {
            if cal.name == calendar_name {
                return Some(&cal);
            }
        }
        None
    }
}

impl Calendar {
    pub fn get_color(&self) -> &str {
        match &self.color {
            Some(c) => &c,
            None => "#ffffff",
        }
    }

    pub fn fancy_name(&self) -> String {
        let color = self.get_color();
        format!("\x1B[48;2;{};{};{}m  \x1B[0m {}",
            // Convert hex color to RGB
            u8::from_str_radix(&color[1..3], 16).unwrap_or(255),
            u8::from_str_radix(&color[3..5], 16).unwrap_or(255),
            u8::from_str_radix(&color[5..7], 16).unwrap_or(255),
            self.name
        )
    }
}
