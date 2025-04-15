/// The struct containing information about the ellipsoid. This must be
/// initialized by geod_init() before use.
#[derive(Debug, Default, Clone, Copy)]
pub struct GeodGeodesic {
    /// the equatorial radius
    pub a: f64,
    /// the flattening
    pub f: f64,
    /// the second flattening
    pub f1: f64,
    /// second eccentricity
    pub e2: f64,
    /// the second eccentricity squared
    pub ep2: f64,
    /// third  flattening
    pub n: f64,
    /// semiminor axis
    pub b: f64,
    /// TODO: I don't know what this represents
    pub c2: f64,
    /// the tolerance
    pub etol2: f64,
    /// TODO: I don't know what this represents
    pub a3x: [f64; 6],
    /// TODO: I don't know what this represents
    pub c3x: [f64; 15],
    /// TODO: I don't know what this represents
    pub c4x: [f64; 21],
}

//   /**
//    * The struct containing information about a single geodesic.  This must be
//    * initialized by geod_lineinit(), geod_directline(), geod_gendirectline(),
//    * or geod_inverseline() before use.
//    **********************************************************************/
//   pub struct GeodGeodesicline {
//     double lat1;                /**< the starting latitude */
//     double lon1;                /**< the starting longitude */
//     double azi1;                /**< the starting azimuth */
//     double a;                   /**< the equatorial radius */
//     double f;                   /**< the flattening */
//     double salp1;               /**< sine of \e azi1 */
//     double calp1;               /**< cosine of \e azi1 */
//     double a13;                 /**< arc length to reference point */
//     double s13;                 /**< distance to reference point */
//     /**< @cond SKIP */
//     double b, c2, f1, salp0, calp0, k2,
//       ssig1, csig1, dn1, stau1, ctau1, somg1, comg1,
//       A1m1, A2m1, A3c, B11, B21, B31, A4, B41;
//     double C1a[6+1], C1pa[6+1], C2a[6+1], C3a[6], C4a[6];
//     /**< @endcond */
//     unsigned caps;              /**< the capabilities */
//   }

//   /**
//    * The struct for accumulating information about a geodesic polygon.  This is
//    * used for computing the perimeter and area of a polygon.  This must be
//    * initialized by geod_polygon_init() before use.
//    **********************************************************************/
//   pub struct GeodPolygon {
//     double lat;                 /**< the current latitude */
//     double lon;                 /**< the current longitude */
//     /**< @cond SKIP */
//     double lat0;
//     double lon0;
//     double A[2];
//     double P[2];
//     int polyline;
//     int crossings;
//     /**< @endcond */
//     unsigned num;               /**< the number of points so far */
//   };
