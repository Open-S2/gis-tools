use core::f64::consts::PI;

use libm::{atan, floor, round, sqrt, tan};

use super::coords_internal::{K_FACE_UVW_AXES, K_FACE_UVW_FACES};
use crate::geometry::{Point, S2Point};

// This file contains documentation of the various coordinate systems used
// throughout the library.  Most importantly, S2 defines a framework for
// decomposing the unit sphere into a hierarchy of "cells".  Each cell is a
// quadrilateral bounded by four geodesics.  The top level of the hierarchy is
// obtained by projecting the six faces of a cube onto the unit sphere, and
// lower levels are obtained by subdividing each cell into four children
// recursively.  Cells are numbered such that sequentially increasing cells
// follow a continuous space-filling curve over the entire sphere.  The
// transformation is designed to make the cells at each level fairly uniform
// in size.
//
//
////////////////////////// S2Cell Decomposition /////////////////////////
//
// The following methods define the cube-to-sphere projection used by
// the S2Cell decomposition.
//
// In the process of converting a latitude-longitude pair to a 64-bit cell
// id, the following coordinate systems are used:
//
//  (id)
//    An S2CellId is a 64-bit encoding of a face and a Hilbert curve position
//    on that face.  The Hilbert curve position implicitly encodes both the
//    position of a cell and its subdivision level (see s2cell_id.h).
//
//  (face, i, j)
//    Leaf-cell coordinates.  "i" and "j" are integers in the range
//    [0,(2**30)-1] that identify a particular leaf cell on the given face.
//    The (i, j) coordinate system is right-handed on each face, and the
//    faces are oriented such that Hilbert curves connect continuously from
//    one face to the next.
//
//  (face, s, t)
//    Cell-space coordinates.  "s" and "t" are real numbers in the range
//    [0,1] that identify a point on the given face.  For example, the point
//    (s, t) = (0.5, 0.5) corresponds to the center of the top-level face
//    cell.  This point is also a vertex of exactly four cells at each
//    subdivision level greater than zero.
//
//  (face, si, ti)
//    Discrete cell-space coordinates.  These are obtained by multiplying
//    "s" and "t" by 2**31 and rounding to the nearest u32eger.
//    Discrete coordinates lie in the range [0,2**31].  This coordinate
//    system can represent the edge and center positions of all cells with
//    no loss of precision (including non-leaf cells).  In binary, each
//    coordinate of a level-k cell center ends with a 1 followed by
//    (30 - k) 0s.  The coordinates of its edges end with (at least)
//    (31 - k) 0s.
//
//  (face, u, v)
//    Cube-space coordinates in the range [-1,1].  To make the cells at each
//    level more uniform in size after they are projected onto the sphere,
//    we apply a nonlinear transformation of the form u=f(s), v=f(t).
//    The (u, v) coordinates after this transformation give the actual
//    coordinates on the cube face (modulo some 90 degree rotations) before
//    it is projected onto the unit sphere.
//
//  (face, u, v, w)
//    Per-face coordinate frame.  This is an extension of the (face, u, v)
//    cube-space coordinates that adds a third axis "w" in the direction of
//    the face normal.  It is always a right-handed 3D coordinate system.
//    Cube-space coordinates can be converted to this frame by setting w=1,
//    while (u,v,w) coordinates can be projected onto the cube face by
//    dividing by w, i.e. (face, u/w, v/w).
//
//  (x, y, z)
//    Direction vector (S2Point).  Direction vectors are not necessarily unit
//    length, and are often chosen to be points on the biunit cube
//    [-1,+1]x[-1,+1]x[-1,+1].  They can be be normalized to obtain the
//    corresponding point on the unit sphere.
//
//  (lon, lat)
//    Latitude and longitude (lonlat).  Latitudes must be between -90 and
//    90 degrees inclusive, and longitudes must be between -180 and 180
//    degrees inclusive.
//
// Note that the (i, j), (s, t), (si, ti), and (u, v) coordinate systems are
// right-handed on all six faces.

// S2 is a namespace for constants and simple utility functions that are used
// throughout the S2 library.  The name "S2" is derived from the mathematical
// symbol for the two-dimensional unit sphere (note that the "2" refers to the
// dimension of the surface, not the space it is embedded in).

/// This is the number of levels needed to specify a leaf cell.  This
/// constant is defined here so that the S2::Metric class and the conversion
/// functions below can be implemented without including s2cell_id.h.  Please
/// see s2cell_id.h for other useful constants and conversion functions.
pub const K_MAX_CELL_LEVEL: u8 = 30;

/// The maximum index of a valid leaf cell plus one.  The range of valid leaf
/// cell indices is [0..kLimitIJ-1].
pub const K_LIMIT_IJ: u32 = 1 << K_MAX_CELL_LEVEL; // == S2CellId::kMaxSize

/// The maximum value of an si- or ti-coordinate.  The range of valid (si,ti)
/// values is [0..kMaxSiTi].
pub const K_MAX_SI_TI: u32 = 1 << (K_MAX_CELL_LEVEL + 1);

/// We have implemented three different projections from cell-space (s,t) to
/// cube-space (u,v): linear, quadratic, and tangent.  They have the following
/// tradeoffs:
///
///   Linear - This is the fastest transformation, but also produces the least
///   uniform cell sizes.  Cell areas vary by a factor of about 5.2, with the
///   largest cells at the center of each face and the smallest cells in
///   the corners.
///
///   Tangent - Transforming the coordinates via atan() makes the cell sizes
///   more uniform.  The areas vary by a maximum ratio of 1.4 as opposed to a
///   maximum ratio of 5.2.  However, each call to atan() is about as expensive
///   as all of the other calculations combined when converting from points to
///   cell ids, i.e. it reduces performance by a factor of 3.
///
///   Quadratic - This is an approximation of the tangent projection that
///   is much faster and produces cells that are almost as uniform in size.
///   It is about 3 times faster than the tangent projection for converting
///   cell ids to points or vice versa.  Cell areas vary by a maximum ratio of
///   about 2.1.
///
/// Here is a table comparing the cell uniformity using each projection.  "Area
/// ratio" is the maximum ratio over all subdivision levels of the largest cell
/// area to the smallest cell area at that level, "edge ratio" is the maximum
/// ratio of the longest edge of any cell to the shortest edge of any cell at
/// the same level, and "diag ratio" is the ratio of the longest diagonal of
/// any cell to the shortest diagonal of any cell at the same level.  "ToPoint"
/// and "FromPoint" are the times in microseconds required to convert cell ids
/// to and from points (unit vectors) respectively.  "ToPointRaw" is the time
/// to convert to a non-unit-length vector, which is all that is needed for
/// some purposes.
///              Area    Edge    Diag   ToPointRaw  ToPoint  FromPoint
///              Ratio   Ratio   Ratio             (microseconds)
/// -------------------------------------------------------------------
/// Linear:      5.200   2.117   2.959      0.020     0.087     0.085
/// Tangent:     1.414   1.414   1.704      0.237     0.299     0.258
/// Quadratic:   2.082   1.802   1.932      0.033     0.096     0.108
///
/// The worst-case cell aspect ratios are about the same with all three
/// projections.  The maximum ratio of the longest edge to the shortest edge
/// within the same cell is about 1.4 and the maximum ratio of the diagonals
/// within the same cell is about 1.7.
///
/// NOTE: Currently Tan only has 1e-12 accuracy while Quadratic is within 1e-15.
#[derive(Default)]
pub enum S2Projection {
    /// Linear projection
    S2LinearProjection,
    /// Tangent projection
    S2TanProjection,
    /// Quadratic projection
    #[default]
    S2QuadraticProjection,
}

/// Convert an s- or t-value to the corresponding u- or v-value.  This is
/// a non-linear transformation from [0,1] to [-1,1] that attempts to
/// make the cell sizes more uniform.
pub fn st_to_uvlinear(s: f64) -> f64 {
    2. * s - 1.
}
/// Convert an s- or t-value to the corresponding u- or v-value.  This is
/// a non-linear transformation from [0,1] to [-1,1] that attempts to
/// make the cell sizes more uniform.
pub fn st_to_uvquadratic(s: f64) -> f64 {
    if s >= 0.5 {
        (1.0 / 3.0) * (4.0 * s * s - 1.0)
    } else {
        (1.0 / 3.0) * (1.0 - 4.0 * (1.0 - s) * (1.0 - s))
    }
}
/// Convert an s- or t-value to the corresponding u- or v-value.  This is
/// a non-linear transformation from [0,1] to [-1,1] that attempts to
/// make the cell sizes more uniform.
pub fn st_to_uvtan(s_: f64) -> f64 {
    use core::f64::consts::PI;
    // Unfortunately, tan(M_PI_4) is slightly less than 1.0.  This isn't due to
    // a flaw in the implementation of tan(), it's because the derivative of
    // tan(x) at x=pi/4 is 2, and it happens that the two adjacent floating
    // point numbers on either side of the infinite-precision value of pi/4 have
    // tangents that are slightly below and slightly above 1.0 when rounded to
    // the nearest double-precision result.
    let s = tan(PI / 2.0 * s_ - PI / 4.0);
    s + (1.0 / (1_u64 << 53) as f64) * s
}

/// The default projection is quadratic
#[cfg(feature = "quadratic")]
pub const ST_TO_UV: fn(f64) -> f64 = st_to_uvquadratic;
/// If settings are updated you can use the tangent projection
#[cfg(feature = "tan")]
pub const ST_TO_UV: fn(f64) -> f64 = st_to_uvtan;
/// If settings are updated you can use the linear projection
#[cfg(feature = "linear")]
pub const ST_TO_UV: fn(f64) -> f64 = st_to_uvlinear;

/// The inverse of the STtoUV transformation.  Note that it is not always
/// true that UV_TO_ST(STtoUV(x)) == x due to numerical errors.
pub fn uv_to_stlinear(u: f64) -> f64 {
    0.5 * (u + 1.0)
}
/// The inverse of the STtoUV transformation.  Note that it is not always
/// true that UV_TO_ST(STtoUV(x)) == x due to numerical errors.
pub fn uv_to_st_quadratic(u: f64) -> f64 {
    if u >= 0. {
        0.5 * sqrt(1.0 + 3.0 * u)
    } else {
        1.0 - 0.5 * sqrt(1.0 - 3.0 * u)
    }
}
/// The inverse of the STtoUV transformation.  Note that it is not always
/// true that UV_TO_ST(STtoUV(x)) == x due to numerical errors.
pub fn uv_to_st_tan(u: f64) -> f64 {
    let a: f64 = atan(u);
    (2.0 * (1.0 / PI)) * (a + (PI / 4.0))
}

/// The default projection is quadratic
#[cfg(feature = "quadratic")]
pub const UV_TO_ST: fn(f64) -> f64 = uv_to_st_quadratic;
/// If settings are updated you can use the tangent projection
#[cfg(feature = "tan")]
pub const UV_TO_ST: fn(f64) -> f64 = uv_to_st_tan;
/// If settings are updated you can use the linear projection
#[cfg(feature = "linear")]
pub const UV_TO_ST: fn(f64) -> f64 = uv_to_stlinear;

/// Convert the i- or j-index of a leaf cell to the minimum corresponding s-
/// or t-value contained by that cell.  The argument must be in the range
/// [0..2**30], i.e. up to one position beyond the normal range of valid leaf
/// cell indices.
pub fn ij_to_st(i: u32) -> f64 {
    if !(0..=K_LIMIT_IJ).contains(&i) {
        unreachable!();
    }
    i as f64 / K_LIMIT_IJ as f64
}

/// Return the i- or j-index of the leaf cell containing the given
/// s- or t-value.  If the argument is outside the range spanned by valid
/// leaf cell indices, return the index of the closest valid leaf cell (i.e.,
/// return values are clamped to the range of valid leaf cell indices).
pub fn st_to_ij(s: f64) -> u32 {
    let k_limit_ij = K_LIMIT_IJ as f64;
    (k_limit_ij * s).clamp(0.0, k_limit_ij - 1.0) as u32
}

/// Convert an si- or ti-value to the corresponding s- or t-value.
pub fn si_ti_to_st(si: u32) -> f64 {
    if si > K_MAX_SI_TI {
        unreachable!();
    }
    (1.0 / K_MAX_SI_TI as f64) * (si as f64)
    // return (1 / 2_147_483_648) * si;
}

/// Return the si- or ti-coordinate that is nearest to the given s- or
/// t-value.  The result may be outside the range of valid (si,ti)-values.
pub fn st_to_si_ti(s: f64) -> u32 {
    round(s * K_MAX_SI_TI as f64) as u32
}

/// Convert a direction vector (not necessarily unit length) to an (s,t) point.
pub struct ST {
    /// the s coordinate
    pub s: f64,
    /// the t coordinate
    pub t: f64,
}

/// Convert an S2Point to an (s,t) point.
pub fn to_face_st(p: &S2Point, face: u8) -> ST {
    let uv = to_face_uv(p, face);
    ST { s: UV_TO_ST(uv.u), t: UV_TO_ST(uv.v) }
}

/// A U-V coordinate pair.
pub struct UV {
    /// the u coordinate
    pub u: f64,
    /// the v coordinate
    pub v: f64,
}

/// Convert an S2Point to an (u,v) point.
pub fn to_face_uv(p: &S2Point, face: u8) -> UV {
    let (valid, u, v) = face_xyz_to_uv(face, p);
    debug_assert!(valid, "face_xyz_to_uv failed");
    UV { u, v }
}

/// Convert (face, u, v) coordinates to a direction vector (not
/// necessarily unit length).
pub fn face_uv_to_xyz(face: u8, u: f64, v: f64) -> S2Point {
    match face {
        0 => S2Point::new(1.0, u, v),
        1 => S2Point::new(-u, 1.0, v),
        2 => S2Point::new(-u, -v, 1.0),
        3 => S2Point::new(-1.0, -v, -u),
        4 => S2Point::new(v, -1.0, -u),
        _ => S2Point::new(v, u, -1.0),
    }
}

/// Given a *valid* face for the given point p (meaning that dot product
/// of p with the face normal is positive), return the corresponding
/// u and v values (which may lie outside the range [-1,1]).
/// Returns (pu, pv).
pub fn valid_face_xyz_to_uv(face: u8, p: &S2Point) -> (f64, f64) {
    if p.dot(&get_norm(face)) <= 0.0 {
        unreachable!();
    }
    match face {
        0 => (p.y / p.x, p.z / p.x),
        1 => (-p.x / p.y, p.z / p.y),
        2 => (-p.x / p.z, -p.y / p.z),
        3 => (p.z / p.x, p.y / p.x),
        4 => (p.z / p.y, -p.x / p.y),
        _ => (-p.y / p.z, -p.x / p.z),
    }
}

/// Return the face containing the given direction vector.  (For points on
/// the boundary between faces, the result is arbitrary but repeatable.)
pub fn get_face(p: &S2Point) -> u8 {
    let mut face: u8 = p.largest_abs_component();
    if p.face(face) < 0.0 {
        face += 3;
    }
    face
}

/// Convert a direction vector (not necessarily unit length) to
/// (face, u, v) coordinates.
/// Returns (face, pu, pv)
pub fn xyz_to_face_uv(p: &S2Point) -> (u8, f64, f64) {
    let face: u8 = get_face(p);
    let (pu, pv) = valid_face_xyz_to_uv(face, p);
    (face, pu, pv)
}

/// Convert a direction vector (not necessarily unit length) to
/// (face, u, v) coordinates.
/// Returns (face, ps, pt)
pub fn xyz_to_face_st(p: &S2Point) -> (u8, f64, f64) {
    let face: u8 = get_face(p);
    let (pu, pv) = valid_face_xyz_to_uv(face, p);
    (face, UV_TO_ST(pu), UV_TO_ST(pv))
}

/// If the dot product of p with the given face normal is positive,
/// set the corresponding u and v values (which may lie outside the range
/// [-1,1]) and return true.  Otherwise return false.
pub fn face_xyz_to_uv(face: u8, p: &S2Point) -> (bool, f64, f64) {
    if face < 3 {
        if p.face(face) <= 0.0 {
            return (false, 0.0, 0.0);
        }
    } else if p.face(face - 3) >= 0.0 {
        return (false, 0.0, 0.0);
    }
    let (pu, pv) = valid_face_xyz_to_uv(face, p);
    (true, pu, pv)
}

/// Transform the given point P to the (u,v,w) coordinate frame of the given
/// face (where the w-axis represents the face normal).
pub fn face_xyz_to_uvw(face: u8, p: &S2Point) -> S2Point {
    // The result coordinates are simply the dot products of P with the (u,v,w)
    // axes for the given face (see kFaceUVWAxes).
    match face {
        0 => S2Point::new(p.y, p.z, p.x),
        1 => S2Point::new(-p.x, p.z, p.y),
        2 => S2Point::new(-p.x, -p.y, p.z),
        3 => S2Point::new(-p.z, -p.y, -p.x),
        4 => S2Point::new(-p.z, p.x, -p.y),
        _ => S2Point::new(p.y, p.x, -p.z),
    }
}

/// Return the right-handed normal (not necessarily unit length) for an
/// edge in the direction of the positive v-axis at the given u-value on
/// the given face.  (This vector is perpendicular to the plane through
/// the sphere origin that contains the given edge.)
pub fn get_u_norm(face: u8, u: f64) -> S2Point {
    match face {
        0 => S2Point::new(u, -1.0, 0.0),
        1 => S2Point::new(1.0, u, 0.0),
        2 => S2Point::new(1.0, 0.0, u),
        3 => S2Point::new(-u, 0.0, 1.0),
        4 => S2Point::new(0.0, -u, 1.0),
        _ => S2Point::new(0.0, -1., -u),
    }
}

/// Return the right-handed normal (not necessarily unit length) for an
/// edge in the direction of the positive u-axis at the given v-value on
/// the given face.
pub fn get_v_norm(face: u8, v: f64) -> S2Point {
    match face {
        0 => S2Point::new(-v, 0.0, 1.0),
        1 => S2Point::new(0.0, -v, 1.0),
        2 => S2Point::new(0.0, -1.0, -v),
        3 => S2Point::new(v, -1.0, 0.0),
        4 => S2Point::new(1.0, v, 0.0),
        _ => S2Point::new(1.0, 0.0, v),
    }
}

/// Return the unit-length normal for the given face.
pub fn get_norm(face: u8) -> S2Point {
    get_uvw_axis(face, 2)
}
/// Return the u-axis for the given face.
pub fn get_u_axis(face: u8) -> S2Point {
    get_uvw_axis(face, 0)
}
/// Return the v-axis for the given face.
pub fn get_v_axis(face: u8) -> S2Point {
    get_uvw_axis(face, 1)
}

/// Return the given axis of the given face (u=0, v=1, w=2).
pub fn get_uvw_axis(face: u8, axis: usize) -> S2Point {
    let p = K_FACE_UVW_AXES[face as usize][axis];
    S2Point::new(p[0], p[1], p[2])
}

/// With respect to the (u,v,w) coordinate system of a given face, return the
/// face that lies in the given direction (negative=0, positive=1) of the
/// given axis (u=0, v=1, w=2).  For example, GetUVWFace(4, 0, 1) returns the
/// face that is adjacent to face 4 in the positive u-axis direction.
pub fn get_uvw_face(face: u8, axis: usize, direction: usize) -> i32 {
    if !(0..=5).contains(&face) {
        unreachable!();
    }
    if !(0..=2).contains(&axis) {
        unreachable!();
    }
    if !(0..=1).contains(&direction) {
        unreachable!();
    }
    K_FACE_UVW_FACES[face as usize][axis][direction]
}

/// Convert a direction vector (not necessarily unit length) to
/// (face, si, ti) coordinates and, if p is exactly equal to the center of a
/// cell, return the level of this cell (31 otherwise as its outside the bounds of levels).
/// Return (face, zoom, si, ti).
pub fn xyz_to_face_si_ti(p: &S2Point) -> (u8, u8, u32, u32) {
    let (face, u, v) = xyz_to_face_uv(p);
    let si = st_to_si_ti(UV_TO_ST(u));
    let ti = st_to_si_ti(UV_TO_ST(v));
    // If the levels corresponding to si,ti are not equal, then p is not a cell
    // center.  The si,ti values 0 and kMaxSiTi need to be handled specially
    // because they do not correspond to cell centers at any valid level; they
    // are mapped to level -1 by the code below.
    // let level = K_MAX_CELL_LEVEL - (si.* | K_MAX_SI_TI);
    let level: i32 = K_MAX_CELL_LEVEL as i32 - ((si | K_MAX_SI_TI).trailing_zeros() as i32);
    // if (level < 0 or level != K_MAX_CELL_LEVEL - @ctz(ti.* | K_MAX_SI_TI)) return 31;
    if level < 0 || level != K_MAX_CELL_LEVEL as i32 - ((ti | K_MAX_SI_TI).trailing_zeros() as i32)
    {
        return (face, 31, si, ti);
    }
    // In infinite precision, this test could be changed to ST == SiTi. However,
    // due to rounding errors, UV_TO_ST(xyz_to_face_uv(face_uv_to_xyz(STtoUV(...)))) is
    // not idempotent. On the other hand, center_raw is computed exactly the same
    // way p was originally computed (if it is indeed the center of an S2Cell):
    // the comparison can be exact.
    // let center: &S2Point = FaceSiTitoXYZ(face, si, ti).Normalize();
    let mut center = face_si_ti_to_xyz(face, si, ti);
    center.normalize();
    if *p == center {
        (face, level as u8, si, ti)
    } else {
        (face, 31, si, ti)
    }
}

/// Convert (face, si, ti) coordinates to a direction vector (not necessarily
/// unit length).
pub fn face_si_ti_to_xyz(face: u8, si: u32, ti: u32) -> S2Point {
    let u = ST_TO_UV(si_ti_to_st(si));
    let v = ST_TO_UV(si_ti_to_st(ti));
    face_uv_to_xyz(face, u, v)
}

/// Convert an u-v-zoom coordinate to a tile coordinate
/// returns the tile X-Y coordinate
pub fn tile_xy_from_uv_zoom(u: f64, v: f64, zoom: u8) -> Point {
    tile_xy_from_st_zoom(UV_TO_ST(u), UV_TO_ST(v), zoom)
}

/// Convert an s-t-zoom coordinate to a tile coordinate
/// returns the tile X-Y coordinate
pub fn tile_xy_from_st_zoom(s: f64, t: f64, zoom: u8) -> Point {
    let division_factor = (2 / (1 << zoom)) as f64 * 0.5;

    (floor(s / division_factor), floor(t / division_factor))
}

// **** TEST FUNCTIONS ****

/// swap the i and j axes
pub fn swap_axes(ij: usize) -> usize {
    ((ij >> 1) & 1) + ((ij & 1) << 1)
}

/// invert the i and j axes
pub fn invert_bits(ij: usize) -> usize {
    ij ^ 3
}
