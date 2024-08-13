use rust_decimal::Decimal;
use rust_decimal::MathematicalOps;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::str::FromStr;
use std::cmp::Ordering;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct IndicatorValue(Decimal);

impl IndicatorValue {

    #[inline(always)]
    pub fn value(&self) -> Decimal {
        self.0
    }

    #[inline(always)]
    pub fn to_f64(&self) -> f64 {
        self.0.to_f64().unwrap_or_else(|| panic!("Failed to convert Decimal to f64"))
    }
    
    #[inline(always)]
    pub fn sqrt(&self) -> Self {
        Self(self.0.sqrt().unwrap_or_else(|| panic!("Failed to compute sqrt")))
    }
}

impl From<f64> for IndicatorValue {
    #[inline(always)]
    fn from(value: f64) -> Self {
        Self(Decimal::from_f64(value).unwrap_or_else(|| panic!("Failed to convert f64 to Decimal")))
    }
}

impl From<&str> for IndicatorValue {
    #[inline(always)]
    fn from(value: &str) -> Self {
        Self(Decimal::from_str(value).unwrap_or_else(|_| panic!("Invalid number format")))
    }
}

impl From<usize> for IndicatorValue {
    #[inline(always)]
    fn from(value: usize) -> Self {
        Self(Decimal::from_usize(value).unwrap_or_else(|| panic!("Failed to convert usize to Decimal")))
    }
}

impl From<u64> for IndicatorValue {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self(Decimal::from_u64(value).unwrap_or_else(|| panic!("Failed to convert u64 to Decimal")))
    }
}

impl PartialEq for IndicatorValue {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for IndicatorValue {}

impl PartialOrd for IndicatorValue {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }

    #[inline(always)]
    fn lt(&self, other: &Self) -> bool {
        self.0 < other.0
    }

    #[inline(always)]
    fn le(&self, other: &Self) -> bool {
        self.0 <= other.0
    }

    #[inline(always)]
    fn gt(&self, other: &Self) -> bool {
        self.0 > other.0
    }

    #[inline(always)]
    fn ge(&self, other: &Self) -> bool {
        self.0 >= other.0
    }
}

impl Neg for IndicatorValue {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Add for IndicatorValue {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sub for IndicatorValue {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl Mul for IndicatorValue {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0)
    }
}

impl Div for IndicatorValue {
    type Output = Self;

    #[inline(always)]
    fn div(self, other: Self) -> Self::Output {
        Self(self.0 / other.0)
    }
}

impl AddAssign for IndicatorValue {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl SubAssign for IndicatorValue {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl MulAssign for IndicatorValue {
    #[inline(always)]
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}

impl DivAssign for IndicatorValue {
    #[inline(always)]
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
    }
}
