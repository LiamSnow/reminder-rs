use minidom::Element;
use url::Url;

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

pub fn add_path(url: &str, path: &str) -> String {
    let base = url.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{}/{}", base, path)
}

pub fn go_back(url: &str) -> String {
    let url = url.trim_end_matches('/');
    match Url::parse(url) {
        Ok(mut parsed_url) => {
            let segments: Vec<_> = parsed_url.path_segments()
                .map(|segments| segments.collect::<Vec<_>>())
                .unwrap_or_default();
            if !segments.is_empty() {
                let new_path = segments[..segments.len() - 1].join("/");
                parsed_url.set_path(&new_path);
            } else {
                parsed_url.set_path("/");
            }
            parsed_url.to_string()
        },
        Err(_) => url.to_string(),
    }
}
