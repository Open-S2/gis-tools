// References:
// - Wikipedia: https://en.wikipedia.org/wiki/Military_Grid_Reference_System
// - GEOTRANS: https://earth-info.nga.mil/#geotrans

use crate::TransformCoordinates;
use alloc::{format, string::String, vec::Vec};
use libm::{cos, floor, pow, sin, sqrt, tan, trunc};
use s2json::BBox;

/// UTM zones are grouped, and assigned to one of a group of 6 sets.
const NUM_100K_SETS: u8 = 6;
/// The column letters (for easting) of the lower left value, per set.
const SET_ORIGIN_COLUMN_LETTERS: &str = "AJSAJS";
/// The row letters (for northing) of the lower left value, per set.
const SET_ORIGIN_ROW_LETTERS: &str = "AFAFAF";
/// Column letter list
const ALPHABET: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

/// First eccentricity squared
const ECC_SQUARED: f64 = 0.00669438;
/// Scale factor along the central meridian
const SCALE_FACTOR: f64 = 0.9996;
/// Semimajor axis (half the width of the earth) in meters
const SEMI_MAJOR_AXIS: f64 = 6378137.;
/// The easting of the central meridian of each UTM zone
const EASTING_OFFSET: f64 = 500000.;
/// The northing of the equator for southern hemisphere locations (in UTM)
const NORTHING_OFFFSET: f64 = 10000000.;
/// UTM zone width in degrees
const UTM_ZONE_WIDTH: f64 = 6.;
/// Half the width of a UTM zone in degrees
const HALF_UTM_ZONE_WIDTH: f64 = UTM_ZONE_WIDTH / 2.;

const A_U32: u32 = 65;
const I_U32: u32 = 73;
const O_U32: u32 = 79;
const V_U32: u32 = 86;
const Z_U32: u32 = 90;

/// Convert lat/lon to MGRS.
///
/// - `ll`: Array with longitude and latitude on a WGS84 ellipsoid.
/// - `accuracy`: Accuracy in digits (5 for 1 m, 4 for 10 m, 3 for
///   100 m, 2 for 1 km, 1 for 10 km or 0 for 100 km). Optional, default is 5.
///
/// returns the MGRS string for the given location and accuracy.
pub fn mgrs_forward<P: TransformCoordinates>(ll: &P, accuracy: Option<u8>) -> String {
    let accuracy = accuracy.unwrap_or(5); // default accuracy 1m

    let x = ll.get_x();
    let y = ll.get_y();
    if !(-180. ..=180.).contains(&x) {
        panic!("forward received an invalid longitude of {x:}");
    }
    if !(-90. ..=90.).contains(&y) {
        panic!("forward received an invalid latitude of {y:}");
    }
    if !(-80. ..=84.).contains(&y) {
        panic!(
            "forward received a latitude of {y:}, but this library does not support conversions \
             of points in polar regions below 80°S and above 84°N",
        );
    }

    encode(ll_to_utm(ll), accuracy as usize)
}

/// Convert MGRS to lat/lon bounding box given an MGRS string
///
/// returns an array with left (longitude), bottom (latitude), right (longitude) and top (latitude)
/// values in WGS84, representing the bounding box for the provided MGRS reference.
pub fn mgrs_inverse(mgrs: &str) -> Option<BBox> {
    utm_to_ll(decode(mgrs.to_uppercase()))
}

/// Convert MGRS to lat/lon given an MGRS string
///
/// @returns the center of the MGRS bounding box
pub fn mgrs_to_point<P: TransformCoordinates>(mgrs: &str) -> Option<P> {
    let bbox = utm_to_ll(decode(mgrs.to_uppercase()));
    if let Some(bbox) = bbox {
        let mut point = P::default();
        point.set_x((bbox.left + bbox.right) / 2.);
        point.set_y((bbox.top + bbox.bottom) / 2.);
        Some(point)
    } else {
        None
    }
}

/// Converts a set of Longitude and Latitude co-ordinates to UTM
/// using the WGS84 ellipsoid.
/// @param ll Object literal with lat and lon properties
///     representing the WGS84 coordinate to be converted.
/// @returns Object literal containing the UTM value with easting,
///     northing, zone_number and zone_letter properties, and an optional
///     accuracy property in digits. Returns null if the conversion failed.
fn ll_to_utm<P: TransformCoordinates>(ll: &P) -> Utm {
    let lon = ll.get_x();
    let lat = ll.get_y();
    let a = SEMI_MAJOR_AXIS;
    let lat_rad = lat.to_radians();
    let long_rad = lon.to_radians();
    let mut zone_number = floor((lon + 180.) / 6.) + 1.;

    // Make sure the longitude 180 is in Zone 60
    if lon == 180. {
        zone_number = 60.;
    }

    // Special zone for Norway
    if (56. ..64.).contains(&lat) && (3. ..12.).contains(&lon) {
        zone_number = 32.;
    }

    // Special zones for Svalbard
    if (72. ..84.).contains(&lat) {
        if (0. ..9.).contains(&lon) {
            zone_number = 31.;
        } else if (9. ..21.).contains(&lon) {
            zone_number = 33.;
        } else if (21. ..33.).contains(&lon) {
            zone_number = 35.;
        } else if (33. ..42.).contains(&lon) {
            zone_number = 37.;
        }
    }

    // +HALF_UTM_ZONE_WIDTH puts origin in middle of zone
    let long_origin = (zone_number - 1.) * UTM_ZONE_WIDTH - 180. + HALF_UTM_ZONE_WIDTH;
    let long_origin_rad = long_origin.to_radians();

    let ecc_prime_squared = ECC_SQUARED / (1. - ECC_SQUARED);

    let n = a / sqrt(1. - ECC_SQUARED * sin(lat_rad) * sin(lat_rad));
    let t = tan(lat_rad) * tan(lat_rad);
    let c = ecc_prime_squared * cos(lat_rad) * cos(lat_rad);
    let a_2 = cos(lat_rad) * (long_rad - long_origin_rad);

    let m = a
        * ((1.
            - ECC_SQUARED / 4.
            - (3. * ECC_SQUARED * ECC_SQUARED) / 64.
            - (5. * ECC_SQUARED * ECC_SQUARED * ECC_SQUARED) / 256.)
            * lat_rad
            - ((3. * ECC_SQUARED) / 8.
                + (3. * ECC_SQUARED * ECC_SQUARED) / 32.
                + (45. * ECC_SQUARED * ECC_SQUARED * ECC_SQUARED) / 1024.)
                * sin(2. * lat_rad)
            + ((15. * ECC_SQUARED * ECC_SQUARED) / 256.
                + (45. * ECC_SQUARED * ECC_SQUARED * ECC_SQUARED) / 1024.)
                * sin(4. * lat_rad)
            - ((35. * ECC_SQUARED * ECC_SQUARED * ECC_SQUARED) / 3072.) * sin(6. * lat_rad));

    let utm_easting = SCALE_FACTOR
        * n
        * (a_2
            + ((1. - t + c) * a_2 * a_2 * a_2) / 6.
            + ((5. - 18. * t + t * t + 72. * c - 58. * ecc_prime_squared)
                * a_2
                * a_2
                * a_2
                * a_2
                * a_2)
                / 120.)
        + EASTING_OFFSET;

    let mut utm_northing = SCALE_FACTOR
        * (m + n
            * tan(lat_rad)
            * ((a_2 * a_2) / 2.
                + ((5. - t + 9. * c + 4. * c * c) * a_2 * a_2 * a_2 * a_2) / 24.
                + ((61. - 58. * t + t * t + 600. * c - 330. * ecc_prime_squared)
                    * a_2
                    * a_2
                    * a_2
                    * a_2
                    * a_2
                    * a_2)
                    / 720.));
    if lat < 0. {
        utm_northing += NORTHING_OFFFSET;
    }

    Utm {
        northing: trunc(utm_northing),
        easting: trunc(utm_easting),
        zone_number: zone_number as u8,
        zone_letter: mgrs_get_letter_designator(lat),
        accuracy: None,
    }
}

/// UTM parameters
#[derive(Debug, Copy, Clone)]
struct Utm {
    northing: f64,
    easting: f64,
    zone_number: u8,
    zone_letter: char,
    accuracy: Option<f64>,
}

/// Converts UTM coords to lat/long, using the WGS84 ellipsoid. This is a convenience
/// class where the Zone can be specified as a single string eg."60N" which
/// is then broken down into the Zone_number and Zone_letter.
/// @param utm An object literal with northing, easting, zone_number
///     and zone_letter properties. If an optional accuracy property is
///     provided (in meters), a bounding box will be returned instead of
///     latitude and longitude.
/// @returns An object literal containing either lat and lon values
///     (if no accuracy was provided), or top, right, bottom and left values
///     for the bounding box calculated according to the provided accuracy.
///     Returns null if the conversion failed.
fn utm_to_ll(utm: Utm) -> Option<BBox> {
    let utm_northing = utm.northing;
    let utm_easting = utm.easting;
    let Utm { zone_letter, zone_number, .. } = utm;
    // check the ZoneNummber is valid
    if !(0..=60).contains(&zone_number) {
        return None;
    }

    let a = SEMI_MAJOR_AXIS;
    let e1 = (1. - sqrt(1. - ECC_SQUARED)) / (1. + sqrt(1. - ECC_SQUARED));

    // remove 500,000 meter offset for longitude
    let x = utm_easting - EASTING_OFFSET;
    let mut y = utm_northing;

    // We must know somehow if we are in the Northern or Southern
    // hemisphere, this is the only time we use the letter So even
    // if the Zone letter isn't exactly correct it should indicate
    // the hemisphere correctly
    if zone_letter < 'N' {
        y -= NORTHING_OFFFSET; // remove offset used for southern hemisphere
    }

    // +HALF_UTM_ZONE_WIDTH puts origin in middle of zone
    let long_origin = (zone_number as f64 - 1.) * UTM_ZONE_WIDTH - 180. + HALF_UTM_ZONE_WIDTH;

    let ecc_prime_squared = ECC_SQUARED / (1. - ECC_SQUARED);

    let m = y / SCALE_FACTOR;
    let mu = m
        / (a * (1.
            - ECC_SQUARED / 4.
            - (3. * ECC_SQUARED * ECC_SQUARED) / 64.
            - (5. * ECC_SQUARED * ECC_SQUARED * ECC_SQUARED) / 256.));

    let phi1_rad = mu
        + ((3. * e1) / 2. - (27. * e1 * e1 * e1) / 32.) * sin(2. * mu)
        + ((21. * e1 * e1) / 16. - (55. * e1 * e1 * e1 * e1) / 32.) * sin(4. * mu)
        + ((151. * e1 * e1 * e1) / 96.) * sin(6. * mu);
    // double phi1 = ProjradToDeg(phi1_rad);

    let n1 = a / sqrt(1. - ECC_SQUARED * sin(phi1_rad) * sin(phi1_rad));
    let t1 = tan(phi1_rad) * tan(phi1_rad);
    let c1 = ecc_prime_squared * cos(phi1_rad) * cos(phi1_rad);
    let r1 = (a * (1. - ECC_SQUARED)) / pow(1. - ECC_SQUARED * sin(phi1_rad) * sin(phi1_rad), 1.5);
    let d = x / (n1 * SCALE_FACTOR);

    let mut lat = phi1_rad
        - ((n1 * tan(phi1_rad)) / r1)
            * ((d * d) / 2.
                - ((5. + 3. * t1 + 10. * c1 - 4. * c1 * c1 - 9. * ecc_prime_squared)
                    * d
                    * d
                    * d
                    * d)
                    / 24.
                + ((61. + 90. * t1 + 298. * c1 + 45. * t1 * t1
                    - 252. * ecc_prime_squared
                    - 3. * c1 * c1)
                    * d
                    * d
                    * d
                    * d
                    * d
                    * d)
                    / 720.);
    lat = lat.to_degrees();

    let mut lon = (d - ((1. + 2. * t1 + c1) * d * d * d) / 6.
        + ((5. - 2. * c1 + 28. * t1 - 3. * c1 * c1 + 8. * ecc_prime_squared + 24. * t1 * t1)
            * d
            * d
            * d
            * d
            * d)
            / 120.)
        / cos(phi1_rad);
    lon = long_origin + lon.to_degrees();

    if let Some(accuracy) = utm.accuracy {
        let top_right = utm_to_ll(Utm {
            northing: utm.northing + accuracy,
            easting: utm.easting + accuracy,
            zone_letter: utm.zone_letter,
            zone_number: utm.zone_number,
            accuracy: None,
        })
        .unwrap_or_default();
        Some(BBox { top: top_right.left, right: top_right.bottom, bottom: lat, left: lon })
    } else {
        Some(BBox { top: lat, right: lon, bottom: lat, left: lon })
    }
}

/// Calculates the MGRS letter designator for the given latitude.
///
/// `latitude`: The latitude in WGS84 to get the letter designator for.
/// returns The letter designator.
fn mgrs_get_letter_designator(latitude: f64) -> char {
    if (72. ..=84.).contains(&latitude) {
        // the X band is 12 degrees high
        'X'
    } else if (-80. ..72.).contains(&latitude) {
        // Latitude bands are lettered C through X, excluding I and O
        let band_letters = "CDEFGHJKLMNPQRSTUVWX";
        let band_height = 8.;
        let min_latitude = -80.;
        let index = floor((latitude - min_latitude) / band_height) as usize;
        band_letters.chars().nth(index).unwrap()
    } else {
        // This is here as an error flag to show that the Latitude is
        // outside MGRS limits
        'Z'
    }
}

/// Encodes a UTM location as MGRS string.
///
/// `utm`: An object literal with easting, northing, zone_letter, zone_number
/// `accuracy`: Accuracy in digits (0-5).
/// returns MGRS string for the given UTM location.
fn encode(utm: Utm, accuracy: usize) -> String {
    // prepend with leading zeroes
    let seasting: String = format!("00000{}", utm.easting);
    let snorthing: String = format!("00000{}", utm.northing);
    let se_len = seasting.len() - 5;
    let sn_len = snorthing.len() - 5;

    format!(
        "{}{}{}{}{}",
        utm.zone_number,
        utm.zone_letter,
        get_100k_id(utm.easting, utm.northing, utm.zone_number),
        &seasting[se_len..se_len + accuracy],
        &snorthing[sn_len..sn_len + accuracy]
    )
}

/// Get the two letter 100k designator for a given UTM easting, northing and zone number value.
///
/// `easting`: UTM easting
/// `northing`: UTM northing
/// `zone_number`: UTM zone number
/// returns the two letter 100k designator for the given UTM location.
fn get_100k_id(easting: f64, northing: f64, zone_number: u8) -> String {
    let set_parm = get_100k_set_for_zone(zone_number);
    let set_column = floor(easting / 100000.);
    let set_row = floor(northing / 100000.) % 20.;
    get_letter_100k_id(set_column as u32, set_row as u32, set_parm as usize)
}

/// Given a UTM zone number, figure out the MGRS 100K set it is in.
///
/// `i`: An UTM zone number.
/// returns the 100k set the UTM zone is in.
fn get_100k_set_for_zone(i: u8) -> u8 {
    let mut set_parm = i % NUM_100K_SETS;
    if set_parm == 0 {
        set_parm = NUM_100K_SETS;
    }

    set_parm
}

/// Get the two-letter MGRS 100k designator given information translated from the UTM northing,
/// easting and zone number.
///
/// - `column`: the column index as it relates to the MGRS
///   100k set spreadsheet, created from the UTM easting.
///   Values are 1-8.
/// - `row`: the row index as it relates to the MGRS 100k set
///   spreadsheet, created from the UTM northing value. Values
///   are from 0-19.
/// - `parm`: the set block, as it relates to the MGRS 100k set
///   spreadsheet, created from the UTM zone. Values are from
///   1-60.
///
/// returns two letter MGRS 100k code.
fn get_letter_100k_id(mut col: u32, mut row: u32, parm: usize) -> String {
    let index = parm - 1;
    let col_origin = SET_ORIGIN_COLUMN_LETTERS.chars().nth(index).unwrap() as u32;
    let row_origin = SET_ORIGIN_ROW_LETTERS.chars().nth(index).unwrap() as u32;

    col += col_origin - 1;
    row += row_origin;

    let mut rollover_col = false;
    while col > Z_U32 {
        col -= Z_U32 - A_U32 + 1;
        rollover_col = true;
    }

    if col == I_U32
        || (col_origin < I_U32 && col > I_U32)
        || ((col > I_U32 || col_origin < I_U32) && rollover_col)
    {
        col += 1;
    }

    if col == O_U32
        || (col_origin < O_U32 && col > O_U32)
        || ((col > O_U32 || col_origin < O_U32) && rollover_col)
    {
        col += 1;
        if col == I_U32 {
            col += 1;
        }
    }

    while col > Z_U32 {
        col -= Z_U32 - A_U32 + 1;
    }

    let mut rollover_row = false;
    while row > V_U32 {
        row -= V_U32 - A_U32 + 1;
        rollover_row = true;
    }

    if row == I_U32
        || (row_origin < I_U32 && row > I_U32)
        || ((row > I_U32 || row_origin < I_U32) && rollover_row)
    {
        row += 1;
    }

    if row == O_U32
        || (row_origin < O_U32 && row > O_U32)
        || ((row > O_U32 || row_origin < O_U32) && rollover_row)
    {
        row += 1;
        if row == I_U32 {
            row += 1;
        }
    }

    while row > V_U32 {
        row -= V_U32 - A_U32 + 1;
    }

    let col_char = char::from_u32(col).unwrap();
    let row_char = char::from_u32(row).unwrap();

    format!("{}{}", col_char, row_char)
}

/// Decode the UTM parameters from a MGRS string.
/// @param mgrs_string - an UPPERCASE coordinate string is expected.
/// @returns An object literal with easting, northing, zone_letter,
///     zone_number and accuracy (in meters) properties.
fn decode(mut mgrs_string: String) -> Utm {
    if mgrs_string.is_empty() {
        panic!("MGRSPoint coverting from nothing");
    }

    // remove any spaces in MGRS String
    mgrs_string = mgrs_string.trim().into();
    let length = mgrs_string.len();
    let mut sb = String::new();
    let mut i = 0;

    // get Zone number
    while i < mgrs_string.len() {
        let test_char = mgrs_string.chars().nth(i).unwrap();
        if test_char.is_ascii_uppercase() {
            break; // Found the latitude band letter
        }
        if i >= 2 {
            panic!("MGRSPoint bad conversion from: {}", mgrs_string);
        }
        sb.push(test_char);
        i += 1;
    }

    let zone_number = sb.parse::<u8>().unwrap();

    if i == 0 || i + 3 > length {
        // A good MGRS string has to be 4-5 digits long,
        // ##AAA/#AAA at least.
        panic!("MGRSPoint bad conversion from {}", mgrs_string);
    }

    let zone_letter = mgrs_string.chars().nth(i).unwrap();
    i += 1;

    // Should we check the zone letter here? Why not.
    if zone_letter <= 'A'
        || zone_letter == 'B'
        || zone_letter == 'Y'
        || zone_letter >= 'Z'
        || zone_letter == 'I'
        || zone_letter == 'O'
    {
        panic!("MGRSPoint zone letter ${zone_letter} not handled: {}", mgrs_string);
    }

    let hunk = &mgrs_string[i..i + 2];
    let hunk_chars = hunk.chars().collect::<Vec<char>>();
    i += 2;

    let set = get_100k_set_for_zone(zone_number) as usize;

    let east100k = get_easting_from_char(hunk_chars[0], set);
    let mut north100k = get_northing_from_char(hunk_chars[1], set);

    // We have a bug where the northing may be 2000000 too low.
    // How
    // do we know when to roll over?

    while north100k < get_min_northing(zone_letter) {
        north100k += 2000000.;
    }

    // calculate the char index for easting/northing separator
    let remainder = length - i;

    if remainder % 2 != 0 {
        panic!(
            "MGRSPoint has to have an even number of digits after the zone letter and two 100km \
             letters - front half for easting meters, second half for northing meters {}",
            mgrs_string
        );
    }

    let sep = remainder / 2;
    let sep_f64 = sep as f64;
    let mut sep_easting = 0.;
    let mut sep_northing = 0.;
    let mut accuracy = 0.;
    let sep_easting_string: &str;
    let sep_northing_string: &str;
    if sep > 0 {
        accuracy = 100000. / pow(10., sep_f64);
        sep_easting_string = &mgrs_string[i..i + sep];
        sep_easting = sep_easting_string.parse::<f64>().unwrap() * accuracy;
        sep_northing_string = &mgrs_string[i + sep..];
        sep_northing = sep_northing_string.parse::<f64>().unwrap() * accuracy;
    }

    let easting = sep_easting + east100k;
    let northing = sep_northing + north100k;

    Utm { easting, northing, zone_letter, zone_number, accuracy: Some(accuracy) }
}

/// Given the first letter from a two-letter MGRS 100k zone, and given the
/// MGRS table set for the zone number, figure out the easting value that
/// should be added to the other, secondary easting value.
///
/// `e`: The first letter from a two-letter MGRS 100´k zone.
/// `set`: The MGRS table set for the zone number.
/// returns The easting value for the given letter and set.
fn get_easting_from_char(e: char, set: usize) -> f64 {
    let origin_col = SET_ORIGIN_COLUMN_LETTERS.chars().nth(set - 1).unwrap();
    let origin_index = ALPHABET.iter().position(|&c| c == origin_col).unwrap();
    let target_index = ALPHABET.iter().position(|&c| c == e).unwrap_or_else(|| {
        panic!("Bad character: {}", e);
    });
    let diff =
        (target_index as i32 - origin_index as i32 + ALPHABET.len() as i32) % ALPHABET.len() as i32;

    (diff as f64) * 100000.0
}

/// Given the second letter from a two-letter MGRS 100k zone, and given the
/// MGRS table set for the zone number, figure out the northing value that
/// should be added to the other, secondary northing value. You have to
/// remember that Northings are determined from the equator, and the vertical
/// cycle of letters mean a 2000000 additional northing meters. This happens
/// approx. every 18 degrees of latitude. This method does *NOT* count any
/// additional northings. You have to figure out how many 2000000 meters need
/// to be added for the zone letter of the MGRS coordinate.
///
/// `n`: Second letter of the MGRS 100k zone
/// `set`: The MGRS table set number, which is dependent on the UTM zone number.
/// returns the northing value for the given letter and set.
fn get_northing_from_char(n: char, set: usize) -> f64 {
    if n > 'V' {
        panic!("MGRSPoint given invalid Northing {}", n);
    }

    let origin_row = SET_ORIGIN_ROW_LETTERS.chars().nth(set - 1).unwrap();
    let origin_index = ALPHABET.iter().position(|&c| c == origin_row).unwrap();
    let target_index = ALPHABET.iter().position(|&c| c == n).unwrap_or_else(|| {
        panic!("Bad character: {}", n);
    });
    let diff =
        (target_index as i32 - origin_index as i32 + ALPHABET.len() as i32) % ALPHABET.len() as i32;

    (diff as f64) * 100000.0
}

/// The function get_min_northing returns the minimum northing value of a MGRS
/// zone.
///
/// Ported from Geotrans' c Lattitude_Band_Value structure table.
///
/// `zone_letter`: The MGRS zone to get the min northing for.
/// returns the minimum northing value of the MGRS zone.
fn get_min_northing(zone_letter: char) -> f64 {
    let northing = match zone_letter {
        'C' => 1100000.,
        'D' => 2000000.,
        'E' => 2800000.,
        'F' => 3700000.,
        'G' => 4600000.,
        'H' => 5500000.,
        'J' => 6400000.,
        'K' => 7300000.,
        'L' => 8200000.,
        'M' => 9100000.,
        'N' => 0.,
        'P' => 800000.,
        'Q' => 1700000.,
        'R' => 2600000.,
        'S' => 3500000.,
        'T' => 4400000.,
        'U' => 5300000.,
        'V' => 6200000.,
        'W' => 7000000.,
        'X' => 7900000.,
        _ => -1.,
    };
    if northing == -1. {
        panic!("Invalid zone letter: {}", zone_letter);
    }

    northing
}
