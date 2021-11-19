//! The organization of this module deserves some explanation.
//!
use crate::simd::{LaneCount, Mask, Simd, SimdElement, SupportedLaneCount, Vector};
use core::ops::{Add, Mul}; // commutative arithmetic binary ops
use core::ops::{BitAnd, BitOr, BitXor};
use core::ops::{Div, Rem, Sub}; // non-commutative arithmetic binary ops
use core::ops::{Shl, Shr}; // non-commutative bit binary ops

mod assign;
mod splat;
mod unop;

// Indexing

impl<I, T, const LANES: usize> core::ops::Index<I> for Simd<T, LANES>
where
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
    I: core::slice::SliceIndex<[T]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.as_array()[index]
    }
}

impl<I, T, const LANES: usize> core::ops::IndexMut<I> for Simd<T, LANES>
where
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
    I: core::slice::SliceIndex<[T]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.as_mut_array()[index]
    }
}

macro_rules! unsafe_base_op {
    ( impl<const LANES: usize> $op:ident for Simd<$scalar:ty, LANES> {
        fn $call:ident(self, rhs: Self) -> Self::Output {
            unsafe{ $simd_call:ident }
        }
    }) => {
        impl<const LANES: usize> $op for Simd<$scalar, LANES>
        where
            $scalar: SimdElement,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;

            #[inline]
            fn $call(self, rhs: Self) -> Self::Output {
                unsafe { $crate::intrinsics::$simd_call(self, rhs) }
            }
        }
    };
}

unsafe_base_op! {
    impl<const LANES: usize> Add for Simd<f32, LANES> {
        fn add(self, rhs: Self) -> Self::Output {
            unsafe { simd_add }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Add for Simd<f64, LANES> {
        fn add(self, rhs: Self) -> Self::Output {
            unsafe { simd_add }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Add for Simd<i8, LANES> {
        fn add(self, rhs: Self) -> Self::Output {
            unsafe { simd_add }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Add for Simd<i16, LANES> {
        fn add(self, rhs: Self) -> Self::Output {
            unsafe { simd_add }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Add for Simd<i32, LANES> {
        fn add(self, rhs: Self) -> Self::Output {
            unsafe { simd_add }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Add for Simd<i64, LANES> {
        fn add(self, rhs: Self) -> Self::Output {
            unsafe { simd_add }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Add for Simd<isize, LANES> {
        fn add(self, rhs: Self) -> Self::Output {
            unsafe { simd_add }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Add for Simd<u8, LANES> {
        fn add(self, rhs: Self) -> Self::Output {
            unsafe { simd_add }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Add for Simd<u16, LANES> {
        fn add(self, rhs: Self) -> Self::Output {
            unsafe { simd_add }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Add for Simd<u32, LANES> {
        fn add(self, rhs: Self) -> Self::Output {
            unsafe { simd_add }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Add for Simd<u64, LANES> {
        fn add(self, rhs: Self) -> Self::Output {
            unsafe { simd_add }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Add for Simd<usize, LANES> {
        fn add(self, rhs: Self) -> Self::Output {
            unsafe { simd_add }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Mul for Simd<f32, LANES> {
        fn mul(self, rhs: Self) -> Self::Output {
            unsafe { simd_mul }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Mul for Simd<f64, LANES> {
        fn mul(self, rhs: Self) -> Self::Output {
            unsafe { simd_mul }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Mul for Simd<i8, LANES> {
        fn mul(self, rhs: Self) -> Self::Output {
            unsafe { simd_mul }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Mul for Simd<i16, LANES> {
        fn mul(self, rhs: Self) -> Self::Output {
            unsafe { simd_mul }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Mul for Simd<i32, LANES> {
        fn mul(self, rhs: Self) -> Self::Output {
            unsafe { simd_mul }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Mul for Simd<i64, LANES> {
        fn mul(self, rhs: Self) -> Self::Output {
            unsafe { simd_mul }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Mul for Simd<isize, LANES> {
        fn mul(self, rhs: Self) -> Self::Output {
            unsafe { simd_mul }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Mul for Simd<u8, LANES> {
        fn mul(self, rhs: Self) -> Self::Output {
            unsafe { simd_mul }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Mul for Simd<u16, LANES> {
        fn mul(self, rhs: Self) -> Self::Output {
            unsafe { simd_mul }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Mul for Simd<u32, LANES> {
        fn mul(self, rhs: Self) -> Self::Output {
            unsafe { simd_mul }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Mul for Simd<u64, LANES> {
        fn mul(self, rhs: Self) -> Self::Output {
            unsafe { simd_mul }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Mul for Simd<usize, LANES> {
        fn mul(self, rhs: Self) -> Self::Output {
            unsafe { simd_mul }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Sub for Simd<f32, LANES> {
        fn sub(self, rhs: Self) -> Self::Output {
            unsafe { simd_sub }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Sub for Simd<f64, LANES> {
        fn sub(self, rhs: Self) -> Self::Output {
            unsafe { simd_sub }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Sub for Simd<i8, LANES> {
        fn sub(self, rhs: Self) -> Self::Output {
            unsafe { simd_sub }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Sub for Simd<i16, LANES> {
        fn sub(self, rhs: Self) -> Self::Output {
            unsafe { simd_sub }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Sub for Simd<i32, LANES> {
        fn sub(self, rhs: Self) -> Self::Output {
            unsafe { simd_sub }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Sub for Simd<i64, LANES> {
        fn sub(self, rhs: Self) -> Self::Output {
            unsafe { simd_sub }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Sub for Simd<isize, LANES> {
        fn sub(self, rhs: Self) -> Self::Output {
            unsafe { simd_sub }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Sub for Simd<u8, LANES> {
        fn sub(self, rhs: Self) -> Self::Output {
            unsafe { simd_sub }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Sub for Simd<u16, LANES> {
        fn sub(self, rhs: Self) -> Self::Output {
            unsafe { simd_sub }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Sub for Simd<u32, LANES> {
        fn sub(self, rhs: Self) -> Self::Output {
            unsafe { simd_sub }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Sub for Simd<u64, LANES> {
        fn sub(self, rhs: Self) -> Self::Output {
            unsafe { simd_sub }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Sub for Simd<usize, LANES> {
        fn sub(self, rhs: Self) -> Self::Output {
            unsafe { simd_sub }
        }
    }
}

// Division
// Remainder
// These get a bit more complicated for the ones past floats.

unsafe_base_op! {
    impl<const LANES: usize> Div for Simd<f32, LANES> {
        fn div(self, rhs: Self) -> Self::Output {
            unsafe { simd_div }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Div for Simd<f64, LANES> {
        fn div(self, rhs: Self) -> Self::Output {
            unsafe { simd_div }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Rem for Simd<f32, LANES> {
        fn rem(self, rhs: Self) -> Self::Output {
            unsafe { simd_rem }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> Rem for Simd<f64, LANES> {
        fn rem(self, rhs: Self) -> Self::Output {
            unsafe { simd_rem }
        }
    }
}

macro_rules! sint_divrem_guard {
    ( impl<const LANES: usize> Div for Simd<$sint:ty, LANES> {
        fn $call:ident
    }) => {
        impl<const LANES: usize> Div for Simd<$sint, LANES>
        where
            $sint: SimdElement,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;

            #[inline]
            fn $call(self, rhs: Self) -> Self::Output {
                if rhs.lanes_eq(Simd::splat(0)).any() {
                    panic!("attempt to divide by zero");
                } else if self.lanes_eq(Simd::splat(<$sint>::MIN)) & rhs.lanes_eq(Simd::splat(-1))
                    != Mask::splat(false)
                {
                    panic!("attempt to divide with overflow");
                } else {
                    unsafe { $crate::intrinsics::simd_div(self, rhs) }
                }
            }
        }
    };
    ( impl<const LANES: usize> Rem for Simd<$sint:ty, LANES> {
        fn $call:ident
    }) => {
        impl<const LANES: usize> Rem for Simd<$sint, LANES>
        where
            $sint: SimdElement,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;

            #[inline]
            fn $call(self, rhs: Self) -> Self::Output {
                if rhs.lanes_eq(Simd::splat(0)).any() {
                    panic!("attempt to calculate the remainder with a divisor of zero");
                } else if self.lanes_eq(Simd::splat(<$sint>::MIN)) & rhs.lanes_eq(Simd::splat(-1))
                    != Mask::splat(false)
                {
                    panic!("attempt to calculate the remainder with overflow");
                } else {
                    unsafe { $crate::intrinsics::simd_rem(self, rhs) }
                }
            }
        }
    };
}

macro_rules! uint_divrem_guard {
    ( impl<const LANES: usize> Div for Simd<$uint:ty, LANES> {
        fn $call:ident
    }) => {
        impl<const LANES: usize> Div for Simd<$uint, LANES>
        where
            $uint: SimdElement,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;

            #[inline]
            fn $call(self, rhs: Self) -> Self::Output {
                if rhs.lanes_eq(Simd::splat(0)).any() {
                    panic!("attempt to divide by zero");
                } else {
                    unsafe { $crate::intrinsics::simd_div(self, rhs) }
                }
            }
        }
    };
    ( impl<const LANES: usize> Rem for Simd<$uint:ty, LANES> {
        fn $call:ident
    }) => {
        impl<const LANES: usize> Rem for Simd<$uint, LANES>
        where
            $uint: SimdElement,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;

            #[inline]
            fn $call(self, rhs: Self) -> Self::Output {
                if rhs.lanes_eq(Simd::splat(0)).any() {
                    panic!("attempt to calculate the remainder with a divisor of zero");
                } else {
                    unsafe { $crate::intrinsics::simd_rem(self, rhs) }
                }
            }
        }
    };
}

sint_divrem_guard! {
    impl<const LANES: usize> Div for Simd<i8, LANES> {
        fn div
    }
}

sint_divrem_guard! {
    impl<const LANES: usize> Rem for Simd<i8, LANES> {
        fn rem
    }
}

sint_divrem_guard! {
    impl<const LANES: usize> Div for Simd<i16, LANES> {
        fn div
    }
}

sint_divrem_guard! {
    impl<const LANES: usize> Rem for Simd<i16, LANES> {
        fn rem
    }
}

sint_divrem_guard! {
    impl<const LANES: usize> Div for Simd<i32, LANES> {
        fn div
    }
}

sint_divrem_guard! {
    impl<const LANES: usize> Rem for Simd<i32, LANES> {
        fn rem
    }
}

sint_divrem_guard! {
    impl<const LANES: usize> Div for Simd<i64, LANES> {
        fn div
    }
}

sint_divrem_guard! {
    impl<const LANES: usize> Rem for Simd<i64, LANES> {
        fn rem
    }
}

sint_divrem_guard! {
    impl<const LANES: usize> Div for Simd<isize, LANES> {
        fn div
    }
}

sint_divrem_guard! {
    impl<const LANES: usize> Rem for Simd<isize, LANES> {
        fn rem
    }
}

uint_divrem_guard! {
    impl<const LANES: usize> Div for Simd<u8, LANES> {
        fn div
    }
}

uint_divrem_guard! {
    impl<const LANES: usize> Rem for Simd<u8, LANES> {
        fn rem
    }
}

uint_divrem_guard! {
    impl<const LANES: usize> Div for Simd<u16, LANES> {
        fn div
    }
}

uint_divrem_guard! {
    impl<const LANES: usize> Rem for Simd<u16, LANES> {
        fn rem
    }
}

uint_divrem_guard! {
    impl<const LANES: usize> Div for Simd<u32, LANES> {
        fn div
    }
}

uint_divrem_guard! {
    impl<const LANES: usize> Rem for Simd<u32, LANES> {
        fn rem
    }
}

uint_divrem_guard! {
    impl<const LANES: usize> Div for Simd<u64, LANES> {
        fn div
    }
}

uint_divrem_guard! {
    impl<const LANES: usize> Rem for Simd<u64, LANES> {
        fn rem
    }
}

uint_divrem_guard! {
    impl<const LANES: usize> Div for Simd<usize, LANES> {
        fn div
    }
}

uint_divrem_guard! {
    impl<const LANES: usize> Rem for Simd<usize, LANES> {
        fn rem
    }
}

// Bit Logic
// Only applies to integer types

unsafe_base_op! {
    impl<const LANES: usize> BitAnd for Simd<i8, LANES> {
        fn bitand(self, rhs: Self) -> Self::Output {
            unsafe { simd_and }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitAnd for Simd<i16, LANES> {
        fn bitand(self, rhs: Self) -> Self::Output {
            unsafe { simd_and }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitAnd for Simd<i32, LANES> {
        fn bitand(self, rhs: Self) -> Self::Output {
            unsafe { simd_and }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitAnd for Simd<i64, LANES> {
        fn bitand(self, rhs: Self) -> Self::Output {
            unsafe { simd_and }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitAnd for Simd<isize, LANES> {
        fn bitand(self, rhs: Self) -> Self::Output {
            unsafe { simd_and }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitAnd for Simd<u8, LANES> {
        fn bitand(self, rhs: Self) -> Self::Output {
            unsafe { simd_and }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitAnd for Simd<u16, LANES> {
        fn bitand(self, rhs: Self) -> Self::Output {
            unsafe { simd_and }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitAnd for Simd<u32, LANES> {
        fn bitand(self, rhs: Self) -> Self::Output {
            unsafe { simd_and }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitAnd for Simd<u64, LANES> {
        fn bitand(self, rhs: Self) -> Self::Output {
            unsafe { simd_and }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitAnd for Simd<usize, LANES> {
        fn bitand(self, rhs: Self) -> Self::Output {
            unsafe { simd_and }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitOr for Simd<i8, LANES> {
        fn bitor(self, rhs: Self) -> Self::Output {
            unsafe { simd_or }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitOr for Simd<i16, LANES> {
        fn bitor(self, rhs: Self) -> Self::Output {
            unsafe { simd_or }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitOr for Simd<i32, LANES> {
        fn bitor(self, rhs: Self) -> Self::Output {
            unsafe { simd_or }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitOr for Simd<i64, LANES> {
        fn bitor(self, rhs: Self) -> Self::Output {
            unsafe { simd_or }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitOr for Simd<isize, LANES> {
        fn bitor(self, rhs: Self) -> Self::Output {
            unsafe { simd_or }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitOr for Simd<u8, LANES> {
        fn bitor(self, rhs: Self) -> Self::Output {
            unsafe { simd_or }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitOr for Simd<u16, LANES> {
        fn bitor(self, rhs: Self) -> Self::Output {
            unsafe { simd_or }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitOr for Simd<u32, LANES> {
        fn bitor(self, rhs: Self) -> Self::Output {
            unsafe { simd_or }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitOr for Simd<u64, LANES> {
        fn bitor(self, rhs: Self) -> Self::Output {
            unsafe { simd_or }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitOr for Simd<usize, LANES> {
        fn bitor(self, rhs: Self) -> Self::Output {
            unsafe { simd_or }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitXor for Simd<i8, LANES> {
        fn bitxor(self, rhs: Self) -> Self::Output {
            unsafe { simd_xor }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitXor for Simd<i16, LANES> {
        fn bitxor(self, rhs: Self) -> Self::Output {
            unsafe { simd_xor }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitXor for Simd<i32, LANES> {
        fn bitxor(self, rhs: Self) -> Self::Output {
            unsafe { simd_xor }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitXor for Simd<i64, LANES> {
        fn bitxor(self, rhs: Self) -> Self::Output {
            unsafe { simd_xor }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitXor for Simd<isize, LANES> {
        fn bitxor(self, rhs: Self) -> Self::Output {
            unsafe { simd_xor }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitXor for Simd<u8, LANES> {
        fn bitxor(self, rhs: Self) -> Self::Output {
            unsafe { simd_xor }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitXor for Simd<u16, LANES> {
        fn bitxor(self, rhs: Self) -> Self::Output {
            unsafe { simd_xor }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitXor for Simd<u32, LANES> {
        fn bitxor(self, rhs: Self) -> Self::Output {
            unsafe { simd_xor }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitXor for Simd<u64, LANES> {
        fn bitxor(self, rhs: Self) -> Self::Output {
            unsafe { simd_xor }
        }
    }
}

unsafe_base_op! {
    impl<const LANES: usize> BitXor for Simd<usize, LANES> {
        fn bitxor(self, rhs: Self) -> Self::Output {
            unsafe { simd_xor }
        }
    }
}

macro_rules! bitshift_guard {
    (impl<const LANES: usize> $op:ident for Simd<$int:ty, LANES> {
        fn $call:ident(self, rhs: Self) -> Self::Output {
            unsafe { $simd_call:ident }
        }
    }) => {
        impl<const LANES: usize> $op for Simd<$int, LANES>
        where
            $int: SimdElement,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;

            #[inline]
            fn $call(self, rhs: Self) -> Self::Output {
                unsafe {
                    $crate::intrinsics::$simd_call(self, rhs.bitand(<$int>::BITS as $int - 1))
                }
            }
        }
    };
}

bitshift_guard! {
    impl<const LANES: usize> Shl for Simd<i8, LANES> {
        fn shl(self, rhs: Self) -> Self::Output {
            unsafe { simd_shl }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shl for Simd<i16, LANES> {
        fn shl(self, rhs: Self) -> Self::Output {
            unsafe { simd_shl }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shl for Simd<i32, LANES> {
        fn shl(self, rhs: Self) -> Self::Output {
            unsafe { simd_shl }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shl for Simd<i64, LANES> {
        fn shl(self, rhs: Self) -> Self::Output {
            unsafe { simd_shl }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shl for Simd<isize, LANES> {
        fn shl(self, rhs: Self) -> Self::Output {
            unsafe { simd_shl }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shl for Simd<u8, LANES> {
        fn shl(self, rhs: Self) -> Self::Output {
            unsafe { simd_shl }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shl for Simd<u16, LANES> {
        fn shl(self, rhs: Self) -> Self::Output {
            unsafe { simd_shl }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shl for Simd<u32, LANES> {
        fn shl(self, rhs: Self) -> Self::Output {
            unsafe { simd_shl }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shl for Simd<u64, LANES> {
        fn shl(self, rhs: Self) -> Self::Output {
            unsafe { simd_shl }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shl for Simd<usize, LANES> {
        fn shl(self, rhs: Self) -> Self::Output {
            unsafe { simd_shl }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shr for Simd<i8, LANES> {
        fn shr(self, rhs: Self) -> Self::Output {
            unsafe { simd_shr }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shr for Simd<i16, LANES> {
        fn shr(self, rhs: Self) -> Self::Output {
            unsafe { simd_shr }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shr for Simd<i32, LANES> {
        fn shr(self, rhs: Self) -> Self::Output {
            unsafe { simd_shr }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shr for Simd<i64, LANES> {
        fn shr(self, rhs: Self) -> Self::Output {
            unsafe { simd_shr }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shr for Simd<isize, LANES> {
        fn shr(self, rhs: Self) -> Self::Output {
            unsafe { simd_shr }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shr for Simd<u8, LANES> {
        fn shr(self, rhs: Self) -> Self::Output {
            unsafe { simd_shr }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shr for Simd<u16, LANES> {
        fn shr(self, rhs: Self) -> Self::Output {
            unsafe { simd_shr }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shr for Simd<u32, LANES> {
        fn shr(self, rhs: Self) -> Self::Output {
            unsafe { simd_shr }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shr for Simd<u64, LANES> {
        fn shr(self, rhs: Self) -> Self::Output {
            unsafe { simd_shr }
        }
    }
}

bitshift_guard! {
    impl<const LANES: usize> Shr for Simd<usize, LANES> {
        fn shr(self, rhs: Self) -> Self::Output {
            unsafe { simd_shr }
        }
    }
}

// Reference ops
// Implicit deref on method calls should handle the LHS, so manually deref RHS

impl<T, const LANES: usize> Add<&Self> for Simd<T, LANES>
where
    Self: Add<Self, Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        self.add(*rhs)
    }
}

impl<T, const LANES: usize> Mul<&Self> for Simd<T, LANES>
where
    Self: Mul<Self, Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &Self) -> Self::Output {
        self.mul(*rhs)
    }
}

impl<T, const LANES: usize> Div<&Self> for Simd<T, LANES>
where
    Self: Div<Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: &Self) -> Self::Output {
        self.div(*rhs)
    }
}

impl<T, const LANES: usize> Rem<&Self> for Simd<T, LANES>
where
    Self: Rem<Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn rem(self, rhs: &Self) -> Self::Output {
        self.rem(*rhs)
    }
}

impl<T, const LANES: usize> BitAnd<&Self> for Simd<T, LANES>
where
    Self: BitAnd<Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: &Self) -> Self::Output {
        self.bitand(*rhs)
    }
}

impl<T, const LANES: usize> BitOr<&Self> for Simd<T, LANES>
where
    Self: BitOr<Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: &Self) -> Self::Output {
        self.bitor(*rhs)
    }
}

impl<T, const LANES: usize> BitXor<&Self> for Simd<T, LANES>
where
    Self: BitXor<Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: &Self) -> Self::Output {
        self.bitxor(*rhs)
    }
}

impl<T, const LANES: usize> Shl<&Self> for Simd<T, LANES>
where
    Self: Shl<Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn shl(self, rhs: &Self) -> Self::Output {
        self.shl(*rhs)
    }
}

impl<T, const LANES: usize> Shr<&Self> for Simd<T, LANES>
where
    Self: Shr<Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn shr(self, rhs: &Self) -> Self::Output {
        self.shr(*rhs)
    }
}
