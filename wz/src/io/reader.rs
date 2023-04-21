//! File Reader

use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
};

pub type WzReader = BufReader<File>;

impl WzReader {
    pub fn read_slice(&mut self, length: usize) -> &[u8] {
    }
}
