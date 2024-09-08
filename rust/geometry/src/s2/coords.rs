use core::f64::consts::PI;
use libm::{atan, round, sqrt};

use crate::s2::point::S2Point;

use super::coords_internal::{K_FACE_UVW_AXES, K_FACE_UVW_FACES};

// const std = @import("std");
// const S2_PROJECTION = @import("buildOptions").S2_PROJECTION;
// const testing = std.testing;
// const internal = @import("coordsInternal.zig");
// const kFaceUVWAxes = internal.kFaceUVWAxes;
// const kFaceUVWFaces = internal.kFaceUVWFaces;
// const kIJtoPos = internal.kIJtoPos;
// const kSwapMask = internal.kSwapMask;
// const kPosToIJ = internal.kPosToIJ;
// const kInvertMask = internal.kInvertMask;
// const FastUIntRound = internal.FastUIntRound;
// const S2Point = @import("../point.zig").S2Point;
// const R2Point = @import("../r2/point.zig").R2Point;

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
//  (lat, lng)
//    Latitude and longitude (S2LatLng).  Latitudes must be between -90 and
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

//////////////////   Implementation details follow   ////////////////////

// We have implemented three different projections from cell-space (s,t) to
// cube-space (u,v): linear, quadratic, and tangent.  They have the following
// tradeoffs:
//
//   Linear - This is the fastest transformation, but also produces the least
//   uniform cell sizes.  Cell areas vary by a factor of about 5.2, with the
//   largest cells at the center of each face and the smallest cells in
//   the corners.
//
//   Tangent - Transforming the coordinates via atan() makes the cell sizes
//   more uniform.  The areas vary by a maximum ratio of 1.4 as opposed to a
//   maximum ratio of 5.2.  However, each call to atan() is about as expensive
//   as all of the other calculations combined when converting from points to
//   cell ids, i.e. it reduces performance by a factor of 3.
//
//   Quadratic - This is an approximation of the tangent projection that
//   is much faster and produces cells that are almost as uniform in size.
//   It is about 3 times faster than the tangent projection for converting
//   cell ids to points or vice versa.  Cell areas vary by a maximum ratio of
//   about 2.1.
//
// Here is a table comparing the cell uniformity using each projection.  "Area
// ratio" is the maximum ratio over all subdivision levels of the largest cell
// area to the smallest cell area at that level, "edge ratio" is the maximum
// ratio of the longest edge of any cell to the shortest edge of any cell at
// the same level, and "diag ratio" is the ratio of the longest diagonal of
// any cell to the shortest diagonal of any cell at the same level.  "ToPoint"
// and "FromPoint" are the times in microseconds required to convert cell ids
// to and from points (unit vectors) respectively.  "ToPointRaw" is the time
// to convert to a non-unit-length vector, which is all that is needed for
// some purposes.
//
//               Area    Edge    Diag   ToPointRaw  ToPoint  FromPoint
//              Ratio   Ratio   Ratio             (microseconds)
// -------------------------------------------------------------------
// Linear:      5.200   2.117   2.959      0.020     0.087     0.085
// Tangent:     1.414   1.414   1.704      0.237     0.299     0.258
// Quadratic:   2.082   1.802   1.932      0.033     0.096     0.108
//
// The worst-case cell aspect ratios are about the same with all three
// projections.  The maximum ratio of the longest edge to the shortest edge
// within the same cell is about 1.4 and the maximum ratio of the diagonals
// within the same cell is about 1.7.
//
// NOTE: Currently Tan only has 1e-12 accuracy while Quadratic is within 1e-15.
#[derive(Default)]
pub enum S2Projection {
    S2LinearProjection,
    S2TanProjection,
    #[default]
    S2QuadraticProjection,
}

// Convert an s- or t-value to the corresponding u- or v-value.  This is
// a non-linear transformation from [0,1] to [-1,1] that attempts to
// make the cell sizes more uniform.
pub fn st_to_uvlinear(s: f64) -> f64 {
    2. * s - 1.
}
pub fn st_to_uvquadratic(s: f64) -> f64 {
    if s >= 0.5 {
        (1.0 / 3.0) * (4.0 * s * s - 1.0)
    } else {
        (1.0 / 3.0) * (1.0 - 4.0 * (1.0 - s) * (1.0 - s))
    }
}
// pub fn st_to_uvtan(s_: f64) -> f64 {
//     // Unfortunately, tan(M_PI_4) is slightly less than 1.0.  This isn't due to
//     // a flaw in the implementation of tan(), it's because the derivative of
//     // tan(x) at x=pi/4 is 2, and it happens that the two adjacent floating
//     // point numbers on either side of the infinite-precision value of pi/4 have
//     // tangents that are slightly below and slightly above 1.0 when rounded to
//     // the nearest double-precision result.
//     let s = tan((pi / 2.0) * s_ - (pi / 4.0));
//     return s + (1.0 / @as(f64, @floatFromInt(@as(i64, 1) << 53))) * s;
// }

// pub const ST_TO_UV: fn(f64) -> f64 = if (S2Projection == S2Projection::S2QuadraticProjection) {
//     st_to_uvquadratic
// } else if (S2Projection == S2Projection::S2TanProjection) {
//     st_to_uvtan
// } else {
//     st_to_uvlinear
// };
pub const ST_TO_UV: fn(f64) -> f64 = st_to_uvquadratic;

// The inverse of the STtoUV transformation.  Note that it is not always
// true that UV_TO_ST(STtoUV(x)) == x due to numerical errors.
pub fn uv_to_stlinear(u: f64) -> f64 {
    0.5 * (u + 1.0)
}
pub fn uv_to_st_quadratic(u: f64) -> f64 {
    if u >= 0. {
        0.5 * sqrt(1.0 + 3.0 * u)
    } else {
        1.0 - 0.5 * sqrt(1.0 - 3.0 * u)
    }
}
pub fn uv_to_st_tan(u: f64) -> f64 {
    let a: f64 = atan(u);
    (2.0 * (1.0 / PI)) * (a + (PI / 4.0))
}

// pub const UV_TO_ST: (fn(f64) -> f64) = if (S2Projection == S2Projection::S2QuadraticProjection) {
//     uv_to_stquadratic
// } else if (S2Projection == S2Projection::S2TanProjection) {
//     uvto_sttan
// } else {
//     uvto_stlinear
// };
pub const UV_TO_ST: fn(f64) -> f64 = uv_to_st_quadratic;

/// Convert the i- or j-index of a leaf cell to the minimum corresponding s-
/// or t-value contained by that cell.  The argument must be in the range
/// [0..2**30], i.e. up to one position beyond the normal range of valid leaf
/// cell indices.
// pub fn ij_to_st_min(i: u32) -> f64 {
//     if (i < 0 || i > kLimitIJ) { unreachable!(); }
//     return (1.0 / @as(f64, kLimitIJ)) * @as(f64, @floatFromInt(i));
// }

/// Return the i- or j-index of the leaf cell containing the given
/// s- or t-value.  If the argument is outside the range spanned by valid
/// leaf cell indices, return the index of the closest valid leaf cell (i.e.,
/// return values are clamped to the range of valid leaf cell indices).
pub fn st_to_ij(s: f64) -> u32 {
    (round(K_LIMIT_IJ as f64 * s - 0.5) as u32).clamp(0, K_LIMIT_IJ - 1)
}

/// Convert an si- or ti-value to the corresponding s- or t-value.
pub fn si_ti_to_st(si: u32) -> f64 {
    if si > K_MAX_SI_TI {
        unreachable!();
    }
    (1.0 / K_LIMIT_IJ as f64) * si as f64
}

/// Return the si- or ti-coordinate that is nearest to the given s- or
/// t-value.  The result may be outside the range of valid (si,ti)-values.
// pub fn st_to_si_ti(s: f64) -> u32 {
//     FastUIntRound(s * kMaxSiTi)
// }

pub struct ST {
    pub s: f64,
    pub t: f64,
}

pub fn to_face_st(p: &S2Point, face_: u8) -> ST {
    let uv = to_face_uv(p, face_);
    ST {
        s: UV_TO_ST(uv.u),
        t: UV_TO_ST(uv.v),
    }
}

pub struct UV {
    pub u: f64,
    pub v: f64,
}

pub fn to_face_uv(p: &S2Point, face_: u8) -> UV {
    let mut u: f64 = 0.;
    let mut v: f64 = 0.;
    if !face_xyz_to_uv(face_, p, &mut u, &mut v) {
        unreachable!();
    }
    _ = face_xyz_to_uv(face_, p, &mut u, &mut v);
    UV { u, v }
}

/// Convert (face, u, v) coordinates to a direction vector (not
/// necessarily unit length).
pub fn face_uvto_xyz(face: u8, u: f64, v: f64) -> S2Point {
    match face {
        0 => S2Point::new(1.0, u, v),
        1 => S2Point::new(-u, 1.0, v),
        2 => S2Point::new(-u, -v, 1.0),
        3 => S2Point::new(-1.0, -v, -u),
        4 => S2Point::new(v, -1.0, -u),
        _ => S2Point::new(v, u, -1.0),
    }
}
// pub fn face_uvto_xyzr2_point(face: u8, uv: &R2Point) -> S2Point {
//      ace_uv_to_xyz(face, uv.x, uv.y)
// }

/// Given a *valid* face for the given point p (meaning that dot product
/// of p with the face normal is positive), return the corresponding
/// u and v values (which may lie outside the range [-1,1]).
pub fn valid_face_xyz_to_uv(face: u8, p: &S2Point, pu: &mut f64, pv: &mut f64) {
    if p.dot(&get_norm(face)) <= 0.0 {
        unreachable!();
    }
    match face {
        0 => {
            *pu = p.y / p.x;
            *pv = p.z / p.x;
        }
        1 => {
            *pu = -p.x / p.y;
            *pv = p.z / p.y;
        }
        2 => {
            *pu = -p.x / p.z;
            *pv = -p.y / p.z;
        }
        3 => {
            *pu = p.z / p.x;
            *pv = p.y / p.x;
        }
        4 => {
            *pu = p.z / p.y;
            *pv = -p.x / p.y;
        }
        _ => {
            *pu = -p.y / p.z;
            *pv = -p.x / p.z;
        }
    }
}

// pub fn valid_face_xyz_to_uv_r2_point(face: u8, p: &S2Point, puv: &R2Point) {
//     valid_face_xyz_to_uv(face, p, &puv.x, &puv.y);
// }

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
pub fn xyz_to_face_uv(p: &S2Point, pu: &mut f64, pv: &mut f64) -> u8 {
    let face: u8 = get_face(p);
    valid_face_xyz_to_uv(face, p, pu, pv);
    face
}

/// Convert a direction vector (not necessarily unit length) to
/// (face, u, v) coordinates.
pub fn xyz_to_face_st(p: &S2Point, ps: &mut f64, pt: &mut f64) -> u8 {
    let face: u8 = get_face(p);
    let mut pu: f64 = 0.;
    let mut pv: f64 = 0.;
    valid_face_xyz_to_uv(face, p, &mut pu, &mut pv);
    *ps = UV_TO_ST(pu);
    *pt = UV_TO_ST(pv);
    face
}

// pub fn xyz_to_face_uv_r2_point(p: &S2Point, puv: &R2Point) -> i32 {
//     xyz_to_face_uv(p, puv.x, puv.y)
// }

/// If the dot product of p with the given face normal is positive,
/// set the corresponding u and v values (which may lie outside the range
/// [-1,1]) and return true.  Otherwise return false.
pub fn face_xyz_to_uv(face: u8, p: &S2Point, pu: &mut f64, pv: &mut f64) -> bool {
    if face < 3 {
        if p.face(face) <= 0.0 {
            return false;
        }
    } else if p.face(face - 3) >= 0.0 {
        return false;
    }
    valid_face_xyz_to_uv(face, p, pu, pv);
    true
}

// pub fn face_xyz_to_uv_r2_point(face: u8, p: &S2Point, puv: &R2Point) -> bool {
//     return face_xyz_to_uv(face, p, &puv.x, &puv.y);
// }

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
// /// cell, return the level of this cell (31 otherwise as its outside the bounds of levels).
// pub fn XYZtoFaceSiTi(p: &S2Point, face: &u8, si: &u32, ti: &u32) -> u8 {
//     let u: f64 = 0.0;
//     let v: f64 = 0.0;
//     *face = xyz_to_face_uv(p, &u, &v);
//     *si = STtoSiTi(UV_TO_ST(u));
//     *ti = STtoSiTi(UV_TO_ST(v));
//     // If the levels corresponding to si,ti are not equal, then p is not a cell
//     // center.  The si,ti values 0 and kMaxSiTi need to be handled specially
//     // because they do not correspond to cell centers at any valid level; they
//     // are mapped to level -1 by the code below.
//     let level = kMaxCellLevel - @ctz(si.* | kMaxSiTi);
//     if (level < 0 or level != kMaxCellLevel - @ctz(ti.* | kMaxSiTi)) return 31;
//     // In infinite precision, this test could be changed to ST == SiTi. However,
//     // due to rounding errors, UV_TO_ST(xyz_to_face_uv(face_uv_to_xyz(STtoUV(...)))) is
//     // not idempotent. On the other hand, center_raw is computed exactly the same
//     // way p was originally computed (if it is indeed the center of an S2Cell):
//     // the comparison can be exact.
//     const center = FaceSiTitoXYZ(face, si, ti).Normalize();
//     if (p.eq(center)) return level;
//     return 31;
// }

/// Convert (face, si, ti) coordinates to a direction vector (not necessarily
/// unit length).
pub fn face_si_ti_to_xyz(face: u8, si: u32, ti: u32) -> S2Point {
    let u = ST_TO_UV(si_ti_to_st(si));
    let v = ST_TO_UV(si_ti_to_st(ti));
    face_uvto_xyz(face, u, v)
}

// **** TEST FUNCTIONS ****
pub fn swap_axes(ij: usize) -> usize {
    ((ij >> 1) & 1) + ((ij & 1) << 1)
}
pub fn invert_bits(ij: usize) -> usize {
    ij ^ 3
}
