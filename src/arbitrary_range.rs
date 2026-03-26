use crate::{Arbitrary, Gen};

macro_rules! in_range_types {
    { $( $int_type:ty, $exclusive:ident, $inclusive:ident ; )* }=> {
        $(
            #[doc = concat!("An arbitrary ", stringify!($int_type), " in the range `", stringify!(START), "..", stringify!(END), "`.")]
            #[derive(Clone, Debug)]
            pub struct $exclusive<const START: $int_type, const END: $int_type>(pub $int_type);

            impl<const START: $int_type, const END: $int_type> Arbitrary for $exclusive<START, END> {

                fn arbitrary(g: &mut Gen) -> Self {
                    assert!(START < END, "START must be less than END");

                    // See comments on the inclusive case below.
                    if START + 1 == END {
                        return $exclusive(START);
                    }
                    if START == $int_type::MIN && END == $int_type::MAX {
                        return $exclusive($int_type::arbitrary(g));
                    }
                    let size = END.abs_diff(START);
                    let min_to_keep = ($int_type::MAX) % size + 1;
                    let position = loop {
                        let generated = $int_type::arbitrary(g);
                        if generated <= min_to_keep {
                            break generated % size;
                        }
                    };
                    $exclusive(position.strict_add(START))                }
            }

            #[doc = concat!("An arbitrary ", stringify!($int_type), " in the range `", stringify!(START), "..=", stringify!(END), "`.")]
            #[derive(Clone, Debug)]
            pub struct $inclusive<const START: $int_type, const END: $int_type>(pub $int_type);

            impl<const START: $int_type, const END: $int_type> Arbitrary for $inclusive<START, END> {

                fn arbitrary(g: &mut Gen) -> Self {
                    assert!(START <= END, "START must be less than or equal to END");

                    if START == END {
                        // If the range is a single value, then we can just return that value.
                        return $inclusive(START);
                    }
                    if START == <$int_type>::MIN && END == <$int_type>::MAX {
                        // If the range is the entire range of the type, then we
                        // can just generate an arbitrary value.  This case is
                        // important because it prevents an overflow when
                        // calculating a `size` below.
                        return $inclusive(<$int_type>::arbitrary(g));
                    }

                    // To create a value in the range START..END, we first find
                    // a position in 0..(END-START). This minimizes the number
                    // of cases to consider. The range checks above ensure that
                    // there is no overflow.
                    let size = END.abs_diff(START) + 1;

                    // At this point a naive solution would be to let position =
                    // $int_type::arbitrary(g) % size, but this can lead to bias
                    // if size does not divide the range of $int_type. For
                    // example, with a range of 0..=250u8, the five numbers in
                    // 0..=4 would be generated twice as often as the numbers in
                    // 5..=250. To avoid this bias, we reject values from
                    // u8::arbitrary(g) that are in the range 0..4, resulting in
                    // a uniform distribution of generated values % size.
                    let min_to_keep = ($int_type::MAX) % size + 1;
                    let position = loop {
                        let generated = $int_type::arbitrary(g);
                        if generated <= min_to_keep {
                            break generated % size;
                        }
                    };

                    // Now we have the position in range 0..(END-START), so we
                    // can add it to START to get a value in the desired range.
                    $inclusive(position.strict_add(START))
                }
            }
        )*
    };
}

in_range_types! {
    i8, I8InRange, I8InRangeInclusive;
    i16, I16InRange, I16InRangeInclusive;
    i32, I32InRange, I32InRangeInclusive;
    i64, I64InRange, I64InRangeInclusive;
    i128, I128InRange, I128InRangeInclusive;
    u8, U8InRange, U8InRangeInclusive;
    u16, U16InRange, U16InRangeInclusive;
    u32, U32InRange, U32InRangeInclusive;
    u64, U64InRange, U64InRangeInclusive;
    u128, U128InRange, U128InRangeInclusive;
}

// #[doc = concat!("An arbitrary ", stringify!(char), " in the range `", stringify!(START), "..", stringify!(END), "`.")]
// #[derive(Clone, Debug)]
// pub struct CharInRange<const START: char, const END: char>(pub char);

// impl<const START: char, const END: char> Arbitrary
//     for CharInRange<START, END>
// {
//     fn arbitrary(g: &mut Gen) -> Self {
//         CharInRange(
//             U32InRange::<{ START as u32 }, { END as u32 }>::arbitrary(g)
//                 .0
//                 .into(),
//         )
//     }
// }

// #[doc = concat!("An arbitrary ", stringify!(char), " in the range `", stringify!(START), "..=", stringify!(END), "`.")]
// #[derive(Clone, Debug)]
// pub struct CharInRangeExclusive<const START: char, const END: char>(pub char);

// impl<const START: char, const END: char> Arbitrary
//     for CharInRangeExclusive<START, END>
// {
//     fn arbitrary(g: &mut Gen) -> Self {
//         assert!(START <= END, "START must be less than or equal to END");

//         if START == END {
//             // If the range is a single value, then we can just return that value.
//             return CharInRangeExclusive(START);
//         }
//         if START == <char>::MIN && END == <char>::MAX {
//             // If the range is the entire range of the type, then we
//             // can just generate an arbitrary value.  This case is
//             // important because it prevents an overflow when
//             // calculating a `size` below.
//             return CharInRangeExclusive(<char>::arbitrary(g));
//         }

//         // To create a value in the range START..END, we first find
//         // a position in 0..(END-START). This minimizes the number
//         // of cases to consider. The range checks above ensure that
//         // there is no overflow.
//         let size = END.abs_diff(START) + 1;

//         // At this point a naive solution would be to let position =
//         // char::arbitrary(g) % size, but this can lead to bias
//         // if size does not divide the range of char. For
//         // example, with a range of 0..=250u8, the five numbers in
//         // 0..=4 would be generated twice as often as the numbers in
//         // 5..=250. To avoid this bias, we reject values from
//         // u8::arbitrary(g) that are in the range 0..4, resulting in
//         // a uniform distribution of generated values % size.
//         let min_to_keep = (char::MAX) % size + 1;
//         let position = loop {
//             let generated = char::arbitrary(g);
//             if generated <= min_to_keep {
//                 break generated % size;
//             }
//         };

//         // Now we have the position in range 0..(END-START), so we
//         // can add it to START to get a value in the desired range.
//         CharInRangeExclusive(position.strict_add(START))
//     }
// }
