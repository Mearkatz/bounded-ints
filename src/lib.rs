#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum BoundedError {
    /// The value provided was less than the minimum.
    LessThanMinimum,

    /// The value provided was greater than the minimum.
    GreaterThanMaximum,
}

/// Creates a new `Bounded_*` with consts for the minimum and maximum value of an instance.
macro_rules! bounded_impl {
    ($name: ident, $t: ty) => {
        /// An integer which is known to exist in the range `MIN`..`MAX`
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name<const MIN: $t, const MAX: $t> {
            #[allow(dead_code)]
            value: $t,
        }

        impl<const MIN: $t, const MAX: $t> $name<MIN, MAX> {
            /// Creates a new `Self` from `value`.
            /// # Errors
            /// - If `value` < `minimum` this returns `BoundedError::LessThanMinimum`.
            /// - If `value` > `maximum` this returns `BoundedError::LessThanMinimum`.
            pub const fn new(value: $t) -> Result<Self, BoundedError> {
                if value < MIN {
                    Err(BoundedError::LessThanMinimum)
                } else if value > MAX {
                    Err(BoundedError::GreaterThanMaximum)
                } else {
                    Ok(unsafe { Self::new_unchecked(value) })
                }
            }

            /// Creates a new `Self` from `value`.
            ///
            /// # Safety
            /// `value` must be known to be in the range `MIN..MAX`
            #[must_use]
            pub const unsafe fn new_unchecked(value: $t) -> Self {
                Self { value }
            }

            #[must_use]
            pub const fn get(self) -> $t {
                self.value
            }
        }
    };
}

bounded_impl!(BoundedU8, u8);
bounded_impl!(BoundedU16, u16);
bounded_impl!(BoundedU32, u32);
bounded_impl!(BoundedU64, u64);
bounded_impl!(BoundedU128, u128);
bounded_impl!(BoundedI8, i8);
bounded_impl!(BoundedI16, i16);
bounded_impl!(BoundedI32, i32);
bounded_impl!(BoundedI64, i64);
bounded_impl!(BoundedI128, i128);

/// An integer which is known to exist in the range `self.minimum`..`self.maximum`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bounded<T> {
    minimum: T,
    maximum: T,
    value: T,
}

impl<T> Bounded<T>
where
    T: PartialOrd + Ord,
{
    /// Creates a new `Self` from `value`.
    /// # Errors
    /// - If `value` < `minimum` this returns `BoundedError::LessThanMinimum`.
    /// - If `value` > `maximum` this returns `BoundedError::LessThanMinimum`.
    pub fn new(value: T, minimum: T, maximum: T) -> Result<Self, BoundedError> {
        if value < minimum {
            Err(BoundedError::LessThanMinimum)
        } else if value > maximum {
            Err(BoundedError::GreaterThanMaximum)
        } else {
            Ok(unsafe { Self::new_unchecked(value, minimum, maximum) })
        }
    }

    /// Creates a new `Self` from `value`.
    ///
    /// # Safety
    /// `value` must be known to be in the range `MIN..MAX`
    #[must_use]
    pub const unsafe fn new_unchecked(value: T, minimum: T, maximum: T) -> Self {
        Self {
            minimum,
            maximum,
            value,
        }
    }
}
