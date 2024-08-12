use rust_decimal::Decimal;
use rust_decimal::MathematicalOps;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub struct IndicatorValue {
    value: Decimal,
}

impl IndicatorValue {

    #[inline(always)]
    pub fn get_value(&self) -> Decimal {
        self.value
    }

    #[inline(always)]
    pub fn to_f64(&self) -> f64 {
        self.value.to_f64().expect("Failed to convert Decimal to f64")
    }
    
    #[inline(always)]
    pub fn sqrt(&self) -> Self {
        Self {
            value: self.value.sqrt().expect("Failed to compute sqrt")
        }
    }
}

impl From<f64> for IndicatorValue {
    fn from(value: f64) -> Self {
        Self {
            value: Decimal::from_f64(value).expect("Failed to convert f64 to Decimal")
        }
    }
}

impl From<&str> for IndicatorValue {
    fn from(value: &str) -> Self {
        Self {
            value: Decimal::from_str(value).expect("Invalid number format")
        }
    }
}

impl From<usize> for IndicatorValue {
    #[inline(always)]
    fn from(value: usize) -> Self {
        Self {
            value: Decimal::from_usize(value).expect("Failed to convert f64 to Decimal")
        }
    }
}

impl From<u64> for IndicatorValue {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self {
            value: Decimal::from_u64(value).expect("Failed to convert f64 to Decimal")
        }
    }
}

impl PartialEq for IndicatorValue {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for IndicatorValue {}

impl PartialOrd for IndicatorValue {

    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
    
    #[inline(always)]
    fn lt(&self, other: &Self) -> bool {
        self.value < other.value
    }

    #[inline(always)]
    fn le(&self, other: &Self) -> bool {
        self.value <= other.value
    }

    #[inline(always)]
    fn gt(&self, other: &Self) -> bool {
        self.value > other.value
    }

    #[inline(always)]
    fn ge(&self, other: &Self) -> bool {
        self.value >= other.value
    }
}

impl Neg for IndicatorValue {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self{
            value: self.value.neg()
        }
    }
}

impl Add for IndicatorValue {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self::Output {
        Self {
            value: self.value + other.value,
        }
    }
}

impl Sub for IndicatorValue {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        Self {
            value: self.value - other.value,
        }
    }
}

impl Mul for IndicatorValue {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self::Output {
        Self {
            value: self.value * other.value,
        }
    }
}

impl Div for IndicatorValue {
    type Output = Self;

    #[inline(always)]
    fn div(self, other: Self) -> Self::Output {
        Self {
            value: self.value / other.value,
        }
    }
}

impl AddAssign for IndicatorValue {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}

impl SubAssign for IndicatorValue {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value;
    }
}
impl MulAssign for IndicatorValue {
    #[inline(always)]
    fn mul_assign(&mut self, other: Self) {
        self.value *= other.value;
    }
}
impl DivAssign for IndicatorValue {
    #[inline(always)]
    fn div_assign(&mut self, other: Self) {
        self.value /= other.value;
    }
}