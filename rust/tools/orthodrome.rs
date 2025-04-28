use libm::{atan2, cos, sin, sqrt};
use s2json::VectorPoint;

use crate::geometry::LonLat;

/// # Orthodrome
///
/// ## Description
/// Represents an orthodrome, which is the shortest path between two points on a sphere.
/// [Learn more here](http://www.movable-type.co.uk/scripts/latlong.html)
///
/// ## NOTE
/// There is no reason to use this outside verbosity. You can create an S1Angle or use the utility functions in LonLat
///
/// ## Links
/// - http://www.movable-type.co.uk/scripts/latlong.html
#[derive(Debug, Clone, Default)]
pub struct Orthodrome {
    /// start longitude in radians
    lon1: f64,
    /// start latitude in radians
    lat1: f64,
    /// end longitude in radians
    lon2: f64,
    /// end latitude in radians
    lat2: f64,
    /// distance property
    a: f64,
    /// distance property
    dist: f64,
}
impl Orthodrome {
    /// Create an orthodrome
    pub fn new(start_lon: f64, start_lat: f64, end_lon: f64, end_lat: f64) -> Orthodrome {
        let lon1 = start_lon.to_radians();
        let lat1 = start_lat.to_radians();
        let lon2 = end_lon.to_radians();
        let lat2 = end_lat.to_radians();
        let d_lat = lat2 - lat1;
        let d_lon = lon2 - lon1;
        let a = sin(d_lat / 2.) * sin(d_lat / 2.)
            + cos(lat1) * cos(lat2) * sin(d_lon / 2.) * sin(d_lon / 2.);
        let dist = 2. * atan2(sqrt(a), sqrt(1. - a));

        Orthodrome { lon1, lat1, lon2, lat2, a, dist }
    }

    /// Create an orthodrome from two points
    pub fn from_points(p1: &LonLat, p2: &LonLat) -> Orthodrome {
        Orthodrome::new(p1.lon(), p1.lat(), p2.lon(), p2.lat())
    }

    /// Create an orthodrome from two points
    pub fn from_vector_points<M1: Clone, M2: Clone>(
        p1: &VectorPoint<M1>,
        p2: &VectorPoint<M2>,
    ) -> Orthodrome {
        Orthodrome::new(p1.x, p1.y, p2.x, p2.y)
    }

    /// input t 0->1. Find a point along the orthodrome.
    /// @param t - distance along the orthodrome to find
    /// @returns [lon, lat]
    pub fn intermediate_point(&self, t: f64) -> LonLat {
        let Self { lon1, lon2, lat1, lat2, dist, .. } = self;

        // check corner cases first
        if t == 0. {
            return LonLat::new(lon1.to_degrees(), lat1.to_degrees(), None);
        } else if t == 1. {
            return LonLat::new(lon2.to_degrees(), lat2.to_degrees(), None);
        }
        // check if points are equal
        else if lon1 == lon2 && lat1 == lat2 {
            return LonLat::new(lon1.to_degrees(), lat1.to_degrees(), None);
        }

        let a = sin((1. - t) * dist) / sin(*dist);
        let b = sin(t * dist) / sin(*dist);

        let x = a * cos(*lat1) * cos(*lon1) + b * cos(*lat2) * cos(*lon2);
        let y = a * cos(*lat1) * sin(*lon1) + b * cos(*lat2) * sin(*lon2);
        let z = a * sin(*lat1) + b * sin(*lat2);

        let lat = atan2(z, sqrt(x * x + y * y));
        let lon = atan2(y, x);

        LonLat::new(lon.to_degrees(), lat.to_degrees(), None)
    }

    /// returns the bearing in degrees between the two points
    pub fn bearing(&self) -> f64 {
        let Self { lon1, lat1, lon2, lat2, .. } = self;

        let y = sin(lon2 - lon1) * cos(*lat2);
        let x = cos(*lat1) * sin(*lat2) - sin(*lat1) * cos(*lat2) * cos(lon2 - lon1);
        let angle_rad = atan2(y, x);

        (angle_rad.to_degrees() + 360.) % 360. // in degrees
    }

    /// Finds the distance between the two points in kilometers
    /// projected normalized (0->1)
    /// returns the total distance between the two points
    pub fn distance_to(&self) -> f64 {
        2. * atan2(sqrt(self.a), sqrt(1. - self.a))
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use s2json::MValue;

    use super::*;

    #[test]
    fn orthodrome() {
        let ortho = Orthodrome::new(0., 0., 0., 0.);
        assert_eq!(ortho.a, 0.);
        assert_eq!(ortho.dist, 0.);

        let ortho = Orthodrome::default();
        assert_eq!(ortho.a, 0.);
        assert_eq!(ortho.dist, 0.);
    }

    #[test]
    fn from_points() {
        let ortho = Orthodrome::from_points(&LonLat::new(0., 0., None), &LonLat::new(0., 0., None));
        assert_eq!(ortho.a, 0.);
        assert_eq!(ortho.dist, 0.);
    }

    #[test]
    fn from_vector_points() {
        let ortho = Orthodrome::from_vector_points(
            &VectorPoint::<()>::new_xy(0., 0., None),
            &VectorPoint::<MValue>::new_xy(0., 0., None),
        );
        assert_eq!(ortho.a, 0.);
        assert_eq!(ortho.dist, 0.);
    }

    #[test]
    fn intermediate_point_same() {
        let ortho = Orthodrome::new(0., 0., 0., 0.);
        assert_eq!(ortho.intermediate_point(0.5), LonLat::new(0., 0., None));
    }

    #[test]
    fn intermediate_point_far() {
        let ortho = Orthodrome::new(-60., -40., 20., 10.);
        assert_eq!(ortho.intermediate_point(0.), LonLat::new(-59.99999999999999, -40., None));
        assert_eq!(
            ortho.intermediate_point(0.2),
            LonLat::new(-39.13793657428956, -33.728521975616516, None)
        );
        assert_eq!(
            ortho.intermediate_point(0.4),
            LonLat::new(-21.692497560895635, -24.50037918247324, None)
        );
        assert_eq!(
            ortho.intermediate_point(0.6),
            LonLat::new(-6.830669211476937, -13.564157442008685, None)
        );
        assert_eq!(
            ortho.intermediate_point(0.8),
            LonLat::new(6.673353815433631, -1.8320330896428323, None)
        );
        assert_eq!(ortho.intermediate_point(1.), LonLat::new(20., 10., None));
    }

    #[test]
    fn distance_to_same() {
        let ortho = Orthodrome::new(0., 0., 0., 0.);
        assert_eq!(ortho.distance_to(), 0.);
    }

    #[test]
    fn distance_to_far() {
        let ortho = Orthodrome::new(-60., -40., 20., 10.);
        assert_eq!(ortho.distance_to(), 1.5514126949321814);
    }

    #[test]
    fn bearing() {
        let ortho = Orthodrome::new(-60., -40., 20., 10.);
        assert_eq!(ortho.bearing(), 75.936859467864);
    }
}
