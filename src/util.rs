//! Misc utilities

use xml_builder::XMLElement;

pub fn value_elem(name: &str, value: impl std::fmt::Display) -> XMLElement {
    let mut elem = XMLElement::new(name);
    elem.add_attribute("value", value.to_string().as_str());
    elem
}
