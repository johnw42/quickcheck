use rand::RngExt;
use std::ops::{Bound, RangeBounds, RangeInclusive, RangeToInclusive};

use crate::Gen;

/// Implementation of [`Gen::choose_in_range`].
pub(crate) fn rand_in_range<T, R>(g: &mut Gen, range: R) -> T
where
    T: IntRangeBound + Copy,
    T::Unsigned: rand::distr::uniform::SampleUniform,
    R: RangeBounds<T>,
    RangeToInclusive<T::Unsigned>:
        rand::distr::uniform::SampleRange<T::Unsigned>,
{
    let range = to_range_inclusive(range);
    T::from_integer(range.start().strict_add_unsigned(
        g.rng.random_range(to_unsigned_range_to_inclusive(range)),
    ))
}

/// Converts any of Rust's built-in integer range types to a `RangeInclusive`.
fn to_range_inclusive<T, R>(range: R) -> RangeInclusive<T::Integer>
where
    T: IntRangeBound + Copy,
    R: RangeBounds<T>,
    RangeToInclusive<T::Unsigned>:
        rand::distr::uniform::SampleRange<T::Unsigned>,
{
    let start = match range.start_bound() {
        Bound::Included(&start) => start.to_integer(),
        Bound::Excluded(&start) => start.add(T::ONE),
        Bound::Unbounded => T::MIN,
    };

    let end = match range.end_bound() {
        Bound::Included(&end) => end.to_integer(),
        Bound::Excluded(&end) => end.sub(T::ONE),
        Bound::Unbounded => T::MAX,
    };

    start..=end
}

/// Converts any of Rust's built-in integer range types to an inclusive `RangeTo`.
fn to_unsigned_range_to_inclusive<T, R>(
    range: R,
) -> RangeToInclusive<T::Unsigned>
where
    T: IntRangeBound + Copy,
    R: RangeBounds<T>,
    RangeToInclusive<T::Unsigned>:
        rand::distr::uniform::SampleRange<T::Unsigned>,
{
    let i = to_range_inclusive(range);
    RangeToInclusive { end: i.end().abs_diff(i.start().to_integer()) }
}

/// A trait implemented by all of Rust's built-in integer types plus `char`,
/// used for generating random integers in a range.
pub trait IntRangeBound: Copy + Sized {
    // The unsigned version of this integer type.
    #[doc(hidden)]
    type Unsigned: IntRangeBound<Unsigned = Self::Unsigned>;

    // An integer that can hold all values of this type.
    #[doc(hidden)]
    type Integer: IntRangeBound<Unsigned = Self::Unsigned>
        + TryInto<Self>
        + From<Self>;

    #[doc(hidden)]
    const ONE: Self::Integer;

    #[doc(hidden)]
    const MIN: Self::Integer;

    #[doc(hidden)]
    const MAX: Self::Integer;

    #[doc(hidden)]
    fn add(self, other: Self::Integer) -> Self::Integer;

    #[doc(hidden)]
    fn sub(self, other: Self::Integer) -> Self::Integer;

    #[doc(hidden)]
    fn abs_diff(self, other: Self::Integer) -> Self::Unsigned;

    #[doc(hidden)]
    fn strict_add_unsigned(self, other: Self::Unsigned) -> Self;

    #[doc(hidden)]
    fn to_integer(self) -> Self::Integer {
        Self::Integer::from(self)
    }

    #[doc(hidden)]
    fn from_integer(i: Self::Integer) -> Self {
        i.try_into().ok().expect("value out of range")
    }
}

macro_rules! impl_int_range_bound {
    ($($ty:ty, $uty:ty ;)*) => {
        $(
            impl IntRangeBound for $ty {
                type Unsigned = $uty;
                type Integer = Self;
                const ONE: Self = 1;
                const MIN: Self = Self::MIN;
                const MAX: Self = Self::MAX;

                fn add(self, other: Self::Integer) -> Self::Integer {
                    self + other
                }

                fn sub(self, other: Self::Integer) -> Self::Integer {
                    self - other
                }

                fn abs_diff(self, other: Self) -> Self::Unsigned {
                    self.abs_diff(other)
                }

                fn strict_add_unsigned(self, other: Self::Unsigned) -> Self {
                    self.strict_add_unsigned(other)
                }
            }

            impl IntRangeBound for $uty {
                type Unsigned = $uty;
                type Integer = Self;
                const ONE: Self = 1;
                const MIN: Self = Self::MIN;
                const MAX: Self = Self::MAX;

                fn add(self, other: Self::Integer) -> Self::Integer {
                    self + other
                }

                fn sub(self, other: Self::Integer) -> Self::Integer {
                    self - other
                }

                fn abs_diff(self, other: Self) -> Self::Unsigned {
                    self.abs_diff(other)
                }

                fn strict_add_unsigned(self, other: Self::Unsigned) -> Self {
                    self.strict_add(other)
                }
            }
        )*
    };
}

impl_int_range_bound! {
    i8, u8;
    i16, u16;
    i32, u32;
    i64, u64;
    i128, u128;
    isize, usize;
}

impl IntRangeBound for char {
    type Unsigned = u32;
    type Integer = u32;
    const ONE: u32 = 1;
    const MIN: u32 = char::MIN as u32;
    const MAX: u32 = char::MAX as u32;

    fn add(self, other: u32) -> u32 {
        u32::from(self) + other
    }

    fn sub(self, other: u32) -> u32 {
        u32::from(self) - other
    }

    fn abs_diff(self, other: u32) -> u32 {
        u32::from(self).abs_diff(other)
    }

    fn strict_add_unsigned(self, other: u32) -> Self {
        char::from_u32(u32::from(self) + other).unwrap()
    }
}
