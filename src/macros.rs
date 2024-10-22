/// Creates a new `InRange` with consts for the MIN and MAX value of an instance.
#[macro_export]
macro_rules! in_range {
    ($t: ty, $name: ident) => {
        /// A primitive integer which is known to exist in the range `MIN..MAX`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, Getters)]
        pub struct $name<const MIN: $t, const MAX: $t> {
            value: $t,
        }

        impl<const MIN: $t, const MAX: $t> $name<MIN, MAX> {
            /// Creates a new `InRange` from `value` if it's in the range `MIN..MAX`
            #[must_use]
            pub const fn new(value: $t) -> Option<Self> {
                if (value >= MIN) && (value < MAX) {
                    Some(Self { value })
                } else {
                    None
                }
            }

            /// Creates a new `InRange` from `value`.
            ///
            /// # Safety
            /// `value` must be known to be in the range `MIN..MAX`
            #[must_use]
            pub const unsafe fn new_unchecked(value: $t) -> Self {
                Self { value }
            }
        }
    };
}

/// Creates a new `InRangeInclusive` with consts for the MIN and MAX value of an instance.
#[macro_export]
macro_rules! in_range_inclusive {
    ($t: ty, $name: ident) => {
        /// An integer which is known to exist in the range `MIN..=MAX`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, Getters)]
        pub struct $name<const MIN: $t, const MAX: $t> {
            value: $t,
        }

        impl<const MIN: $t, const MAX: $t> $name<MIN, MAX> {
            /// Creates a new `InRange` from `value` if it's in the range `MIN..=MAX`
            #[must_use]
            pub const fn new(value: $t) -> Option<Self> {
                if (value >= MIN) && (value <= MAX) {
                    Some(Self { value })
                } else {
                    None
                }
            }

            /// Creates a new `InRange` from `value`.
            ///
            /// # Safety
            /// `value` must be known to be in the range `MIN..=MAX`
            #[must_use]
            pub const unsafe fn new_unchecked(value: $t) -> Self {
                Self { value }
            }
        }
    };
}

/// Creates a new `Lt` with a const for the MAX value of an instance.
#[macro_export]
macro_rules! lt {
    ($t: ty, $name: ident) => {
        /// An integer `N` where `N < MAX` is known to be true.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, Getters)]
        pub struct $name<const MAX: $t> {
            value: $t,
        }

        impl<const MAX: $t> $name<MAX> {
            /// Creates a new `Lt` from `value` if it's < `MAX`
            #[must_use]
            pub const fn new(value: $t) -> Option<Self> {
                if value < MAX {
                    Some(Self { value })
                } else {
                    None
                }
            }

            /// Creates a new `Lt` from `value`.
            ///
            /// # Safety
            /// `value < MAX` must be known to be true.
            #[must_use]
            pub const unsafe fn new_unchecked(value: $t) -> Self {
                Self { value }
            }
        }
    };
}

/// Creates a new `Lte` with a const for the MAX value of an instance.
#[macro_export]
macro_rules! lte {
    ($t: ty, $name: ident) => {
        /// An integer `N` where `N <= MAX` is known to be true.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, Getters)]
        pub struct $name<const MAX: $t> {
            value: $t,
        }

        impl<const MAX: $t> $name<MAX> {
            /// Creates a new `Lt` from `value` if it's <= `MAX`
            #[must_use]
            pub const fn new(value: $t) -> Option<Self> {
                if value <= MAX {
                    Some(Self { value })
                } else {
                    None
                }
            }

            /// Creates a new `Lt` from `value`.
            ///
            /// # Safety
            /// `value <= MAX` must be known to be true.
            #[must_use]
            pub const unsafe fn new_unchecked(value: $t) -> Self {
                Self { value }
            }
        }
    };
}

/// Creates a new `Gt` with a const for the MIN value of an instance.
#[macro_export]
macro_rules! gt {
    ($t: ty, $name: ident) => {
        /// An integer `N` where `N > MIN` is known to be true.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, Getters)]
        pub struct $name<const MIN: $t> {
            value: $t,
        }

        impl<const MIN: $t> $name<MIN> {
            /// Creates a new `Gt` from `value` if it's > `MIN`
            #[must_use]
            pub const fn new(value: $t) -> Option<Self> {
                if value > MIN {
                    Some(Self { value })
                } else {
                    None
                }
            }

            /// Creates a new `Gt` from `value`.
            ///
            /// # Safety
            /// `value > MIN` must be known to be true.
            #[must_use]
            pub const unsafe fn new_unchecked(value: $t) -> Self {
                Self { value }
            }
        }
    };
}

/// Creates a new `Gte` with a const for the MIN value of an instance.
#[macro_export]
macro_rules! gte {
    ($t: ty, $name: ident) => {
        /// An integer `N` where `N >= MIN` is known to be true.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, Getters)]
        pub struct $name<const MIN: $t> {
            value: $t,
        }

        impl<const MIN: $t> $name<MIN> {
            /// Creates a new `Gte` from `value` if it's >= `MIN`
            #[must_use]
            pub const fn new(value: $t) -> Option<Self> {
                if value >= MIN {
                    Some(Self { value })
                } else {
                    None
                }
            }

            /// Creates a new `Gte` from `value`.
            ///
            /// # Safety
            /// `value >= MIN` must be known to be true.
            #[must_use]
            pub const unsafe fn new_unchecked(value: $t) -> Self {
                Self { value }
            }
        }
    };
}
