mod clip;
mod convert;
mod lines;
mod points;
mod polys;
mod simplify;

pub use clip::*;
pub use convert::*;
pub use lines::*;
pub use points::*;
pub use polys::*;
pub use simplify::*;

// TODO:

// Line
// - [ ] point to line distance

// Poly
// - [ ] Poly Boolean

// Clean
// - [ ] Dekink: remove kinks from a polygon
// - [ ] Reduce: Remove redundant points
// - [ ] clean: remove redundant points, (multi)linestring and/or (multi)polygon, fixes kinks, merges with poly bool. Put all under flags with all turned on by default
