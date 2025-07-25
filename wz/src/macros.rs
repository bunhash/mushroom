macro_rules! impl_num {
    ($lhs:ty, $rhs:ty ) => {
        impl From<$lhs> for $rhs {
            fn from(other: $lhs) -> $rhs {
                other.0 as $rhs
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
                PartialEq::eq(&Into::<$rhs>::into(*self), other)
            }
        }

        impl PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                PartialEq::eq(self, &Into::<$rhs>::into(*other))
            }
        }

        impl PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<core::cmp::Ordering> {
                PartialOrd::partial_cmp(&Into::<$rhs>::into(*self), other)
            }
        }

        impl PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<core::cmp::Ordering> {
                PartialOrd::partial_cmp(self, &Into::<$rhs>::into(*other))
            }
        }

        impl core::ops::Sub<$lhs> for $lhs {
            type Output = $lhs;

            fn sub(self, other: $lhs) -> Self::Output {
                <Self as From<$rhs>>::from(self.0.sub(*other))
            }
        }

        impl core::ops::Mul<$lhs> for $lhs {
            type Output = $lhs;

            fn mul(self, other: $lhs) -> Self::Output {
                <Self as From<$rhs>>::from(self.0.mul(*other))
            }
        }

        impl core::ops::Div<$lhs> for $lhs {
            type Output = $lhs;

            fn div(self, other: $lhs) -> Self::Output {
                <Self as From<$rhs>>::from(self.0.div(*other))
            }
        }

        impl core::ops::Add<$lhs> for $lhs {
            type Output = $lhs;

            fn add(self, other: $lhs) -> Self::Output {
                <Self as From<$rhs>>::from(self.0.add(*other))
            }
        }

        impl core::ops::Rem<$lhs> for $lhs {
            type Output = $lhs;

            fn rem(self, other: $lhs) -> Self::Output {
                <Self as From<$rhs>>::from(self.0.rem(*other))
            }
        }
    };
}

pub(crate) use impl_num;

macro_rules! impl_from {
    ($lhs:ty, $rhs:ty, $expected:ty ) => {
        impl From<$rhs> for $lhs {
            fn from(other: $rhs) -> $lhs {
                Self(other as $expected)
            }
        }
    };
}

pub(crate) use impl_from;
