//! font tables, encoded in xml

use font_types::Tag;
use read_fonts::{FontRef, TableProvider};

use crate::{args::Args, error::Error};

pub trait Ttx {
    fn write_ttx(&self, into: &mut dyn std::io::Write) -> Result<(), Error>;
}

pub fn dump(args: &Args, font: &FontRef) -> Result<(), Error> {
    for tag in font
        .table_directory
        .table_records()
        .iter()
        .map(|rec| rec.tag())
        .filter(|tag| args.include_table(*tag))
    {
        write_table_ttx(font, tag, args)?;
    }

    Ok(())
}

fn write_table_ttx(font: &FontRef, tag: Tag, args: &Args) -> Result<(), Error> {
    let mut target = args.target(tag)?;
    match tag {
        read_fonts::tables::head::TAG => font.head()?.write_ttx(&mut target),
        read_fonts::tables::hhea::TAG => font.hhea()?.write_ttx(&mut target),
        read_fonts::tables::maxp::TAG => font.maxp()?.write_ttx(&mut target),
        other => {
            eprintln!("ttx is not yet implemented for table {other}");
            Ok(())
        }
    }
}
