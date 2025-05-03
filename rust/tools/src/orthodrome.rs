use libm::{atan2, cos, sin, sqrt};
use s2json::VectorPoint;

use geometry::LonLat;

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
    pub lon1: f64,
    /// start latitude in radians
    pub lat1: f64,
    /// end longitude in radians
    pub lon2: f64,
    /// end latitude in radians
    pub lat2: f64,
    /// distance property
    pub a: f64,
    /// distance property
    pub dist: f64,
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
