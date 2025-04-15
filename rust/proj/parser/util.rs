use alloc::string::String;

/// Convert a string to camelCase
pub fn to_camel_case(s: &str) -> String {
    let trimmed_s = s.trim();
    let mut result = String::new();
    let mut capitalize_next = false;
    let mut first_word = true;

    for c in trimmed_s.chars() {
        if c == ' ' || c == '_' || c == '-' {
            capitalize_next = true;
            first_word = false;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
            first_word = false;
        } else if first_word {
            result.push(c.to_ascii_lowercase());
            first_word = false;
        } else {
            result.push(c);
        }
    }
    result
}

/// Returns the prime meridian in degrees for the given name. Falls back to 0.0 (greenwich)
/// if the name is not found
pub fn get_prime_meridian(name: &str) -> f64 {
    match name.to_lowercase().as_str() {
        "greenwich" => 0.0,             // "0dE",
        "lisbon" => -9.131906111111,    // "9d07'54.862\"W",
        "paris" => 2.337229166667,      // "2d20'14.025\"E",
        "bogota" => -74.080916666667,   // "74d04'51.3\"W",
        "madrid" => -3.687938888889,    // "3d41'16.58\"W",
        "rome" => 12.452333333333,      // "12d27'8.4\"E",
        "bern" => 7.439583333333,       // "7d26'22.5\"E",
        "jakarta" => 106.807719444444,  // "106d48'27.79\"E",
        "ferro" => -17.666666666667,    // "17d40'W",
        "brussels" => 4.367975,         // "4d22'4.71\"E",
        "stockholm" => 18.058277777778, // "18d3'29.8\"E",
        "athens" => 23.7163375,         // "23d42'58.815\"E",
        "budapest" => 19.040236111111,  // "19d0'36.9\"E",
        "oslo" => 10.722916666667,      // "10d55'39.5\"E",
        _ => 0.0,
    }
}

/// Convert a linear unit name to a multiplier to convert input units to meters
pub fn linear_unit_to_meters(unit: &str) -> f64 {
    match unit.trim().to_lowercase().as_str() {
        "km" | "kilometer" => 1000.0,
        "metre" | "meter" | "m" => 1.0,
        "dm" | "decimeter" => 0.1,
        "cm" | "centimeter" => 0.01,
        "mm" | "millimeter" => 0.001,
        "kmi" | "internationalnauticalmile" => 1852.0,
        "in" | "internationalinch" => 0.0254,
        "ft" | "internationalfoot" => 0.3048,
        "yd" | "internationalyard" => 0.9144,
        "mi" | "internationalstatutemile" => 1609.344,
        "fath" | "internationalfathom" => 1.8288,
        "ch" | "internationalchain" => 20.1168,
        "link" | "internationallink" => 0.201168,
        "us-in" | "ussurveyorinch" => 100.0 / 3937.0,
        "us-ft" | "ussurveyorfoot" => 1200.0 / 3937.0,
        "us-yd" | "ussurveyoryard" => 3600.0 / 3937.0,
        "us-ch" | "ussurveyorchain" => 79200.0 / 3937.0,
        "us-mi" | "ussurveyorstatutemile" => 6336000.0 / 3937.0,
        "ind-yd" | "indianyard" => 0.91439523,
        "ind-ft" | "indianfoot" => 0.30479841,
        "ind-ch" | "indianchain" => 20.11669506,
        _ => 0.0,
    }
}

/// Convert an angular unit name to a multiplier to convert input units to degrees
pub fn angular_unit_to_degrees(unit: &str) -> f64 {
    match unit.trim().to_lowercase().as_str() {
        "rad" | "radian" => 180.0 / core::f64::consts::PI,
        "grad" | "grade" => 1.0 / 200.0 * 180.0,
        "degree" => 1.0,
        _ => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
