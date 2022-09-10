use crate::{WzReader, WzResult};

pub trait WzNode {
    fn is_dir(&self) -> bool;
    fn name(&self) -> &str;
    fn size(&self) -> Option<u64>;
    fn offset(&self) -> Option<u64>;
    fn load(&mut self, reader: &mut WzReader, abs_pos: u64) -> WzResult<()>;
}
