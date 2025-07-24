mod clip;
mod convert;
mod points;
mod simplify;

pub use clip::*;
pub use convert::*;
pub use points::*;
pub use simplify::*;

// TODO:
// - [ ] Nearest Point
// - [ ] Area
// - [ ] Poly Boolean
// - [ ] Point inside: points check if equal, line if on line, polygon if inside
// - [ ] Dekink: remove kinks from a polygon
// - [ ] Length: length of item
// - [ ] along: (given linestring find the point at distance provided)
// - [ ] clean: remove redundant points, (multi)linestring and/or (multi)polygon, fixes kinks, etc.

// - [ ] point to line distance
