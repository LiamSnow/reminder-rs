use crate::ical::vtodo::VTodo;

use super::parser::{follow_tree, format_ns_attrs, parse_cal_propfind, parse_todo_report, NS_D};
use minidom::Element;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Method};

pub struct Calendar {
    pub url: String,
    pub name: String,
    pub ctag: String,
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
            .request(method, self.base_url.to_string() + &path)
            .header("Depth", depth)
            .header(CONTENT_TYPE, "application/xml")
            .basic_auth(&self.username, Some(&self.password))
            .body(body)
            .send()
            .await
            .ok()?;
        res.text().await.ok()
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
            </d:prop>
        "#;
        let root = self.propfind(&self.home, 1, body).await.unwrap();
        let cals: Vec<Calendar> = root
            .children()
            .filter_map(|response| parse_cal_propfind(response))
            .collect();
        self.calendars = cals;
    }

    pub async fn get_todos(&self, path: &str) -> Vec<VTodo> {
        let body = r#"
            <d:prop>
                <d:getetag />
                <c:calendar-data />
            </d:prop>
            <c:filter>
                <c:comp-filter name="VCALENDAR">
                    <c:comp-filter name="VTODO" />
                </c:comp-filter>
            </c:filter>
        "#;
        if let Some(root) = self.calquery(path, 1, body).await {
            root.children()
                .filter_map(|response| parse_todo_report(response))
                .collect()
        }
        else {
            vec![]
        }
    }
}
