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
        "copenhagen" => 12.0,           // 1026 - "12d34'39.9\"E",
        "greenwich" => 0.0,             // 8901 - "0dE",
        "lisbon" => -9.131906111111,    // 8902 - "9d07'54.862\"W",
        "paris" => 2.337229166667,      // 8903 - "2d20'14.025\"E",
        "bogota" => -74.080916666667,   // 8904 - "74d04'51.3\"W",
        "madrid" => -3.687938888889,    // 8905 - "3d41'16.58\"W",
        "rome" => 12.452333333333,      // 8906 - "12d27'8.4\"E",
        "bern" => 7.439583333333,       // 8907 - "7d26'22.5\"E",
        "jakarta" => 106.807719444444,  // 8908 - "106d48'27.79\"E",
        "ferro" => -17.666666666667,    // 8909 - "17d40'W",
        "brussels" => 4.367975,         // 8910 - "4d22'4.71\"E",
        "stockholm" => 18.058277777778, // 8911 - "18d3'29.8\"E",
        "athens" => 23.7163375,         // 8912 - "23d42'58.815\"E",
        "budapest" => 19.040236111111,  // "19d0'36.9\"E",
        "oslo" => 10.722916666667,      // 8913 - "10d55'39.5\"E",
        // "paris_rgs" => // 8914 (UNUSED / REPLACED by paris) - "2d20'13.95\"E",
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
