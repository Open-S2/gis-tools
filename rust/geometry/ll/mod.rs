use libm::{asin, atan2, cos, fabs, sin, sqrt};

use core::cmp::Ordering;
use core::f64::consts::PI;
use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::geometry::{MValue, S2CellId, S2Point, VectorPoint};

/// This class represents a point on the unit sphere as a pair
/// of latitude-longitude coordinates.  Like the rest of the "geometry"
/// package, the intent is to represent spherical geometry as a mathematical
/// abstraction, so functions that are specifically related to the Earth's
/// geometry (e.g. easting/northing conversions) should be put elsewhere.
#[derive(Clone, PartialEq, Debug)]
pub struct LonLat(VectorPoint);

impl LonLat {
    /// Build a new LonLat
    pub fn new(lon: f64, lat: f64, m: Option<MValue>) -> LonLat {
        LonLat(VectorPoint::new(lon, lat, None, m))
    }

    /// Return the longitude
    pub fn lon(&self) -> f64 {
        self.0.x
    }

    /// Return the latitude
    pub fn lat(&self) -> f64 {
        self.0.y
    }

    /// Convert a S2CellId to an LonLat
    pub fn from_s2cellid(cellid: &S2CellId) -> LonLat {
        let p = cellid.to_point();
        LonLat::from_s2_point(&p)
    }

    /// Convert a direction vector (not necessarily unit length) to an LonLat.
    pub fn from_s2_point(p: &S2Point) -> LonLat {
        let lon_rad = atan2(p.y + 0.0, p.x + 0.0);
        let lat_rad = atan2(p.z, sqrt(p.x * p.x + p.y * p.y));
        let ll = LonLat::new((lon_rad).to_degrees(), (lat_rad).to_degrees(), None);
        if !ll.is_valid() {
            unreachable!();
        }
        ll
    }

    /// return the value of the axis
    pub fn from_axis(&self, axis: u8) -> f64 {
        if axis == 0 {
            self.lon()
        } else {
            self.lat()
        }
    }

    /// Normalize the coordinates to the range [-180, 180] and [-90, 90] deg.
    pub fn normalize(&mut self) {
        let mut lon = self.lon();
        let mut lat = self.lat();
        while lon < -180. {
            lon += 360.
        }
        while lon > 180. {
            lon -= 360.
        }
        while lat < -90. {
            lat += 180.
        }
        while lat > 90. {
            lat -= 180.
        }
    }

    /// Return the latitude or longitude coordinates in degrees.
    pub fn coords(self) -> (f64, f64) {
        (self.lon(), self.lat())
    }

    /// Return true if the latitude is between -90 and 90 degrees inclusive
    /// and the longitude is between -180 and 180 degrees inclusive.
    pub fn is_valid(&self) -> bool {
        fabs(self.lat()) <= (PI / 2.0) && fabs(self.lon()) <= PI
    }

    /// Converts an LonLat to the equivalent unit-length vector.  Unnormalized
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
        if !self.is_valid() {
            unreachable!();
        }
        let lon: f64 = (self.lon()).to_degrees();
        let lat: f64 = (self.lat()).to_degrees();
        S2Point::new(
            cos(lat) * cos(lon), // x
            cos(lat) * sin(lon), // y
            sin(lat),            // z
        )
    }

    /// An alternative to to_point() that returns a GPU compatible vector.
    pub fn to_point_gl(&self) -> S2Point {
        let lon: f64 = (self.lon()).to_degrees();
        let lat: f64 = (self.lat()).to_degrees();
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
    pub fn get_distance(&self, b: &LonLat) -> f64 {
        // This implements the Haversine formula, which is numerically stable for
        // small distances but only gets about 8 digits of precision for very large
        // distances (e.g. antipodal points).  Note that 8 digits is still accurate
        // to within about 10cm for a sphere the size of the Earth.
        //
        // This could be fixed with another sin() and cos() below, but at that point
        // you might as well just convert both arguments to S2Points and compute the
        // distance that way (which gives about 15 digits of accuracy for all
        // distances).
        if !self.is_valid() || !b.is_valid() {
            unreachable!();
        }

        let lat1 = self.lat();
        let lat2 = b.lat();
        let lon1 = self.lon();
        let lon2 = b.lon();
        let dlat = sin(0.5 * (lat2 - lat1));
        let dlon = sin(0.5 * (lon2 - lon1));
        let x = dlat * dlat + dlon * dlon * cos(lat1) * cos(lat2);
        2. * asin(sqrt(f64::min(1., x)))
    }
}
impl From<&S2CellId> for LonLat {
    fn from(c: &S2CellId) -> Self {
        LonLat::from_s2cellid(c)
    }
}
impl From<&S2Point> for LonLat {
    fn from(p: &S2Point) -> Self {
        LonLat::from_s2_point(p)
    }
}
impl Add<f64> for LonLat {
    type Output = LonLat;
    fn add(self, rhs: f64) -> Self::Output {
        LonLat::new(self.lat() + rhs, self.lon() + rhs, self.0.m)
    }
}
impl Add<LonLat> for LonLat {
    type Output = LonLat;
    fn add(self, rhs: LonLat) -> Self::Output {
        LonLat::new(self.lat() + rhs.lat(), self.lon() + rhs.lon(), self.0.m)
    }
}
impl Sub<LonLat> for LonLat {
    type Output = LonLat;
    fn sub(self, rhs: LonLat) -> Self::Output {
        LonLat::new(self.lat() - rhs.lat(), self.lon() - rhs.lon(), self.0.m)
    }
}
impl Sub<f64> for LonLat {
    type Output = LonLat;
    fn sub(self, rhs: f64) -> Self::Output {
        LonLat::new(self.lat() - rhs, self.lon() - rhs, self.0.m)
    }
}
impl Mul<f64> for LonLat {
    type Output = LonLat;
    fn mul(self, rhs: f64) -> Self::Output {
        LonLat::new(self.lat() * rhs, self.lon() * rhs, self.0.m)
    }
}
impl Mul<LonLat> for f64 {
    type Output = LonLat;
    fn mul(self, rhs: LonLat) -> Self::Output {
        LonLat::new(self * rhs.lat(), self * rhs.lon(), rhs.0.m)
    }
}
impl Div<f64> for LonLat {
    type Output = LonLat;
    fn div(self, rhs: f64) -> Self::Output {
        LonLat::new(self.lat() / rhs, self.lon() / rhs, self.0.m)
    }
}
impl Div<LonLat> for LonLat {
    type Output = LonLat;
    fn div(self, rhs: LonLat) -> Self::Output {
        LonLat::new(self.lat() / rhs.lat(), self.lon() / rhs.lon(), self.0.m)
    }
}
impl Neg for LonLat {
    type Output = LonLat;
    fn neg(self) -> Self::Output {
        LonLat::new(-self.lat(), -self.lon(), self.0.m)
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
