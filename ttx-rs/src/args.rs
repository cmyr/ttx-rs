//! command line argument parsing

use std::{
    convert::Infallible,
    fs::File,
    io::{Stdout, Write},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;
use font_types::Tag;

use crate::error::Error;

#[derive(Debug, Parser)]
pub struct Args {
    /// List table info
    ///
    /// Instead of dumping to a TTX file, list some minimal info about each table.
    #[arg(short)]
    pub list: bool,
    #[arg(short, value_parser(Destination::from_str))]
    out: Option<Destination>,
    /// Specify a table to dump.
    ///
    /// Multiple -t options are allowed. When no -t option is specified,
    /// all tables will be dumped.
    #[arg(short, value_parser(Tag::from_str))]
    table: Vec<Tag>,
    /// Specify a table to exclude from the dump.
    ///
    /// Multiple -x options are allowed. -t and -x are mutually exclusive.
    #[arg(short = 'x', value_parser(Tag::from_str), conflicts_with("table"))]
    exclude: Vec<Tag>,
    pub input: PathBuf,
}

#[derive(Debug, Clone)]
enum Destination {
    File(PathBuf),
    Stdout,
}

/// A target for writing xml: either stdout or a normal file.
pub enum WriteTarget {
    Stdout(Stdout),
    File(File),
}

impl Args {
    /// Returns `true` if the provided tag should be handled, based on the provided args.
    pub fn include_table(&self, tag: Tag) -> bool {
        if !self.table.is_empty() {
            return self.table.contains(&tag);
        } else if self.exclude.contains(&tag) {
            return false;
        }
        true
    }

    /// Returns a type suitable for writing, representing the target file or tty.
    pub fn target(&self, _tag: Tag) -> Result<WriteTarget, Error> {
        match &self.out {
            Some(Destination::File(path)) => {
                let path = next_free_file_name(path.clone())?;
                dbg!(&path);
                Ok(File::create(path).map(WriteTarget::File)?)
            }
            Some(Destination::Stdout) => Ok(WriteTarget::Stdout(std::io::stdout())),
            None => {
                let mut target = self.input.clone();
                target.set_extension("ttx");
                let target = next_free_file_name(target)?;
                dbg!(&target);
                Ok(File::create(target).map(WriteTarget::File)?)
            }
        }
    }
}

fn next_free_file_name(mut base: PathBuf) -> Result<PathBuf, Error> {
    let extension = base.extension().map(|s| s.to_os_string());
    let stem = match base
        .file_stem()
        .and_then(|s| s.to_str().map(str::to_string))
    {
        Some(s) => s,
        None => return Err(Error::InvalidPath(base)),
    };
    let mut next_i = 1;
    while base.exists() {
        let next_stem = format!("{stem}#{next_i}");
        base.set_file_name(next_stem);
        if let Some(extension) = &extension {
            base.set_extension(extension);
        }
        next_i += 1;
    }

    Ok(base)
}

impl Write for WriteTarget {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            WriteTarget::Stdout(file) => file.write(buf),
            WriteTarget::File(file) => file.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            WriteTarget::Stdout(file) => file.flush(),
            WriteTarget::File(file) => file.flush(),
        }
    }
}

impl FromStr for Destination {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            Ok(Destination::Stdout)
        } else {
            PathBuf::from_str(s).map(Destination::File)
        }
    }
}

//// clap requires this? I'm not sure why
//impl std::fmt::Display for Destination {
//fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//write!(f, "destination")
//}
//}
