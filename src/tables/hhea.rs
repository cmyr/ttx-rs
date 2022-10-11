//! The 'hhea' table.

use read_fonts::tables::hhea::Hhea;
use xml_builder::XMLElement;

use crate::ttx::Ttx;
use crate::util::value_elem;

impl Ttx for Hhea<'_> {
    fn write_ttx(&self, mut into: &mut dyn std::io::Write) -> Result<(), crate::error::Error> {
        let mut root = XMLElement::new("hhea");
        root.add_child(value_elem("tableVersion", self.version()))
            .unwrap();
        root.add_child(value_elem("ascent", self.ascender()))
            .unwrap();
        root.add_child(value_elem("descent", self.descender()))
            .unwrap();
        root.add_child(value_elem("lineGap", self.line_gap()))
            .unwrap();
        root.add_child(value_elem("advanceWidthMax", self.advance_width_max()))
            .unwrap();
        root.add_child(value_elem(
            "minLeftSideBearing",
            self.min_left_side_bearing(),
        ))
        .unwrap();
        root.add_child(value_elem(
            "minRightSideBearing",
            self.min_right_side_bearing(),
        ))
        .unwrap();
        root.add_child(value_elem("xMaxExtent", self.x_max_extent()))
            .unwrap();
        root.add_child(value_elem("caretSlopeRise", self.caret_slope_rise()))
            .unwrap();
        root.add_child(value_elem("caretSlopeRun", self.caret_slope_run()))
            .unwrap();
        root.add_child(value_elem("caretOffset", self.caret_offset()))
            .unwrap();
        for i in 0..4 {
            root.add_child(value_elem(format!("reserved{}", i).as_str(), 0))
                .unwrap();
        }
        root.add_child(value_elem("metricDataFormat", self.metric_data_format()))
            .unwrap();
        root.add_child(value_elem("numberOfHMetrics", self.number_of_h_metrics()))
            .unwrap();
        root.render(&mut into, false, true).unwrap();
        Ok(())
    }
}
