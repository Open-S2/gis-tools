extern crate alloc;

use libm::{fmax, fmin};

use alloc::{string::String, vec::Vec};

use crate::{
    face_si_ti_to_xyz, face_uv_to_xyz, ij_to_st, si_ti_to_st, st_to_ij, xyz_to_face_uv, BBox,
    LonLat, S2Point, K_INVERT_MASK, K_MAX_CELL_LEVEL, K_SWAP_MASK, LOOKUP_POS, ST_TO_UV, UV_TO_ST,
};

use super::LOOKUP_IJ;

/// Cell ID works with both S2 and WM with a common interface
pub type CellId = S2CellId;

// The following lookup tables are used to convert efficiently between an
// (i,j) cell index and the corresponding position along the Hilbert curve.
// "lookupPos" maps 4 bits of "i", 4 bits of "j", and 2 bits representing the
// orientation of the current cell into 8 bits representing the order in which
// that subcell is visited by the Hilbert curve, plus 2 bits indicating the
// new orientation of the Hilbert curve within that subcell.  (Cell
// orientations are represented as combination of K_SWAP_MASK and kInvertMask.)
//
// "lookupIJ" is an inverted table used for mapping in the opposite
// direction.
//
// We also experimented with looking up 16 bits at a time (14 bits of position
// plus 2 of orientation) but found that smaller lookup tables gave better
// performance. (2KB fits easily in the primary cache.)

// Values for these constants are *declared* in the *.h file. Even though
// the declaration specifies a value for the constant, that declaration
// is not a *definition* of storage for the value. Because the values are
// supplied in the declaration, we don't need the values here. Failing to
// define storage causes link errors for any code that tries to take the
// address of one of these values.

// Although only 60 bits are needed to represent the index of a leaf cell, the
// extra position bit lets us encode each cell as its Hilbert curve position
// at the cell center, which is halfway along the portion of the Hilbert curve
// that fills that cell.

/// The number of bits used to encode the face of the cell.
pub const K_FACE_BITS: u8 = 3;
/// The number of faces in the S2 cell projection
pub const K_NUM_FACES: u8 = 6;
/// The maximum level in the S2 cell decomposition
pub const K_MAX_LEVEL: u64 = K_MAX_CELL_LEVEL as u64; // Valid levels: 0..K_MAX_LEVEL
/// The number of bits used to encode the position along the Hilbert curve
pub const K_POS_BITS: u64 = 2 * K_MAX_LEVEL + 1;
/// The maximum number of cells in the S2 cell decomposition
pub const K_MAX_SIZE: u32 = 1 << K_MAX_LEVEL;
/// The number of bits used to encode the orientation of the Hilbert curve
pub const K_LOOKUP_BITS: u8 = 4;

/// This is the offset required to wrap around from the beginning of the
/// Hilbert curve to the end or vice versa; see nextWrap() and prevWrap().
/// SWIG doesn't understand uint64{}, so use static_cast.
pub const K_WRAP_OFFSET: u64 = (K_NUM_FACES as u64) << K_POS_BITS;

/// An S2CellId is a 64-bit unsigned integer that uniquely identifies a
/// cell in the S2 cell decomposition.  It has the following format:
///
///   id = [face][face_pos]
///
///   face:     a 3-bit number (range 0..5) encoding the cube face.
///
///   face_pos: a 61-bit number encoding the position of the center of this
///             cell along the Hilbert curve over this face (see the Wiki
///             pages for details).
///
/// Sequentially increasing cell ids follow a continuous space-filling curve
/// over the entire sphere.  They have the following properties:
///
///  - The id of a cell at level k consists of a 3-bit face number followed
///    by k bit pairs that recursively select one of the four children of
///    each cell.  The next bit is always 1, and all other bits are 0.
///    Therefore, the level of a cell is determined by the position of its
///    lowest-numbered bit that is turned on (for a cell at level k, this
///    position is 2 * (K_MAX_LEVEL - k).)
///
///  - The id of a parent cell is at the midpoint of the range of ids spanned
///    by its children (or by its descendants at any level).
///
/// Leaf cells are often used to represent points on the unit sphere, and
/// this class provides methods for converting directly between these two
/// representations.  For cells that represent 2D regions rather than
/// discrete point, it is better to use the S2Cell class.
///
/// This class is intended to be copied by value as desired.  It uses
/// the default copy constructor and assignment operator.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(C)]
pub struct S2CellId {
    /// the id contains the face, s, and t components
    pub id: u64,
}
impl S2CellId {
    /// Construct a cell id from the given 64-bit integer.
    pub fn new(id: u64) -> Self {
        S2CellId { id }
    }

    /// Returns an empty cell id.
    pub fn none() -> Self {
        0_u64.into()
    }

    /// Returns an invalid cell id guaranteed to be larger than any
    /// valid cell id.  Useful for creating indexes.
    pub fn sentinel() -> Self {
        (!0_u64).into()
    }

    /// Return the cell corresponding to a given S2 cube face.
    pub fn from_face(face: u8) -> S2CellId {
        (((face as u64) << K_POS_BITS) + lsb_for_level(0)).into()
    }

    /// Construct a leaf cell containing the given normalized S2LatLng.
    pub fn from_lon_lat(ll: &LonLat) -> S2CellId {
        S2CellId::from_s2_point(&ll.to_point())
    }

    /// Construct a leaf cell containing the given point "p".  Usually there is
    /// exactly one such cell, but for points along the edge of a cell, any
    /// adjacent cell may be (deterministically) chosen.  This is because
    /// S2CellIds are considered to be closed sets.  The returned cell will
    /// always contain the given point, i.e.
    ///
    /// S2Cell(S2CellId(p)).contains(p)
    ///
    /// is always true.  The point "p" does not need to be normalized.
    ///
    /// If instead you want every point to be contained by exactly one S2Cell,
    /// you will need to convert the S2CellIds to S2Loops (which implement point
    /// containment this way).
    pub fn from_s2_point(p: &S2Point) -> Self {
        let (face, u, v) = xyz_to_face_uv(p);
        let i: u32 = st_to_ij(UV_TO_ST(u));
        let j: u32 = st_to_ij(UV_TO_ST(v));
        Self::from_face_ij(face, i, j, None)
    }

    /// Construct a leaf cell given its face and (u,v) coordinates.
    pub fn from_face_uv(face: u8, u: f64, v: f64) -> Self {
        S2CellId::from_face_st(face, UV_TO_ST(u), UV_TO_ST(v))
    }

    /// Construct a leaf cell given its face and (s,t) coordinates.
    pub fn from_face_st(face: u8, s: f64, t: f64) -> Self {
        let i: u32 = st_to_ij(s);
        let j: u32 = st_to_ij(t);
        Self::from_face_ij(face, i, j, None)
    }

    /// Return a leaf cell given its cube face (range 0..5) and
    /// i- and j-coordinates (see s2coords.h).
    pub fn from_face_ij(face: u8, i: u32, j: u32, level: Option<u8>) -> S2CellId {
        let mut i = i as u64;
        let mut j = j as u64;
        if let Some(level) = level {
            i <<= K_MAX_LEVEL - level as u64;
            j <<= K_MAX_LEVEL - level as u64;
        }
        // Optimization notes:
        //  - Non-overlapping bit fields can be combined with either "+" or "|".
        //    Generally "+" seems to produce better code, but not always.

        // Note that this value gets shifted one bit to the left at the end
        // of the function.
        let mut n: u64 = (face as u64) << (K_POS_BITS - 1);

        // Alternating faces have opposite Hilbert curve orientations; this
        // is necessary in order for all faces to have a right-handed
        // coordinate system.
        let mut bits: u64 = (face & K_SWAP_MASK) as u64;

        // Each iteration maps 4 bits of "i" and "j" into 8 bits of the Hilbert
        // curve position.  The lookup table transforms a 10-bit key of the form
        // "iiiijjjjoo" to a 10-bit value of the form "ppppppppoo", where the
        // letters [ijpo] denote bits of "i", "j", Hilbert curve position, and
        // Hilbert curve orientation respectively.
        let mut k: u8 = 7;
        loop {
            let mask: u64 = (1 << 4) - 1;
            bits += ((i >> (k * 4)) & mask) << (4 + 2);
            bits += ((j >> (k * 4)) & mask) << 2;
            bits = LOOKUP_POS[bits as usize] as u64;
            n |= (bits >> 2) << (k * 2 * 4);
            bits &= (K_SWAP_MASK | K_INVERT_MASK) as u64;
            if k == 0 {
                break;
            }
            k -= 1;
        }

        let id: S2CellId = ((n << 1) + 1).into();

        if let Some(level) = level {
            id.parent(Some(level))
        } else {
            id
        }
    }

    /// Given a distance and optional zoom level, construct a cell ID at that distance
    pub fn from_distance(distance: u64, level: Option<u8>) -> S2CellId {
        let mut level: u64 = level.unwrap_or(K_MAX_LEVEL as u8) as u64;
        level = 2 * (K_MAX_LEVEL - level);
        ((distance << (level + 1)) + (1 << level)).into()
    }

    /// convert an id to a zoom-i-j after setting it to a new parent zoom
    pub fn to_zoom_ij(&self, level: Option<u8>) -> (u8, u32, u32) {
        let (face, i, j, _or) = self.to_face_ij_orientation(level);
        (face, i, j)
    }

    /// Return the (face, i, j) coordinates for the leaf cell corresponding to
    /// this cell id.  Since cells are represented by the Hilbert curve position
    /// at the center of the cell, the returned (i,j) for non-leaf cells will be
    /// a leaf cell adjacent to the cell center.  If "orientation" is non-nullptr,
    /// also return the Hilbert curve orientation for the current cell.
    /// Returns (face, i, j, orientation)
    pub fn to_face_ij_orientation(&self, level: Option<u8>) -> (u8, u32, u32, u8) {
        let mut i: u32 = 0;
        let mut j: u32 = 0;
        let face: u8 = (self.id >> K_POS_BITS) as u8;
        let mut bits: u64 = (face & 1) as u64;

        // Each iteration maps 8 bits of the Hilbert curve position into
        // 4 bits of "i" and "j".  The lookup table transforms a key of the
        // form "ppppppppoo" to a value of the form "iiiijjjjoo", where the
        // letters [ijpo] represents bits of "i", "j", the Hilbert curve
        // position, and the Hilbert curve orientation respectively.
        //
        // On the first iteration we need to be careful to clear out the bits
        // representing the cube face.
        let mut k = 7;
        while k >= 0 {
            let nbits: u64 = if k == 7 { 2 } else { 4 };
            let kk: u64 = k as u64;
            bits += ((self.id >> (kk * 8 + 1)) & ((1 << (2 * nbits)) - 1)) << 2;
            bits = LOOKUP_IJ[bits as usize] as u64;
            i += (bits as u32 >> K_NUM_FACES as u32) << (kk * 4);
            j += (((bits >> 2) & 15) << (kk * 4)) as u32;
            bits &= K_FACE_BITS as u64;
            k -= 1;
        }

        // adjust bits to the orientation
        let lsb = self.id & (!self.id + 1);
        if (lsb & 1229782938247303424) != 0 {
            bits ^= 1;
        }

        if let Some(level) = level {
            i >>= K_MAX_LEVEL as u32 - level as u32;
            j >>= K_MAX_LEVEL as u32 - level as u32;
        }

        (face, i, j, bits as u8)
    }

    /// Return the (face, si, ti) coordinates of the center of the cell.  Note
    /// that although (si,ti) coordinates span the range [0,2**31] in general,
    /// the cell center coordinates are always in the range [1,2**31-1] and
    /// therefore can be represented using a signed 32-bit integer.
    /// Returns (face, si, ti)
    pub fn get_center_si_ti(&self) -> (u8, u32, u32) {
        // First we compute the discrete (i,j) coordinates of a leaf cell contained
        // within the given cell.  Given that cells are represented by the Hilbert
        // curve position corresponding at their center, it turns out that the cell
        // returned by to_face_ij_orientation is always one of two leaf cells closest
        // to the center of the cell (unless the given cell is a leaf cell itself,
        // in which case there is only one possibility).
        //
        // Given a cell of size s >= 2 (i.e. not a leaf cell), and letting (imin,
        // jmin) be the coordinates of its lower left-hand corner, the leaf cell
        // returned by to_face_ij_orientation() is either (imin + s/2, jmin + s/2)
        // (imin + s/2 - 1, jmin + s/2 - 1).  The first case is the one we want.
        // We can distinguish these two cases by looking at the low bit of "i" or
        // "j".  In the second case the low bit is one, unless s == 2 (i.e. the
        // level just above leaf cells) in which case the low bit is zero.
        //
        // In the code below, the expression ((i ^ (int(id_) >> 2)) & 1) is true
        // if we are in the second case described above.
        let (face, i, j, _or) = self.to_face_ij_orientation(None);
        let mut delta: u32 = 0;
        if self.is_leaf() {
            delta = 1;
        } else if ((i ^ (self.id >> 2) as u32) & 1) != 0 {
            delta = 2;
        }

        // Note that (2 * {i,j} + delta) will never overflow a 32-bit integer.
        let psi = 2 * i + delta;
        let pti = 2 * j + delta;

        (face, psi, pti)
    }

    /// Creates a human readable debug string.  Used for << and available for
    /// direct usage as well.  The format is "f/dd..d" where "f" is a digit in
    /// the range [0-5] representing the S2CellId face, and "dd..d" is a string
    /// of digits in the range [0-3] representing each child's position with
    /// respect to its parent.  (Note that the latter string may be empty.)
    ///
    /// For example "4/" represents S2CellId::from_face(4), and "3/02" represents
    /// S2CellId::from_face(3).child(0).child(2).
    pub fn display_name(&self) -> String {
        let mut out = String::new();
        if !self.is_valid() {
            out.push_str("Invalid");
            return out;
        }
        out.push((self.face() + 48) as char);
        out += "/";
        let mut cur_level: u8 = 1;
        while cur_level <= self.level() {
            out.push((self.child_position(cur_level) + 48) as char);
            cur_level += 1;
        }

        out
    }

    /// Return the child position (0..3) of this cell's ancestor at the given
    /// level within its parent.  For example, childPosition(1) returns the
    /// position of this cell's level-1 ancestor within its top-level face cell.
    /// REQUIRES: 1 <= level <= this->level().
    pub fn child_position(&self, level: u8) -> u8 {
        if !self.is_valid() || level > self.level() {
            unreachable!();
        }
        // return @as(u3, @intCast(self.id >> (2 * (K_MAX_LEVEL - @as(u6, level)) + 1) & 3));
        ((self.id >> (2 * (K_MAX_LEVEL - level as u64) + 1)) & 3) as u8
    }

    /// Converts a string in the format returned by ToString() to an S2CellId.
    /// Returns S2CellId.None() if the string could not be parsed.
    ///
    /// The method name includes "Debug" in order to avoid possible confusion
    /// with FromToken() above.
    pub fn from_string(val: &str) -> S2CellId {
        // This function is reasonably efficient, but is only intended for use in tests.
        let val_bytes = val.as_bytes();
        let level = (val.len() - 2) as u64;
        if !(0..=K_MAX_LEVEL).contains(&level) {
            return S2CellId::none();
        }
        let face: u8 = val_bytes[0] - b'0';
        if !(0..=5).contains(&face) || val_bytes[1] != b'/' {
            return S2CellId::none();
        }
        let mut id = S2CellId::from_face(face);
        let mut i = 2;
        while i < val.len() {
            let child_pos = val_bytes[i] - b'0';
            if !(0..=3).contains(&child_pos) {
                return S2CellId::none();
            }
            id = id.child(child_pos);
            i += 1;
        }

        id
    }

    /// Return the lowest-numbered bit that is on for this cell id, which is
    /// equal to (uint64{1} << (2 * (K_MAX_LEVEL - level))).  So for example,
    /// a.lsb() <= b.lsb() if and only if a.level() >= b.level(), but the
    /// first test is more efficient.
    pub fn lsb(&self) -> u64 {
        if self.id == 0 {
            return 0;
        }
        self.id & (!self.id + 1_u64)
    }

    /// Return the immediate child of this cell at the given traversal order
    /// position (in the range 0 to 3). This cell must not be a leaf cell.
    pub fn child(&self, position: u8) -> S2CellId {
        if !self.is_valid() || self.is_leaf() || position > 3 {
            unreachable!();
        }
        // To change the level, we need to move the least-significant bit two
        // positions downward.  We do this by subtracting (4 * new_lsb) and adding
        // new_lsb.  Then to advance to the given child cell, we add
        // (2 * position * new_lsb).
        let new_lsb = self.lsb() >> 2;
        (self.id - (3 * new_lsb) + (2 * (position as u64) * new_lsb)).into()
    }

    /// Which cube face this cell belongs to, in the range 0..5.
    pub fn face(&self) -> u8 {
        (self.id >> K_POS_BITS) as u8
    }

    /// The position of the cell center along the Hilbert curve over this face,
    /// in the range 0..(2**K_POS_BITS-1).
    pub fn pos(&self) -> u64 {
        self.id & (0u64 >> K_FACE_BITS)
    }

    /// Return the subdivision level of the cell (range 0..K_MAX_LEVEL).
    pub fn level(&self) -> u8 {
        // We can't just S2_DCHECK(isValid()) because we want level() to be
        // defined for end-iterators, i.e. S2CellId::End(kLevel).  However there is
        // no good way to define S2CellId::None().level(), so we do prohibit that.
        if self.id == 0 {
            unreachable!();
        }

        K_MAX_LEVEL as u8 - (self.id.trailing_zeros() as u8 >> 1u8)
    }

    /// Return true if id represents a valid cell.
    ///
    /// All methods require isValid() to be true unless otherwise specified
    /// (although not all methods enforce this).
    pub fn is_valid(&self) -> bool {
        self.face() < K_NUM_FACES && (self.lsb() & 0x1555555555555555) != 0_u64
    }

    /// Return true if this is a leaf cell (more efficient than checking
    /// whether level() == K_MAX_LEVEL).
    pub fn is_leaf(&self) -> bool {
        (self.id & 1u64) != 0
    }

    /// Convert an S2CellID to an S2Point
    pub fn to_point_raw(&self) -> S2Point {
        let (face, si, ti) = self.get_center_si_ti();
        face_si_ti_to_xyz(face, si, ti)
    }

    /// Convert an S2CellID to an S2Point in normalized vector coordinates
    pub fn to_point(&self) -> S2Point {
        let mut p = self.to_point_raw();
        p.normalize();
        p
    }

    /// Convert an S2CellID to an Face-S-T coordinate
    /// Returns (face, s, t)
    pub fn to_st(&self) -> (u8, f64, f64) {
        let (face, i, j, _or) = self.to_face_ij_orientation(None);
        let s = ij_to_st(i);
        let t = ij_to_st(j);

        (face, s, t)
    }

    /// Convert an S2CellID to an Face-U-V coordinate
    /// Returns (face, u, v)
    pub fn to_uv(&self) -> (u8, f64, f64) {
        let (face, s, t) = self.to_st();
        let u = ST_TO_UV(s);
        let v = ST_TO_UV(t);
        (face, u, v)
    }

    /// Given an S2CellID, check if it is a Face Cell.
    pub fn is_face(&self) -> bool {
        (self.id & ((1 << 60) - 1)) == 0
    }

    /// Given an S2CellID, get the distance it spans (or length it covers)
    pub fn distance(&self, lev: Option<u8>) -> f64 {
        let level = lev.unwrap_or(self.level());
        (self.id >> (2 * (30 - level) + 1)) as f64
    }

    /// Given an S2CellID, get all the quad children tiles
    pub fn children(&self, orientation: Option<u8>) -> [S2CellId; 4] {
        let mut childs = [self.child(0), self.child(3), self.child(2), self.child(1)];

        if let Some(orientation) = orientation {
            if orientation == 0 {
                childs.swap(1, 3);
            }
        }

        childs
    }

    /// Given an S2CellID, get the quad children tiles using a face-zoom-ij input
    pub fn children_ij(&self, face: u8, level: u8, i: u32, j: u32) -> [Self; 4] {
        let i = i << 1;
        let j = j << 1;
        let level = level + 1;

        [
            S2CellId::from_face_ij(face, i, j, Some(level)),
            S2CellId::from_face_ij(face, i + 1, j, Some(level)),
            S2CellId::from_face_ij(face, i, j + 1, Some(level)),
            S2CellId::from_face_ij(face, i + 1, j + 1, Some(level)),
        ]
    }

    /// Given an S2CellID, get the parent quad tile
    pub fn parent(&self, level: Option<u8>) -> S2CellId {
        let new_lsb = match level {
            Some(level) => 1 << (2 * (K_MAX_LEVEL - level as u64)),
            None => (self.id & (!self.id + 1)) << 2,
        };

        ((self.id & (!new_lsb + 1)) | new_lsb).into()
    }

    /// Given an S2CellID, get the hilbert range it spans
    /// returns (min, max)
    pub fn range(&self) -> (Self, Self) {
        let id = self.id;
        let lsb = id & (!id + 1);

        ((id - (lsb - 1)).into(), (id + (lsb - 1)).into())
    }

    /// Check if the first S2CellID contains the second.
    pub fn contains(&self, other: &S2CellId) -> bool {
        let (min, max) = self.range();
        *other >= min && *other <= max
    }

    /// Check if an S2CellID intersects another. This includes edges touching.
    pub fn intersects(&self, other: &S2CellId) -> bool {
        let (min_self, max_self) = self.range();
        let (min_other, max_other) = other.range();
        min_other <= max_self && max_other >= min_self
    }

    /// Get the next S2CellID in the hilbert space
    pub fn next(&self) -> S2CellId {
        let id = self.id;
        let n = id + ((id & (!id + 1)) << 1);
        if n < K_WRAP_OFFSET {
            n.into()
        } else {
            (n - K_WRAP_OFFSET).into()
        }
    }

    /// Get the previous S2CellID in the hilbert space
    pub fn prev(&self) -> S2CellId {
        let id = self.id;
        let p = id - ((id & (!id + 1)) << 1);
        if p < K_WRAP_OFFSET {
            p.into()
        } else {
            (p + K_WRAP_OFFSET).into()
        }
    }

    /// Given an S2CellID and level (zoom), get the center point of that cell in S-T space
    /// returns (face, s, t)
    pub fn center_st(&self) -> (u8, f64, f64) {
        let id = self.id;
        let (face, i, j, _or) = self.to_face_ij_orientation(None);
        let delta = if (id & 1) != 0 {
            1
        } else if ((i as u64 ^ (id >> 2)) & 1) != 0 {
            2
        } else {
            0
        };
        // Note that (2 * {i,j} + delta) will never overflow a 32-bit integer.
        let si = 2 * i + delta;
        let ti = 2 * j + delta;

        (face, si_ti_to_st(si), si_ti_to_st(ti))
    }

    /// Given an S2CellID and level (zoom), get the S-T bounding range of that cell
    /// returns (s_min, t_min, s_max, t_max)
    pub fn bounds_st(&self, level: Option<u8>) -> BBox {
        let level = level.unwrap_or(self.level());

        let (_face, s, t) = self.center_st();
        let half_size = size_st(level) * 0.5;
        BBox::new(s - half_size, t - half_size, s + half_size, t + half_size)
    }

    /// Given an S2CellID, find the neighboring S2CellIDs
    /// returns neighbors: [up, right, down, left]
    pub fn neighbors(&self) -> [S2CellId; 4] {
        let level = self.level();
        let size = size_ij(level);
        let (face, i, j, _or) = self.to_face_ij_orientation(None);

        [
            S2CellId::from_ij_same(face, i, j - size, j as i32 - size as i32 >= 0)
                .parent(Some(level)),
            S2CellId::from_ij_same(face, i + size, j, i + size < K_MAX_SIZE).parent(Some(level)),
            S2CellId::from_ij_same(face, i, j + size, j + size < K_MAX_SIZE).parent(Some(level)),
            S2CellId::from_ij_same(face, i - size, j, i as i32 - size as i32 >= 0)
                .parent(Some(level)),
        ]
    }

    /// Given a Face-I-J and a desired level (zoom), find the neighboring S2CellIDs
    /// return neighbors: [down, right, up, left]
    pub fn neighbors_ij(&self, face: u8, i: u32, j: u32, level: u8) -> [S2CellId; 4] {
        let size = size_ij(level);

        [
            S2CellId::from_ij_same(face, i, j - size, j as i32 - size as i32 >= 0)
                .parent(Some(level)),
            S2CellId::from_ij_same(face, i + size, j, i + size < K_MAX_SIZE).parent(Some(level)),
            S2CellId::from_ij_same(face, i, j + size, j + size < K_MAX_SIZE).parent(Some(level)),
            S2CellId::from_ij_same(face, i - size, j, i as i32 - size as i32 >= 0)
                .parent(Some(level)),
        ]
    }

    /// Build an S2CellID given a Face-I-J, but ensure the face is the same if desired
    pub fn from_ij_same(face: u8, i: u32, j: u32, same_face: bool) -> S2CellId {
        if same_face {
            S2CellId::from_face_ij(face, i, j, None)
        } else {
            S2CellId::from_face_ij_wrap(face, i, j)
        }
    }

    /// Build an S2CellID given a Face-I-J, but ensure it's a legal value, otherwise wrap before creation
    pub fn from_face_ij_wrap(face: u8, i: u32, j: u32) -> S2CellId {
        // Convert i and j to the coordinates of a leaf cell just beyond the
        // boundary of this face.  This prevents 32-bit overflow in the case
        // of finding the neighbors of a face cell.
        let i = i.clamp(0, K_MAX_SIZE);
        let j = j.clamp(0, K_MAX_SIZE);

        // We want to wrap these coordinates onto the appropriate adjacent face.
        // The easiest way to do this is to convert the (i,j) coordinates to (x,y,z)
        // (which yields a point outside the normal face boundary), and then call
        // S2::XYZtoFaceUV() to project back onto the correct face.
        //
        // The code below converts (i,j) to (si,ti), and then (si,ti) to (u,v) using
        // the linear projection (u=2*s-1 and v=2*t-1).  (The code further below
        // converts back using the inverse projection, s=0.5*(u+1) and t=0.5*(v+1).
        // Any projection would work here, so we use the simplest.)  We also clamp
        // the (u,v) coordinates so that the point is barely outside the
        // [-1,1]x[-1,1] face rectangle, since otherwise the reprojection step
        // (which divides by the new z coordinate) might change the other
        // coordinates enough so that we end up in the wrong leaf cell.
        let k_scale = 1. / K_MAX_SIZE as f64;
        let k_limit = 1. + 2.220_446_049_250_313e-16;
        let u = fmax(
            -k_limit,
            fmin(k_limit, k_scale * (2. * (i as f64 - (K_MAX_SIZE as f64) / 2.) + 1.)),
        );
        let v = fmax(
            -k_limit,
            fmin(k_limit, k_scale * (2. * (j as f64 - (K_MAX_SIZE as f64) / 2.) + 1.)),
        );

        // Find the leaf cell coordinates on the adjacent face, and convert
        // them to a cell id at the appropriate level.
        let (n_face, nu, nv) = xyz_to_face_uv(&face_uv_to_xyz(face, u, v));
        S2CellId::from_face_ij(n_face, st_to_ij(0.5 * (nu + 1.)), st_to_ij(0.5 * (nv + 1.)), None)
    }

    /// Given an S2CellID, find it's nearest neighbors associated with it
    pub fn vertex_neighbors(&self, level: Option<u8>) -> Vec<S2CellId> {
        let level = level.unwrap_or(self.level());
        let mut neighbors: Vec<S2CellId> = Vec::new();

        let (face, i, j, _or) = self.to_face_ij_orientation(None);

        // Determine the i- and j-offsets to the closest neighboring cell in each
        // direction.  This involves looking at the next bit of "i" and "j" to
        // determine which quadrant of this->parent(level) this cell lies in.
        let halfsize = size_ij(level + 1);
        let size = halfsize << 1;
        let isame;
        let jsame;
        let ioffset;
        let joffset;

        if (i & halfsize) != 0 {
            ioffset = size;
            isame = i + size < K_MAX_SIZE;
        } else {
            ioffset = -(size as i32) as u32;
            isame = i as i32 - size as i32 >= 0;
        }
        if (j & halfsize) != 0 {
            joffset = size;
            jsame = j + size < K_MAX_SIZE;
        } else {
            joffset = -(size as i32) as u32;
            jsame = j as i32 - size as i32 >= 0;
        }

        neighbors.push(self.parent(Some(level)));
        neighbors.push(S2CellId::from_ij_same(face, i + ioffset, j, isame).parent(Some(level)));
        neighbors.push(S2CellId::from_ij_same(face, i, j + joffset, jsame).parent(Some(level)));
        if isame || jsame {
            neighbors.push(
                S2CellId::from_ij_same(face, i + ioffset, j + joffset, isame && jsame)
                    .parent(Some(level)),
            );
        }

        neighbors
    }
}
impl From<u64> for S2CellId {
    fn from(value: u64) -> Self {
        S2CellId::new(value)
    }
}
impl From<LonLat> for S2CellId {
    fn from(value: LonLat) -> Self {
        S2CellId::from_lon_lat(&value)
    }
}
impl From<S2Point> for S2CellId {
    fn from(value: S2Point) -> Self {
        S2CellId::from_s2_point(&value)
    }
}
impl From<String> for S2CellId {
    fn from(s: String) -> Self {
        S2CellId::from_string(&s)
    }
}
impl From<&str> for S2CellId {
    fn from(s: &str) -> Self {
        S2CellId::from_string(s)
    }
}

/// Return the lowest-numbered bit that is on for cells at the given level.
pub fn lsb_for_level(level: u8) -> u64 {
    1_u64 << (2 * (K_MAX_LEVEL - (level as u64)))
}

/// Return the range maximum of a level (zoom) in S-T space
pub fn size_st(level: u8) -> f64 {
    ij_to_st(size_ij(level))
}

/// Return the range maximum of a level (zoom) in I-J space
pub fn size_ij(level: u8) -> u32 {
    1 << (30 - level)
}
