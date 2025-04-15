/// An axis swapping guide
#[derive(Debug, Clone, Copy)]
pub struct AxisSwap {
    /// The axis guide itself. A 4-tuple of axis directions to swap
    pub axis: [i64; 4],
    /// The sign guide for faster comparisons
    pub sign: [i32; 4],
}
