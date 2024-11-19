use minidom::Element;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use reqwest::Method;
use types::Calendar;
use util::follow_tree;

mod util;
mod types;

const BASE_URL: &str = "";
const USERNAME: &str = "";
const PASSWORD: Option<&str> = Some("");
const NS_D: &str = "DAV:";
const NS_C: &str = "urn:ietf:params:xml:ns:caldav";
const NS_CS: &str = "http://calendarserver.org/ns/";
const NS_C_ATR: &str = "xmlns:c=\"urn:ietf:params:xml:ns:caldav\"";
const NS_CS_ATR: &str = "xmlns:cs=\"http://calendarserver.org/ns/\"";

pub async fn request(client: &Client, method: Method, path: &str, depth: i32, body: String) -> Option<String> {
    let res = client
        .request(method, BASE_URL.to_string() + &path)
        .header("Depth", depth)
        .header(CONTENT_TYPE, "application/xml")
        .basic_auth(USERNAME, PASSWORD)
        .body(body)
        .send()
        .await
        .ok()?;
    res.text().await.ok()
}

pub async fn propfind(client: &Client, path: &str, depth: i32, attr: &str, props: &str) -> Option<Element> {
    let method = Method::from_bytes(b"PROPFIND").unwrap();
    let body_start = format!("<d:propfind xmlns:d=\"DAV:\" {attr}>");
    let body_end = "</d:propfind>";
    let body = body_start + props + body_end;
    println!("body={body}\n");
    let res = request(client, method, path, depth, body).await?;
    println!("res={res}\n");
    Some(res.parse().ok()?)
}

pub async fn get_principal(client: &Client) -> Option<String> {
    let root = propfind(client, "", 0, "", "<d:prop><d:current-user-principal /></d:prop>").await?;
    Some(follow_tree(&root, "response.propstat.prop.current-user-principal.href", NS_D)?.text())
}

pub async fn get_homeset(client: &Client, path: &str) -> Option<String> {
    let root = propfind(client, path, 0, NS_C_ATR, "<d:self/><d:prop><d:current-home-set /></d:prop>").await?;
    Some(follow_tree(&root, "response.href", NS_D)?.text())
}

pub fn parse_calendar(el: &Element) -> Option<Calendar> {
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

pub async fn list_calendars(client: &Client, path: &str) -> Option<Vec<Calendar>> {
    //<C:supported-calendar-component-set><C:comp name="VTODO" /></C:supported-calendar-component-set>
    let props = r#"
        <d:prop>
            <d:displayname />
            <c:supported-calendar-component-set />
            <cs:getctag />
        </d:prop>
    "#;
    let attr = NS_C_ATR.to_owned() + " " + NS_CS_ATR;
    let root = propfind(client, path, 1, &attr, props).await?;
    let cals: Vec<Calendar> = root.children()
        .filter_map(|response| parse_calendar(response))
        .collect();
    Some(cals)
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let prin = get_principal(&client).await.unwrap();
    println!("Principal: {}\n", prin);

    let home = get_homeset(&client, &prin).await.unwrap();
    println!("Home: {}\n", home);

    let cals = list_calendars(&client, &home).await.unwrap();

    for cal in cals {
      println!("{} {}", cal.name, cal.ctag);
    }
}
