use crate::proj::{
    AxisDirection, CoordinateStep, IoUnits, Proj, ProjectionTransform, Step, TransformCoordinates,
};
use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;

/// An axis swapping guide
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AxisSwap {
    /// The axis guide itself. A 4-tuple of axis directions to swap
    pub axis: [i64; 4],
    /// The sign guide for faster comparisons
    pub sign: [i32; 4],
}
impl Default for AxisSwap {
    fn default() -> Self {
        Self {
            axis: [0, 1, 2, 3], // identity mapping
            sign: [1; 4],
        }
    }
}
impl AxisSwap {
    /// Check if the axis swap is "empty" or in the default state
    pub fn is_empty(&self) -> bool {
        self.axis == [0, 1, 2, 3] && self.sign == [1; 4]
    }
}
impl From<Vec<AxisDirection>> for AxisSwap {
    fn from(directions: Vec<AxisDirection>) -> Self {
        if directions.len() > 4 {
            panic!("Too many axes specified");
        }

        let mut axis = [0, 1, 2, 3]; // identity mapping
        let mut sign = [1; 4];

        for (i, dir) in directions.iter().enumerate() {
            match dir {
                AxisDirection::West => {
                    axis[i] = 0;
                    sign[i] = -1;
                }
                AxisDirection::East => {
                    axis[i] = 0;
                    sign[i] = 1;
                }
                AxisDirection::South => {
                    axis[i] = 1;
                    sign[i] = -1;
                }
                AxisDirection::North => {
                    axis[i] = 1;
                    sign[i] = 1;
                }
                AxisDirection::Down => {
                    axis[i] = 2;
                    sign[i] = -1;
                }
                AxisDirection::Up => {
                    axis[i] = 2;
                    sign[i] = 1;
                }
                _ => {} // Do nothing
            }
        }

        // Check for duplicates
        for a in 0..directions.len() {
            for b in (a + 1)..directions.len() {
                if axis[a] == axis[b] {
                    panic!("Duplicate axis mapping");
                }
            }
        }

        Self { axis, sign }
    }
}

/// An axis swapping converter
#[derive(Debug, Clone, PartialEq)]
pub struct AxisSwapConverter {
    proj: Rc<RefCell<Proj>>,
    /// The axis swapping guide
    pub swap: AxisSwap,
}
impl From<AxisSwapConverter> for ProjectionTransform {
    fn from(asc: AxisSwapConverter) -> ProjectionTransform {
        let mut proj_trans = ProjectionTransform::default();
        proj_trans.proj = asc.proj.clone();
        proj_trans.method = Step::AxisSwap(asc.into());
        proj_trans
    }
}
impl CoordinateStep for AxisSwapConverter {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        {
            let proj = &mut proj.borrow_mut();
            proj.left = IoUnits::RADIANS;
            proj.right = IoUnits::RADIANS;
            proj.is_ll = true;
        }
        AxisSwapConverter { proj, swap: AxisSwap::default() }
    }
    /// Handle the axis swap
    fn forward<P: TransformCoordinates>(&self, coords: &mut P) {
        if self.swap.is_empty() {
            return;
        }

        let mut out = [0.0; 4];
        for i in 0..4 {
            let src_idx = self.swap.axis[i] as usize;
            let sign = self.swap.sign[i] as f64;
            out[i] = coords.get(src_idx) * sign;
        }

        for i in 0..4 {
            coords.set(i, out[i]);
        }
    }
    /// Handle the axis swap
    fn inverse<P: TransformCoordinates>(&self, coords: &mut P) {
        if self.swap.is_empty() {
            return;
        }

        let mut out = [0.0; 4];
        for i in 0..4 {
            let dst_idx = self.swap.axis[i] as usize;
            let sign = self.swap.sign[i] as f64;
            out[dst_idx] = coords.get(i) * sign;
        }

        for i in 0..4 {
            coords.set(i, out[i]);
        }
    }
}
