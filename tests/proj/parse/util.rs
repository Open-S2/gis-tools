#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        angular_unit_to_degrees, axis_name_to_order, get_prime_meridian, linear_unit_to_meters,
        to_camel_case, to_pascal_case,
    };

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("north"), "North");
        assert_eq!(to_pascal_case("North East"), "NorthEast");
        assert_eq!(to_pascal_case("west_by_south_west"), "WestBySouthWest");
        assert_eq!(to_pascal_case("Display Left"), "DisplayLeft");
        assert_eq!(to_pascal_case("Display-Up"), "DisplayUp");
        assert_eq!(to_pascal_case(""), "");
        assert_eq!(to_pascal_case("northNorthEast"), "NorthNorthEast");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("North"), "north");
        assert_eq!(to_camel_case("North North East"), "northNorthEast");
        assert_eq!(to_camel_case("North_East"), "northEast");
        assert_eq!(to_camel_case("East-North-East"), "eastNorthEast");
        assert_eq!(to_camel_case("East"), "east");
        assert_eq!(to_camel_case("East by South East"), "eastBySouthEast");
        assert_eq!(to_camel_case("South East"), "southEast");
        assert_eq!(to_camel_case("South_by_South_East"), "southBySouthEast");
        assert_eq!(to_camel_case("South"), "south");
        assert_eq!(to_camel_case("South-South-West"), "southSouthWest");
        assert_eq!(to_camel_case("South West"), "southWest");
        assert_eq!(to_camel_case("West by South West"), "westBySouthWest");
        assert_eq!(to_camel_case("West"), "west");
        assert_eq!(to_camel_case("West by North West"), "westByNorthWest");
        assert_eq!(to_camel_case("North West"), "northWest");
        assert_eq!(to_camel_case("North_by_North_West"), "northByNorthWest");
        assert_eq!(to_camel_case("Up"), "up");
        assert_eq!(to_camel_case("Down"), "down");
        assert_eq!(to_camel_case("Geocentric X"), "geocentricX");
        assert_eq!(to_camel_case("Geocentric_Y"), "geocentricY");
        assert_eq!(to_camel_case("Geocentric-Z"), "geocentricZ");
        assert_eq!(to_camel_case("Column Positive"), "columnPositive");
        assert_eq!(to_camel_case("Column_Negative"), "columnNegative");
        assert_eq!(to_camel_case("Row-Positive"), "rowPositive");
        assert_eq!(to_camel_case("Row Negative"), "rowNegative");
        assert_eq!(to_camel_case("Display Right"), "displayRight");
        assert_eq!(to_camel_case("Display_Left"), "displayLeft");
        assert_eq!(to_camel_case("Display-Up"), "displayUp");
        assert_eq!(to_camel_case("Display Down"), "displayDown");
        assert_eq!(to_camel_case("Forward"), "forward");
        assert_eq!(to_camel_case("Aft"), "aft");
        assert_eq!(to_camel_case("Port"), "port");
        assert_eq!(to_camel_case("Starboard"), "starboard");
        assert_eq!(to_camel_case("Clockwise"), "clockwise");
        assert_eq!(to_camel_case("Counter Clockwise"), "counterClockwise");
        assert_eq!(to_camel_case("Towards"), "towards");
        assert_eq!(to_camel_case("Away From"), "awayFrom");
        assert_eq!(to_camel_case("Future"), "future");
        assert_eq!(to_camel_case("Past"), "past");
        assert_eq!(to_camel_case("Unspecified"), "unspecified");
        assert_eq!(to_camel_case(" South South West "), "southSouthWest");
        assert_eq!(to_camel_case("WEST"), "west");
    }

    #[test]
    fn test_linear_unit_to_meters() {
        assert_eq!(linear_unit_to_meters("km"), 1000.0);
        assert_eq!(linear_unit_to_meters("kilometer"), 1000.0);
        assert_eq!(linear_unit_to_meters("metre"), 1.0);
        assert_eq!(linear_unit_to_meters("meter"), 1.0);
        assert_eq!(linear_unit_to_meters("m"), 1.0);
        assert_eq!(linear_unit_to_meters("dm"), 0.1);
        assert_eq!(linear_unit_to_meters("decimeter"), 0.1);
        assert_eq!(linear_unit_to_meters("cm"), 0.01);
        assert_eq!(linear_unit_to_meters("centimeter"), 0.01);
        assert_eq!(linear_unit_to_meters("mm"), 0.001);
        assert_eq!(linear_unit_to_meters("millimeter"), 0.001);
        assert_eq!(linear_unit_to_meters("kmi"), 1852.0);
        assert_eq!(linear_unit_to_meters("internationalNauticalMile"), 1852.0);
        assert_eq!(linear_unit_to_meters("in"), 0.0254);
        assert_eq!(linear_unit_to_meters("internationalInch"), 0.0254);
        assert_eq!(linear_unit_to_meters("ft"), 0.3048);
        assert_eq!(linear_unit_to_meters("internationalFoot"), 0.3048);
        assert_eq!(linear_unit_to_meters("yd"), 0.9144);
        assert_eq!(linear_unit_to_meters("internationalYard"), 0.9144);
        assert_eq!(linear_unit_to_meters("mi"), 1609.344);
        assert_eq!(linear_unit_to_meters("internationalStatuteMile"), 1609.344);
        assert_eq!(linear_unit_to_meters("fath"), 1.8288);
        assert_eq!(linear_unit_to_meters("internationalFathom"), 1.8288);
        assert_eq!(linear_unit_to_meters("ch"), 20.1168);
        assert_eq!(linear_unit_to_meters("internationalChain"), 20.1168);
        assert_eq!(linear_unit_to_meters("link"), 0.201168);
        assert_eq!(linear_unit_to_meters("internationalLink"), 0.201168);
        assert_eq!(linear_unit_to_meters("us-in"), 100.0 / 3937.0);
        assert_eq!(linear_unit_to_meters("usSurveyorInch"), 100.0 / 3937.0);
        assert_eq!(linear_unit_to_meters("us-ft"), 1200.0 / 3937.0);
        assert_eq!(linear_unit_to_meters("usSurveyorFoot"), 1200.0 / 3937.0);
        assert_eq!(linear_unit_to_meters("us-yd"), 3600.0 / 3937.0);
        assert_eq!(linear_unit_to_meters("usSurveyorYard"), 3600.0 / 3937.0);
        assert_eq!(linear_unit_to_meters("us-ch"), 79200.0 / 3937.0);
        assert_eq!(linear_unit_to_meters("usSurveyorChain"), 79200.0 / 3937.0);
        assert_eq!(linear_unit_to_meters("us-mi"), 6336000.0 / 3937.0);
        assert_eq!(linear_unit_to_meters("usSurveyorStatuteMile"), 6336000.0 / 3937.0);
        assert_eq!(linear_unit_to_meters("ind-yd"), 0.91439523);
        assert_eq!(linear_unit_to_meters("indianYard"), 0.91439523);
        assert_eq!(linear_unit_to_meters("ind-ft"), 0.30479841);
        assert_eq!(linear_unit_to_meters("indianFoot"), 0.30479841);
        assert_eq!(linear_unit_to_meters("ind-ch"), 20.11669506);
        assert_eq!(linear_unit_to_meters("indianChain"), 20.11669506);
        assert_eq!(linear_unit_to_meters("unknown_unit"), 0.0);
        assert_eq!(linear_unit_to_meters(""), 0.0);
        assert_eq!(linear_unit_to_meters(" M "), 1.0); // Test with whitespace
        assert_eq!(linear_unit_to_meters("kIlOmEtEr"), 1000.0); // Test with mixed case
    }

    #[test]
    fn test_angular_unit_to_degrees() {
        assert_eq!(angular_unit_to_degrees("rad"), 180.0 / core::f64::consts::PI);
        assert_eq!(angular_unit_to_degrees("radian"), 180.0 / core::f64::consts::PI);
        assert_eq!(angular_unit_to_degrees("grad"), 1.0 / 200.0 * 180.0);
        assert_eq!(angular_unit_to_degrees("grade"), 1.0 / 200.0 * 180.0);
        assert_eq!(angular_unit_to_degrees("degree"), 1.0);
        assert_eq!(angular_unit_to_degrees("unknown"), 0.0);
        assert_eq!(angular_unit_to_degrees(""), 0.0);
        assert_eq!(angular_unit_to_degrees(" RAD "), 180.0 / core::f64::consts::PI); // Test with whitespace
        assert_eq!(angular_unit_to_degrees("dEgReE"), 1.0); // Test with mixed case
    }

    #[test]
    fn test_get_prime_meridian_greenwich() {
        assert_eq!(get_prime_meridian("Greenwich"), 0.0);
        assert_eq!(get_prime_meridian("greenwich"), 0.0);
        assert_eq!(get_prime_meridian("GREENWICH"), 0.0);
    }

    #[test]
    fn test_get_prime_meridian_lisbon() {
        assert_eq!(get_prime_meridian("Lisbon"), -9.131906111111);
        assert_eq!(get_prime_meridian("lisbon"), -9.131906111111);
        assert_eq!(get_prime_meridian("LISBON"), -9.131906111111);
    }

    #[test]
    fn test_get_prime_meridian_paris() {
        assert_eq!(get_prime_meridian("Paris"), 2.337229166667);
        assert_eq!(get_prime_meridian("paris"), 2.337229166667);
        assert_eq!(get_prime_meridian("PARIS"), 2.337229166667);
    }

    #[test]
    fn test_get_prime_meridian_bogota() {
        assert_eq!(get_prime_meridian("Bogota"), -74.080916666667);
        assert_eq!(get_prime_meridian("bogota"), -74.080916666667);
        assert_eq!(get_prime_meridian("BOGOTA"), -74.080916666667);
    }

    #[test]
    fn test_get_prime_meridian_madrid() {
        assert_eq!(get_prime_meridian("Madrid"), -3.687938888889);
        assert_eq!(get_prime_meridian("madrid"), -3.687938888889);
        assert_eq!(get_prime_meridian("MADRID"), -3.687938888889);
    }

    #[test]
    fn test_get_prime_meridian_rome() {
        assert_eq!(get_prime_meridian("Rome"), 12.452333333333);
        assert_eq!(get_prime_meridian("rome"), 12.452333333333);
        assert_eq!(get_prime_meridian("ROME"), 12.452333333333);
    }

    #[test]
    fn test_get_prime_meridian_bern() {
        assert_eq!(get_prime_meridian("Bern"), 7.439583333333);
        assert_eq!(get_prime_meridian("bern"), 7.439583333333);
        assert_eq!(get_prime_meridian("BERN"), 7.439583333333);
    }

    #[test]
    fn test_get_prime_meridian_jakarta() {
        assert_eq!(get_prime_meridian("Jakarta"), 106.807719444444);
        assert_eq!(get_prime_meridian("jakarta"), 106.807719444444);
        assert_eq!(get_prime_meridian("JAKARTA"), 106.807719444444);
    }

    #[test]
    fn test_get_prime_meridian_ferro() {
        assert_eq!(get_prime_meridian("Ferro"), -17.666666666667);
        assert_eq!(get_prime_meridian("ferro"), -17.666666666667);
        assert_eq!(get_prime_meridian("FERRO"), -17.666666666667);
    }

    #[test]
    fn test_get_prime_meridian_brussels() {
        assert_eq!(get_prime_meridian("Brussels"), 4.367975);
        assert_eq!(get_prime_meridian("brussels"), 4.367975);
        assert_eq!(get_prime_meridian("BRUSSELS"), 4.367975);
    }

    #[test]
    fn test_get_prime_meridian_stockholm() {
        assert_eq!(get_prime_meridian("Stockholm"), 18.058277777778);
        assert_eq!(get_prime_meridian("stockholm"), 18.058277777778);
        assert_eq!(get_prime_meridian("STOCKHOLM"), 18.058277777778);
    }

    #[test]
    fn test_get_prime_meridian_athens() {
        assert_eq!(get_prime_meridian("Athens"), 23.7163375);
        assert_eq!(get_prime_meridian("athens"), 23.7163375);
        assert_eq!(get_prime_meridian("ATHENS"), 23.7163375);
    }

    #[test]
    fn test_get_prime_meridian_budapest() {
        assert_eq!(get_prime_meridian("Budapest"), 19.040236111111);
        assert_eq!(get_prime_meridian("budapest"), 19.040236111111);
        assert_eq!(get_prime_meridian("BUDAPEST"), 19.040236111111);
    }

    #[test]
    fn test_get_prime_meridian_oslo() {
        assert_eq!(get_prime_meridian("Oslo"), 10.722916666667);
        assert_eq!(get_prime_meridian("oslo"), 10.722916666667);
        assert_eq!(get_prime_meridian("OSLO"), 10.722916666667);
    }

    #[test]
    fn test_get_prime_meridian_default() {
        assert_eq!(get_prime_meridian("London"), 0.0); // A name not in the list
        assert_eq!(get_prime_meridian("New York"), 0.0);
        assert_eq!(get_prime_meridian("tokyo"), 0.0);
        assert_eq!(get_prime_meridian(""), 0.0); // Empty string
        assert_eq!(get_prime_meridian("Some Random City"), 0.0);
    }

    #[test]
    fn test_axis_name_to_order() {
        // 0s
        assert_eq!(axis_name_to_order("X"), 0);
        assert_eq!(axis_name_to_order("(x)"), 0);
        assert_eq!(axis_name_to_order("east"), 0);
        assert_eq!(axis_name_to_order("east (x)"), 0);
        assert_eq!(axis_name_to_order("(e)"), 0);
        assert_eq!(axis_name_to_order("easting"), 0);
        assert_eq!(axis_name_to_order("easting (x)"), 0);

        // 1s
        assert_eq!(axis_name_to_order("y"), 1);
        assert_eq!(axis_name_to_order("(y)"), 1);
        assert_eq!(axis_name_to_order("north"), 1);
        assert_eq!(axis_name_to_order("north (y)"), 1);
        assert_eq!(axis_name_to_order("(n)"), 1);
        assert_eq!(axis_name_to_order("northing"), 1);
        assert_eq!(axis_name_to_order("northing (y)"), 1);

        // 2s
        assert_eq!(axis_name_to_order("z"), 2);
        assert_eq!(axis_name_to_order("(z)"), 2);

        // 3s
        assert_eq!(axis_name_to_order("t"), 3);
        assert_eq!(axis_name_to_order("(t)"), 3);

        // everything else
        assert_eq!(axis_name_to_order("anything"), 0);
    }
}
