//! Size hints for contents

use crate::types::WzInt;

/// Trait used to keep track of the content's total size + metadata
pub trait SizeHint {
    fn size_hint(&self) -> WzInt;
}
