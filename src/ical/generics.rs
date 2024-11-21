#![allow(dead_code)]

use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CalendarObject {
    Component(CalendarComponent),
    Property(CalendarProperty)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarComponent {
    pub name: String,
    pub children: Vec<CalendarObject>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarProperty {
    pub name: String,
    pub value: String,
}

impl CalendarObject {
    pub fn expect_component(&self) -> &CalendarComponent {
        match &self {
            CalendarObject::Component(comp) => comp,
            CalendarObject::Property(_) => panic!("Expected component, found property!"),
        }
    }

    pub fn expect_property(&self) -> &CalendarProperty {
        match &self {
            CalendarObject::Component(_) => panic!("Expected property, found component!"),
            CalendarObject::Property(prop) => prop,
        }
    }

    pub fn name(&self) -> &str {
        match &self {
            CalendarObject::Component(comp) => &comp.name,
            CalendarObject::Property(prop) => &prop.name,
        }
    }

    pub fn is_component(&self) -> bool {
        matches!(self, CalendarObject::Component(_))
    }

    pub fn is_property(&self) -> bool {
        matches!(self, CalendarObject::Property(_))
    }
}

impl CalendarComponent {
    pub fn find_component_index_with_guess(&self, name: &str, guess: usize) -> Option<usize> {
        if guess < self.children.len() {
            let child = &self.children[guess];
            if child.is_component() && child.name() == name {
                return Some(guess);
            }
        }
        self.find_component_index(name)
    }

    pub fn find_component_index(&self, name: &str) -> Option<usize> {
        for (i, child) in self.children.iter().enumerate() {
            if child.is_component() && child.name() == name {
                return Some(i);
            }
        }
        None
    }

    pub fn find_component_with_guess(&self, name: &str, guess: usize) -> Option<&CalendarComponent> {
        let i = self.find_component_index_with_guess(name, guess)?;
        Some(&self.children[i].expect_component())
    }

    pub fn find_component(&self, name: &str) -> Option<&CalendarComponent> {
        let i = self.find_component_index(name)?;
        Some(&self.children[i].expect_component())
    }

    pub fn find_property(&self, name: &str) -> Option<&CalendarProperty> {
        self.children.iter().find_map(|child| {
            if child.is_property() && child.name() == name {
                return Some(child.expect_property());
            }
            None
        })
    }

    pub fn get_property_value(&self, name: &str) -> Option<&str> {
        let prop = self.find_property(&name)?;
        Some(&prop.value)
    }

    pub fn get_property_value_date(&self, name: &str) -> Option<&str> {
        self.get_property_value(name)
        // DateTime::parse_from_rfc3339(datetime_str)
    }
}

// -- DISPLAYING -- \\

impl Display for CalendarObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_level(f, 0)
    }
}

impl CalendarObject {
    fn fmt_level(&self, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
        let t = "  ".repeat(level);

        match self {
            CalendarObject::Component(b) => {
                writeln!(f, "{t}{} [", b.name)?;
                for child in &b.children {
                    child.fmt_level(f, level + 1)?;
                    writeln!(f)?;
                }
                write!(f, "{t}]")
            }
            CalendarObject::Property(p) => write!(f, "{t}{p}"),
        }
    }
}

impl Display for CalendarProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = \"{}\",", self.name, self.value)
    }
}

impl Display for CalendarComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = CalendarObject::Component(self.clone());
        e.fmt_level(f, 0)
    }
}

