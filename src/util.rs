use minidom::Element;

pub fn follow_tree(el: &Element, tree: &str, namespace: &str) -> Option<Element> {
    let parts = tree.split(".");
    let mut cur_el = el;
    for part in parts {
        cur_el = cur_el.get_child(part, namespace)?;
    }
    Some(cur_el.clone())
}
