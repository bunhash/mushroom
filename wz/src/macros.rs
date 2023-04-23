#[macro_export]
macro_rules! impl_primitive {
    ($lhs:ty, $rhs:ty ) => {
        impl From<$rhs> for $lhs {
            fn from(other: $rhs) -> Self {
                Self(other)
            }
        }

        impl core::ops::Deref for $lhs {
            type Target = $rhs;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for $lhs {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                PartialEq::eq(&self.0, other)
            }
            #[inline]
            fn ne(&self, other: &$rhs) -> bool {
                PartialEq::ne(&self.0, other)
            }
        }

        impl PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                PartialEq::eq(self, &other.0)
            }
            #[inline]
            fn ne(&self, other: &$lhs) -> bool {
                PartialEq::ne(self, &other.0)
            }
        }

        impl PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<core::cmp::Ordering> {
                PartialOrd::partial_cmp(&self.0, other)
            }
        }

        impl PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<core::cmp::Ordering> {
                PartialOrd::partial_cmp(self, &other.0)
            }
        }
    };
}
