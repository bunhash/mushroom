//! Parsing of WZ images

use crate::{utils, Key};
use crypto::{KeyStream, GMS_IV, KMS_IV, TRIMMED_KEY};
use std::{fmt::Debug, io, io::Write, path::PathBuf};
use wz::{
    error::Result,
    image::Reader,
    io::{DummyDecryptor, WzRead},
    map::Cursor,
    types::{Property, VerboseDebug},
};

pub(crate) fn do_debug(
    path: &PathBuf,
    directory: &Option<String>,
    verbose: bool,
    key: Key,
) -> Result<()> {
    let name = utils::file_name(path)?;
    let result = match key {
        Key::Gms => debug(
            name,
            Reader::open(path, KeyStream::new(&TRIMMED_KEY, &GMS_IV))?,
            directory,
            verbose,
        ),
        Key::Kms => debug(
            name,
            Reader::open(path, KeyStream::new(&TRIMMED_KEY, &KMS_IV))?,
            directory,
            verbose,
        ),
        Key::None => debug(
            name,
            Reader::open(path, DummyDecryptor)?,
            directory,
            verbose,
        ),
    };
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{} failed: {:?}", path.display(), e);
            Ok(())
        }
    }
}

fn debug_print<'a>(
    f: &mut dyn Write,
    cursor: &Cursor<'a, Property>,
    verbose: bool,
) -> io::Result<()> {
    if verbose {
        VerboseDebug::debug(&cursor.name(), f)?;
    } else {
        write!(f, "{:?}", cursor.name())?;
    }
    write!(f, " : ")?;
    if verbose {
        VerboseDebug::debug(cursor.get(), f)?;
    } else {
        write!(f, "{:?}", cursor.get())?;
    }
    writeln!(f, "")
}

fn debug_recursive<'a>(
    prelude: &str,
    space: &str,
    cursor: &mut Cursor<'a, Property>,
    verbose: bool,
) -> Result<()> {
    let mut lock = io::stdout().lock();
    write!(lock, "{}", prelude)?;
    debug_print(&mut lock, &cursor, verbose)?;
    let mut num_children = cursor.children().count();
    if num_children > 0 {
        cursor.first_child()?;
        loop {
            if num_children <= 1 {
                debug_recursive(
                    &format!("{}`-- ", space),
                    &format!("{}    ", space),
                    cursor,
                    verbose,
                )?;
                break;
            } else {
                debug_recursive(
                    &format!("{}|-- ", space),
                    &format!("{}|   ", space),
                    cursor,
                    verbose,
                )?;
            }
            num_children -= 1;
            cursor.next_sibling()?;
        }
        cursor.parent()?;
    }
    Ok(())
}

fn debug<R>(
    name: &str,
    mut reader: Reader<R>,
    directory: &Option<String>,
    verbose: bool,
) -> Result<()>
where
    R: WzRead,
{
    let map = reader.map(name)?;
    let mut cursor = match directory {
        // Find the optional directory
        Some(ref path) => map.cursor_at(path)?,
        // Get the root
        None => map.cursor(),
    };

    let mut num_children = cursor.children().count();
    if num_children > 0 {
        Ok(debug_recursive("|-- ", "|   ", &mut cursor, verbose)?)
    } else {
        Ok(debug_recursive("`-- ", "", &mut cursor, verbose)?)
    }
}
