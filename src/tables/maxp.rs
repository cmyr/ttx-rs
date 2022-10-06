//! The 'maxp' table.

use read_fonts::tables::maxp::Maxp;
use xml_builder::XMLElement;

use crate::ttx::Ttx;

impl Ttx for Maxp<'_> {
    fn write_ttx(&self, mut into: &mut dyn std::io::Write) -> Result<(), crate::error::Error> {
        let mut root = XMLElement::new("maxp");
        add_u16_child(&mut root, "numGlyphs", Some(self.num_glyphs()));
        add_u16_child(&mut root, "maxPoints", self.max_points());
        add_u16_child(&mut root, "maxContours", self.max_contours());
        add_u16_child(&mut root, "maxCompositePoints", self.max_composite_points());
        add_u16_child(&mut root, "maxCompositeContours", self.max_composite_contours());
        add_u16_child(&mut root, "maxZones", self.max_zones());
        add_u16_child(&mut root, "maxTwilightPoints", self.max_twilight_points());
        add_u16_child(&mut root, "maxStorage", self.max_storage());
        add_u16_child(&mut root, "maxFunctionDefs", self.max_function_defs());
        add_u16_child(&mut root, "maxInstructionDefs", self.max_instruction_defs());
        add_u16_child(&mut root, "maxSizeOfInstructions", self.max_size_of_instructions());
        add_u16_child(&mut root, "maxComponentElements", self.max_component_elements());
        add_u16_child(&mut root, "maxComponentDepth", self.max_component_depth());
        add_u16_child(&mut root, "", self.max_component_depth());
        root.render(&mut into, false, true).unwrap();
        Ok(())
    }
}

fn add_u16_child(root : &mut XMLElement, name: &str, value: Option<u16>) {
    if value.is_some() {
        let mut elem = XMLElement::new(name);
        elem.add_attribute("value", value.unwrap().to_string().as_str());
        root.add_child(elem).unwrap();
    }
}
