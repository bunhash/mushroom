#[macro_export]
macro_rules! impl_primitive {
    ($lhs:ty, $rhs:ty ) => {
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

        impl Add<$lhs> for $lhs {
            type Output = $lhs;

            fn add(self, other: $lhs) -> Self::Output {
                <$lhs>::from(*self + *other)
            }
        }

        impl Sub<$lhs> for $lhs {
            type Output = $lhs;

            fn sub(self, other: $lhs) -> Self::Output {
                <$lhs>::from(*self - *other)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_conversions {
    ($lhs:ty, $expected:ty, $rhs:ty ) => {
        impl From<$rhs> for $lhs {
            fn from(other: $rhs) -> Self {
                Self(other as $expected)
            }
        }

        impl From<$lhs> for $rhs {
            fn from(other: $lhs) -> Self {
                other.0 as $rhs
            }
        }

        impl PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                PartialEq::eq(&Into::<$rhs>::into(*self), other)
            }
            #[inline]
            fn ne(&self, other: &$rhs) -> bool {
                PartialEq::ne(&Into::<$rhs>::into(*self), other)
            }
        }

        impl PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                PartialEq::eq(self, &Into::<$rhs>::into(*other))
            }
            #[inline]
            fn ne(&self, other: &$lhs) -> bool {
                PartialEq::ne(self, &Into::<$rhs>::into(*other))
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

        impl Add<$rhs> for $lhs {
            type Output = $lhs;

            fn add(self, other: $rhs) -> Self::Output {
                Self::Output::from(self.0 + other as $expected)
            }
        }

        impl Add<$lhs> for $rhs {
            type Output = $lhs;

            fn add(self, other: $lhs) -> Self::Output {
                Self::Output::from(self as $expected + other.0)
            }
        }

        impl Sub<$rhs> for $lhs {
            type Output = $lhs;

            fn sub(self, other: $rhs) -> Self::Output {
                Self::Output::from(self.0 - other as $expected)
            }
        }

        impl Sub<$lhs> for $rhs {
            type Output = $lhs;

            fn sub(self, other: $lhs) -> Self::Output {
                Self::Output::from(self as $expected - other.0)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_str {
    ( $type:ty ) => {
        impl From<String> for $type {
            fn from(other: String) -> Self {
                Self(other)
            }
        }

        impl From<&str> for $type {
            fn from(other: &str) -> Self {
                Self(String::from(other))
            }
        }

        impl Deref for $type {
            type Target = str;
            fn deref(&self) -> &Self::Target {
                self.0.as_str()
            }
        }

        impl DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.0.as_mut_str()
            }
        }

        impl AsRef<str> for $type {
            fn as_ref(&self) -> &str {
                self.deref().as_ref()
            }
        }

        impl AsMut<str> for $type {
            fn as_mut(&mut self) -> &mut str {
                self.deref_mut().as_mut()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_str_eq {
    ($lhs:ty, $( $rhs:ty ),+ ) => {
        $(
            #[allow(unused_lifetimes)]
            impl<'a, 'b> PartialEq<$rhs> for $lhs {
                #[inline]
                fn eq(&self, other: &$rhs) -> bool {
                    PartialEq::eq(&self.0[..], &other[..])
                }
                #[inline]
                fn ne(&self, other: &$rhs) -> bool {
                    PartialEq::ne(&self.0[..], &other[..])
                }
            }

            #[allow(unused_lifetimes)]
            impl<'a, 'b> PartialEq<$lhs> for $rhs {
                #[inline]
                fn eq(&self, other: &$lhs) -> bool {
                    PartialEq::eq(&self[..], &other.0[..])
                }
                #[inline]
                fn ne(&self, other: &$lhs) -> bool {
                    PartialEq::ne(&self[..], &other.0[..])
                }
            }
        )+
    };
}
