//! Size hints for contents

use crate::types::WzInt;

pub trait SizeHint {
    fn data_size(&self) -> WzInt;

    #[allow(unused_variables)]
    fn header_size(&self, num_children: usize) -> WzInt {
        WzInt::from(0)
    }

    #[allow(unused_variables)]
    fn metadata_size(&self, name: &str) -> WzInt {
        WzInt::from(0)
    }
}
