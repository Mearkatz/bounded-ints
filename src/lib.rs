/// Creates a new `Bounded_*` with consts for the minimum and maximum value of an instance.
macro_rules! bounded_impl {
    ($name: ident, $t: ty) => {
        /// An integer which is known to exist in the range `MIN`..`MAX`
        #[derive(Debug, Clone, Copy)]
        pub struct $name<const MIN: $t, const MAX: $t> {
            #[allow(dead_code)]
            value: $t,
        }

        impl<const MIN: $t, const MAX: $t> $name<MIN, MAX> {
            /// Creates a new `Self` from `value`.
            /// If `value` < `MIN` || `value` > `MAX`, this returns `None`.
            #[must_use]
            pub const fn new(value: $t) -> Option<Self> {
                if (MIN <= value) && (MAX >= value) {
                    Some(unsafe { Self::new_unchecked(value) })
                } else {
                    None
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
