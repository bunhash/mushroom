//! List.wz Decoder

use crate::error::{Error, Result};
use crate::io::{Decode, DummyDecryptor, WzRead, WzReader};
use crypto::Decryptor;
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::path::Path;
use std::slice::Iter;

pub struct Reader {
    strings: Vec<String>,
}

impl Reader {
    pub fn parse<S, D>(path: S, mut decryptor: D) -> Result<Self>
    where
        S: AsRef<Path>,
        D: Decryptor,
    {
        let mut strings = Vec::new();
        let mut reader = WzReader::new(0, 0, BufReader::new(File::open(path)?), DummyDecryptor);
        loop {
            let length = match u32::decode(&mut reader) {
                Ok(n) => n,
                Err(Error::Io(ErrorKind::UnexpectedEof)) => break,
                Err(e) => return Err(e),
            };
            strings.push(read_unicode_bytes(
                &mut reader,
                &mut decryptor,
                length as usize,
            )?);
            u16::decode(&mut reader)?; // NULL-byte
        }
        let last_index = strings.len() - 1;
        let mut last = String::from(&strings[last_index]);
        last.pop();
        last.push('g');
        strings[last_index] = last;
        Ok(Self { strings })
    }

    pub fn strings(&self) -> Iter<'_, String> {
        self.strings.iter()
    }
}

fn read_unicode_bytes<D>(
    reader: &mut WzReader<BufReader<File>, DummyDecryptor>,
    decryptor: &mut D,
    len: usize,
) -> Result<String>
where
    D: Decryptor,
{
    let mut buf = reader.read_vec(len * 2)?;
    decryptor.decrypt(&mut buf);
    Ok(String::from_utf16(
        buf.chunks(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect::<Vec<u16>>()
            .as_slice(),
    )?)
}
