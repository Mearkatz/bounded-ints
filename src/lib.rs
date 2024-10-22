mod macros;

/// definitions for `InRangeU*` and `InRangeI*` types
pub mod in_range {
    use crate::in_range;
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

/// definitions for `InRangeInclusiveU*` and `InRangeInclusiveI*` types
pub mod in_range_inclusive {
    use crate::in_range_inclusive;
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

/// definitions for `LTU*` and `LTI*` types
pub mod lt {
    use crate::lt;
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
    use crate::lte;
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
    use crate::gt;
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
    use crate::gte;
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
