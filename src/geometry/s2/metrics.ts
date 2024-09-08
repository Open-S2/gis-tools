import { K_MAX_LEVEL } from '../id';

/**
 * The following are various constants that describe the shapes and sizes of
 * S2Cells (see s2coords.h and s2cell_id.h).  They are useful for deciding
 * which cell level to use in order to satisfy a given condition (e.g. that
 * cell vertices must be no further than "x" apart).  All of the raw constants
 * are differential quantities; you can use the getValue(level) method to
 * compute the corresponding length or area on the unit sphere for cells at a
 * given level.  The minimum and maximum bounds are valid for cells at all
 * levels, but they may be somewhat conservative for very large cells
 * (e.g. face cells).
 *
 * All of the values below were obtained by a combination of hand analysis and
 * Mathematica.  In general, S2_TAN_PROJECTION produces the most uniform
 * shapes and sizes of cells, S2_LINEAR_PROJECTION is considerably worse, and
 * S2_QUADRATIC_PROJECTION is somewhere in between (but generally closer to
 * the tangent projection than the linear one).
 *
 * Note that S2_LINEAR_PROJECTION can be useful for analysis even when another
 * projection is being used, since it allows many cell metrics to be bounded
 * in terms of (u,v) coordinates rather than (s,t) coordinates.  (With the
 * linear projection, u = 2 * s - 1 and similarly for v.)  Similarly,
 * S2_TAN_PROJECTION allows cell metrics to be bounded in terms of (u,v)
 * coordinate changes when they are measured as distances on the unit sphere.
 *
 * Defines a cell metric of the given dimension (1 == length, 2 == area).
 */
export class Metric {
  /**
   * @param dim - The dimension of the metric
   * @param deriv - The "deriv" value of a metric is a derivative, and must be multiplied by
   * a length or area in (s,t)-space to get a useful value.
   */
  constructor(
    public dim: number,
    public deriv: number,
  ) {}

  /**
   * Return the value of a metric for cells at the given level. The value is
   * either a length or an area on the unit sphere, depending on the
   * particular metric.
   * @param level - The level at which to compute the metric
   * @returns The value of the metric
   */
  getValue(level: number): number {
    return this.deriv * Math.pow(2, -this.dim * level);
  }

  /**
   * Return the level at which the metric has approximately the given value.
   * For example, K_AVG_EDGE.getClosestLevel(0.1) returns the level at which
   * the average cell edge length is approximately 0.1. The return value is
   * @param value - The value at which to compute the level
   * @returns - the minimum level such that the metric is at most the given value
   */
  getClosestLevel(value: number): number {
    return this.getLevelForMaxValue((this.dim == 1 ? Math.sqrt(2) : 2) * value);
  }

  /**
   * Return the minimum level such that the metric is at most the given value,
   * or S2CellId::kMaxLevel if there is no such level. For example,
   * K_MAX_DIAG.getLevelForMaxValue(0.1) returns the minimum level such
   * that all cell diagonal lengths are 0.1 or smaller.  The return value
   * is always a valid level.
   * @param value - The value at which to compute the level
   * @returns - the minimum level such that the metric is at most the given value
   */
  getLevelForMaxValue(value: number): number {
    if (value <= 0) return K_MAX_LEVEL;
    const { min, max, log2, floor } = Math;
    // This code is equivalent to computing a floating-point "level" value and
    // rounding up.  ilogb() returns the exponent corresponding to a fraction in
    // the range [1,2).
    // level = ilogb(value / this.deriv);
    let level = floor(log2(value / this.deriv));
    level = max(0, min(K_MAX_LEVEL, -(level >> (this.dim - 1))));
    return level;
  }

  /**
   * Return the maximum level such that the metric is at least the given value,
   * or 0 if there is no such level.  For example,
   * K_MAX_DIAG.getLevelForMinValue(0.1) returns the maximum level such that
   * all cells have a minimum width of 0.1 or larger.  The return value is
   * always a valid level.
   * @param value - The value at which to compute the level
   * @returns - the maximum level such that the metric is at least the given value
   */
  getLevelForMinValue(value: number): number {
    if (value <= 0) return K_MAX_LEVEL;
    const { min, max, log2, floor } = Math;
    // This code is equivalent to computing a floating-point "level" value and
    // rounding down.
    // level = ilogb(self.deriv / value);
    let level = floor(log2(value / this.deriv));
    level = max(0, min(K_MAX_LEVEL, level >> (this.dim - 1)));
    return level;
  }
}

/** 1D Length Metric */
export class LengthMetric extends Metric {
  /**
   * @param deriv - The "deriv" value of a metric is a derivative, and must be multiplied by
   * a length or area in (s,t)-space to get a useful value.
   */
  constructor(deriv: number) {
    super(1, deriv);
  }
}

/** 2D Area Metric */
export class AreaMetric extends Metric {
  /**
   * @param deriv - The "deriv" value of a metric is a derivative, and must be multiplied by
   * a length or area in (s,t)-space to get a useful value.
   */
  constructor(deriv: number) {
    super(2, deriv);
  }
}

/**
 * Each cell is bounded by four planes passing through its four edges and
 * the center of the sphere.  These metrics relate to the angle between each
 * pair of opposite bounding planes, or equivalently, between the planes
 * corresponding to two different s-values or two different t-values.  For
 * example, the maximum angle between opposite bounding planes for a cell at
 * level k is K_MAX_ANGLE_SPAN.getValue(k), and the average angle span for all
 * cells at level k is approximately kAvgAngleSpan.getValue(k).
 * Linear -> 1
 * Tan -> pi / 2.0 (1.571)
 * Quadratic -> 4.0 / 3.0 (1.333) [Default]
 */
export const K_MIN_ANGLE_SPAN = new LengthMetric(4 / 3);
/**
 * Each cell is bounded by four planes passing through its four edges and
 * the center of the sphere.  These metrics relate to the angle between each
 * pair of opposite bounding planes, or equivalently, between the planes
 * corresponding to two different s-values or two different t-values.  For
 * example, the maximum angle between opposite bounding planes for a cell at
 * level k is K_MAX_ANGLE_SPAN.getValue(k), and the average angle span for all
 * cells at level k is approximately kAvgAngleSpan.getValue(k).
 * Linear -> 2
 * Tan -> pi / 2.0 (1.571)
 * Quadratic -> 1.704897179199218452 [Default]
 */
export const K_MAX_ANGLE_SPAN = new LengthMetric(1.704897179199218);

export const K_AVG_ANGLE_SPAN = new LengthMetric(Math.PI / 2); // 1.571

// /// The width of geometric figure is defined as the distance between two
// /// parallel bounding lines in a given direction.  For cells, the minimum
// /// width is always attained between two opposite edges, and the maximum
// /// width is attained between two opposite vertices.  However, for our
// /// purposes we redefine the width of a cell as the perpendicular distance
// /// between a pair of opposite edges.  A cell therefore has two widths, one
// /// in each direction.  The minimum width according to this definition agrees
// /// with the classic geometric one, but the maximum width is different.  (The
// /// maximum geometric width corresponds to kMaxDiag defined below.)
// ///
// /// For a cell at level k, the distance between opposite edges is at least
// /// kMinWidth.getValue(k) and at most kMaxWidth.getValue(k).  The average
// /// width in both directions for all cells at level k is approximately
// /// kAvgWidth.getValue(k).
// ///
// /// The width is useful for bounding the minimum or maximum distance from a
// /// point on one edge of a cell to the closest point on the opposite edge.
// /// For example, this is useful when "growing" regions by a fixed distance.
// ///
// /// Note that because S2Cells are not usually rectangles, the minimum width of
// /// a cell is generally smaller than its minimum edge length.  (The interior
// /// angles of an S2Cell range from 60 to 120 degrees.)
// pub const kMinWidth = LengthMetric.Init(if (S2_PROJECTION == .S2_LINEAR_PROJECTION)
//     @sqrt(2.0 / 3.0) // 0.816
// else if (S2_PROJECTION == .S2_TAN_PROJECTION)
//     pi / (2.0 * @sqrt(2.0)) // 1.111
// else if (S2_PROJECTION == .S2_QUADRATIC_PROJECTION)
//     2.0 * @sqrt(2.0) / 3.0 // 0.943
// else
//     0.0);
// pub const kMaxWidth = LengthMetric.Init(kMaxAngleSpan.deriv);
// pub const kAvgWidth = LengthMetric.Init(if (S2_PROJECTION == .S2_LINEAR_PROJECTION)
//     1.411459345844456965 // 1.411
// else if (S2_PROJECTION == .S2_TAN_PROJECTION)
//     1.437318638925160885 // 1.437
// else if (S2_PROJECTION == .S2_QUADRATIC_PROJECTION)
//     1.434523672886099389 // 1.435
// else
//     0.0);

/**
 * The minimum edge length of any cell at level k is at least
 * kMinEdge.getValue(k), and the maximum is at most kMaxEdge.getValue(k).
 * The average edge length is approximately kAvgEdge.getValue(k).
 *
 * The edge length metrics can also be used to bound the minimum, maximum,
 * or average distance from the center of one cell to the center of one of
 * its edge neighbors.  In particular, it can be used to bound the distance
 * between adjacent cell centers along the space-filling Hilbert curve for
 * cells at any given level.
 *
 * linear -> 2.0 * sqrt(2.0) / 3.0     (0.943)
 * tan -> pi / (2.0 * sqrt(2.0))       (1.111)
 * quadratic -> 2.0 * sqrt(2.0) / 3.0  (0.943) [Default]
 */
export const K_MIN_EDGE = new LengthMetric((2 * Math.sqrt(2)) / 3);
export const K_MAX_EDGE = new LengthMetric(K_MAX_ANGLE_SPAN.deriv);
export const K_AVG_EDGE = new LengthMetric(1.459213746386106);

// /// The minimum diagonal length of any cell at level k is at least
// /// kMinDiag.getValue(k), and the maximum is at most kMaxDiag.getValue(k).
// /// The average diagonal length is approximately kAvgDiag.getValue(k).
// ///
// /// The maximum diagonal also happens to be the maximum diameter of any cell,
// /// and also the maximum geometric width (see the discussion above).  So for
// /// example, the distance from an arbitrary point to the closest cell center
// /// at a given level is at most half the maximum diagonal length.
// pub const kMinDiag = LengthMetric.Init(if (S2_PROJECTION == .S2_LINEAR_PROJECTION)
//     2.0 * @sqrt(2.0) / 3.0 // 0.943
// else if (S2_PROJECTION == .S2_TAN_PROJECTION)
//     pi * @sqrt(2.0) / 3.0 // 1.481
// else if (S2_PROJECTION == .S2_QUADRATIC_PROJECTION)
//     8.0 * @sqrt(2.0) / 9.0 // 1.257
// else
//     0.0);
// pub const kMaxDiag = LengthMetric.Init(if (S2_PROJECTION == .S2_LINEAR_PROJECTION)
//     2.0 * @sqrt(2.0) // 2.828
// else if (S2_PROJECTION == .S2_TAN_PROJECTION)
//     pi * @sqrt(2.0 / 3.0) // 2.565
// else if (S2_PROJECTION == .S2_QUADRATIC_PROJECTION)
//     2.438654594434021032 // 2.439
// else
//     0.0);
// pub const kAvgDiag = LengthMetric.Init(if (S2_PROJECTION == .S2_LINEAR_PROJECTION)
//     2.031817866418812674 // 2.032
// else if (S2_PROJECTION == .S2_TAN_PROJECTION)
//     2.063623197195635753 // 2.064
// else if (S2_PROJECTION == .S2_QUADRATIC_PROJECTION)
//     2.060422738998471683 // 2.060
// else
//     0.0);

// /// The minimum area of any cell at level k is at least kMinArea.getValue(k),
// /// and the maximum is at most kMaxArea.getValue(k).  The average area of all
// /// cells at level k is exactly kAvgArea.getValue(k).
// pub const kMinArea = AreaMetric.Init(if (S2_PROJECTION == .S2_LINEAR_PROJECTION)
//     4.0 / (3.0 * @sqrt(3.0)) // 0.770
// else if (S2_PROJECTION == .S2_TAN_PROJECTION)
//     (pi * pi) / (4.0 * @sqrt(2.0)) // 1.745
// else if (S2_PROJECTION == .S2_QUADRATIC_PROJECTION)
//     8.0 * @sqrt(2.0) / 9.0 // 1.257
// else
//     0.0);
// pub const kMaxArea = AreaMetric.Init(if (S2_PROJECTION == .S2_LINEAR_PROJECTION)
//     4.0 // 4.000
// else if (S2_PROJECTION == .S2_TAN_PROJECTION)
//     pi * pi / 4.0 // 2.467
// else if (S2_PROJECTION == .S2_QUADRATIC_PROJECTION)
//     2.635799256963161491 // 2.636
// else
//     0.0);
// pub const kAvgArea = AreaMetric.Init(4 * pi / 6); // 2.094

/**
 * This is the maximum edge aspect ratio over all cells at any level, where
 * the edge aspect ratio of a cell is defined as the ratio of its longest
 * edge length to its shortest edge length.
 * linear -> sqrt(2)
 * tan -> sqrt(2)
 * quadratic -> 1.4426 [default]
 */
export const K_MAX_EDGE_ASPECT = Math.sqrt(1.442615274452682);

/**
 * This is the maximum diagonal aspect ratio over all cells at any level,
 * where the diagonal aspect ratio of a cell is defined as the ratio of its
 * longest diagonal length to its shortest diagonal length.
 */
export const K_MAX_DIAG_ASPECT = Math.sqrt(3);
