use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::cmp::Ordering;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct IndicatorValue(f64);

impl IndicatorValue {

    #[inline(always)]
    pub fn value(&self) -> f64 {
        self.0
    }

    #[inline(always)]
    pub fn to_f64(&self) -> f64 {
        self.0
    }

    #[inline(always)]
    pub fn sqrt(&self) -> Self {
        Self(self.0.sqrt())
    }
    
}

impl From<f64> for IndicatorValue {
    #[inline(always)]
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<&str> for IndicatorValue {
    #[inline(always)]
    fn from(value: &str) -> Self {
        Self(value.parse::<f64>().unwrap())
    }
}

impl From<usize> for IndicatorValue {
    #[inline(always)]
    fn from(value: usize) -> Self {
        Self(value as f64)
    }
}

impl From<u64> for IndicatorValue {
    #[inline(always)]
    fn from(value: u64) -> Self {
        IndicatorValue::from(value as f64)
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
        Self::from(-self.0)
    }
}

impl IndicatorValue {
    #[inline(always)]
    fn add_safe(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }

    #[inline(always)]
    fn sub_safe(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }

    #[inline(always)]
    fn mul_safe(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }

    #[inline(always)]
    fn div_safe(self, other: Self) -> Self {
        Self(self.0 / other.0)
    }
}

impl Add for IndicatorValue {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self::Output {
        self.add_safe(other)
    }
}

impl Sub for IndicatorValue {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self::Output {
        self.sub_safe(other)
    }
}

impl Mul for IndicatorValue {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self::Output {
        self.mul_safe(other)
    }
}

impl Div for IndicatorValue {
    type Output = Self;

    #[inline(always)]
    fn div(self, other: Self) -> Self::Output {
        self.div_safe(other)
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
