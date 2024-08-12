use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

pub struct IndicatorValue {
    value: f64,
}

impl IndicatorValue {

    #[inline(always)]
    pub fn get_value(&self) -> f64 {
        self.value
    }

    #[inline(always)]
    pub fn to_f64(&self) -> f64 {
        self.value
    }
}

impl From<f64> for IndicatorValue {
    #[inline(always)]
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl From<&str> for IndicatorValue {
    #[inline(always)]
    fn from(value: &str) -> Self {
        Self {
            value: value.parse::<f64>().unwrap()
        }
    }
}

impl IndicatorValue {
    #[inline(always)]
    fn add_safe(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }

    #[inline(always)]
    fn sub_safe(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }

    #[inline(always)]
    fn mul_safe(self, other: Self) -> Self {
        Self {
            value: self.value * other.value,
        }
    }

    #[inline(always)]
    fn div_safe(self, other: Self) -> Self {
        Self {
            value: self.value / other.value,
        }
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
