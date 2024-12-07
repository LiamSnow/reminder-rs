use std::cell::RefCell;

use super::calendar::Calendar;
use super::parser::{add_path, go_back, follow_tree, format_ns_attrs, NS_C, NS_D};
use minidom::Element;
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Method};
use anyhow::{Context, anyhow};

pub struct CalDAVClient {
    client: Client,
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
    pub async fn new(base_url: &str, username: &str, password: &str) -> anyhow::Result<Self> {
        let mut client = CalDAVClient {
            client: Client::new(),
            username: username.to_string(),
            password: password.to_string(),
            home: "".to_string(),
            calendars: vec![],
        };
        let principal = client.get_principal(base_url).await.context("Get principal failed")?;
        client.home = client.get_homeset(&principal).await.context("Get homeset failed")?;
        client.calendars = client.get_calendars().await.context("Get calendars failed")?;
        Ok(client)
    }

    async fn request(
        &self,
        method: Method,
        url: &str,
        depth: i32,
        body: String,
    ) -> anyhow::Result<String> {
        let full_url = if url.contains("http") { url.to_string() } else { add_path(&self.home, url) };
        println!("REQ {full_url}");
        let text = self
            .client
            .request(method.clone(), full_url)
            .header("Depth", depth)
            .header(CONTENT_TYPE, "application/xml")
            .basic_auth(&self.username, Some(&self.password))
            .body(body.clone())
            .send()
            .await.context("Request failed")?
            .text()
            .await.context("Request did not return text")?;
        if text == "Bad Request" {
            return Err(anyhow!("Bad Request"));
        }
        Ok(text)
    }

    async fn propfind(&self, url: &str, depth: i32, body: &str) -> anyhow::Result<Element> {
        let method = Method::from_bytes(b"PROPFIND").unwrap();
        let body_start = format!("<d:propfind {}>", format_ns_attrs());
        let body = body_start + body + "</d:propfind>";
        // println!("PROPFIND {}", url);
        let res = self.request(method, url, depth, body).await.context("PROPFIND request failed")?;
        Ok(res.parse().context("Parsing PROPFIND XML failed")?)
    }

    pub(crate) async fn calquery(&self, url: &str, depth: i32, body: &str) -> anyhow::Result<Element> {
        let method = Method::from_bytes(b"REPORT").unwrap();
        let body_start = format!("<d:calendar-query {}>", format_ns_attrs());
        let body = body_start + body + "</d:calendar-query>";
        let res = self.request(method, url, depth, body).await.context("CALENDAR-QUERY request failed")?;
        Ok(res.parse().context("Parsing CALENDAR-QUERY failed")?)
    }

    async fn get_principal(&self, url: &str) -> anyhow::Result<String> {
        let root = self
            .propfind(url, 0, "<d:prop><d:current-user-principal /></d:prop>")
            .await?;
        let href = follow_tree( &root, "response.propstat.prop.current-user-principal.href", NS_D)
            .ok_or(anyhow!("Principal response did not contain href"))?
            .text();
        if href.contains("http") {
            return Ok(href);
        }
        Ok(add_path(url, &href))
    }

    async fn get_homeset(&self, url: &str) -> anyhow::Result<String> {
        let root = self
            .propfind(url, 0, "<d:prop><c:calendar-home-set /></d:prop>")
            .await?;
        let prop = follow_tree(&root, "response.propstat.prop", NS_D)
            .ok_or(anyhow!("Homeset response did not contain prop"))?;
        let home_set = follow_tree(&prop, "calendar-home-set", NS_C)
            .ok_or(anyhow!("Homeset response did not contain calendar-home-set"))?;
        let href = follow_tree(&home_set, "href", NS_D)
            .ok_or(anyhow!("Homeset response did not contain href"))?
            .text();
        if href.contains("http") {
            return Ok(href);
        }
        Ok(add_path(&go_back(url), &href))
    }

    async fn get_calendars(&self) -> anyhow::Result<Vec<RefCell<Calendar>>> {
        let root = self.propfind(&self.home, 1, CALENDAR_PROPS).await?;
        let cals: Vec<RefCell<Calendar>> = root
            .children()
            .filter_map(|response| Some(RefCell::new(Calendar::parse(response)?)))
            .collect();
        Ok(cals)
    }

    pub(crate) async fn refresh_calendar(&self, cal_ref: &RefCell<Calendar>) -> anyhow::Result<bool> {
        let root = self.propfind(&self.home, 1, CALENDAR_PROPS).await?;
        let child = root.children().next().ok_or(anyhow!("Refresh calendar returned nothing!"))?;
        let new_calendar = Calendar::parse(child)
            .ok_or(anyhow!("Refresh calendar failed because it is no longer a calendar?!"))?;
        let changed = new_calendar.ctag != cal_ref.borrow().ctag;
        if changed {
            *cal_ref.borrow_mut() = new_calendar;
        }
        Ok(changed)
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

