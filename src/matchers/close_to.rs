use num::{Float, Zero};
use std::fmt::{self, Display, Debug, Formatter};
use std::{f32, f64};
use {success, Matcher, MatchResult};

/// Compares two floating point values for equality.
///
/// The comparison is based on a relative error metric and uses special
/// fallbacks for certain edge cases like very small numbers. The exact
/// algorithm is described [here](http://floating-point-gui.de/errors/comparison/).
pub struct CloseTo<T> {
    expected: T,
    epsilon: T,
}

impl<T: Debug> Display for CloseTo<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.expected.fmt(f)
    }
}

/// This is just a fix until rust-lang/num#93 is fixed.
pub trait FloatMinPositive {
    /// Returns the smallest positive, normalized value that this type can represent.
    fn min_positive_value() -> Self;
}

impl FloatMinPositive for f32 {
    fn min_positive_value() -> Self {
        f32::MIN_POSITIVE
    }
}

impl FloatMinPositive for f64 {
    fn min_positive_value() -> Self {
        f64::MIN_POSITIVE
    }
}

impl<T: Float + Zero + FloatMinPositive + Debug> Matcher<T> for CloseTo<T> {
    fn matches(&self, actual: T) -> MatchResult {
        let a = self.expected.abs();
        let b = actual.abs();

        let d = (a - b).abs();

        let close =
            // shortcut, handles infinities
            a == b
            // a or b is zero or both are extremely close to it
            // relative error is less meaningful here
            || ((a == Zero::zero() || b == Zero::zero() || d < FloatMinPositive::min_positive_value()) &&
                d < (self.epsilon * FloatMinPositive::min_positive_value()))
            // use relative error
            || d / (a + b).min(Float::max_value()) < self.epsilon;

        if close {
            success()
        } else {
            Err(format!("was {:?}", actual))
        }
    }
}

pub fn close_to<T>(expected: T, epsilon: T) -> CloseTo<T> {
    CloseTo {
        expected: expected,
        epsilon: epsilon
    }
}

#[cfg(test)]
mod test {
    use std::f64;
    use {assert_that,is,not,close_to};

    #[test]
    fn test_equality_of_floats() {
        assert_that(1.0f64, is(close_to(1.0, 0.00001)));
        assert_that(f64::INFINITY, is(close_to(f64::INFINITY, 0.00001)));
        assert_that(1e-40f32, is(close_to(0.0, 0.01)));
        assert_that(1e-40f32, is(not(close_to(0.0, 0.000001))));
        assert_that(2.0, is(not(close_to(1.0f64, 0.00001))));
        assert_that(f64::NAN, is(not(close_to(f64::NAN, 0.00001))));
    }
}