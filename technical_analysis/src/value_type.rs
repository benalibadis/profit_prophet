#[cfg(feature = "precision")]
mod precision {
    use rust_decimal::Decimal;
    use rust_decimal::prelude::FromPrimitive;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct ValueType(Decimal);

    impl ValueType {
        pub const fn new(value: Decimal) -> Self {
            ValueType(value)
        }

        pub const fn zero() -> Self {
            ValueType(Decimal::ZERO)
        }

        pub fn value(self) -> Decimal {
            self.0
        }
    }

    impl From<f64> for ValueType {
        fn from(value: f64) -> Self {
            ValueType(Decimal::from_f64(value).unwrap_or(Decimal::ZERO))
        }
    }

    #[cfg(not(feature = "unsafe"))]
    impl std::ops::Add for ValueType {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            ValueType(self.0.checked_add(other.0).unwrap())
        }
    }

    #[cfg(feature = "unsafe")]
    impl std::ops::Add for ValueType {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            ValueType(self.0 + other.0)
        }
    }

    #[cfg(not(feature = "unsafe"))]
    impl std::ops::Sub for ValueType {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            ValueType(self.0.checked_sub(other.0).unwrap())
        }
    }

    #[cfg(feature = "unsafe")]
    impl std::ops::Sub for ValueType {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            ValueType(self.0 - other.0)
        }
    }

    #[cfg(not(feature = "unsafe"))]
    impl std::ops::Mul for ValueType {
        type Output = Self;

        fn mul(self, other: Self) -> Self::Output {
            ValueType(self.0.checked_mul(other.0).unwrap())
        }
    }

    #[cfg(feature = "unsafe")]
    impl std::ops::Mul for ValueType {
        type Output = Self;

        fn mul(self, other: Self) -> Self::Output {
            ValueType(self.0 * other.0)
        }
    }

    // Division
    #[cfg(not(feature = "unsafe"))]
    impl std::ops::Div for ValueType {
        type Output = Self;

        fn div(self, other: Self) -> Self::Output {
            ValueType(self.0.checked_div(other.0).unwrap())
        }
    }

    #[cfg(feature = "unsafe")]
    impl std::ops::Div for ValueType {
        type Output = Self;

        fn div(self, other: Self) -> Self::Output {
            ValueType(self.0 / other.0)
        }
    }
}

#[cfg(not(feature = "precision"))]
mod no_precision {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct ValueType(f64);

    impl ValueType {
        pub const fn new(value: f64) -> Self {
            ValueType(value)
        }

        pub const fn zero() -> Self {
            ValueType(0.0)
        }

        pub fn value(self) -> f64 {
            self.0
        }
    }

    impl From<f64> for ValueType {
        fn from(value: f64) -> Self {
            ValueType(value)
        }
    }

    impl std::ops::Add for ValueType {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            ValueType(self.0 + other.0)
        }
    }

    impl std::ops::Sub for ValueType {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            ValueType(self.0 - other.0)
        }
    }

    impl std::ops::Mul for ValueType {
        type Output = Self;

        fn mul(self, other: Self) -> Self::Output {
            ValueType(self.0 * other.0)
        }
    }

    impl std::ops::Div for ValueType {
        type Output = Self;

        fn div(self, other: Self) -> Self::Output {
            ValueType(self.0 / other.0)
        }
    }
}

#[cfg(feature = "precision")]
pub use precision::ValueType;

#[cfg(not(feature = "precision"))]
pub use no_precision::ValueType;
