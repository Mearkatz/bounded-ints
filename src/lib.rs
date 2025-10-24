// Re-exports
pub use gt::*;
pub use gte::*;
pub use in_range::*;
pub use in_range_inclusive::*;
pub use lt::*;
pub use lte::*;

/// Creates a new `InRange` with consts for the MIN and MAX value of an instance.
macro_rules! in_range {
    ($t: ty, $name: ident) => {
        /// A primitive integer contained in `MIN..MAX`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, Getters)]
        pub struct $name<const MIN: $t, const MAX: $t> {
            value: $t,
        }

        impl<const MIN: $t, const MAX: $t> $name<MIN, MAX> {
            /// Creates a new `InRange` from `value` if it's in the range `MIN..MAX`
            #[must_use]
            pub const fn new(value: $t) -> Option<Self> {
                then::then!((value >= MIN) && (value < MAX), Self { value })
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
                then::then!((value >= MIN) && (value <= MAX), Self { value })
            }

            /**
            Creates a new `InRange` from `value`.

            # Safety
            `value` must be known to be in the range `MIN..=MAX`
            */
            #[must_use]
            pub const unsafe fn new_unchecked(value: $t) -> Self {
                Self { value }
            }
        }
    };
}

/// Creates a new `Lt` with a const for the MAX value of an instance.
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
                then::then!(value < MAX, Self { value })
            }

            /**
            Creates a new `Lt` from `value`.

            # Safety
            `value < MAX` must be known to be true.
            */
            #[must_use]
            pub const unsafe fn new_unchecked(value: $t) -> Self {
                Self { value }
            }
        }
    };
}

/// Creates a new `Lte` with a const for the MAX value of an instance.
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
                then::then!(value <= MAX, Self { value })
            }

            /**
            Creates a new `Lt` from `value`.

            # Safety
            `value <= MAX` must be known to be true.
            */
            #[must_use]
            pub const unsafe fn new_unchecked(value: $t) -> Self {
                Self { value }
            }
        }
    };
}

/// Creates a new `Gt` with a const for the MIN value of an instance.
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
                then::then!(value > MIN, Self { value })
            }

            /**
            Creates a new `Gt` from `value`.

            # Safety
            `value > MIN` must be known to be true.
            */
            #[must_use]
            pub const unsafe fn new_unchecked(value: $t) -> Self {
                Self { value }
            }
        }
    };
}

/// Creates a new `Gte` with a const for the MIN value of an instance.
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
                then::then!(value >= MIN, Self { value })
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

// These modules only exist for easier navigation

pub mod in_range {

    use derive_getters::Getters;
    use derive_more::Display;

    in_range!(u8, InRangeU8);
    in_range!(u16, InRangeU16);
    in_range!(u32, InRangeU32);
    in_range!(u64, InRangeU64);
    in_range!(u128, InRangeU128);
    in_range!(usize, InRangeUsize);
    in_range!(i8, InRangeI8);
    in_range!(i16, InRangeI16);
    in_range!(i32, InRangeI32);
    in_range!(i64, InRangeI64);
    in_range!(i128, InRangeI128);
    in_range!(isize, InRangeIsize);
}

pub mod in_range_inclusive {

    use derive_getters::Getters;
    use derive_more::Display;

    in_range_inclusive!(u8, InRangeInclusiveU8);
    in_range_inclusive!(u16, InRangeInclusiveU16);
    in_range_inclusive!(u32, InRangeInclusiveU32);
    in_range_inclusive!(u64, InRangeInclusiveU64);
    in_range_inclusive!(u128, InRangeInclusiveU128);
    in_range_inclusive!(usize, InRangeInclusiveUsize);
    in_range_inclusive!(i8, InRangeInclusiveI8);
    in_range_inclusive!(i16, InRangeInclusiveI16);
    in_range_inclusive!(i32, InRangeInclusiveI32);
    in_range_inclusive!(i64, InRangeInclusiveI64);
    in_range_inclusive!(i128, InRangeInclusiveI128);
    in_range_inclusive!(isize, InRangeInclusiveIsize);
}

pub mod lt {

    use derive_getters::Getters;
    use derive_more::Display;
    lt!(u8, LtU8);
    lt!(u16, LtU16);
    lt!(u32, LtU32);
    lt!(u64, LtU64);
    lt!(u128, LtU128);
    lt!(usize, LtUsize);
    lt!(i8, LtI8);
    lt!(i16, LtI16);
    lt!(i32, LtI32);
    lt!(i64, LtI64);
    lt!(i128, LtI128);
    lt!(isize, LtIsize);
}

pub mod lte {

    use derive_getters::Getters;
    use derive_more::Display;
    lte!(u8, LteU8);
    lte!(u16, LteU16);
    lte!(u32, LteU32);
    lte!(u64, LteU64);
    lte!(u128, LteU128);
    lte!(usize, LteUsize);
    lte!(i8, LteI8);
    lte!(i16, LteI16);
    lte!(i32, LteI32);
    lte!(i64, LteI64);
    lte!(i128, LteI128);
    lte!(isize, LteIsize);
}

pub mod gt {
    use derive_getters::Getters;
    use derive_more::Display;
    gt!(u8, GtU8);
    gt!(u16, GtU16);
    gt!(u32, GtU32);
    gt!(u64, GtU64);
    gt!(u128, GtU128);
    gt!(usize, GtUsize);
    gt!(i8, GtI8);
    gt!(i16, GtI16);
    gt!(i32, GtI32);
    gt!(i64, GtI64);
    gt!(i128, GtI128);
    gt!(isize, GtIsize);
}

pub mod gte {
    use derive_getters::Getters;
    use derive_more::Display;
    gte!(u8, GteU8);
    gte!(u16, GteU16);
    gte!(u32, GteU32);
    gte!(u64, GteU64);
    gte!(u128, GteU128);
    gte!(usize, GteUsize);
    gte!(i8, GteI8);
    gte!(i16, GteI16);
    gte!(i32, GteI32);
    gte!(i64, GteI64);
    gte!(i128, GteI128);
    gte!(isize, GteIsize);
}
