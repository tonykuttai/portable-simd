use crate::simd::intrinsics;
use crate::simd::{LaneCount, Simd, SupportedLaneCount};
use core::ops::{Neg, Not}; // unary ops

macro_rules! neg {
    (impl<const LANES: usize> Neg for Simd<$scalar:ty, LANES>) => {
        impl<const LANES: usize> Neg for Simd<$scalar, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self::Output {
                unsafe { intrinsics::simd_neg(self) }
            }
        }
    };
}

neg! {
    impl<const LANES: usize> Neg for Simd<f32, LANES>
}

neg! {
    impl<const LANES: usize> Neg for Simd<f64, LANES>
}

neg! {
    impl<const LANES: usize> Neg for Simd<i8, LANES>
}

neg! {
    impl<const LANES: usize> Neg for Simd<i16, LANES>
}

neg! {
    impl<const LANES: usize> Neg for Simd<i32, LANES>
}

neg! {
    impl<const LANES: usize> Neg for Simd<i64, LANES>
}

neg! {
    impl<const LANES: usize> Neg for Simd<isize, LANES>
}

macro_rules! not {
    (impl<const LANES: usize> Not for Simd<$scalar:ty, LANES>) => {
        impl<const LANES: usize> Not for Simd<$scalar, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;

            #[inline]
            fn not(self) -> Self::Output {
                self ^ (Simd::splat(!(0 as $scalar)))
            }
        }
    };
}

not! {
    impl<const LANES: usize> Not for Simd<i8, LANES>
}

not! {
    impl<const LANES: usize> Not for Simd<i16, LANES>
}

not! {
    impl<const LANES: usize> Not for Simd<i32, LANES>
}

not! {
    impl<const LANES: usize> Not for Simd<i64, LANES>
}

not! {
    impl<const LANES: usize> Not for Simd<isize, LANES>
}

not! {
    impl<const LANES: usize> Not for Simd<u8, LANES>
}

not! {
    impl<const LANES: usize> Not for Simd<u16, LANES>
}

not! {
    impl<const LANES: usize> Not for Simd<u32, LANES>
}

not! {
    impl<const LANES: usize> Not for Simd<u64, LANES>
}

not! {
    impl<const LANES: usize> Not for Simd<usize, LANES>
}
