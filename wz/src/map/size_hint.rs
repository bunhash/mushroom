//! Size hints for contents

use crate::types::WzInt;

pub trait SizeHint {
    fn size_hint(&self) -> WzInt;
}
