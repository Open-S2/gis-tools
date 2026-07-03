// TEMPLATE INFO: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml
use crate::{
    geometry::normalize_ll,
    parsers::Reader,
    readers::{
        Grib2Table3_1, Grib2Table3_2, Grib2Table3_3, Grib2Table3_4, Grib2Table3_4Bit1,
        Grib2Table3_4Bit2, Grib2Table3_5, Grib2Table3_5Bit1,
    },
};
use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};
use libm::{asin, atan, atan2, cos, fabs, log, pow, sin, sqrt, tan};
use s2json::{VectorMultiPoint, VectorPoint};

// grib individually ports all of proj4 as well:
// https://github.com/NOAA-EMC/wgrib2/blob/develop/src/geo.c#L262

/// # Grid Units
///
/// ## Links
/// - [Docs](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect3.shtml)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2GridUnits {
    /// degrees
    Degrees,
    /// meters
    Meters,
}

/// Returns a template generator for the given template number
/// All templates are listed [here](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml)
#[derive(Debug, Clone, PartialEq)]
pub enum Grib2Template3 {
    /// Latitude/Longitude (or equidistant cylindrical, or Plate Carree)
    EquatorialTemplate(EquatorialTemplate),
    /// Rotate Latitude/Longitude (or equidistant/cylindrical, or Plate Carree)
    RotatedLonLatTemplate(RotatedLonLatTemplate),
    /// Mercator Projection
    MercatorTemplate(MercatorTemplate),
    /// Polar Stereographic Projection (Can be North or South)
    PolarTemplate(PolarTemplate),
    /// Lambert Conformal Conic Projection
    LambertTemplate(LambertConformalTemplate),
    /// Gaussian Latitude/Longitude
    GaussianTemplate(GaussianTemplate),
}
impl Grib2Template3 {
    /// Create a new instance of Grib2Template3
    ///
    /// - `template`: template number parse block
    /// - `section`: byte block
    ///
    /// ## Returns
    /// Template generator
    pub fn new<T: Reader>(template: Grib2Table3_1, section: &T) -> Self {
        // TODO: Addd all Grib2Table3_1 options and set correct transform
        match template {
            Grib2Table3_1::LatitudeLongitude => {
                Grib2Template3::EquatorialTemplate(EquatorialTemplate::new(section))
            }
            Grib2Table3_1::RotatedLatitudeLongitude => {
                Grib2Template3::RotatedLonLatTemplate(RotatedLonLatTemplate::new(section))
            }
            Grib2Table3_1::Mercator => {
                Grib2Template3::MercatorTemplate(MercatorTemplate::new(section))
            }
            Grib2Table3_1::PolarStereographicProjection => {
                Grib2Template3::PolarTemplate(PolarTemplate::new(section))
            }
            Grib2Table3_1::LambertConformal => {
                Grib2Template3::LambertTemplate(LambertConformalTemplate::new(section))
            }
            Grib2Table3_1::GaussianLatitudeLongitude => {
                Grib2Template3::GaussianTemplate(GaussianTemplate::new(section))
            }
            _ => panic!("Template 3.{template} not defined"),
        }
    }
    /// Convert this section into grid data
    pub fn build_grid(&mut self) -> VectorMultiPoint {
        match self {
            Grib2Template3::EquatorialTemplate(template) => template.build_grid(),
            Grib2Template3::RotatedLonLatTemplate(template) => template.build_grid(),
            Grib2Template3::MercatorTemplate(template) => template.build_grid(),
            Grib2Template3::PolarTemplate(template) => template.build_grid(),
            Grib2Template3::LambertTemplate(template) => template.build_grid(),
            Grib2Template3::GaussianTemplate(template) => template.build_grid(),
        }
    }
}

/// # GRIB2 - GRID DEFINITION TEMPLATE 3.0
///
/// ## Latitude/Longitude (or equidistant cylindrical, or Plate Carree)
///
/// ## Links
///
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-0.shtml)
///
/// ## Notes
///
/// - Basic angle of the initial production domain and subdivisions of this basic angle are provided
///   to manage cases where the recommended unit of 10-6 degrees is not applicable to describe the
///   extreme longitudes and latitudes, and direction increments. For these last six descriptors, the
///   unit is equal to the ratio of the basic angle and the subdivisions number. For ordinary cases,
///   zero and missing values should be coded, equivalent to respective values of 1 and 106  (10-6
///   degrees unit).
///
/// - For data on a quasi-regular grid, in which all the rows or columns do not necessarily have the
///   same number of grid points either Ni (octets 31-34) of Nj (octets 35-38) and the corresponding Di
///   (octets 64-67) or Dj (octets 68-71) shall be coded with all bits set to 1 (missing). The actual
///   number of points along each parallel or meridian shall be coded in the octets immediately following
///   the grid definition template (octets [xx+1]-nn), as described in the description of the grid
///   definition section.
///
/// - A quasi-regular grid is only defined for appropriate grid scanning modes. Either rows or columns,
///   but not both simultaneously, may have variable numbers of points or variable spacing. The first
///   point in each row (column) shall be positioned at the meridian (parallel) indicted by octets 47-54.
///   The grid points shall be evenly spaced in latitude (longitude).
///
/// A scale value of radius of spherical Earth, or major axis of oblate spheroid Earth is delivered
/// from applying appropriate scale factor to the value expressed in meters.
///
/// - It is recommended to use unsigned direction increments.
///
/// - In most cases, multiplying Ni (octets 31-34) by Nj (octets 35-38) yields the total number of
///   points in the grid. However, this may not be true if bit 8 of the scanning mode flags (octet 72)
///   is set to 1.
#[derive(Debug, Clone, PartialEq)]
pub struct EquatorialTemplate {
    /// Shape of Earth [Table 3.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-2.shtml)
    pub shape: Grib2Table3_2,
    /// Scale Factor of radius of spherical Earth
    pub radius_scale_factor: u8,
    /// Scale value of radius of spherical Earth
    pub radius_scale_value: u32,
    /// Scale factor of major axis of oblate spheroid Earth
    pub major_axis_scale_factor: u8,
    /// Scale value of major axis of oblate spheroid Earth
    pub major_axis_scale_value: u32,
    /// Scale factor of minor axis of oblate spheroid Earth
    pub minor_axis_scale_factor: u8,
    /// Scale value of minor axis of oblate spheroid Earth
    pub minor_axis_scale_value: u32,
    /// Number of points along a parallel (W-E)
    pub nx: u32,
    /// Number of points along a meridian (N-S)
    pub ny: u32,
    /// Basic angle of the initial production domain
    pub basic_angle: f64,
    /// Subdivisions of basic angle used to define extreme longitudes and latitudes, and direction increments
    pub subdivisions: f64,
    /// Latitude of first grid point
    pub lat1: f64,
    /// Longitude of first grid point
    pub lon1: f64,
    /// Resolution and component flags [Table 3.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-3.shtml)
    pub resolution: Grib2Table3_3,
    /// Latitude of last grid point
    pub lat2: f64,
    /// Longitude of last grid point
    pub lon2: f64,
    /// i direction increment
    pub dx: f64,
    /// j direction increment
    pub dy: f64,
    /// Scanning mode [Table 3.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml)
    pub scan_mode: Grib2Table3_4,
    /// Grid Units
    pub grid_units: Grib2GridUnits,
}
impl EquatorialTemplate {
    /// Create a new instance of EquatorialTemplate
    ///
    /// ## Parameters
    /// - `section`: byte block for template 3.0
    ///
    /// ## Returns
    /// The parsed template
    pub fn new<T: Reader>(section: &T) -> Self {
        let shape = section.uint8(Some(14));
        let basic_angle = section.uint32_be(Some(38)) as f64;
        let subdivisions = section.uint32_be(Some(42)) as f64;
        // build resolution values
        let resolution_code = section.uint8(Some(54));
        // build scan_mode values
        let scan_mode_code = section.uint8(Some(71));

        let ratio = if basic_angle == 0. { 1.0e-6 } else { basic_angle / subdivisions };

        Self {
            shape: shape.into(),
            radius_scale_factor: section.uint8(Some(15)),
            radius_scale_value: section.uint32_be(Some(16)),
            major_axis_scale_factor: section.uint8(Some(20)),
            major_axis_scale_value: section.uint32_be(Some(21)),
            minor_axis_scale_factor: section.uint8(Some(25)),
            minor_axis_scale_value: section.uint32_be(Some(26)),
            nx: section.uint32_be(Some(30)),
            ny: section.uint32_be(Some(34)),
            basic_angle,
            subdivisions,
            lat1: read_grib_int(section.int32_be(Some(46))) * ratio,
            lon1: read_grib_int(section.int32_be(Some(50))) * ratio,
            resolution: resolution_code.into(),
            lat2: read_grib_int(section.int32_be(Some(55))) * ratio,
            lon2: read_grib_int(section.int32_be(Some(59))) * ratio,
            dx: read_grib_int(section.int32_be(Some(63))) * ratio,
            dy: read_grib_int(section.int32_be(Some(67))) * ratio,
            scan_mode: scan_mode_code.into(),
            grid_units: Grib2GridUnits::Degrees,
        }
    }
    /// Convert this section into grid data
    pub fn build_grid(&mut self) -> VectorMultiPoint {
        // for now let's just follow the most basic scan mode
        let Self { lat1, lat2, lon1, lon2, nx, ny, .. } = *self;
        // Step sizes for interpolation
        let lon_step = (lon2 - lon1) / (nx as f64 - 1.);
        let lat_step = (lat2 - lat1) / (ny as f64 - 1.);

        let mut res = Vec::with_capacity((nx * ny) as usize);

        for y in 0..ny {
            let y = y as f64;
            let lat = lat1 + y * lat_step;
            for x in 0..nx {
                let x = x as f64;
                // Interpolate longitude and latitude
                let lon = lon1 + x * lon_step;
                // create point and apply transform if provided (this grid is already in the correct projection)
                let mut ll = VectorPoint::new_xy(lon, lat, None);
                normalize_ll(&mut ll);
                res.push(ll);
            }
        }

        res
    }
}

/// # GRIB2 - GRID DEFINITION TEMPLATE 3.1
///
/// ## Rotate Latitude/Longitude (or equidistant/cylindrical, or Plate Carree)
///
/// ## Links
///
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-1.shtml)
///
/// ## Notes
///
/// - Basic angle of the initial production domain and subdivisions of this basic angle are provided
///   to manage cases where the recommended unit of 10-6 degrees is not applicable to describe the
///   extreme longitudes and latitudes, and direction increments. For these last six descriptors, the
///   unit is equal to the ratio of the basic angle and the subdivisions number. For ordinary cases,
///   zero and missing values should be coded, equivalent to respective values of 1 and 106  (10-6
///   degrees unit).
///
/// - Three parameters define a general latitude/longitude coordinate system, formed by a general
///   rotation of the sphere. One choice for these parameters is:
///   - (a) The geographic latitude in degrees of the southern pole of the coordinate system,06 for example.
///   - (b) The geographic longitude in degrees of the southern pole of the coordinate system,λp for example.
///   - (c) The angle of rotation in degrees about the new polar axis (measured clockwise when looking
///     from the southern to the northern pole) of the coordinate system, assuming the new axis to
///     have been obtained by first rotating the sphere through λp degrees about the geographic
///     polar axis and then rotating through (90 + 0p) degrees so that the southern pole moved along
///     the (previously rotated) Greenwich meridian.
///
/// - A quasi-regular grid is only defined for appropriate grid scanning modes. Either rows or columns,
///   but not both simultaneously, may have variable numbers of points or variable spacing. The first
///   point in each row (column) shall be positioned at the meridian (parallel) indicted by octets 47-54.
///   The grid points shall be evenly spaced in latitude (longitude).
///
/// - It is recommended to use unsigned direction increments.
#[derive(Debug, Clone, PartialEq)]
pub struct RotatedLonLatTemplate {
    /// Shape of Earth [Table 3.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-2.shtml)
    pub shape: Grib2Table3_2,
    /// Scale Factor of radius of spherical Earth
    pub radius_scale_factor: u8,
    /// Scale value of radius of spherical Earth
    pub radius_scale_value: u32,
    /// Scale factor of major axis of oblate spheroid Earth
    pub major_axis_scale_factor: u8,
    /// Scale value of major axis of oblate spheroid Earth
    pub major_axis_scale_value: u32,
    /// Scale factor of minor axis of oblate spheroid Earth
    pub minor_axis_scale_factor: u8,
    /// Scale value of minor axis of oblate spheroid Earth
    pub minor_axis_scale_value: u32,
    /// Number of points along a parallel (W-E)
    pub nx: u32,
    /// Number of points along a meridian (N-S)
    pub ny: u32,
    /// Basic angle of the initial production domain
    pub basic_angle: f64,
    /// Subdivisions of basic angle used to define extreme longitudes and latitudes, and direction increments
    pub subdivisions: f64,
    /// Latitude of first grid point
    pub lat1: f64,
    /// Longitude of first grid point
    pub lon1: f64,
    /// Resolution and component flags [Table 3.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-3.shtml)
    pub resolution: Grib2Table3_3,
    /// Latitude of last grid point
    pub lat2: f64,
    /// Longitude of last grid point
    pub lon2: f64,
    /// i direction increment
    pub dx: f64,
    /// j direction increment
    pub dy: f64,
    /// Scanning mode [Table 3.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml)
    pub scan_mode: Grib2Table3_4,
    /// Latitude of the southern pole of projection
    pub lat_sp: f64,
    /// Longitude of the southern pole of projection
    pub lon_sp: f64,
    /// Angle of rotation of projection
    pub rot_angle: f64,
    /// Grid Units
    pub grid_units: Grib2GridUnits,
}
impl RotatedLonLatTemplate {
    /// Create a new instance of EquatorialTemplate
    ///
    /// ## Parameters
    /// - `section`: byte block for template 3.0
    ///
    /// ## Returns
    /// The parsed template
    pub fn new<T: Reader>(section: &T) -> Self {
        let shape = section.uint8(Some(14));
        let basic_angle = section.uint32_be(Some(38)) as f64;
        let subdivisions = section.uint32_be(Some(42)) as f64;
        // build resolution values
        let resolution_code = section.uint8(Some(54));
        // build scan_mode values
        let scan_mode_code = section.uint8(Some(71));

        let ratio = if basic_angle == 0. { 1.0e-6 } else { basic_angle / subdivisions };

        Self {
            shape: shape.into(),
            radius_scale_factor: section.uint8(Some(15)),
            radius_scale_value: section.uint32_be(Some(16)),
            major_axis_scale_factor: section.uint8(Some(20)),
            major_axis_scale_value: section.uint32_be(Some(21)),
            minor_axis_scale_factor: section.uint8(Some(25)),
            minor_axis_scale_value: section.uint32_be(Some(26)),
            nx: section.uint32_be(Some(30)),
            ny: section.uint32_be(Some(34)),
            basic_angle,
            subdivisions,
            lat1: read_grib_int(section.int32_be(Some(46))) * ratio,
            lon1: read_grib_int(section.int32_be(Some(50))) * ratio,
            resolution: resolution_code.into(),
            lat2: read_grib_int(section.int32_be(Some(55))) * ratio,
            lon2: read_grib_int(section.int32_be(Some(59))) * ratio,
            dx: read_grib_int(section.int32_be(Some(63))) * ratio,
            dy: read_grib_int(section.int32_be(Some(67))) * ratio,
            scan_mode: scan_mode_code.into(),
            lat_sp: read_grib_int(section.int32_be(Some(72))) * ratio,
            lon_sp: read_grib_int(section.int32_be(Some(76))) * ratio,
            rot_angle: read_grib_int(section.int32_be(Some(80))) * ratio,
            grid_units: Grib2GridUnits::Degrees,
        }
    }
    /// Convert this section into grid data
    pub fn build_grid(&mut self) -> VectorMultiPoint {
        // https://github.com/NOAA-EMC/wgrib2/blob/develop/src/rotll.c#L89
        let Self { lat1, lat2, lon1, lon2, nx, ny, lat_sp, lon_sp, rot_angle, .. } = *self;

        // inverse transformation, reverse rotation angle
        let angle_rot = -rot_angle;

        let a = (90.0 + lat_sp).to_radians();
        let b = lon_sp.to_radians();
        let r = angle_rot.to_radians();
        let sin_a = sin(a);
        let cos_a = cos(a);

        // south pole to north pole
        let lon_step = (lon2 - lon1) / ((nx - 1) as f64);
        let lat_step = (lat2 - lat1) / ((ny - 1) as f64);

        let mut res = Vec::with_capacity((nx * ny) as usize);

        for y in 0..ny {
            let y = y as f64;
            let pr = (lat1 + y * lat_step).to_radians();
            for x in 0..nx {
                let x = x as f64;
                // Current point in rotated degrees, set to radian, adjust for rotation
                let gr = -(lon1 + x * lon_step).to_radians();
                let pm = asin(cos(pr) * cos(gr));
                let gm = atan2(cos(pr) * sin(gr), -sin(pr));

                let glat = asin(sin_a * sin(pm) - cos_a * cos(pm) * cos(gm - r)).to_degrees();
                let glon = (-b
                    + atan2(
                        cos(pm) * sin(gm - r),
                        sin_a * cos(pm) * cos(gm - r) + cos_a * sin(pm),
                    ))
                .to_degrees();

                // create point and apply transform if provided (this grid is already in the correct projection)
                let mut ll = VectorPoint::new_xy(glon, glat, None);
                normalize_ll(&mut ll);
                res.push(ll);
            }
        }

        res
    }
}

/// # GRIB2 - GRID DEFINITION TEMPLATE 3.10
///
/// ## Mercator
///
/// ## Links
///
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-10.shtml)
///
/// ## Notes
///
/// - Limited to the range of  0 to 90 degrees; if the angle of orientation of the grid is neither 0
///   nor 90 degrees, Di and Dj must be equal to each other.
///
/// - Grid lengths are in units of 10-3  m, at the latitude specified by LaD.
///
/// - A scale value of radius of spherical Earth, or major or minor axis of oblate spheroid Earth is
///   derived from applying appropriate scale factor to the value expressed in metres.
#[derive(Debug, Clone, PartialEq)]
pub struct MercatorTemplate {
    /// Shape of Earth [Table 3.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-2.shtml)
    pub shape: Grib2Table3_2,
    /// Scale Factor of radius of spherical Earth
    pub radius_scale_factor: u8,
    /// Scale value of radius of spherical Earth
    pub radius_scale_value: u32,
    /// Scale factor of major axis of oblate spheroid Earth
    pub major_axis_scale_factor: u8,
    /// Scale value of major axis of oblate spheroid Earth
    pub major_axis_scale_value: u32,
    /// Scale factor of minor axis of oblate spheroid Earth
    pub minor_axis_scale_factor: u8,
    /// Scale value of minor axis of oblate spheroid Earth
    pub minor_axis_scale_value: u32,
    /// Number of points along a parallel (W-E)
    pub nx: u32,
    /// Number of points along a meridian (N-S)
    pub ny: u32,
    /// Latitude of first grid point
    pub lat1: f64,
    /// Longitude of first grid point
    pub lon1: f64,
    /// Resolution and component flags [Table 3.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-3.shtml)
    pub resolution: Grib2Table3_3,
    /// LaD — latitude(s) at which the Mercator projection intersects the Earth (Latitude(s) where Di and Dj are specified)
    pub lat_d: f64,
    /// Latitude of last grid point
    pub lat2: f64,
    /// Longitude of last grid point
    pub lon2: f64,
    /// Scanning mode [Table 3.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml)
    pub scan_mode: Grib2Table3_4,
    /// Orientation of the grid, angle between i direction on the map and the Equator (see Note1)
    pub orient: f64,
    /// i direction increment
    pub dx: f64,
    /// j direction increment
    pub dy: f64,
    /// Grid Units
    pub grid_units: Grib2GridUnits,
}
impl MercatorTemplate {
    /// Create a new instance of `MercatorTemplate`
    ///
    /// ## Parameters
    /// - `section`: byte block for template 3.10
    ///
    /// ## Returns
    /// The parsed template
    pub fn new<T: Reader>(section: &T) -> Self {
        let shape = section.uint8(Some(14));
        // build resolution values
        let resolution_code = section.uint8(Some(46));
        // build scan mode
        let scan_mode_code = section.uint8(Some(59));

        Self {
            shape: shape.into(),
            radius_scale_factor: section.uint8(Some(15)),
            radius_scale_value: section.uint32_be(Some(16)),
            major_axis_scale_factor: section.uint8(Some(20)),
            major_axis_scale_value: section.uint32_be(Some(21)),
            minor_axis_scale_factor: section.uint8(Some(25)),
            minor_axis_scale_value: section.uint32_be(Some(26)),
            nx: section.uint32_be(Some(30)),
            ny: section.uint32_be(Some(34)),
            lat1: read_grib_int(section.int32_be(Some(38))) * 1e-6,
            lon1: read_grib_int(section.int32_be(Some(42))) * 1e-6,
            resolution: resolution_code.into(),
            lat_d: read_grib_int(section.int32_be(Some(47))) * 1e-6,
            lat2: read_grib_int(section.int32_be(Some(51))) * 1e-6,
            lon2: read_grib_int(section.int32_be(Some(55))) * 1e-6,
            scan_mode: scan_mode_code.into(),
            orient: read_grib_int(section.int32_be(Some(60))) * 1e-6,
            dx: read_grib_int(section.int32_be(Some(64))) * 1e-3,
            dy: read_grib_int(section.int32_be(Some(68))) * 1e-3,
            grid_units: Grib2GridUnits::Meters,
        }
    }
    /// Convert this section into grid data
    pub fn build_grid(&mut self) -> VectorMultiPoint {
        let Self { lat1, lat2, lon1, lon2, lat_d, nx, ny, scan_mode, .. } = *self;
        let nx = nx as usize;
        let ny = ny as usize;

        if lon1 < 0. || lon2 < 0. || lon1 > 360. || lon2 > 360. {
            panic!("BAD grid definition lon");
        }
        if lat1 < -90. || lat2 < -90. || lat1 > 90. || lat2 > 90. {
            panic!("BAD grid definition lat");
        }
        if nx < 1 || ny < 1 {
            panic!("Sorry geo/mercator code does not handle variable nx/ny yet\n");
        }

        // Mercator projection math requires s < n.
        let s_lat = lat1.min(lat2);
        let n_lat = lat1.max(lat2);

        // Longitude handling
        let (w_lon, e_lon) =
            if scan_mode.bit1 == Grib2Table3_4Bit1::PointsFirstRowColumnScanPlusIDirection {
                (lon1, lon2)
            } else {
                (lon2, lon1)
            };

        let mut e_lon_adj = e_lon;
        if e_lon_adj <= w_lon {
            e_lon_adj += 360.0;
        }

        let dlon = (e_lon_adj - w_lon) / (nx as f64 - 1.0);
        let radius = self.shape.earth_radius(
            self.radius_scale_factor as f64,
            self.radius_scale_value as f64,
            self.major_axis_scale_factor as f64,
            self.major_axis_scale_value as f64,
            self.minor_axis_scale_factor as f64,
            self.minor_axis_scale_value as f64,
        );

        let circum = 2.0 * PI * radius * lat_d.to_radians().cos();
        let mut dx_deg = self.dx * 360.0 / circum;

        if dx_deg != 0.0 {
            let error = (dx_deg - dlon).abs() / dx_deg.abs();
            if error >= 0.001 {
                // println!("Warning: Inconsistent dx vs dlon: {} vs {}", dx_deg, dlon);
            }
            dx_deg = dlon; // Domain trumps calculated resolution
        }

        // Map degrees to the Mercator projected Y space
        let s_proj = ((45.0 + s_lat / 2.0).to_radians().tan()).ln();
        let n_proj = ((45.0 + n_lat / 2.0).to_radians().tan()).ln();
        let dy_proj = (n_proj - s_proj) / (ny as f64 - 1.0);

        // Determine Iteration Start and Step
        // Bit 2: 0 = Southward (Top-Down), 1 = Northward (Bottom-Up)
        let (y_start_proj, y_step) =
            if scan_mode.bit2 == Grib2Table3_4Bit2::PointsFirstRowColumnScanPlusJDirection {
                (s_proj, dy_proj) // Start at south, move up
            } else {
                (n_proj, -dy_proj) // Start at north, move down
            };
        // Bit 1: 0 = Eastward (+i), 1 = Westward (-i)
        let x_step = if scan_mode.bit1 == Grib2Table3_4Bit1::PointsFirstRowColumnScanPlusIDirection
        {
            dx_deg
        } else {
            -dx_deg
        };

        let mut res = Vec::with_capacity(nx * ny);
        for j in 0..ny {
            let current_y_proj = y_start_proj + (j as f64 * y_step);
            // Reverse Mercator: Convert projected Y back to Latitude
            let row_lat = (current_y_proj.exp().atan().to_degrees() - 45.0) * 2.0;

            for i in 0..nx {
                let col_lon = lon1 + (i as f64 * x_step);
                let mut point = VectorPoint::new_xy(col_lon, row_lat, None);
                normalize_ll(&mut point);
                res.push(point);
            }
        }

        res
    }
}

/// # GRIB2 - GRID DEFINITION TEMPLATE 3.20
///
/// ## Polar Stereographic Projection (Can be North or South)
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-20.shtml)
///
/// ## Notes
/// - The orientation of the grid is given by the longitude of the meridian along which the
///   y-axis increases, LoV.
/// - The projection is defined by the latitude at which Dx and Dy are specified, LaD.
/// - Grid lengths Dx and Dy are in meters at the latitude LaD.
/// - Bit 3 of the resolution and component flags should be set to 1 to indicate that Dx and Dy
///   are given in meters.
#[derive(Debug, Clone, PartialEq)]
pub struct PolarTemplate {
    /// Shape of Earth [Table 3.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-2.shtml)
    pub shape: Grib2Table3_2,
    /// Scale Factor of radius of spherical Earth
    pub radius_scale_factor: u8,
    /// Scale value of radius of spherical Earth
    pub radius_scale_value: u32,
    /// Scale factor of major axis of oblate spheroid Earth
    pub major_axis_scale_factor: u8,
    /// Scale value of major axis of oblate spheroid Earth
    pub major_axis_scale_value: u32,
    /// Scale factor of minor axis of oblate spheroid Earth
    pub minor_axis_scale_factor: u8,
    /// Scale value of minor axis of oblate spheroid Earth
    pub minor_axis_scale_value: u32,
    /// Number of points along the x-axis
    pub nx: u32,
    /// Number of points along the y-axis
    pub ny: u32,
    /// Latitude of first grid point
    pub lat1: f64,
    /// Longitude of first grid point
    pub lon1: f64,
    /// Latitude where Dx and Dy are specified
    pub lat_d: f64,
    /// Orientation of the grid (LoV)
    pub lon_v: f64,
    /// Resolution and component flags [Table 3.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-3.shtml)
    pub resolution: Grib2Table3_3,
    /// x-direction grid length (meters at LaD)
    pub dx: f64,
    /// y-direction grid length (meters at LaD)
    pub dy: f64,
    /// Projection center flag [Table 3.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-5.shtml)
    pub proj_center: Grib2Table3_5,
    /// Scanning mode [Table 3.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml)
    pub scan_mode: Grib2Table3_4,
    /// Grid Units
    pub grid_units: Grib2GridUnits,
}
impl PolarTemplate {
    /// Create a new instance of PolarTemplate
    ///
    /// ## Parameters
    /// - `section`: byte block for template 3.20
    ///
    /// ## Returns
    /// The parsed template
    pub fn new<T: Reader>(section: &T) -> Self {
        let shape = section.uint8(Some(14));
        let proj_center = section.uint8(Some(63));
        // build resolution values
        let resolution_code = section.uint8(Some(54));
        // build scan mode
        let scan_mode_code = section.uint8(Some(64));

        Self {
            shape: shape.into(),
            radius_scale_factor: section.uint8(Some(15)),
            radius_scale_value: section.uint32_be(Some(16)),
            major_axis_scale_factor: section.uint8(Some(20)),
            major_axis_scale_value: section.uint32_be(Some(21)),
            minor_axis_scale_factor: section.uint8(Some(25)),
            minor_axis_scale_value: section.uint32_be(Some(26)),
            nx: section.uint32_be(Some(30)),
            ny: section.uint32_be(Some(34)),
            lat1: read_grib_int(section.int32_be(Some(38))) * 1e-6,
            lon1: read_grib_int(section.int32_be(Some(42))) * 1e-6,
            lat_d: read_grib_int(section.int32_be(Some(47))) * 1e-6,
            lon_v: read_grib_int(section.int32_be(Some(51))) * 1e-6,
            resolution: resolution_code.into(),
            dx: read_grib_int(section.int32_be(Some(55))) * 1e-3,
            dy: read_grib_int(section.int32_be(Some(59))) * 1e-3,
            proj_center: proj_center.into(),
            scan_mode: scan_mode_code.into(),
            grid_units: Grib2GridUnits::Meters,
        }
    }
    /// Convert this section into grid data
    pub fn build_grid(&mut self) -> VectorMultiPoint {
        let Self { lat1, dx, lon1, dy, nx, ny, lon_v, lat_d, proj_center, .. } = *self;
        let mut res = Vec::with_capacity((nx * ny) as usize);

        // 1. Math always uses absolute dx/dy for projection constants
        let abs_dx = fabs(dx);
        let abs_dy = fabs(dy);

        let lat1_rad = lat1.to_radians();
        let lon1_rad = lon1.to_radians();
        let mut orient = lon_v.to_radians();
        let lat_d_rad = fabs(lat_d).to_radians();
        let mut h = 1.0;

        if proj_center.bit1 == Grib2Table3_5Bit1::SouthPoleOnProjectionPlane {
            h = -1.;
            orient -= PI;
        }

        let radius = self.shape.earth_radius(
            self.radius_scale_factor as f64,
            self.radius_scale_value as f64,
            self.major_axis_scale_factor as f64,
            self.major_axis_scale_value as f64,
            self.minor_axis_scale_factor as f64,
            self.minor_axis_scale_value as f64,
        );

        let de = (1.0 + lat_d_rad.sin()) * radius;
        let dr = de * lat1_rad.cos() / (1. + h * lat1_rad.sin());

        // Calculate xp/yp using absolute values as in the original iplib/wgrib2 logic
        let mut xp = -h * (lon1_rad - orient).sin() * dr / abs_dx;
        let mut yp = (lon1_rad - orient).cos() * dr / abs_dy;

        // 2. Adjust xp/yp offsets based on Scan Mode (mimicking wgrib2 logic)
        // Bit 1: 0 = +i (East), 1 = -i (West)
        if self.scan_mode.bit1 == Grib2Table3_4Bit1::PointsFirstRowColumnScanMinusIDirection {
            xp = xp - nx as f64 + 1.0;
        }
        // Bit 2: 0 = -j (South/Top-Down), 1 = +j (North/Bottom-Up)
        if self.scan_mode.bit2 != Grib2Table3_4Bit2::PointsFirstRowColumnScanMinusJDirection {
            yp = yp - ny as f64 + 1.0;
        }

        let de2 = de * de;

        for iy in 0..ny {
            let iy_f = iy as f64;
            let dj = (iy_f - yp) * abs_dy;
            for ix in 0..nx {
                let ix_f = ix as f64;
                let di = (ix_f - xp) * abs_dx;

                let dr2 = di * di + dj * dj;
                let (lon, lat) = if dr2 < de2 * 1e-6 {
                    (0.0, h * 90.0)
                } else {
                    (
                        (orient + h * di.atan2(-dj)).to_degrees(),
                        h * ((de2 - dr2) / (de2 + dr2)).asin().to_degrees(),
                    )
                };

                let mut point = VectorPoint::new_xy(lon, lat, None);
                normalize_ll(&mut point);
                res.push(point);
            }
        }
        res
    }
}

/// # GRIB2 - GRID DEFINITION TEMPLATE 3.30
///
/// ## Lambert Conformal
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-30.shtml)
///
/// ## Notes
/// - Grid lengths are in units on 10-3 m, at the latitude specified by LaD.
/// - The projection is defined by the latitude at which Dx and Dy are specified, LaD.
/// - Grid lengths Dx and Dy are in meters at the latitude LaD.
/// - Bit 3 of the resolution and component flags should be set to 1 to indicate that Dx and Dy
///   are given in meters.
#[derive(Debug, Clone, PartialEq)]
pub struct LambertConformalTemplate {
    /// Shape of Earth [Table 3.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-2.shtml)
    pub shape: Grib2Table3_2,
    /// Scale Factor of radius of spherical Earth
    pub radius_scale_factor: u8,
    /// Scale value of radius of spherical Earth
    pub radius_scale_value: u32,
    /// Scale factor of major axis of oblate spheroid Earth
    pub major_axis_scale_factor: u8,
    /// Scale value of major axis of oblate spheroid Earth
    pub major_axis_scale_value: u32,
    /// Scale factor of minor axis of oblate spheroid Earth
    pub minor_axis_scale_factor: u8,
    /// Scale value of minor axis of oblate spheroid Earth
    pub minor_axis_scale_value: u32,
    /// Number of points along the x-axis
    pub nx: u32,
    /// Number of points along the y-axis
    pub ny: u32,
    /// Latitude of first grid point
    pub lat1: f64,
    /// Longitude of first grid point
    pub lon1: f64,
    /// Latitude where Dx and Dy are specified
    pub lat_d: f64,
    /// Orientation of the grid (LoV)
    pub lon_v: f64,
    /// Resolution and component flags [Table 3.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-3.shtml)
    pub resolution: Grib2Table3_3,
    /// x-direction grid length (meters at LaD)
    pub dx: f64,
    /// y-direction grid length (meters at LaD)
    pub dy: f64,
    /// Projection center flag [Table 3.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-5.shtml)
    pub proj_center: Grib2Table3_5,
    /// Scanning mode [Table 3.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml)
    pub scan_mode: Grib2Table3_4,
    /// Latin 1 ― first latitude from the pole at which the secant cone cuts the sphere
    pub latin1: f64,
    /// Latin 2 ― second latitude from the pole at which the secant cone cuts the sphere
    pub latin2: f64,
    /// Latitude of the southern pole of projection
    pub lat_south: f64,
    /// Longitude of the southern pole of projection
    pub lon_south: f64,
    /// Grid Units
    pub grid_units: Grib2GridUnits,
}
impl LambertConformalTemplate {
    /// Create a new instance of LambertConformalTemplate
    ///
    /// ## Parameters
    /// - `section`: byte block for template 3.20
    ///
    /// ## Returns
    /// The parsed template
    pub fn new<T: Reader>(section: &T) -> Self {
        let shape = section.uint8(Some(14));
        let proj_center = section.uint8(Some(63));
        // build resolution values
        let resolution_code = section.uint8(Some(46));
        // build scan mode
        let scan_mode_code = section.uint8(Some(64));

        Self {
            shape: shape.into(),
            radius_scale_factor: section.uint8(Some(15)),
            radius_scale_value: section.uint32_be(Some(16)),
            major_axis_scale_factor: section.uint8(Some(20)),
            major_axis_scale_value: section.uint32_be(Some(21)),
            minor_axis_scale_factor: section.uint8(Some(25)),
            minor_axis_scale_value: section.uint32_be(Some(26)),
            nx: section.uint32_be(Some(30)),
            ny: section.uint32_be(Some(34)),
            lat1: read_grib_int(section.int32_be(Some(38))) * 1e-6,
            lon1: read_grib_int(section.int32_be(Some(42))) * 1e-6,
            lat_d: read_grib_int(section.int32_be(Some(47))) * 1e-6,
            lon_v: read_grib_int(section.int32_be(Some(51))) * 1e-6,
            resolution: resolution_code.into(),
            dx: read_grib_int(section.int32_be(Some(55))) * 1e-3,
            dy: read_grib_int(section.int32_be(Some(59))) * 1e-3,
            proj_center: proj_center.into(),
            scan_mode: scan_mode_code.into(),
            latin1: read_grib_int(section.int32_be(Some(65))) * 1e-6,
            latin2: read_grib_int(section.int32_be(Some(69))) * 1e-6,
            lat_south: read_grib_int(section.int32_be(Some(73))) * 1e-6,
            lon_south: read_grib_int(section.int32_be(Some(77))) * 1e-6,
            grid_units: Grib2GridUnits::Meters,
        }
    }
    /// Convert this section into grid data
    pub fn build_grid(&mut self) -> VectorMultiPoint {
        // https://github.com/NOAA-EMC/wgrib2/blob/d1cef8f4551caf28a5fa339234384eb4894cb6f2/src/geo.c#L345
        let Self { lat1, dx, lon1, dy, nx, ny, lon_v, lat_d, latin1, latin2, .. } = *self;
        let mut res = Vec::with_capacity((nx * ny) as usize);

        // Step 1: Prep all variables for polar projection. Convert degrees to radians
        let lat1r = lat1.to_radians();
        let lon1r = lon1.to_radians();
        let lon2d = lon_v;
        let lon2r = lon_v.to_radians();
        let latin1r = latin1.to_radians();
        let latin2r = latin2.to_radians();
        let latdr = lat_d.to_radians();

        if lon1r < 0. {
            panic!("bad grid definition, lon1r < 0.0");
        }

        let n = if fabs(latin1r - latin2r) < 1E-09 {
            sin(latin1r)
        } else {
            log(cos(latin1r) / cos(latin2r))
                / log(tan(FRAC_PI_4 + latin2r / 2.0) / tan(FRAC_PI_4 + latin1r / 2.0))
        };

        let earth_radius = self.shape.earth_radius(
            self.radius_scale_factor as f64,
            self.radius_scale_value as f64,
            self.major_axis_scale_factor as f64,
            self.major_axis_scale_value as f64,
            self.minor_axis_scale_factor as f64,
            self.minor_axis_scale_value as f64,
        );
        let f = (cos(latin1r) * pow(tan(FRAC_PI_4 + latin1r / 2.0), n)) / n;

        let rho = earth_radius * f * pow(tan(FRAC_PI_4 + lat1r / 2.0), -n);
        // old rhoref = earth_radius * f * pow(tan(FRAC_PI_4 + latin1r/2.0),-n);
        let rhoref = earth_radius * f * pow(tan(FRAC_PI_4 + latdr / 2.0), -n);

        // 2/2009 .. new code
        let mut d_lon = lon1r - lon2r;
        if d_lon > PI {
            d_lon -= 2. * PI;
        }
        if d_lon < -PI {
            d_lon += 2. * PI;
        }
        let theta = n * d_lon;

        let startx = rho * sin(theta);
        let starty = rhoref - rho * cos(theta);

        let i_step =
            if self.scan_mode.bit1 == Grib2Table3_4Bit1::PointsFirstRowColumnScanMinusIDirection {
                -dx
            } else {
                dx
            };
        let j_step =
            if self.scan_mode.bit2 == Grib2Table3_4Bit2::PointsFirstRowColumnScanMinusJDirection {
                -dy
            } else {
                dy
            };

        for j in 0..ny {
            let j = j as f64;
            let y = starty + (j * j_step);
            for i in 0..nx {
                let i = i as f64;
                // Interpolate longitude and latitude
                let x = startx + (i * i_step);
                let tmp = rhoref - y;
                let theta = atan(x / tmp);
                let rho = sqrt(x * x + tmp * tmp);
                let rho = if n > 0. { rho } else { -rho };
                let mut lond = lon2d + (theta / n).to_degrees();
                let latd =
                    (2.0 * atan(pow(earth_radius * f / rho, 1.0 / n)) - FRAC_PI_2).to_degrees();
                lond = if lond >= 360.0 { lond - 360.0 } else { lond };
                lond = if lond < 0.0 { lond + 360.0 } else { lond };

                let mut point = VectorPoint::new_xy(lond, latd, None);
                normalize_ll(&mut point);
                res.push(point);
            }
        }

        res
    }
}

/// # GRIB2 - GRID DEFINITION TEMPLATE 3.40
///
/// ## Gaussian Latitude/Longitude
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-40.shtml)
///
/// ## Notes
/// - Basic angle of the initial production domain and subdivisions of this basic angle are provided
///   to manage cases where the recommended unit of 10-6 degrees is not applicable to describe the
///   extreme longitudes and latitudes, and direction increments. For these last six descriptors, unit
///   is equal to the ratio of the equivalent to respective values of 1 and 106 (10-6 degrees unit).
/// - The number of parallels between a pole and the equator is used to establish the variable
///   (Gaussian) spacing of the parallels; this value must always be given.
/// - A scaled value of radius of spherical Earth, or major or minor axis of oblate spheriod Earth
///   is derived from applying appropriate scale factor to the value expressed in metres.
/// - A quasi-regular grid is only defined for appropriate grid scanning modes. Either rows or
///   columns, but not both simultaneously, may have variable numbers of points. The first point in
///   each row(column) shall be positioned at the meridian (parallel) indicated by Octets 47-54. The
///   grid points shall be evenly spaced in latitude (longitude).
/// - It is recommended to use unsigned direction increments.
#[derive(Debug, Clone, PartialEq)]
pub struct GaussianTemplate {
    /// Shape of Earth [Table 3.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-2.shtml)
    pub shape: Grib2Table3_2,
    /// Scale Factor of radius of spherical Earth
    pub radius_scale_factor: u8,
    /// Scale value of radius of spherical Earth
    pub radius_scale_value: u32,
    /// Scale factor of major axis of oblate spheroid Earth
    pub major_axis_scale_factor: u8,
    /// Scale value of major axis of oblate spheroid Earth
    pub major_axis_scale_value: u32,
    /// Scale factor of minor axis of oblate spheroid Earth
    pub minor_axis_scale_factor: u8,
    /// Scale value of minor axis of oblate spheroid Earth
    pub minor_axis_scale_value: u32,
    /// Number of points along the x-axis
    pub nx: u32,
    /// Number of points along the y-axis
    pub ny: u32,
    /// Basic angle of the initial production domain
    pub basic_angle: f64,
    /// Subdivisions of basic angle used to define extreme longitudes and latitudes, and direction increments
    pub subdivisions: f64,
    /// Latitude of first grid point
    pub lat1: f64,
    /// Longitude of first grid point
    pub lon1: f64,
    /// Resolution and component flags [Table 3.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-3.shtml)
    pub resolution: Grib2Table3_3,
    /// x-direction grid length (meters at LaD)
    pub lat2: f64,
    /// y-direction grid length (meters at LaD)
    pub lon2: f64,
    /// i direction increment
    pub dx: f64,
    /// N - number of paralells between a pole and the equator (see Note 2)
    pub n: u32,
    /// Scanning mode [Table 3.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml)
    pub scan_mode: Grib2Table3_4,
    /// Grid Units
    pub grid_units: Grib2GridUnits,
}
impl GaussianTemplate {
    /// Create a new instance of GaussianTemplate
    ///
    /// ## Parameters
    /// - `section`: byte block for template 3.40
    ///
    /// ## Returns
    /// The parsed template
    pub fn new<T: Reader>(section: &T) -> Self {
        let shape = section.uint8(Some(14));
        let basic_angle = section.uint32_be(Some(38)) as f64;
        let subdivisions = section.uint32_be(Some(42)) as f64;
        // build resolution values
        let resolution_code = section.uint8(Some(54));
        // build scan_mode values
        let scan_mode_code = section.uint8(Some(71));

        let ratio = if basic_angle == 0. { 1.0e-6 } else { basic_angle / subdivisions };

        Self {
            shape: shape.into(),
            radius_scale_factor: section.uint8(Some(15)),
            radius_scale_value: section.uint32_be(Some(16)),
            major_axis_scale_factor: section.uint8(Some(20)),
            major_axis_scale_value: section.uint32_be(Some(21)),
            minor_axis_scale_factor: section.uint8(Some(25)),
            minor_axis_scale_value: section.uint32_be(Some(26)),
            nx: section.uint32_be(Some(30)),
            ny: section.uint32_be(Some(34)),
            basic_angle,
            subdivisions,
            lat1: read_grib_int(section.int32_be(Some(46))) * ratio,
            lon1: read_grib_int(section.int32_be(Some(50))) * ratio,
            resolution: resolution_code.into(),
            lat2: read_grib_int(section.int32_be(Some(55))) * ratio,
            lon2: read_grib_int(section.int32_be(Some(59))) * ratio,
            dx: read_grib_int(section.int32_be(Some(63))) * ratio,
            n: section.uint32_be(Some(67)),
            scan_mode: scan_mode_code.into(),
            grid_units: Grib2GridUnits::Meters,
        }
    }

    /// Convert this section into grid data
    pub fn build_grid(&mut self) -> VectorMultiPoint {
        // https://github.com/NOAA-EMC/wgrib2/blob/d1cef8f4551caf28a5fa339234384eb4894cb6f2/src/Proj4.c#L154
        let Self { lon1, nx, ny, scan_mode, dx, .. } = *self;
        let nx = nx as usize;
        let ny = ny as usize;
        let mut res = Vec::with_capacity(nx * ny);

        // 1. Generate latitudes for the full globe
        let mut all_lats = generate_gaussian_lats(ny);

        // 2. Find where our specific grid starts in the global Gaussian sequence
        //   const is_scan_j_positive = scan_mode.value.yDir.code === 1;
        let is_scan_j_positive =
            scan_mode.bit2 == Grib2Table3_4Bit2::PointsFirstRowColumnScanPlusJDirection;
        if is_scan_j_positive {
            all_lats.reverse();
        }

        // 3. Step sizes for Longitude (Regularly spaced)
        // Note: GRIB2 Gaussian grids are often global (lon2 is lon1 + span)
        let d_lon = if scan_mode.bit1 == Grib2Table3_4Bit1::PointsFirstRowColumnScanPlusIDirection {
            fabs(dx)
        } else {
            -fabs(dx)
        };

        for lat in all_lats.iter().take(ny) {
            for i in 0..nx {
                let lon = lon1 + (i as f64) * d_lon;

                let mut point = VectorPoint::new_xy(lon, *lat, None);
                normalize_ll(&mut point);
                res.push(point);
            }
        }

        res
    }
}

/**
 * Converts a GRIB2 integer value to a number. Data values are stored as signed 32-bit integers.
 * So if the first bit is 1, the value is negative and we need to invert it.
 * @param val - value to evaluate
 * @returns the fixed value
 */
fn read_grib_int(val: i32) -> f64 {
    let fix = if val < 0 { -val ^ 0x80000000u32 as i32 } else { val };
    fix as f64
}

/// Calculates Gaussian latitudes in degrees using the Newton-Raphson method
///
/// ## Parameters
/// - `n`: number of latitudes
///
/// ## Returns
/// array of latitudes
fn generate_gaussian_lats(n: usize) -> Vec<f64> {
    let n_64 = n as f64;
    let coeff = 1. - 1. / (8. * n_64 * n_64) + 1. / (8. * n_64 * n_64 * n_64);
    let mut lats = Vec::with_capacity(n);

    for i in 0..n {
        let i_64 = i as f64;
        // 1. Initial Guess (Tricomi/Lether)
        let mut x = coeff * cos(((4. * i_64 + 3.) * PI) / (4. * n_64 + 2.));

        // 2. Newton-Raphson Root Finding
        for _ in 0..10 {
            // legendre_polynomial returns (P_{n-1}, P_n)
            let (p_prev, p) = legendre_polynomial(n, x);
            // derivative: n * (P_{n-1} - x * P_n) / (1 - x^2)
            let fpx = (n_64 * (p_prev - x * p)) / (1.0 - x * x);

            let dx = p / fpx;
            x -= dx;

            if fabs(dx) < f64::EPSILON {
                break;
            }
        }

        // 3. Convert root to Latitude: asin(x) converted to degrees
        lats.push(asin(x).to_degrees());
    }

    lats
}

/// Calculates Legendre polynomials
///
/// ## Parameters
/// - `n`: number of latitudes
/// - `x`: value to evaluate
///
/// ## Returns
/// tuple of (P_{n-1}, P_n)
fn legendre_polynomial(n: usize, x: f64) -> (f64, f64) {
    let mut p0 = 1.;
    let mut p1 = x;
    for k in 2..=n {
        let k_64 = k as f64;
        let pk = ((2. * k_64 - 1.) * x * p1 - (k_64 - 1.) * p0) / k_64;
        p0 = p1;
        p1 = pk;
    }

    (p0, p1)
}
