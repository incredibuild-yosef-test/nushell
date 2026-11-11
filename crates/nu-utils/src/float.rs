use std::fmt::{Display, LowerExp};

/// A f64 wrapper that formats whole numbers with a decimal point.
///
/// `ObviousFloat(3.0)` displays as `3.0` rather than `3`, so a float can never be
/// mistaken for an integer in output. Non-whole values use the normal `f64` formatting.
pub struct ObviousFloat(pub f64);

/// Whole numbers get exactly one decimal place; everything else defers to `f64`.
impl Display for ObviousFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let val = self.0;
        if val.fract() == 0.0 {
            write!(f, "{val:.1}")
        } else {
            Display::fmt(&val, f)
        }
    }
}

/// Scientific notation is unaffected by the wrapper.
impl LowerExp for ObviousFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerExp::fmt(&self.0, f)
    }
}
