#[macro_export]
macro_rules! impl_num {
    ($lhs:ty, $rhs:ty ) => {
        impl From<$lhs> for $rhs {
            fn from(other: $lhs) -> $rhs {
                other.0 as $rhs
            }
        }

        impl Deref for $lhs {
            type Target = $rhs;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $lhs {
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

        impl Sub<$lhs> for $lhs {
            type Output = $lhs;

            fn sub(self, other: $lhs) -> Self::Output {
                <Self as From<$rhs>>::from(self.0.sub(*other))
            }
        }

        impl Mul<$lhs> for $lhs {
            type Output = $lhs;

            fn mul(self, other: $lhs) -> Self::Output {
                <Self as From<$rhs>>::from(self.0.mul(*other))
            }
        }

        impl Div<$lhs> for $lhs {
            type Output = $lhs;

            fn div(self, other: $lhs) -> Self::Output {
                <Self as From<$rhs>>::from(self.0.div(*other))
            }
        }

        impl Add<$lhs> for $lhs {
            type Output = $lhs;

            fn add(self, other: $lhs) -> Self::Output {
                <Self as From<$rhs>>::from(self.0.add(*other))
            }
        }

        impl Rem<$lhs> for $lhs {
            type Output = $lhs;

            fn rem(self, other: $lhs) -> Self::Output {
                <Self as From<$rhs>>::from(self.0.rem(*other))
            }
        }
    };
}
#[macro_export]
macro_rules! impl_from {
    ($lhs:ty, $rhs:ty, $expected:ty ) => {
        impl From<$rhs> for $lhs {
            fn from(other: $rhs) -> $lhs {
                Self(other as $expected)
            }
        }
    };
}
