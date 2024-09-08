extern crate alloc;

use alloc::string::String;

use crate::s2::coords::{xyz_to_face_uv, st_to_ij, UV_TO_ST};
use crate::s2::point::S2Point;
use crate::lonlat::LonLat;

use super::coords::K_MAX_CELL_LEVEL;
use super::coords_internal::{K_INVERT_MASK, K_SWAP_MASK, LOOKUP_POS};

// The following lookup tables are used to convert efficiently between an
// (i,j) cell index and the corresponding position along the Hilbert curve.
// "lookupPos" maps 4 bits of "i", 4 bits of "j", and 2 bits representing the
// orientation of the current cell into 8 bits representing the order in which
// that subcell is visited by the Hilbert curve, plus 2 bits indicating the
// new orientation of the Hilbert curve within that subcell.  (Cell
// orientations are represented as combination of kSwapMask and kInvertMask.)
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
pub const K_FACE_BITS: u8 = 3;
pub const K_NUM_FACES: u8 = 6;
pub const K_MAX_LEVEL: u64 = K_MAX_CELL_LEVEL as u64; // Valid levels: 0..kMaxLevel
pub const K_POS_BITS: u64 = 2 * K_MAX_LEVEL + 1;
pub const K_MAX_SIZE: u64 = 1 << K_MAX_LEVEL;

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
///    position is 2 * (kMaxLevel - k).)
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
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub struct S2CellId {
    /// the id contains the face, s, and t components
    pub id: u64,
}
impl S2CellId {
    pub fn new(id: u64) -> Self {
        S2CellId { id }
    }

    pub fn none() -> Self {
        S2CellId::new(0)
    }

    /// Returns an invalid cell id guaranteed to be larger than any
    /// valid cell id.  Useful for creating indexes.
    pub fn sentinel() -> Self {
        S2CellId::new(!0)
    }

    /// Return the cell corresponding to a given S2 cube face.
    pub fn from_face(face_: u8) -> S2CellId {
        S2CellId::new(((face_ as u64) << K_POS_BITS) + lsb_for_level(0))
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
        let mut u: f64 = 0.0;
        let mut v: f64 = 0.0;
        let face = xyz_to_face_uv(p, &mut u, &mut v);
        let i: u32 = st_to_ij(UV_TO_ST(u));
        let j: u32 = st_to_ij(UV_TO_ST(v));
        Self::from_face_ij(face, i, j)
    }

    pub fn from_face_st(face: u8, s: f64, t: f64) -> Self {
        let i: u32 = st_to_ij(s);
        let j: u32 = st_to_ij(t);
        Self::from_face_ij(face, i, j)
    }

    /// Return a leaf cell given its cube face (range 0..5) and
    /// i- and j-coordinates (see s2coords.h).
    pub fn from_face_ij(face: u8, i: u32, j: u32) -> S2CellId {
        let i = i as u64;
        let j = j as u64;
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
            if k == 0 { break; }
            k -= 1;
        }

        // return S2CellId.Init(@addWithOverflow(@mulWithOverflow(n, 2)[0], 1)[0]);
        S2CellId::new((n << 1) + 1)
    }

    /// Creates a human readable debug string.  Used for << and available for
    /// direct usage as well.  The format is "f/dd..d" where "f" is a digit in
    /// the range [0-5] representing the S2CellId face, and "dd..d" is a string
    /// of digits in the range [0-3] representing each child's position with
    /// respect to its parent.  (Note that the latter string may be empty.)
    ///
    /// For example "4/" represents S2CellId::from_face(4), and "3/02" represents
    /// S2CellId::from_face(3).child(0).child(2).
    pub fn display_name(self) -> String {
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
    pub fn child_position(self, level: u8) -> u8 {
        if !self.is_valid() || level > self.level() { unreachable!(); }
        // return @as(u3, @intCast(self.id >> (2 * (kMaxLevel - @as(u6, level)) + 1) & 3));
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
    /// equal to (uint64{1} << (2 * (kMaxLevel - level))).  So for example,
    /// a.lsb() <= b.lsb() if and only if a.level() >= b.level(), but the
    /// first test is more efficient.
    pub fn lsb(self) -> u64 {
        if self.id == 0 { return 0; }
        self.id & (!self.id + 1_u64)
    }

    /// Return the immediate child of this cell at the given traversal order
    /// position (in the range 0 to 3). This cell must not be a leaf cell.
    pub fn child(self, position: u8) -> S2CellId {
        if !self.is_valid() || self.is_leaf() { unreachable!(); }
        // To change the level, we need to move the least-significant bit two
        // positions downward.  We do this by subtracting (4 * new_lsb) and adding
        // new_lsb.  Then to advance to the given child cell, we add
        // (2 * position * new_lsb).
        let new_lsb = self.lsb() >> 2;
        S2CellId::new(self.id - (3 * new_lsb) + (2 * (position as u64) * new_lsb))
    }

    // Which cube face this cell belongs to, in the range 0..5.
    pub fn face(self) -> u8 {
        (self.id >> K_POS_BITS) as u8
    }

    /// The position of the cell center along the Hilbert curve over this face,
    /// in the range 0..(2**K_POS_BITS-1).
    pub fn pos(self) -> u64 {
        self.id & (0u64 >> K_FACE_BITS)
    }

    /// Return the subdivision level of the cell (range 0..kMaxLevel).
    pub fn level(self) -> u8 {
        // We can't just S2_DCHECK(isValid()) because we want level() to be
        // defined for end-iterators, i.e. S2CellId::End(kLevel).  However there is
        // no good way to define S2CellId::None().level(), so we do prohibit that.
        if self.id == 0 { unreachable!(); }

        K_MAX_LEVEL as u8 - (self.id.trailing_zeros() as u8 >> 1u8)
    }

    /// Return true if id represents a valid cell.
    ///
    /// All methods require isValid() to be true unless otherwise specified
    /// (although not all methods enforce this).
    pub fn is_valid(self) -> bool {
        self.face() < K_NUM_FACES && (self.lsb() & 0x1555555555555555) != 0_u64
    }

    /// Return true if this is a leaf cell (more efficient than checking
    /// whether level() == kMaxLevel).
    pub fn is_leaf(self) -> bool {
        (self.id & 1u64) != 0
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
