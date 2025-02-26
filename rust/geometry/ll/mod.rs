use libm::{asin, atan2, cos, sin, sqrt};

use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::geometry::{MValue, S1Angle, S2CellId, S2Point, VectorPoint};

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
    pub fn from_s2cellid(cellid: &S2CellId) -> LonLat {
        let p = cellid.to_point();
        LonLat::from_s2_point(&p)
    }

    /// Convert a direction vector (not necessarily unit length) to an LonLat.
    pub fn from_s2_point(p: &S2Point) -> LonLat {
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
        debug_assert!(self.is_valid(), "get_bearing() called on invalid LonLat (self)");
        debug_assert!(b.is_valid(), "get_bearing() called on invalid LonLat (b)");

        let lat1 = self.lat().to_radians();
        let lat2 = b.lat().to_radians();
        let lon1 = self.lon().to_radians();
        let lon2 = b.lon().to_radians();
        let dlat = sin(0.5 * (lat2 - lat1));
        let dlon = sin(0.5 * (lon2 - lon1));
        let x = dlat * dlat + dlon * dlon * cos(lat1) * cos(lat2);
        2. * asin(sqrt(f64::min(1., x)))
    }

    /// Returns the bearing from the first point to the second point.
    pub fn get_bearing(&self, b: &LonLat) -> f64 {
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
impl Add<LonLat> for LonLat {
    type Output = LonLat;
    fn add(self, rhs: LonLat) -> Self::Output {
        LonLat::new(self.lon() + rhs.lon(), self.lat() + rhs.lat(), self.0.m)
    }
}
impl Sub<LonLat> for LonLat {
    type Output = LonLat;
    fn sub(self, rhs: LonLat) -> Self::Output {
        LonLat::new(self.lon() - rhs.lon(), self.lat() - rhs.lat(), self.0.m)
    }
}
impl Mul<LonLat> for LonLat {
    type Output = LonLat;
    fn mul(self, rhs: LonLat) -> Self::Output {
        LonLat::new(self.lon() * rhs.lon(), self.lat() * rhs.lat(), rhs.0.m)
    }
}
impl Div<LonLat> for LonLat {
    type Output = LonLat;
    fn div(self, rhs: LonLat) -> Self::Output {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_points() {
        let ll = LonLat::from(&S2Point { x: 0.0, y: 0.0, z: 0.0 });
        assert_eq!(ll, LonLat::new(0.0, 0.0, None));
        let ll = LonLat::from(&S2Point { x: 1.0, y: 0.0, z: 0.0 });
        assert_eq!(ll, LonLat::new(0.0, 0.0, None));
        let ll = LonLat::from(&S2Point { x: 0.0, y: 1.0, z: 0.0 });
        assert_eq!(ll, LonLat::new(90.0, 0.0, None));
        let ll = LonLat::from(&S2Point { x: 0.0, y: 0.0, z: -1.0 });
        assert_eq!(ll, LonLat::new(0.0, -90.0, None));
        let ll = LonLat::from(&S2Point { x: 0.0, y: 0.0, z: 1.0 });
        assert_eq!(ll, LonLat::new(0.0, 90.0, None));
    }

    #[test]
    fn from_s2cell_id() {
        let ll = LonLat::from(&S2CellId::new(1152921504606846977));
        assert_eq!(ll, LonLat::new(0.0, 0.0, None));
    }

    #[test]
    fn coords() {
        let ll = LonLat::new(20.0, 50.0, None);
        assert_eq!(ll.coords(), (20.0, 50.0));
    }

    #[test]
    fn get_distance() {
        let ll = LonLat::new(0.0, 0.0, None);
        assert_eq!(ll.get_distance(&LonLat::new(0.0, 0.0, None)), 0.0);
        assert_eq!(
            ll.get_distance(&LonLat::new(0.017453292519943295, 0.0, None)),
            0.00030461741978670857
        );
    }

    #[test]
    fn normalize() {
        let mut ll = LonLat::new(0.0, 0.0, None);
        ll.normalize();
        assert_eq!(ll, LonLat::new(0.0, 0.0, None));
        let mut ll = LonLat::new(0.01745329251994, 0.111111, None);
        ll.normalize();
        assert_eq!(ll, LonLat::new(0.01745329251991734, 0.111111, None));
        let mut ll = LonLat::new(640.0, 100.0, None);
        ll.normalize();
        assert_eq!(ll, LonLat::new(-80.0, 90.0, None));
        let mut ll = LonLat::new(-640.0, -100.0, None);
        ll.normalize();
        assert_eq!(ll, LonLat::new(80.0, -90.0, None));
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn to_angles() {
        let ll = LonLat::new(0.0, 0.0, None);
        assert_eq!(ll.to_angles(), (0.0.into(), 0.0.into()));
        let ll = LonLat::new(0.01745329251994, 0.111111, None);
        assert_eq!(ll.to_angles(), (0.00030461741978665105.into(), 0.0019392527851834196.into()));
        let ll = LonLat::new(90.0, 180.0, None);
        assert_eq!(ll.to_angles(), (1.5707963267948966.into(), 3.141592653589793.into()));
    }

    #[test]
    fn to_point_and_gl() {
        let ll = LonLat::new(0.0, 0.0, None);
        assert_eq!(S2Point::from(&ll), S2Point { x: 1.0, y: 0.0, z: 0.0 });
        let ll = LonLat::new(90.0, 0.0, None);
        assert_eq!(ll.to_point(), S2Point { x: 6.123233995736766e-17, y: 1.0, z: 0.0 });
        let ll = LonLat::new(0.0, 90.0, None);
        assert_eq!(ll.to_point(), S2Point { x: 6.123233995736766e-17, y: 0.0, z: 1.0 });
        let ll = LonLat::new(180.0, 0., None);
        assert_eq!(ll.to_point(), S2Point { x: -1.0, y: 1.2246467991473532e-16, z: 0.0 });
        assert_eq!(ll.to_point_gl(), S2Point { x: 1.2246467991473532e-16, y: 0.0, z: -1.0 });
    }

    #[test]
    fn bearing() {
        let ll = LonLat::new(0.0, 0.0, None);
        assert_eq!(ll.get_bearing(&LonLat::new(0.0, 0.0, None)), 0.0);
        assert_eq!(ll.get_bearing(&LonLat::new(90.0, 0.0, None)), 90.0);
        assert_eq!(ll.get_bearing(&LonLat::new(180.0, 0.0, None)), 90.0);
        assert_eq!(ll.get_bearing(&LonLat::new(0.0, 90.0, None)), 0.0);
        assert_eq!(ll.get_bearing(&LonLat::new(-89.9, 0.0, None)), 270.0);
        assert_eq!(ll.get_bearing(&LonLat::new(0.0, -90.0, None)), 180.0);
        assert_eq!(ll.get_bearing(&LonLat::new(-180.0, 0.0, None)), 270.0);
        let ll = LonLat::new(-60.0, -40.0, None);
        assert_eq!(ll.get_bearing(&LonLat::new(20.0, 10.0, None)), 75.936859467864);
    }

    #[test]
    fn maths() {
        // ADD
        let ll1 = LonLat::new(15.0, -20.0, None);
        let ll2 = LonLat::new(30.0, 40.0, None);
        let ll3 = ll1 + ll2;
        assert_eq!(ll3, LonLat::new(45.0, 20.0, None));
        // SUB
        let ll1 = LonLat::new(15.0, -20.0, None);
        let ll2 = LonLat::new(30.0, 40.0, None);
        let ll3 = ll1 - ll2;
        assert_eq!(ll3, LonLat::new(-15.0, -60.0, None));
        // MUL
        let ll1 = LonLat::new(15.0, -20.0, None);
        let ll2 = LonLat::new(30.0, 40.0, None);
        let ll3 = ll1 * ll2;
        assert_eq!(ll3, LonLat::new(450.0, -800.0, None));
        // DIV
        let ll1 = LonLat::new(15.0, -20.0, None);
        let ll2 = LonLat::new(30.0, 40.0, None);
        let ll3 = ll1 / ll2;
        assert_eq!(ll3, LonLat::new(0.5, -0.5, None));
        // NEG
        let ll1 = LonLat::new(15.0, -20.0, None);
        let ll2 = -ll1;
        assert_eq!(ll2, LonLat::new(-15.0, 20.0, None));
        // CMP
        let ll1 = LonLat::new(15.0, -20.0, None);
        let ll2 = LonLat::new(30.0, 40.0, None);
        assert!(ll1 < ll2);
        assert!(ll1 <= ll2);
        assert!(ll2 > ll1);
        assert!(ll2 >= ll1);

        let ll1 = LonLat::new(15.0, -20.0, None);
        let ll2 = LonLat::new(15.0, 40.0, None);
        assert!(ll1 < ll2);
        assert!(ll1 <= ll2);
        assert!(ll2 > ll1);
        assert!(ll2 >= ll1);

        let ll1 = LonLat::new(15.0, -20.0, None);
        let ll2 = LonLat::new(15.0, -20.0, None);
        assert!(ll1 == ll2);

        let ll1 = LonLat::new(15.0, f64::NAN, None);
        let ll2 = LonLat::new(15.0, f64::NAN, None);
        assert!(ll1 != ll2);
    }
}
