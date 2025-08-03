//! Size hint for encoded objects

/// Trait for calculated the size of the encoded object
pub trait SizeHint {
    /// Return the size of the encoded object in bytes. WZ files cannot be larger than `u32::MAX`
    /// but a `u64` is returned here so the error can be thrown later.
    fn size_hint(&self) -> u64;
}

impl SizeHint for i8 {
    fn size_hint(&self) -> u64 {
        1
    }
}

impl SizeHint for i16 {
    fn size_hint(&self) -> u64 {
        2
    }
}

impl SizeHint for i32 {
    fn size_hint(&self) -> u64 {
        4
    }
}

impl SizeHint for i64 {
    fn size_hint(&self) -> u64 {
        8
    }
}

impl SizeHint for u8 {
    fn size_hint(&self) -> u64 {
        1
    }
}

impl SizeHint for u16 {
    fn size_hint(&self) -> u64 {
        2
    }
}

impl SizeHint for u32 {
    fn size_hint(&self) -> u64 {
        4
    }
}

impl SizeHint for u64 {
    fn size_hint(&self) -> u64 {
        8
    }
}

impl SizeHint for f32 {
    fn size_hint(&self) -> u64 {
        match self {
            0f32 => 1,
            _ => 5,
        }
    }
}

impl SizeHint for f64 {
    fn size_hint(&self) -> u64 {
        8
    }
}

impl SizeHint for &str {
    fn size_hint(&self) -> u64 {
        let length = self.len() as u64;
        if length == 0 {
            return 1;
        }
        if self.is_ascii() {
            if length > (i8::MAX as u64) {
                5 + length
            } else {
                1 + length
            }
        } else {
            if length >= (i8::MAX as u64) {
                5 + (length * 2)
            } else {
                1 + (length * 2)
            }
        }
    }
}

impl SizeHint for String {
    fn size_hint(&self) -> u64 {
        self.as_str().size_hint()
    }
}
