// TEMPLATE INFO: https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-1.shtml
use crate::{
    parsers::Reader,
    readers::{Grib2Table3_1, Grib2Table3_2, Grib2Table3_3, Grib2Table3_4, Grib2Table3_5},
};
use alloc::vec;
use s2json::{VectorMultiPoint, VectorPoint};

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
///
/// @param template - template number parse block
/// @param section - byte block
/// @returns Template generator
#[derive(Debug, Clone, PartialEq)]
pub enum Grib2Template3 {
    /// Latitude/Longitude (or equidistant cylindrical, or Plate Carree)
    EquatorialTemplate(EquatorialTemplate),
    /// Polar Stereographic Projection (Can be North or South)
    PolarTemplate(PolarTemplate),
}
impl Grib2Template3 {
    /// Create a new instance of Grib2Template3
    pub fn new<T: Reader>(template: Grib2Table3_1, section: &T) -> Self {
        // TODO: Addd all Grib2Table3_1 options and set correct transform
        match template {
            Grib2Table3_1::LatitudeLongitude => {
                Grib2Template3::EquatorialTemplate(EquatorialTemplate::new(section))
            }
            Grib2Table3_1::PolarStereographicProjection => {
                Grib2Template3::PolarTemplate(PolarTemplate::new(section))
            }
            _ => panic!("Template 3.{template} not defined"),
        }
    }
    /// Convert this section into grid data
    pub fn build_grid(&mut self) -> VectorMultiPoint {
        match self {
            Grib2Template3::EquatorialTemplate(template) => template.build_grid(),
            Grib2Template3::PolarTemplate(template) => template.build_grid(),
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
/// to manage cases where the recommended unit of 10-6 degrees is not applicable to describe the
/// extreme longitudes and latitudes, and direction increments. For these last six descriptors, the
/// unit is equal to the ratio of the basic angle and the subdivisions number. For ordinary cases,
/// zero and missing values should be coded, equivalent to respective values of 1 and 106  (10-6
/// degrees unit).
///
/// - For data on a quasi-regular grid, in which all the rows or columns do not necessarily have the
/// same number of grid points either Ni (octets 31-34) of Nj (octets 35-38) and the corresponding Di
/// (octets 64-67) or Dj (octets 68-71) shall be coded with all bits set to 1 (missing). The actual
/// number of points along each parallel or meridian shall be coded in the octets immediately following
/// the grid definition template (octets [xx+1]-nn), as described in the description of the grid
/// definition section.
///
/// - A quasi-regular grid is only defined for appropriate grid scanning modes. Either rows or columns,
/// but not both simultaneously, may have variable numbers of points or variable spacing. The first
/// point in each row (column) shall be positioned at the meridian (parallel) indicted by octets 47-54.
/// The grid points shall be evenly spaced in latitude (longitude).
///
/// A scale value of radius of spherical Earth, or major axis of oblate spheroid Earth is delivered
/// from applying appropriate scale factor to the value expressed in meters.
///
/// - It is recommended to use unsigned direction increments.
///
/// - In most cases, multiplying Ni (octets 31-34) by Nj (octets 35-38) yields the total number of
/// points in the grid. However, this may not be true if bit 8 of the scanning mode flags (octet 72)
/// is set to 1.
///
/// @param section - byte block for template 3.0
/// @returns - The parsed template
#[derive(Debug, Clone, PartialEq)]
pub struct EquatorialTemplate {
    /// Shape of Earth [Table 3.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-2.shtml)
    shape: Grib2Table3_2,
    /// Scale Factor of radius of spherical Earth
    radius_scale_factor: u8,
    /// Scale value of radius of spherical Earth
    radius_scale_value: u32,
    /// Scale factor of major axis of oblate spheroid Earth
    major_axis_scale_factor: u8,
    /// Scale value of major axis of oblate spheroid Earth
    major_axis_scale_value: u32,
    /// Scale factor of minor axis of oblate spheroid Earth
    minor_axis_scale_factor: u8,
    /// Scale value of minor axis of oblate spheroid Earth
    minor_axis_scale_value: u32,
    /// Number of points along a parallel (W-E)
    nx: u32,
    /// Number of points along a meridian (N-S)
    ny: u32,
    /// Basic angle of the initial production domain
    basic_angle: f64,
    /// Subdivisions of basic angle used to define extreme longitudes and latitudes, and direction increments
    subdivisions: f64,
    /// Latitude of first grid point
    lat1: f64,
    /// Longitude of first grid point
    lon1: f64,
    /// Resolution and component flags [Table 3.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-3.shtml)
    resolution: Grib2Table3_3,
    /// Latitude of last grid point
    lat2: f64,
    /// Longitude of last grid point
    lon2: f64,
    /// i direction increment
    dx: f64,
    /// j direction increment
    dy: f64,
    /// Scanning mode [Table 3.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml)
    scan_mode: Grib2Table3_4,
    /// Grid Units
    grid_units: Grib2GridUnits,
}
impl EquatorialTemplate {
    /// Create a new instance of EquatorialTemplate
    pub fn new<T: Reader>(section: &T) -> Self {
        let shape = section.uint8(Some(14));
        let basic_angle = section.uint32_be(Some(38)) as f64;
        let subdivisions = section.uint32_be(Some(42)) as f64;
        let lat1 = section.int32_be(Some(46));
        let lat2 = section.int32_be(Some(55));
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
            lat1: (if lat1 < 0 { -lat1 ^ 0x80000000u32 as i32 } else { lat1 }) as f64 * ratio,
            lon1: section.int32_be(Some(50)) as f64 * ratio,
            resolution: resolution_code.into(),
            lat2: (if lat2 < 0 { -lat2 ^ 0x80000000u32 as i32 } else { lat2 }) as f64 * ratio,
            lon2: section.int32_be(Some(59)) as f64 * ratio,
            dx: section.int32_be(Some(63)) as f64 * ratio,
            dy: section.int32_be(Some(67)) as f64 * ratio,
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

        let mut res = vec![];

        for y in 0..ny {
            let y = y as f64;
            for x in 0..nx {
                let x = x as f64;
                // Interpolate longitude and latitude
                let lon = lon1 + x * lon_step;
                let lat = lat1 + y * lat_step;
                // create point and apply transform if provided (this grid is already in the correct projection)
                res.push(VectorPoint::new_xy(lon, lat, None));
            }
        }

        res
    }
}

/**
 * # GRIB2 - GRID DEFINITION TEMPLATE 3.20
 *
 * ## Polar Stereographic Projection (Can be North or South)
 *
 * ## Links
 * - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp3-20.shtml)
 *
 * ## Notes
 * - The orientation of the grid is given by the longitude of the meridian along which the
 *   y-axis increases, LoV.
 * - The projection is defined by the latitude at which Dx and Dy are specified, LaD.
 * - Grid lengths Dx and Dy are in meters at the latitude LaD.
 * - Bit 3 of the resolution and component flags should be set to 1 to indicate that Dx and Dy
 *   are given in meters.
 * @param section - byte block for template 3.20
 * @returns - The parsed template
 */
#[derive(Debug, Clone, PartialEq)]
pub struct PolarTemplate {
    /// Shape of Earth [Table 3.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-2.shtml)
    shape: Grib2Table3_2,
    /// Scale Factor of radius of spherical Earth
    radius_scale_factor: u8,
    /// Scale value of radius of spherical Earth
    radius_scale_value: u32,
    /// Scale factor of major axis of oblate spheroid Earth
    major_axis_scale_factor: u8,
    /// Scale value of major axis of oblate spheroid Earth
    major_axis_scale_value: u32,
    /// Scale factor of minor axis of oblate spheroid Earth
    minor_axis_scale_factor: u8,
    /// Scale value of minor axis of oblate spheroid Earth
    minor_axis_scale_value: u32,
    /// Number of points along the x-axis
    nx: u32,
    /// Number of points along the y-axis
    ny: u32,
    /// Latitude of first grid point
    lat1: f64,
    /// Longitude of first grid point
    lon1: f64,
    /// Latitude where Dx and Dy are specified
    lat_d: f64,
    /// Orientation of the grid (LoV)
    lon_v: f64,
    /// Resolution and component flags [Table 3.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-3.shtml)
    resolution: Grib2Table3_3,
    /// x-direction grid length (meters at LaD)
    dx: f64,
    /// y-direction grid length (meters at LaD)
    dy: f64,
    /// Projection center flag [Table 3.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-5.shtml)
    proj_center: Grib2Table3_5,
    /// Scanning mode [Table 3.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table3-4.shtml)
    scan_mode: Grib2Table3_4,
    /// Grid Units
    grid_units: Grib2GridUnits,
}
impl PolarTemplate {
    /// Create a new instance of PolarTemplate
    pub fn new<T: Reader>(section: &T) -> Self {
        let shape = section.uint8(Some(14));
        let lat1 = section.int32_be(Some(38)) as f64;
        let lon1 = section.int32_be(Some(42)) as f64;
        let lat_d = section.int32_be(Some(47)) as f64;
        let lon_v = section.int32_be(Some(51)) as f64;
        let dx = section.int32_be(Some(55)) as f64;
        let dy = section.int32_be(Some(59)) as f64;
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
            lat1,
            lon1,
            lat_d,
            lon_v,
            resolution: resolution_code.into(),
            dx,
            dy,
            proj_center: proj_center.into(),
            scan_mode: scan_mode_code.into(),
            grid_units: Grib2GridUnits::Meters,
        }
    }
    /// Convert this section into grid data
    pub fn build_grid(&mut self) -> VectorMultiPoint {
        // for now let's just follow the most basic scan mode
        let Self { lat1, dx, lon1, dy, nx, ny, .. } = *self;
        let mut res = vec![];

        for y in 0..ny {
            let y = y as f64;
            for x in 0..nx {
                let x = x as f64;
                // Interpolate longitude and latitude
                let lon = lon1 + x * dx;
                let lat = lat1 + y * dy;
                // create point
                let point = VectorPoint::new_xy(lon, lat, None);
                // apply transform if provided
                //   if (transformer !== undefined) point = transformer.forward(point);
                res.push(point);
            }
        }

        res
    }
}
