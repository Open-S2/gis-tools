pub use crate::geometry::K_MAX_CELL_LEVEL;

use libm::{ilogb, ldexp, sqrt};

use core::f64::consts::{PI, SQRT_2, SQRT_3};

/// sqrt(2/3)
#[cfg(feature = "tan")]
const SQRT_2_3: f64 = 0.816496580927726;

/// The following are various constants that describe the shapes and sizes of
/// S2Cells (see s2coords.h and s2cell_id.h).  They are useful for deciding
/// which cell level to use in order to satisfy a given condition (e.g. that
/// cell vertices must be no further than "x" apart).  All of the raw constants
/// are differential quantities; you can use the getValue(level) method to
/// compute the corresponding length or area on the unit sphere for cells at a
/// given level.  The minimum and maximum bounds are valid for cells at all
/// levels, but they may be somewhat conservative for very large cells
/// (e.g. face cells).
///
/// All of the values below were obtained by a combination of hand analysis and
/// Mathematica.  In general, S2_TAN_PROJECTION produces the most uniform
/// shapes and sizes of cells, S2_LINEAR_PROJECTION is considerably worse, and
/// S2_QUADRATIC_PROJECTION is somewhere in between (but generally closer to
/// the tangent projection than the linear one).
///
/// Note that S2_LINEAR_PROJECTION can be useful for analysis even when another
/// projection is being used, since it allows many cell metrics to be bounded
/// in terms of (u,v) coordinates rather than (s,t) coordinates.  (With the
/// linear projection, u = 2 * s - 1 and similarly for v.)  Similarly,
/// S2_TAN_PROJECTION allows cell metrics to be bounded in terms of (u,v)
/// coordinate changes when they are measured as distances on the unit sphere.
///
/// Defines a cell metric of the given dimension (1 == length, 2 == area).
pub struct Metric<const DIM: i32> {
    /// The "deriv" value of a metric is a derivative, and must be multiplied by
    /// a length or area in (s,t)-space to get a useful value.
    pub deriv: f64,
}
impl<const DIM: i32> Metric<DIM> {
    /// Return the value of a metric for cells at the given level. The value is
    /// either a length or an area on the unit sphere, depending on the
    /// particular metric.
    pub fn get_value(&self, level: i32) -> f64 {
        self.deriv * ldexp(2.0, -DIM * level)
    }

    /// Return the level at which the metric has approximately the given value.
    /// For example, K_AVG_EDGE.getClosestLevel(0.1) returns the level at which
    /// the average cell edge length is approximately 0.1. The return value is
    pub fn get_closest_level(&self, value: f64) -> i32 {
        self.get_level_for_max_value((if DIM == 1 { sqrt(2.) } else { 2. }) * value)
    }

    /// Return the minimum level such that the metric is at most the given value,
    /// or S2CellId::kMaxLevel if there is no such level. For example,
    /// K_MAX_DIAG.get_level_for_max_value(0.1) returns the minimum level such
    /// that all cell diagonal lengths are 0.1 or smaller.  The return value
    /// is always a valid level.
    pub fn get_level_for_max_value(&self, value: f64) -> i32 {
        let k_max_cell_level: i32 = K_MAX_CELL_LEVEL.into();
        if value <= 0. {
            return k_max_cell_level;
        }
        let mut level = ilogb(value / self.deriv);
        level = (-(level >> (DIM - 1))).clamp(0, k_max_cell_level);

        level
    }

    /// Return the maximum level such that the metric is at least the given value,
    /// or 0 if there is no such level.  For example,
    /// K_MAX_DIAG.getLevelForMinValue(0.1) returns the maximum level such that
    /// all cells have a minimum width of 0.1 or larger.  The return value is
    /// always a valid level.
    pub fn get_level_for_min_value(&self, value: f64) -> i32 {
        let k_max_cell_level: i32 = K_MAX_CELL_LEVEL.into();
        if value <= 0. {
            return k_max_cell_level;
        }
        let mut level = ilogb(self.deriv / value);
        level = (level >> (DIM - 1)).clamp(0, k_max_cell_level);

        level
    }
}

/// Length based metrics
pub type LengthMetric = Metric<1>;
/// Area based metrics
pub type AreaMetric = Metric<2>;

/// Each cell is bounded by four planes passing through its four edges and
/// the center of the sphere.  These metrics relate to the angle between each
/// pair of opposite bounding planes, or equivalently, between the planes
/// corresponding to two different s-values or two different t-values.  For
/// example, the maximum angle between opposite bounding planes for a cell at
/// level k is kMaxAngleSpan.GetValue(k), and the average angle span for all
/// cells at level k is approximately kAvgAngleSpan.GetValue(k).
#[cfg(any(feature = "quadratic", feature = "tan", feature = "linear"))]
pub const K_MIN_ANGLE_SPAN: LengthMetric = LengthMetric {
    deriv: match () {
        #[cfg(feature = "quadratic")]
        _ => 4. / 3.,
        #[cfg(feature = "tan")]
        _ => PI / 2.,
        #[cfg(feature = "linear")]
        _ => 1.,
    },
};
/// Each cell is bounded by four planes passing through its four edges and
/// the center of the sphere.  These metrics relate to the angle between each
/// pair of opposite bounding planes, or equivalently, between the planes
/// corresponding to two different s-values or two different t-values.  For
/// example, the maximum angle between opposite bounding planes for a cell at
/// level k is kMaxAngleSpan.GetValue(k), and the average angle span for all
/// cells at level k is approximately kAvgAngleSpan.GetValue(k).
pub const K_MAX_ANGLE_SPAN: LengthMetric = LengthMetric {
    deriv: match () {
        #[cfg(feature = "quadratic")]
        _ => 1.704_897_179_199_218_5, // 1.705
        #[cfg(feature = "tan")]
        _ => PI / 2., // 1.571
        #[cfg(feature = "linear")]
        _ => 2., // 2
    },
};
/// Each cell is bounded by four planes passing through its four edges and
/// the center of the sphere.  These metrics relate to the angle between each
/// pair of opposite bounding planes, or equivalently, between the planes
/// corresponding to two different s-values or two different t-values.  For
/// example, the maximum angle between opposite bounding planes for a cell at
/// level k is kMaxAngleSpan.GetValue(k), and the average angle span for all
/// cells at level k is approximately kAvgAngleSpan.GetValue(k).
pub const K_AVG_ANGLE_SPAN: LengthMetric = LengthMetric { deriv: PI / 2. }; // 1.571

/// The width of geometric figure is defined as the distance between two
/// parallel bounding lines in a given direction.  For cells, the minimum
/// width is always attained between two opposite edges, and the maximum
/// width is attained between two opposite vertices.  However, for our
/// purposes we redefine the width of a cell as the perpendicular distance
/// between a pair of opposite edges.  A cell therefore has two widths, one
/// in each direction.  The minimum width according to this definition agrees
/// with the classic geometric one, but the maximum width is different.  (The
/// maximum geometric width corresponds to kMaxDiag defined below.)
///
/// For a cell at level k, the distance between opposite edges is at least
/// kMinWidth.getValue(k) and at most kMaxWidth.getValue(k).  The average
/// width in both directions for all cells at level k is approximately
/// kAvgWidth.getValue(k).
///
/// The width is useful for bounding the minimum or maximum distance from a
/// point on one edge of a cell to the closest point on the opposite edge.
/// For example, this is useful when "growing" regions by a fixed distance.
///
/// Note that because S2Cells are not usually rectangles, the minimum width of
/// a cell is generally smaller than its minimum edge length.  (The interior
/// angles of an S2Cell range from 60 to 120 degrees.)
///
/// Linear -> sqrt(2.0 / 3.0)            (0.816)
/// Tan ->  pi / (2.0 * @sqrt(2.0))      (1.111)
/// Quadratic -> 2.0 * @sqrt(2.0) / 3.0  (0.943) [Default]
pub const K_MIN_WIDTH: LengthMetric = LengthMetric {
    deriv: match () {
        #[cfg(feature = "quadratic")]
        _ => 2. * SQRT_2 / 3., // 0.943
        #[cfg(feature = "tan")]
        _ => PI / (2. * SQRT_2), // 1.111
        #[cfg(feature = "linear")]
        _ => sqrt(2. / 3.), // 0.816
    },
};

/// The same as kMaxAngleSpan's derivative.
pub const K_MAX_WIDTH: LengthMetric = LengthMetric { deriv: K_MAX_ANGLE_SPAN.deriv };

/// - Linear -> 1.411459345844456965
/// - Tan -> 1.437318638925160885
/// - Quadratic -> 1.434523672886099389
pub const K_AVG_WIDTH: LengthMetric = LengthMetric {
    deriv: match () {
        #[cfg(feature = "quadratic")]
        _ => 1.434_523_672_886_099_5, // 1.435
        #[cfg(feature = "tan")]
        _ => 1.437318638925160885, // 1.437
        #[cfg(feature = "linear")]
        _ => 1.411459345844456965, // 1.411
    },
};

/// The minimum edge length of any cell at level k is at least
/// kMinEdge.getValue(k), and the maximum is at most kMaxEdge.getValue(k).
/// The average edge length is approximately kAvgEdge.getValue(k).
///
/// The edge length metrics can also be used to bound the minimum, maximum,
/// or average distance from the center of one cell to the center of one of
/// its edge neighbors.  In particular, it can be used to bound the distance
/// between adjacent cell centers along the space-filling Hilbert curve for
/// cells at any given level.
///
/// - linear -> 2.0 * sqrt(2.0) / 3.0     (0.943)
/// - tan -> pi / (2.0 * sqrt(2.0))       (1.111)
/// - quadratic -> 2.0 * sqrt(2.0) / 3.0  (0.943) [Default]
pub const K_MIN_EDGE: LengthMetric = LengthMetric {
    deriv: match () {
        #[cfg(feature = "quadratic")]
        _ => 2. * SQRT_2 / 3., // 0.943
        #[cfg(feature = "tan")]
        _ => PI / (2. * SQRT_2), // 1.111
        #[cfg(feature = "linear")]
        _ => 2. * SQRT_2 / 3., // 0.943
    },
};

/// The same as kMaxAngleSpan's derivative.
pub const K_MAX_EDGE: LengthMetric = LengthMetric { deriv: K_MAX_ANGLE_SPAN.deriv };

/// Average edge length
pub const K_AVG_EDGE: LengthMetric = LengthMetric {
    deriv: match () {
        #[cfg(feature = "quadratic")]
        _ => 1.459_213_746_386_106_1, // 1.459
        #[cfg(feature = "tan")]
        _ => 1.461667032546739266, // 1.462
        #[cfg(feature = "linear")]
        _ => 1.440034192955603643, // 1.440
    },
};

/// The minimum diagonal length of any cell at level k is at least
/// kMinDiag.getValue(k), and the maximum is at most kMaxDiag.getValue(k).
/// The average diagonal length is approximately kAvgDiag.getValue(k).
///
/// The maximum diagonal also happens to be the maximum diameter of any cell,
/// and also the maximum geometric width (see the discussion above).  So for
/// example, the distance from an arbitrary point to the closest cell center
/// at a given level is at most half the maximum diagonal length.
///
/// - Linear -> 2.0 * @sqrt(2.0) / 3.0     (0.943)
/// - Tan -> pi * @sqrt(2.0) / 3.0         (1.481)
/// - Quadratic -> 8.0 * @sqrt(2.0) / 9.0  (1.257) [Default]
pub const K_MIN_DIAG: LengthMetric = LengthMetric {
    deriv: match () {
        #[cfg(feature = "quadratic")]
        _ => 8. * SQRT_2 / 9., // 1.257
        #[cfg(feature = "tan")]
        _ => PI * SQRT_2 / 3., // 1.481
        #[cfg(feature = "linear")]
        _ => 2. * SQRT_2 / 3., // 0.943
    },
};

/// - Linear -> 2.0 * @sqrt(2.0)        (2.828)
/// - Tan -> pi * @sqrt(2.0 / 3.0)      (2.565)
/// - Quadratic -> 2.438654594434021032 [Default]
pub const K_MAX_DIAG: LengthMetric = LengthMetric {
    deriv: match () {
        #[cfg(feature = "quadratic")]
        _ => 2.438_654_594_434_021, // 2.439
        #[cfg(feature = "tan")]
        _ => PI * SQRT_2_3, // 2.565
        #[cfg(feature = "linear")]
        _ => 2. * SQRT_2, // 2.828
    },
};

/// - Linear -> 2.031817866418812674
/// - Tan -> 2.063623197195635753
/// - Quadratic -> 2.060422738998471683 [Default]
pub const K_AVG_DIAG: LengthMetric = LengthMetric {
    deriv: match () {
        #[cfg(feature = "quadratic")]
        _ => 2.060_422_738_998_471_7, // 2.060
        #[cfg(feature = "tan")]
        _ => 2.063623197195635753, // 2.064
        #[cfg(feature = "linear")]
        _ => 2.031817866418812674, // 2.032
    },
};

/// The minimum area of any cell at level k is at least kMinArea.getValue(k),
/// and the maximum is at most kMaxArea.getValue(k).  The average area of all
/// cells at level k is exactly kAvgArea.getValue(k).
///
/// - Linear -> 4.0 / (3.0 * @sqrt(3.0))   (0.770)
/// - Tan -> pi * pi / (4.0 * @sqrt(2.0))  (1.745)
/// - Quadratic -> 8.0 * @sqrt(2.0) / 9.0  (1.257) [Default]
pub const K_MIN_AREA: AreaMetric = AreaMetric {
    deriv: match () {
        #[cfg(feature = "quadratic")]
        _ => 8. * SQRT_2 / 9., // 1.257
        #[cfg(feature = "tan")]
        _ => (PI * PI) / (4. * SQRT_2), // 1.745
        #[cfg(feature = "linear")]
        _ => 4. / (3. * SQRT_3), // 0.770
    },
};

/// Max Area Metric
/// - Linear -> 4.0
/// - Tan -> pi * pi / 4.0              (2.467)
/// - Quadratic -> 2.635799256963161491 [Default]
pub const K_MAX_AREA: AreaMetric = AreaMetric {
    deriv: match () {
        #[cfg(feature = "quadratic")]
        _ => 2.635_799_256_963_161_4, // 2.636
        #[cfg(feature = "tan")]
        _ => (PI * PI) / 4., // 2.467
        #[cfg(feature = "linear")]
        _ => 4., // 4.000
    },
};

/// returns the AreaMetric of 4 * Math.PI / 6) (true for all projections)
pub const K_AVG_AREA: AreaMetric = AreaMetric { deriv: 4. * PI / 6. };

/// This is the maximum edge aspect ratio over all cells at any level, where
/// the edge aspect ratio of a cell is defined as the ratio of its longest
/// edge length to its shortest edge length.
///
/// - linear ->    sqrt(2)
/// - tan ->       sqrt(2)
/// - quadratic -> 1.4426 [default]
#[cfg(feature = "quadratic")]
pub const K_MAX_EDGE_ASPECT: f64 = SQRT_2; // 1.414
#[cfg(feature = "tan")]
pub const K_MAX_EDGE_ASPECT: f64 = SQRT_2; // 1.414
#[cfg(feature = "linear")]
pub const K_MAX_EDGE_ASPECT: f64 = 1.442615274452682920;

/// The maximum aspect ratio of a diagonal of a cell.
pub const K_MAX_DIAG_ASPECT: f64 = SQRT_3;

#[cfg(test)]
#[allow(clippy::approx_constant)]
mod tests {
    use super::*;

    #[test]
    fn test_k_avg_angle_span() {
        // get_value
        assert_eq!(K_AVG_ANGLE_SPAN.get_value(0), 3.141592653589793);
        assert_eq!(K_AVG_ANGLE_SPAN.get_value(1), 1.5707963267948966);
        assert_eq!(K_AVG_ANGLE_SPAN.get_value(2), 0.7853981633974483);
        assert_eq!(K_AVG_ANGLE_SPAN.get_value(3), 0.39269908169872414);

        // get_closest_level
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(0.), 30);
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(3.141592653589793), 0);
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(1.5707963267948966), 0);
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(0.7853981633974483), 1);
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(0.77), 1);
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(0.44), 2);
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(0.39269908169872414), 2);
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(0.19634954084936207), 3);

        // get_level_for_max_value
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_max_value(0.), 30);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_max_value(3.141592653589793), 0);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_max_value(1.5707963267948966), 0);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_max_value(0.77), 2);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_max_value(0.44), 2);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_max_value(0.39269908169872414), 2);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_max_value(0.19634954084936207), 3);

        // get_level_for_min_value
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_min_value(0.), 30);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_min_value(1.5707963267948966), 0);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_min_value(0.77), 1);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_min_value(0.44), 1);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_min_value(0.39269908169872414), 2);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_min_value(0.19634954084936207), 3);
    }
}
