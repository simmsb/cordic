use fixed::types::extra::{ If, True };
use fixed::types::U0F64;
use fixed::{FixedI16, FixedI32, FixedI64, FixedI8};
use core::ops::{Add, AddAssign, Div, Mul, Neg, Shl, Shr, Sub, SubAssign};

/// A number that can be used by the CORDIC-based algorithms.
///
/// This covers most fixed-point numbers, with some restriction on the maximal number
/// of decimal bits in order to allow some constraints (like PI) to fit.
pub trait CordicNumber:
    Copy
    + PartialOrd
    + AddAssign
    + SubAssign
    + Div<Output = Self>
    + Mul<Output = Self> // FIXME: remove that
    + Neg<Output = Self>
    + Sub<Output = Self>
    + Add<Output = Self>
    + Shr<u8, Output = Self>
    + Shl<u8, Output = Self>
{
    fn floor(self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn half() -> Self {
        Self::one() >> 1
    }
    fn frac_pi_2() -> Self;
    fn pi() -> Self;
    fn e() -> Self;
    fn from_u0f64(val: U0F64) -> Self;
    fn num_fract_bits() -> u8;
    fn num_bits() -> u8;
}

// The generic constraints are for (in order):
// - The Fixed type
// - The FRAC_PI_2 constant.
// - The PI constant.
impl<const FRAC: i32> CordicNumber for FixedI8<FRAC>
    where 
        If<{ (0 <= FRAC) & (FRAC <= 8) }>: True,
        If<{ (0 <= FRAC) & (FRAC <= 6) }>: True,
        If<{ (0 <= FRAC) & (FRAC <= 5) }>: True
{
    #[inline(always)]
    fn floor(self) -> Self {
        self.floor()
    }

    #[inline(always)]
    fn zero() -> Self {
        Self::from_bits(0)
    }

    #[inline(always)]
    fn one() -> Self {
        Self::from_num(1.0)
    }

    #[inline(always)]
    fn frac_pi_2() -> Self {
        Self::FRAC_PI_2
    }

    #[inline(always)]
    fn pi() -> Self {
        Self::PI
    }

    #[inline(always)]
    fn e() -> Self {
        Self::E
    }

    #[inline(always)]
    fn from_u0f64(val: U0F64) -> Self {
        val.to_num()
    }

    #[inline(always)]
    fn num_fract_bits() -> u8 {
        FRAC as u8
    }

    #[inline(always)]
    fn num_bits() -> u8 {
        8
    }
}

impl<const FRAC: i32> CordicNumber for FixedI32<FRAC>
    where 
        If<{ (0 <= FRAC) & (FRAC <= 32) }>: True,
        If<{ (0 <= FRAC) & (FRAC <= 30) }>: True,
        If<{ (0 <= FRAC) & (FRAC <= 29) }>: True
{
    #[inline(always)]
    fn floor(self) -> Self {
        self.floor()
    }

    #[inline(always)]
    fn zero() -> Self {
        Self::from_bits(0)
    }

    #[inline(always)]
    fn one() -> Self {
        Self::from_num(1.0)
    }

    #[inline(always)]
    fn frac_pi_2() -> Self {
        Self::FRAC_PI_2
    }

    #[inline(always)]
    fn pi() -> Self {
        Self::PI
    }

    #[inline(always)]
    fn e() -> Self {
        Self::E
    }

    #[inline(always)]
    fn from_u0f64(val: U0F64) -> Self {
        val.to_num()
    }

    #[inline(always)]
    fn num_fract_bits() -> u8 {
        FRAC as u8
    }

    #[inline(always)]
    fn num_bits() -> u8 {
        32
    }
}

impl<const FRAC: i32> CordicNumber for FixedI16<FRAC>
    where 
        If<{ (0 <= FRAC) & (FRAC <= 16) }>: True,
        If<{ (0 <= FRAC) & (FRAC <= 14) }>: True,
        If<{ (0 <= FRAC) & (FRAC <= 13) }>: True
{
    #[inline(always)]
    fn floor(self) -> Self {
        self.floor()
    }

    #[inline(always)]
    fn zero() -> Self {
        Self::from_bits(0)
    }

    #[inline(always)]
    fn one() -> Self {
        Self::from_num(1.0)
    }

    #[inline(always)]
    fn frac_pi_2() -> Self {
        Self::FRAC_PI_2
    }

    #[inline(always)]
    fn pi() -> Self {
        Self::PI
    }

    #[inline(always)]
    fn e() -> Self {
        Self::E
    }

    #[inline(always)]
    fn from_u0f64(val: U0F64) -> Self {
        val.to_num()
    }

    #[inline(always)]
    fn num_fract_bits() -> u8 {
        FRAC as u8
    }

    #[inline(always)]
    fn num_bits() -> u8 {
        16
    }
}

impl<const FRAC: i32> CordicNumber for FixedI64<FRAC>
    where 
        If<{ (0 <= FRAC) & (FRAC <= 64) }>: True,
        If<{ (0 <= FRAC) & (FRAC <= 62) }>: True,
        If<{ (0 <= FRAC) & (FRAC <= 61) }>: True
{
    #[inline(always)]
    fn floor(self) -> Self {
        self.floor()
    }

    #[inline(always)]
    fn zero() -> Self {
        Self::from_bits(0)
    }

    #[inline(always)]
    fn one() -> Self {
        Self::from_num(1.0)
    }

    #[inline(always)]
    fn frac_pi_2() -> Self {
        Self::FRAC_PI_2
    }

    #[inline(always)]
    fn pi() -> Self {
        Self::PI
    }

    #[inline(always)]
    fn e() -> Self {
        Self::E
    }

    #[inline(always)]
    fn from_u0f64(val: U0F64) -> Self {
        val.to_num()
    }

    #[inline(always)]
    fn num_fract_bits() -> u8 {
        FRAC as u8
    }

    #[inline(always)]
    fn num_bits() -> u8 {
        64
    }
}
