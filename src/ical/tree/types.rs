#![allow(dead_code)]

use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TreeObject {
    Component(TreeComponent),
    Property(TreeProperty)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeComponent {
    pub name: String,
    pub params: TreeParameterMap,
    pub children: Vec<TreeObject>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreeProperty {
    pub name: String,
    pub params: TreeParameterMap,
    pub value: String,
}

pub type TreeParameterMap = HashMap<String, String>;

impl TreeObject {
    pub fn expect_component(&self) -> &TreeComponent {
        match &self {
            TreeObject::Component(comp) => comp,
            TreeObject::Property(_) => panic!("Expected component, found property!"),
        }
    }

    pub fn expect_property(&self) -> &TreeProperty {
        match &self {
            TreeObject::Component(_) => panic!("Expected property, found component!"),
            TreeObject::Property(prop) => prop,
        }
    }

    pub fn name(&self) -> &str {
        match &self {
            TreeObject::Component(comp) => &comp.name,
            TreeObject::Property(prop) => &prop.name,
        }
    }

    pub fn is_component(&self) -> bool {
        matches!(self, TreeObject::Component(_))
    }

    pub fn is_property(&self) -> bool {
        matches!(self, TreeObject::Property(_))
    }
}

impl TreeComponent {
    pub fn find_component_index(&self, name: &str) -> Option<usize> {
        for (i, child) in self.children.iter().enumerate() {
            if child.is_component() && child.name() == name {
                return Some(i);
            }
        }
        None
    }

    pub fn find_property_index(&self, name: &str) -> Option<usize> {
        for (i, child) in self.children.iter().enumerate() {
            if child.is_property() && child.name() == name {
                return Some(i);
            }
        }
        None
    }

    pub fn find_component(&self, name: &str) -> Option<&TreeComponent> {
        let i = self.find_component_index(name)?;
        Some(&self.children[i].expect_component())
    }

    pub fn find_property(&self, name: &str) -> Option<&TreeProperty> {
        let i = self.find_property_index(name)?;
        Some(&self.children[i].expect_property())
    }

    pub fn pop_component(&mut self, name: &str) -> Option<TreeComponent> {
        let i = self.find_component_index(name)?;
        let comp = self.children.remove(i);
        Some(comp.expect_component().clone())
    }

    pub fn pop_property(&mut self, name: &str) -> Option<TreeProperty> {
        let i = self.find_property_index(name)?;
        let prop = self.children.remove(i);
        Some(prop.expect_property().clone())
    }

    pub fn pop_property_value_or_empty(&mut self, name: &str) -> String {
        if let Some(prop) = self.pop_property(name) {
            return prop.value;
        }
        "".to_string()
    }
}

// -- DISPLAYING -- \\

impl Display for TreeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_level(f, 0)
    }
}

impl TreeObject {
    fn fmt_level(&self, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
        let t = "  ".repeat(level);

        match self {
            TreeObject::Component(b) => {
                writeln!(f, "{t}{} [", b.name)?;
                for child in &b.children {
                    child.fmt_level(f, level + 1)?;
                    writeln!(f)?;
                }
                write!(f, "{t}]")
            }
            TreeObject::Property(p) => write!(f, "{t}{p}"),
        }
    }
}

impl Display for TreeProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = \"{}\",", self.name, self.value)
    }
}

impl Display for TreeComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = TreeObject::Component(self.clone());
        e.fmt_level(f, 0)
    }
}

