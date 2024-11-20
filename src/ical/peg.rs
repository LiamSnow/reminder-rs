use std::fmt::Display;

use peg::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Element {
    Block(Block),
    Parameter(Parameter),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    pub name: String,
    pub children: Vec<Element>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub value: String,
}

peg::parser! {
  pub grammar parser() for str {
      rule name() -> String = s:$(['a'..='z' | 'A'..='Z' | '-' | '=' | '/' | '_']+) {
          s.to_string()
      }

      rule value() -> String = s:$([^'\n']*) {
          s.to_string()
      }

      rule parameter() -> Parameter = "\n" name:name() ":" value:value() {
        Parameter { name, value }
      }

      pub rule element() -> Element = precedence!{
        "\n" "BEGIN:" name:name() "\n" children:(element()*) "END:" name:name() {
            Element::Block(Block { name, children })
        }
        --
        p:parameter() {
            Element::Parameter(p)
        }
      }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let str = r#"
BEGIN:VCALENDAR
VERSION:2.0
END:VCALENDAR"#;
//         let str = r#"BEGIN:VCALENDAR
// VERSION:2.0
// CALSCALE:GREGORIAN
// PRODID:-//Apple Inc.//iOS 18.0.1//EN
// BEGIN:VTODO
// COMPLETED:20241016T020342Z
// CREATED:20241014T211812Z
// DTSTAMP:20241027T214435Z
// LAST-MODIFIED:20241016T020342Z
// PERCENT-COMPLETE:100
// STATUS:COMPLETED
// DESCRIPTION:
// SUMMARY:Chop Saw
// UID:F87D9736-8ADE-47E4-AC46-638B5C86E7D0
// X-APPLE-SORT-ORDER:740793996
// END:VTODO
// END:VCALENDAR"#;
        let res = parser::element(str).expect("Failed to parse ical");
        let f = format!("{res}");
        println!("{f}");
//         assert_eq!(
//             f,
//             r#"VCALENDAR [
//   VERSION = "2.0",
//   CALSCALE = "GREGORIAN",
//   PRODID = "-//Apple Inc.//iOS 18.0.1//EN",
//   VTODO [
//     COMPLETED = "20241016T020342Z",
//     CREATED = "20241014T211812Z",
//     DTSTAMP = "20241027T214435Z",
//     LAST-MODIFIED = "20241016T020342Z",
//     PERCENT-COMPLETE = "100",
//     STATUS = "COMPLETED",
//     DESCRIPTION = "",
//     SUMMARY = "Chop Saw",
//     UID = "F87D9736-8ADE-47E4-AC46-638B5C86E7D0",
//     X-APPLE-SORT-ORDER = "740793996",
//   ]
// ]"#
//         );
    }
}

impl Element {
    pub fn expect_block(&self, name: &str) -> Option<&Block> {
        match self {
            Element::Block(block) if block.name == name => Some(block),
            _ => None,
        }
    }
}

impl Block {
    pub fn find_block(&self, name: &str) -> Option<&Block> {
        self.children.iter().find_map(|element| {
            if let Element::Block(block) = element {
                if block.name == name {
                    return Some(block);
                }
            }
            None
        })
    }

    pub fn find_param(&self, name: &str) -> Option<&Parameter> {
        self.children.iter().find_map(|element| {
            if let Element::Parameter(param) = element {
                if param.name == name {
                    return Some(param);
                }
            }
            None
        })
    }

    pub fn get_param_value(&self, name: &str) -> Option<String> {
        let p = self.find_param(name)?;
        Some(p.value.clone())
    }

    pub fn get_param_value_date(&self, name: &str) -> Option<String> {
        self.get_param_value(name)
        // DateTime::parse_from_rfc3339(datetime_str)
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_level(f, 0)
    }
}

impl Element {
    fn fmt_level(&self, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
        let t = "  ".repeat(level);

        match self {
            Element::Block(b) => {
                writeln!(f, "{t}{} [", b.name)?;
                for child in &b.children {
                    child.fmt_level(f, level + 1)?;
                    writeln!(f)?;
                }
                write!(f, "{t}]")
            }
            Element::Parameter(p) => write!(f, "{t}{p}"),
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = \"{}\",", self.name, self.value)
    }
}
