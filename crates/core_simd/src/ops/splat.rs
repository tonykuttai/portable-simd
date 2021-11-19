use super::*;
use crate::simd::Mask;

// LHS splats

macro_rules! simple_lhs_splat {
    ( impl<const LANES: usize$(, $tvar:ident)?> $op:ident<Simd<$scalar:ty, LANES>> for $lhs:ty {
        fn $call:ident
    }) => {
        impl<$($tvar,)* const LANES: usize> $op<Simd<$scalar, LANES>> for <Simd<$scalar, LANES> as Vector>::Scalar
        where
            $lhs: SimdElement,
            Simd<$scalar, LANES>: $op<Simd<$lhs, LANES>, Output = Simd<$lhs, LANES>>,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Simd<$scalar, LANES>;

            #[inline]
            fn $call(self, rhs: Simd<$scalar, LANES>) -> Self::Output {
                Simd::splat(self).$call(rhs)
            }
        }
    }
}

// Arithmetic

simple_lhs_splat! {
    impl<const LANES: usize, T> Add<Simd<T, LANES>> for T {
        fn add
    }
}

simple_lhs_splat! {
    impl<const LANES: usize, T> Mul<Simd<T, LANES>> for T {
        fn mul
    }
}

simple_lhs_splat! {
    impl<const LANES: usize, T> Sub<Simd<T, LANES>> for T {
        fn sub
    }
}

// Bitwise

simple_lhs_splat! {
    impl<const LANES: usize, T> BitAnd<Simd<T, LANES>> for T {
        fn bitand
    }
}

simple_lhs_splat! {
    impl<const LANES: usize, T> BitOr<Simd<T, LANES>> for T {
        fn bitor
    }
}

simple_lhs_splat! {
    impl<const LANES: usize, T> BitXor<Simd<T, LANES>> for T {
        fn bitxor
    }
}

simple_lhs_splat! {
    impl<const LANES: usize, T> Shl<Simd<T, LANES>> for T {
        fn shl
    }
}

simple_lhs_splat! {
    impl<const LANES: usize, T> Shr<Simd<T, LANES>> for T {
        fn shr
    }
}

simple_lhs_splat! {
    impl<const LANES: usize> Div<Simd<f32, LANES>> for f32 {
        fn div
    }
}

simple_lhs_splat! {
    impl<const LANES: usize> Div<Simd<f64, LANES>> for f64 {
        fn div
    }
}

simple_lhs_splat! {
    impl<const LANES: usize> Rem<Simd<f32, LANES>> for f32 {
        fn rem
    }
}

simple_lhs_splat! {
    impl<const LANES: usize> Rem<Simd<f64, LANES>> for f64 {
        fn rem
    }
}

macro_rules! sint_divrem_lhs {
    ( impl<const LANES: usize> Div<Simd<$scalar:ty, LANES>> for $sint:ty {
        fn $call:ident
    }) => {
        impl<const LANES: usize> Div<Simd<$scalar, LANES>>
            for <Simd<$sint, LANES> as Vector>::Scalar
        where
            $sint: SimdElement,
            Simd<$sint, LANES>: Div<Simd<$sint, LANES>, Output = Simd<$sint, LANES>>,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Simd<$sint, LANES>;

            #[inline]
            fn $call(self, rhs: Simd<$scalar, LANES>) -> Self::Output {
                if rhs.lanes_ne(Simd::splat(0)) != Mask::splat(true) {
                    panic!("attempt to divide by zero");
                } else if self == <$sint>::MIN {
                    assert!(
                        rhs.lanes_ne(Simd::splat(-1)) == Mask::splat(true),
                        "attempt to divide with overflow"
                    );
                    Simd::splat(self).$call(rhs)
                } else {
                    Simd::splat(self).$call(rhs)
                }
            }
        }
    };
    ( impl<const LANES: usize> Rem<Simd<$scalar:ty, LANES>> for $sint:ty {
        fn $call:ident
    }) => {
        impl<const LANES: usize> Rem<Simd<$scalar, LANES>>
            for <Simd<$sint, LANES> as Vector>::Scalar
        where
            $sint: SimdElement,
            Simd<$sint, LANES>: Rem<Simd<$sint, LANES>, Output = Simd<$sint, LANES>>,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Simd<$sint, LANES>;

            fn $call(self, rhs: Simd<$scalar, LANES>) -> Self::Output {
                if rhs.lanes_eq(Simd::splat(0)) != Mask::splat(false) {
                    panic!("attempt to calculate the remainder with a divisor of zero");
                } else if self == <$sint>::MIN {
                    assert!(
                        rhs.lanes_eq(Simd::splat(-1)) == Mask::splat(false),
                        "attempt to calculate the remainder with overflow"
                    );
                    Simd::splat(self).$call(rhs)
                } else {
                    Simd::splat(self).$call(rhs)
                }
            }
        }
    };
}

sint_divrem_lhs! {
    impl<const LANES: usize> Div<Simd<i8, LANES>> for i8 {
        fn div
    }
}

sint_divrem_lhs! {
    impl<const LANES: usize> Div<Simd<i16, LANES>> for i16 {
        fn div
    }
}

sint_divrem_lhs! {
    impl<const LANES: usize> Div<Simd<i32, LANES>> for i32 {
        fn div
    }
}

sint_divrem_lhs! {
    impl<const LANES: usize> Div<Simd<i64, LANES>> for i64 {
        fn div
    }
}

sint_divrem_lhs! {
    impl<const LANES: usize> Div<Simd<isize, LANES>> for isize {
        fn div
    }
}

sint_divrem_lhs! {
    impl<const LANES: usize> Rem<Simd<i8, LANES>> for i8 {
        fn rem
    }
}

sint_divrem_lhs! {
    impl<const LANES: usize> Rem<Simd<i16, LANES>> for i16 {
        fn rem
    }
}

sint_divrem_lhs! {
    impl<const LANES: usize> Rem<Simd<i32, LANES>> for i32 {
        fn rem
    }
}

sint_divrem_lhs! {
    impl<const LANES: usize> Rem<Simd<i64, LANES>> for i64 {
        fn rem
    }
}

sint_divrem_lhs! {
    impl<const LANES: usize> Rem<Simd<isize, LANES>> for isize {
        fn rem
    }
}

macro_rules! uint_divrem_lhs {
    ( impl<const LANES: usize> Div<Simd<$scalar:ty, LANES>> for $uint:ty {
        fn $call:ident
    }) => {
        impl<const LANES: usize> Div<Simd<$scalar, LANES>>
            for <Simd<$uint, LANES> as Vector>::Scalar
        where
            $uint: SimdElement,
            Simd<$uint, LANES>: Div<Simd<$uint, LANES>, Output = Simd<$uint, LANES>>,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Simd<$uint, LANES>;

            #[inline]
            fn $call(self, rhs: Simd<$scalar, LANES>) -> Self::Output {
                if rhs.lanes_eq(Simd::splat(0)) != Mask::splat(false) {
                    panic!("attempt to divide by zero");
                } else {
                    Simd::splat(self).$call(rhs)
                }
            }
        }
    };
    ( impl<const LANES: usize> Rem<Simd<$scalar:ty, LANES>> for $uint:ty {
        fn $call:ident
    }) => {
        impl<const LANES: usize> Rem<Simd<$scalar, LANES>>
            for <Simd<$uint, LANES> as Vector>::Scalar
        where
            $uint: SimdElement,
            Simd<$uint, LANES>: Rem<Simd<$uint, LANES>, Output = Simd<$uint, LANES>>,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Simd<$uint, LANES>;

            fn $call(self, rhs: Simd<$scalar, LANES>) -> Self::Output {
                if rhs.lanes_eq(Simd::splat(0)) != Mask::splat(false) {
                    panic!("attempt to calculate the remainder with a divisor of zero");
                } else {
                    Simd::splat(self).$call(rhs)
                }
            }
        }
    };
}

uint_divrem_lhs! {
    impl<const LANES: usize> Div<Simd<u8, LANES>> for u8 {
        fn div
    }
}

uint_divrem_lhs! {
    impl<const LANES: usize> Div<Simd<u16, LANES>> for u16 {
        fn div
    }
}

uint_divrem_lhs! {
    impl<const LANES: usize> Div<Simd<u32, LANES>> for u32 {
        fn div
    }
}

uint_divrem_lhs! {
    impl<const LANES: usize> Div<Simd<u64, LANES>> for u64 {
        fn div
    }
}

uint_divrem_lhs! {
    impl<const LANES: usize> Div<Simd<usize, LANES>> for usize {
        fn div
    }
}

uint_divrem_lhs! {
    impl<const LANES: usize> Rem<Simd<u8, LANES>> for u8 {
        fn rem
    }
}

uint_divrem_lhs! {
    impl<const LANES: usize> Rem<Simd<u16, LANES>> for u16 {
        fn rem
    }
}

uint_divrem_lhs! {
    impl<const LANES: usize> Rem<Simd<u32, LANES>> for u32 {
        fn rem
    }
}

uint_divrem_lhs! {
    impl<const LANES: usize> Rem<Simd<u64, LANES>> for u64 {
        fn rem
    }
}

uint_divrem_lhs! {
    impl<const LANES: usize> Rem<Simd<usize, LANES>> for usize {
        fn rem
    }
}

// RHS splats

/// This covers RHS splats for simple cases
/// where the RHS is always valid.
macro_rules! simple_rhs_splat {
    ( impl<const LANES: usize$(, $tvar:ident)?> $op:ident<$operand:ty> for Simd<$scalar:ty, LANES> {
        fn $call:ident
    }) => {
        impl<$($tvar,)* const LANES: usize> $op<$operand> for Simd<$scalar, LANES>
        where
            Self: $op<Output=Self>,
            $operand: SimdElement,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;

            #[inline]
            fn $call(self, rhs: $operand) -> Self::Output {
                self.$call(Self::splat(rhs))
            }
        }
    }
}

// Arithmetic
// Commutative

simple_rhs_splat! {
    impl<const LANES: usize, T> Add<T> for Simd<T, LANES> {
        fn add
    }
}

simple_rhs_splat! {
    impl<const LANES: usize, T> Mul<T> for Simd<T, LANES> {
        fn mul
    }
}

// Non-commutative

simple_rhs_splat! {
    impl<const LANES: usize, T> Sub<T> for Simd<T, LANES> {
        fn sub
    }
}

// Bitwise

simple_rhs_splat! {
    impl<const LANES: usize, T> BitAnd<T> for Simd<T, LANES> {
        fn bitand
    }
}

simple_rhs_splat! {
    impl<const LANES: usize, T> BitOr<T> for Simd<T, LANES> {
        fn bitor
    }
}

simple_rhs_splat! {
    impl<const LANES: usize, T> BitXor<T> for Simd<T, LANES> {
        fn bitxor
    }
}

simple_rhs_splat! {
    impl<const LANES: usize, T> Shl<T> for Simd<T, LANES> {
        fn shl
    }
}

simple_rhs_splat! {
    impl<const LANES: usize, T> Shr<T> for Simd<T, LANES> {
        fn shr
    }
}

simple_rhs_splat! {
    impl<const LANES: usize> Div<f32> for Simd<f32, LANES> {
        fn div
    }
}

simple_rhs_splat! {
    impl<const LANES: usize> Div<f64> for Simd<f64, LANES> {
        fn div
    }
}

simple_rhs_splat! {
    impl<const LANES: usize> Rem<f32> for Simd<f32, LANES> {
        fn rem
    }
}

simple_rhs_splat! {
    impl<const LANES: usize> Rem<f64> for Simd<f64, LANES> {
        fn rem
    }
}

macro_rules! sint_divrem_rhs {
    ( impl<const LANES: usize> Div<$sint:ty> for Simd<$scalar:ty, LANES> {
        fn $call:ident
    }) => {
        impl<const LANES: usize> Div<$sint> for Simd<$scalar, LANES>
        where
            Self: Div<Output = Self>,
            $sint: SimdElement,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;
            fn $call(self, rhs: $sint) -> Self::Output {
                if rhs == 0 {
                    panic!("attempt to divide by zero");
                } else if rhs == -1 as _ {
                    assert!(
                        self.lanes_eq(Simd::splat(<$sint>::MIN)) == Mask::splat(false),
                        "attempt to divide with overflow"
                    );
                    self.$call(Simd::splat(rhs))
                } else {
                    self.$call(Simd::splat(rhs))
                }
            }
        }
    };
    ( impl<const LANES: usize> Rem<$sint:ty> for Simd<$scalar:ty, LANES> {
        fn $call:ident
    }) => {
        impl<const LANES: usize> Rem<$sint> for Simd<$scalar, LANES>
        where
            Self: Rem<Output = Self>,
            $sint: SimdElement,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;
            fn $call(self, rhs: $sint) -> Self::Output {
                if rhs == 0 {
                    panic!("attempt to calculate the remainder with a divisor of zero");
                } else if rhs == -1 as _ {
                    assert!(
                        self.lanes_eq(Simd::splat(<$sint>::MIN)) == Mask::splat(false),
                        "attempt to calculate the remainder with overflow"
                    );
                    self.$call(Simd::splat(rhs))
                } else {
                    self.$call(Simd::splat(rhs))
                }
            }
        }
    };
}

sint_divrem_rhs! {
    impl<const LANES: usize> Div<i8> for Simd<i8, LANES> {
        fn div
    }
}

sint_divrem_rhs! {
    impl<const LANES: usize> Div<i16> for Simd<i16, LANES> {
        fn div
    }
}

sint_divrem_rhs! {
    impl<const LANES: usize> Div<i32> for Simd<i32, LANES> {
        fn div
    }
}

sint_divrem_rhs! {
    impl<const LANES: usize> Div<i64> for Simd<i64, LANES> {
        fn div
    }
}

sint_divrem_rhs! {
    impl<const LANES: usize> Div<isize> for Simd<isize, LANES> {
        fn div
    }
}

sint_divrem_rhs! {
    impl<const LANES: usize> Rem<i8> for Simd<i8, LANES> {
        fn rem
    }
}

sint_divrem_rhs! {
    impl<const LANES: usize> Rem<i16> for Simd<i16, LANES> {
        fn rem
    }
}

sint_divrem_rhs! {
    impl<const LANES: usize> Rem<i32> for Simd<i32, LANES> {
        fn rem
    }
}

sint_divrem_rhs! {
    impl<const LANES: usize> Rem<i64> for Simd<i64, LANES> {
        fn rem
    }
}

sint_divrem_rhs! {
    impl<const LANES: usize> Rem<isize> for Simd<isize, LANES> {
        fn rem
    }
}

macro_rules! uint_divrem_rhs {
    ( impl<const LANES: usize> Div<$uint:ty> for Simd<$scalar:ty, LANES> {
        fn $call:ident
    }) => {
        impl<const LANES: usize> Div<$uint> for Simd<$scalar, LANES>
        where
            Self: Div<Output = Self>,
            $uint: SimdElement,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;
            fn $call(self, rhs: $uint) -> Self::Output {
                if rhs == 0 {
                    panic!("attempt to divide by zero");
                } else {
                    self.$call(Simd::splat(rhs))
                }
            }
        }
    };
    ( impl<const LANES: usize> Rem<$uint:ty> for Simd<$scalar:ty, LANES> {
        fn $call:ident
    }) => {
        impl<const LANES: usize> Rem<$uint> for Simd<$scalar, LANES>
        where
            Self: Rem<Output = Self>,
            $uint: SimdElement,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;
            fn $call(self, rhs: $uint) -> Self::Output {
                if rhs == 0 {
                    panic!("attempt to calculate the remainder with a divisor of zero");
                } else {
                    self.$call(Simd::splat(rhs))
                }
            }
        }
    };
}

uint_divrem_rhs! {
    impl<const LANES: usize> Div<u8> for Simd<u8, LANES> {
        fn div
    }
}

uint_divrem_rhs! {
    impl<const LANES: usize> Div<u16> for Simd<u16, LANES> {
        fn div
    }
}

uint_divrem_rhs! {
    impl<const LANES: usize> Div<u32> for Simd<u32, LANES> {
        fn div
    }
}

uint_divrem_rhs! {
    impl<const LANES: usize> Div<u64> for Simd<u64, LANES> {
        fn div
    }
}

uint_divrem_rhs! {
    impl<const LANES: usize> Div<usize> for Simd<usize, LANES> {
        fn div
    }
}

uint_divrem_rhs! {
    impl<const LANES: usize> Rem<u8> for Simd<u8, LANES> {
        fn rem
    }
}

uint_divrem_rhs! {
    impl<const LANES: usize> Rem<u16> for Simd<u16, LANES> {
        fn rem
    }
}

uint_divrem_rhs! {
    impl<const LANES: usize> Rem<u32> for Simd<u32, LANES> {
        fn rem
    }
}

uint_divrem_rhs! {
    impl<const LANES: usize> Rem<u64> for Simd<u64, LANES> {
        fn rem
    }
}

uint_divrem_rhs! {
    impl<const LANES: usize> Rem<usize> for Simd<usize, LANES> {
        fn rem
    }
}
