//! The 'head' table

use read_fonts::tables::head::Head;
use xml_builder::XMLElement;

use crate::ttx::Ttx;

use ttx_rs_macros::ttx_me_baby;

struct TtxField {
    xml_name: &'static str
}

#[ttx_me_baby]
static BLEH: [TtxField; 2] = [
    TtxField { xml_name: "tableVersion" },
    TtxField { xml_name: "fontRevision" },
];

impl Ttx for Head<'_> {
    fn write_ttx(&self, mut into: &mut dyn std::io::Write) -> Result<(), crate::error::Error> {
        let mut root = XMLElement::new("head");
        root.add_child(value_elem("tableVersion", self.version()))
            .unwrap();
        root.add_child(value_elem("fontRevision", self.font_revision()))
            .unwrap();
        root.add_child(value_elem(
            "checkSumAdjustment",
            format!("0x{:x}", self.checksum_adjustment()),
        ))
        .unwrap();
        root.add_child(value_elem(
            "magicNumber",
            format!("0x{:x}", self.magic_number()),
        ))
        .unwrap();

        root.add_child(value_elem("flags", format!("{:016b}", self.flags())))
            .unwrap();
        root.add_child(value_elem("unitsPerEm", self.units_per_em()))
            .unwrap();
        root.add_child(value_elem("created", self.created().as_secs()))
            .unwrap();
        root.add_child(value_elem("modified", self.modified().as_secs()))
            .unwrap();
        root.add_child(value_elem("xMin", self.x_min())).unwrap();
        root.add_child(value_elem("yMin", self.y_min())).unwrap();
        root.add_child(value_elem("xMax", self.x_max())).unwrap();
        root.add_child(value_elem("yMax", self.y_max())).unwrap();
        root.add_child(value_elem("macStyle", format!("{:016b}", self.mac_style())))
            .unwrap();
        root.add_child(value_elem("lowestRecPPEM", self.lowest_rec_ppem()))
            .unwrap();
        root.add_child(value_elem("fontDirectionHint", self.font_direction_hint()))
            .unwrap();
        root.add_child(value_elem("indexToLocFormat", self.index_to_loc_format()))
            .unwrap();
        root.add_child(value_elem("glyphDataFormat", self.index_to_loc_format()))
            .unwrap();
        root.render(&mut into, false, true).unwrap();
        Ok(())
    }
}

fn value_elem(name: &str, value: impl std::fmt::Display) -> XMLElement {
    let mut elem = XMLElement::new(name);
    elem.add_attribute("value", value.to_string().as_str());
    elem
}
