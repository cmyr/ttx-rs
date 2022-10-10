//! ttx-rs: a Rust port of the [`ttx`` command line utility][ttx]
//!
//! [ttx]: https://fonttools.readthedocs.io/en/latest/ttx.html

mod args;
mod error;
mod tables;
mod ttx;
mod util;

use clap::Parser;
use read_fonts::{FontData, FontRef};

use args::Args;
use error::Error;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    // initially we only support writing *to* ttx, so assume input is a font file.
    let input = std::fs::read(&args.input)?;
    let data = FontData::new(&input);
    let font = FontRef::new(data)?;

    if args.list {
        list_tables(&font);
        return Ok(());
    }

    ttx::dump(&args, &font)
}

fn list_tables(_font: &FontRef) {
    todo!("implement list!")
}
