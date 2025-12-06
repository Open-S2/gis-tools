use super::{S1Angle, S2CellId, S2Point};
use core::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Neg, Sub},
};
use libm::{asin, atan2, cos, fmin, sin, sqrt};
use s2json::{GetM, GetXY, MValue, NewXY, VectorPoint};

/// # Longitude-Latitude Point container
///
/// ## Description
/// This class represents a point on the unit sphere as a pair
/// of latitude-longitude coordinates.  Like the rest of the "geometry"
/// package, the intent is to represent spherical geometry as a mathematical
/// abstraction, so functions that are specifically related to the Earth's
/// geometry (e.g. easting/northing conversions) should be put elsewhere.
///
/// This struct implements the [`GetXY`] and [`GetM`] traits.
///
/// ## Usage
///
/// Methods that are available:
/// - [`LonLat::new`]: Create a new LonLat
/// - [`LonLat::take`]: Take ownership of the underlying VectorPoint
/// - [`LonLat::lon`]: Return the longitude in degrees
/// - [`LonLat::lat`]: Return the latitude in degrees
/// - [`LonLat::from_s2cellid`]: Convert a [`S2CellId`] to an LonLat
/// - [`LonLat::from_s2_point`]: Convert a [`S2Point`] to an LonLat
/// - [`LonLat::normalize`]: Normalize the coordinates to the range [-180, 180] and [-90, 90] deg.
/// - [`LonLat::coords`]: Return the latitude or longitude coordinates in degrees.
/// - [`LonLat::to_angles`]: Return the latitude and longitude coordinates in radians.
/// - [`LonLat::is_valid`]: Return true if the latitude is between -90 and 90 degrees inclusive and the longitude is between -180 and 180 degrees inclusive.
/// - [`LonLat::to_point`]: Converts an LonLat to the equivalent unit-length vector.
/// - [`LonLat::to_point_gl`]: An alternative to [`LonLat::to_point`] that returns a GPU compatible vector.
/// - [`LonLat::get_distance`]:Returns the distance (measured along the surface of the sphere) to the given LonLat.
/// - [`LonLat::get_bearing`]: Returns the bearing from the first point to the second point.
#[derive(Clone, PartialEq, Debug)]
pub struct LonLat<M: Clone + Default = MValue>(pub VectorPoint<M>);
impl GetXY for LonLat {
    fn x(&self) -> f64 {
        self.0.x
    }
    fn y(&self) -> f64 {
        self.0.y
    }
}
impl<M: Clone + Default> GetM<M> for LonLat<M> {
    fn m(&self) -> Option<&M> {
        self.0.m.as_ref()
    }
}
impl NewXY for LonLat {
    fn new_xy(x: f64, y: f64) -> LonLat {
        LonLat(VectorPoint::new(x, y, None, None))
    }
}
impl<M: Clone + Default> LonLat<M> {
    /// Build a new LonLat
    pub fn new(lon: f64, lat: f64, m: Option<M>) -> LonLat<M> {
        LonLat(VectorPoint::new(lon, lat, None, m))
    }

    /// Take the underlying VectorPoint
    pub fn take(&mut self) -> VectorPoint<M> {
        core::mem::take(&mut self.0)
    }

    /// Return the longitude in degrees
    #[inline]
    pub fn lon(&self) -> f64 {
        self.0.x
    }

    /// Return the latitude in degrees
    #[inline]
    pub fn lat(&self) -> f64 {
        self.0.y
    }

    /// Convert a S2CellId to an LonLat
    pub fn from_s2cellid(cellid: S2CellId) -> Self {
        let p = cellid.to_point();
        Self::from_s2_point(&p)
    }

    /// Convert a direction vector (not necessarily unit length) to an LonLat.
    pub fn from_s2_point(p: &S2Point) -> Self {
        let lon_rad = atan2(p.y + 0.0, p.x + 0.0);
        let lat_rad = atan2(p.z, sqrt(p.x * p.x + p.y * p.y));
        let ll = LonLat::new((lon_rad).to_degrees(), (lat_rad).to_degrees(), None);
        debug_assert!(ll.is_valid(), "from_s2_point() called on invalid LonLat");

        ll
    }

    /// Normalize the coordinates to the range [-180, 180] and [-90, 90] deg.
    pub fn normalize(&mut self) {
        let mut lon = self.lon();
        let mut lat = self.lat();
        // Normalize longitude using modulo
        lon = ((((lon + 180.) % 360.) + 360.) % 360.) - 180.;
        // Clamp latitude between -90 and 90
        lat = lat.clamp(-90., 90.);

        self.0.x = lon;
        self.0.y = lat;
    }

    /// Return the latitude or longitude coordinates in degrees.
    pub fn coords(self) -> (f64, f64) {
        (self.lon(), self.lat())
    }

    /// Return the latitude and longitude coordinates in radians.
    pub fn to_angles(&self) -> (S1Angle, S1Angle) {
        (S1Angle::from_degrees(self.lon()), S1Angle::from_degrees(self.lat()))
    }

    /// Return true if the latitude is between -90 and 90 degrees inclusive
    /// and the longitude is between -180 and 180 degrees inclusive.
    pub fn is_valid(&self) -> bool {
        // OLD RAD IMPL:
        // fabs(self.lat()) <= (PI / 2.0) && fabs(self.lon()) <= PI
        let lat = self.lat();
        let lon = self.lon();

        (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon)
    }

    /// Converts an LonLat to the equivalent unit-length vector. Unnormalized
    /// values (see Normalize()) are wrapped around the sphere as would be expected
    /// based on their definition as spherical angles.  So for example the
    /// following pairs yield equivalent points (modulo numerical error):
    ///     (90.5, 10) =~ (89.5, -170)
    ///     (a, b) =~ (a + 360 * n, b)
    /// The maximum error in the result is 1.5 * DBL_EPSILON.  (This does not
    /// include the error of converting degrees, E5, E6, or E7 to radians.)
    ///
    /// Can be used just like an S2Point constructor.  For example:
    ///   S2Cap cap;
    ///   cap.AddPoint(S2Point(latlon));
    pub fn to_point(&self) -> S2Point {
        debug_assert!(self.is_valid(), "to_point() called on invalid LonLat");
        let lon: f64 = (self.lon()).to_radians();
        let lat: f64 = (self.lat()).to_radians();
        S2Point::new(
            cos(lat) * cos(lon), // x
            cos(lat) * sin(lon), // y
            sin(lat),            // z
        )
    }

    /// An alternative to to_point() that returns a GPU compatible vector.
    pub fn to_point_gl(&self) -> S2Point {
        debug_assert!(self.is_valid(), "to_point_gl() called on invalid LonLat");
        let lon: f64 = (self.lon()).to_radians();
        let lat: f64 = (self.lat()).to_radians();
        S2Point::new(
            cos(lat) * sin(lon), // y
            sin(lat),            // z
            cos(lat) * cos(lon), // x
        )
    }

    /// Returns the distance (measured along the surface of the sphere) to the
    /// given LonLat, implemented using the Haversine formula.  This is
    /// equivalent to
    ///
    ///   S1Angle(ToPoint(), o.ToPoint())
    ///
    /// except that this function is slightly faster, and is also somewhat less
    /// accurate for distances approaching 180 degrees (see s1angle.h for
    /// details).  Both LngLats must be normalized.
    pub fn get_distance<M2: Clone + Default>(&self, b: &LonLat<M2>) -> f64 {
        // This implements the Haversine formula, which is numerically stable for
        // small distances but only gets about 8 digits of precision for very large
        // distances (e.g. antipodal points).  Note that 8 digits is still accurate
        // to within about 10cm for a sphere the size of the Earth.
        //
        // This could be fixed with another sin() and cos() below, but at that point
        // you might as well just convert both arguments to S2Points and compute the
        // distance that way (which gives about 15 digits of accuracy for all
        // distances).
        debug_assert!(self.is_valid(), "get_bearing() called on invalid LonLat (self)");
        debug_assert!(b.is_valid(), "get_bearing() called on invalid LonLat (b)");

        let lat1 = self.lat().to_radians();
        let lat2 = b.lat().to_radians();
        let lon1 = self.lon().to_radians();
        let lon2 = b.lon().to_radians();
        let dlat = sin(0.5 * (lat2 - lat1));
        let dlon = sin(0.5 * (lon2 - lon1));
        let x = dlat * dlat + dlon * dlon * cos(lat1) * cos(lat2);
        2. * asin(sqrt(fmin(1., x)))
    }

    /// Returns the bearing from the first point to the second point.
    pub fn get_bearing<M2: Clone + Default>(&self, b: &LonLat<M2>) -> f64 {
        debug_assert!(self.is_valid(), "get_bearing() called on invalid LonLat (self)");
        debug_assert!(b.is_valid(), "get_bearing() called on invalid LonLat (self)");
        let lat1 = self.lat().to_radians();
        let lat2 = b.lat().to_radians();
        let lon1 = self.lon().to_radians();
        let lon2 = b.lon().to_radians();
        let y = sin(lon2 - lon1) * cos(lat2);
        let x = cos(lat1) * sin(lat2) - sin(lat1) * cos(lat2) * cos(lon2 - lon1);

        (atan2(y, x).to_degrees() + 360.) % 360.
    }
}
impl<M: Clone + Default> From<S2CellId> for LonLat<M> {
    fn from(c: S2CellId) -> Self {
        LonLat::from_s2cellid(c)
    }
}
impl<M: Clone + Default> From<&S2Point> for LonLat<M> {
    fn from(p: &S2Point) -> Self {
        LonLat::from_s2_point(p)
    }
}
impl<M1: Clone + Default, M2: Clone + Default> Add<LonLat<M2>> for LonLat<M1> {
    type Output = LonLat<M1>;
    fn add(self, rhs: LonLat<M2>) -> Self::Output {
        LonLat::new(self.lon() + rhs.lon(), self.lat() + rhs.lat(), self.0.m)
    }
}
impl<M1: Clone + Default, M2: Clone + Default> Sub<LonLat<M2>> for LonLat<M1> {
    type Output = LonLat<M1>;
    fn sub(self, rhs: LonLat<M2>) -> Self::Output {
        LonLat::new(self.lon() - rhs.lon(), self.lat() - rhs.lat(), self.0.m)
    }
}
impl<M1: Clone + Default, M2: Clone + Default> Mul<LonLat<M2>> for LonLat<M1> {
    type Output = LonLat<M1>;
    fn mul(self, rhs: LonLat<M2>) -> Self::Output {
        LonLat::new(self.lon() * rhs.lon(), self.lat() * rhs.lat(), self.0.m)
    }
}
impl<M1: Clone + Default, M2: Clone + Default> Div<LonLat<M2>> for LonLat<M1> {
    type Output = LonLat<M1>;
    fn div(self, rhs: LonLat<M2>) -> Self::Output {
        LonLat::new(self.lon() / rhs.lon(), self.lat() / rhs.lat(), self.0.m)
    }
}
impl Neg for LonLat {
    type Output = LonLat;
    fn neg(self) -> Self::Output {
        LonLat::new(-self.lon(), -self.lat(), self.0.m)
    }
}
impl Eq for LonLat {}
impl Ord for LonLat {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.lon().partial_cmp(&other.lon()) {
            Some(Ordering::Equal) => {}
            other => return other.unwrap(), // Handle cases where `lon` comparison is not equal
        }
        match self.lat().partial_cmp(&other.lat()) {
            Some(order) => order,
            None => Ordering::Equal, // This handles the NaN case safely
        }
    }
}
impl PartialOrd for LonLat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
