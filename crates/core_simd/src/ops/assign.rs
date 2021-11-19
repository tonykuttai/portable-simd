use super::*;
use core::ops::{AddAssign, MulAssign}; // commutative binary op-assignment
use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign}; // commutative bit binary op-assignment
use core::ops::{DivAssign, RemAssign, SubAssign}; // non-commutative binary op-assignment
use core::ops::{ShlAssign, ShrAssign}; // non-commutative bit binary op-assignment

// Arithmetic

impl<T, U, const LANES: usize> AddAssign<U> for Simd<T, LANES>
where
    Self: Add<U, Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn add_assign(&mut self, rhs: U) {
        *self = self.add(rhs);
    }
}

impl<T, U, const LANES: usize> MulAssign<U> for Simd<T, LANES>
where
    Self: Mul<U, Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn mul_assign(&mut self, rhs: U) {
        *self = self.mul(rhs);
    }
}

impl<T, U, const LANES: usize> SubAssign<U> for Simd<T, LANES>
where
    Self: Sub<U, Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn sub_assign(&mut self, rhs: U) {
        *self = self.sub(rhs);
    }
}

impl<T, U, const LANES: usize> DivAssign<U> for Simd<T, LANES>
where
    Self: Div<U, Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn div_assign(&mut self, rhs: U) {
        *self = self.div(rhs);
    }
}

impl<T, U, const LANES: usize> RemAssign<U> for Simd<T, LANES>
where
    Self: Rem<U, Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn rem_assign(&mut self, rhs: U) {
        *self = self.rem(rhs);
    }
}

// Bit-whacking

impl<T, U, const LANES: usize> BitAndAssign<U> for Simd<T, LANES>
where
    T: SimdElement,
    Simd<T, LANES>: BitAnd<U, Output = Simd<T, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn bitand_assign(&mut self, rhs: U) {
        *self = self.bitand(rhs);
    }
}

impl<T, U, const LANES: usize> BitOrAssign<U> for Simd<T, LANES>
where
    T: SimdElement,
    Simd<T, LANES>: BitOr<U, Output = Simd<T, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn bitor_assign(&mut self, rhs: U) {
        *self = self.bitor(rhs);
    }
}

impl<T, U, const LANES: usize> BitXorAssign<U> for Simd<T, LANES>
where
    T: SimdElement,
    Simd<T, LANES>: BitXor<U, Output = Simd<T, LANES>>,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn bitxor_assign(&mut self, rhs: U) {
        *self = self.bitxor(rhs);
    }
}

// Bitshifts

impl<T, U, const LANES: usize> ShlAssign<U> for Simd<T, LANES>
where
    Self: Shl<U, Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn shl_assign(&mut self, rhs: U) {
        *self = self.shl(rhs)
    }
}

impl<T, U, const LANES: usize> ShrAssign<U> for Simd<T, LANES>
where
    Self: Shr<U, Output = Self>,
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn shr_assign(&mut self, rhs: U) {
        *self = self.shr(rhs)
    }
}
