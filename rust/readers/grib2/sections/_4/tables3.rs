#![cfg_attr(feature = "nightly", coverage(off))]

use crate::readers::{
    TableCategory, grib2_lookup_table42_00, grib2_lookup_table42_01, grib2_lookup_table42_02,
    grib2_lookup_table42_03, grib2_lookup_table42_04, grib2_lookup_table42_05,
    grib2_lookup_table42_06, grib2_lookup_table42_07, grib2_lookup_table42_10,
    grib2_lookup_table42_11, grib2_lookup_table42_12, grib2_lookup_table42_013,
    grib2_lookup_table42_014, grib2_lookup_table42_015, grib2_lookup_table42_016,
    grib2_lookup_table42_017, grib2_lookup_table42_018, grib2_lookup_table42_019,
    grib2_lookup_table42_020, grib2_lookup_table42_20, grib2_lookup_table42_021,
    grib2_lookup_table42_21, grib2_lookup_table42_022, grib2_lookup_table42_23,
    grib2_lookup_table42_24, grib2_lookup_table42_25, grib2_lookup_table42_26,
    grib2_lookup_table42_30, grib2_lookup_table42_31, grib2_lookup_table42_32,
    grib2_lookup_table42_33, grib2_lookup_table42_34, grib2_lookup_table42_35,
    grib2_lookup_table42_36, grib2_lookup_table42_40, grib2_lookup_table42_41,
    grib2_lookup_table42_42, grib2_lookup_table42_43, grib2_lookup_table42_44,
    grib2_lookup_table42_45, grib2_lookup_table42_46, grib2_lookup_table42_47,
    grib2_lookup_table42_48, grib2_lookup_table42_49, grib2_lookup_table42_100,
    grib2_lookup_table42_101, grib2_lookup_table42_102, grib2_lookup_table42_103,
    grib2_lookup_table42_104, grib2_lookup_table42_0190, grib2_lookup_table42_0191,
    grib2_lookup_table42_0192, grib2_lookup_table42_410, grib2_lookup_table42_2000,
    grib2_lookup_table42_2001, grib2_lookup_table42_2002, grib2_lookup_table42_2003,
    grib2_lookup_table42_10191,
};
use alloc::string::String;

/// Type and Unit categorizing
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeAndUnit {
    r#type: String,
    unit: String,
}
impl core::fmt::Display for TypeAndUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ({})", self.r#type, self.unit)
    }
}

fn no_op(_: u8) -> TableCategory {
    TableCategory {
        parameter: String::from("Reserved"),
        units: String::from(""),
        abbrev: String::from("Reserved"),
    }
}

/// GRIB2 - CODE TABLE 4.2: PARAMETER NUMBER BY PRODUCT DISCIPLINE AND PARAMETER CATEGORY
///
/// **Created**: 12/07/2023
/// **Revised**: 12/07/2023
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-1.shtml)
///
/// ## Notes
/// - By convention, the flux sign is positive if downward.
/// - When a new parameter is to be added to Code table 4.2 and more than one category applies, the
///   choice of category should be made base on the intended use of product. The discipline and
///   category are an important part of any product definition, so it is possible to have the same
///   parameter name in more than one category. For example, "Water Temperature" in discipline 10
///   (Oceanographic Products), category 4 (sub-surface properties) is used to reporting water
///   temperature in the ocean or open sea, and is not the same as "Water temperature" in discipline
///   1 (Hydrological Products), category 2 (Inland water and sediment properties) which is used for
///   reporting water temperature in freshwater lakes and rivers.
///
/// ## Reads as
/// `{ [discipline]: { [param catagory]: { TableCategory } }}`
pub fn grib2_lookup_table42(discipline: u8, category: u8) -> fn(category: u8) -> TableCategory {
    match discipline {
        // Product Discipline 0 - Meteorological products
        0 => {
            match category {
                0 => grib2_lookup_table42_00,
                1 => grib2_lookup_table42_01,
                2 => grib2_lookup_table42_02,
                3 => grib2_lookup_table42_03,
                4 => grib2_lookup_table42_04,
                5 => grib2_lookup_table42_05,
                6 => grib2_lookup_table42_06,
                7 => grib2_lookup_table42_07,
                13 => grib2_lookup_table42_013,
                14 => grib2_lookup_table42_014,
                15 => grib2_lookup_table42_015,
                16 => grib2_lookup_table42_016,
                17 => grib2_lookup_table42_017,
                18 => grib2_lookup_table42_018,
                19 => grib2_lookup_table42_019,
                20 => grib2_lookup_table42_020,
                21 => grib2_lookup_table42_021,
                22 => grib2_lookup_table42_022,
                190 => grib2_lookup_table42_0190,
                191 => grib2_lookup_table42_0191,
                // 192-254 Reserved for Local Use
                192 => grib2_lookup_table42_019,
                _ => no_op,
            }
        }
        // Product Discipline 1, Hydrologic products
        1 => {
            match category {
                0 => grib2_lookup_table42_10,
                1 => grib2_lookup_table42_11,
                2 => grib2_lookup_table42_12,
                // 3-191 Reserved
                // 192-254 Reserved for Local Use
                _ => no_op,
            }
        }
        // Product Discipline 2, Land Surface products
        2 => {
            match category {
                0 => grib2_lookup_table42_20,
                1 => grib2_lookup_table42_21,
                3 => grib2_lookup_table42_23,
                4 => grib2_lookup_table42_24,
                5 => grib2_lookup_table42_25,
                6 => grib2_lookup_table42_26,
                // 7-191 Reserved
                // 192-254 Reserved for Local Use
                _ => no_op,
            }
        }
        // Product Discipline 3, Space products
        3 => {
            match category {
                0 => grib2_lookup_table42_30,
                1 => grib2_lookup_table42_31,
                2 => grib2_lookup_table42_32,
                3 => grib2_lookup_table42_33,
                4 => grib2_lookup_table42_34,
                5 => grib2_lookup_table42_35,
                6 => grib2_lookup_table42_36,
                // 7-191 Reserved
                // 192-254 Reserved for Local Use
                192 => grib2_lookup_table42_0192,
                _ => no_op,
            }
        }
        // Product Discipline 4, Space Weather products
        4 => {
            match category {
                0 => grib2_lookup_table42_40,
                1 => grib2_lookup_table42_41,
                2 => grib2_lookup_table42_42,
                3 => grib2_lookup_table42_43,
                4 => grib2_lookup_table42_44,
                5 => grib2_lookup_table42_45,
                6 => grib2_lookup_table42_46,
                7 => grib2_lookup_table42_47,
                8 => grib2_lookup_table42_48,
                9 => grib2_lookup_table42_49,
                10 => grib2_lookup_table42_410,
                // 11-191 Reserved
                // 192-254 Reserved for Local Use
                _ => no_op,
            }
        }
        // Product Discipline 10, Oceanographic products
        10 => {
            match category {
                0 => grib2_lookup_table42_100,
                1 => grib2_lookup_table42_101,
                2 => grib2_lookup_table42_102,
                3 => grib2_lookup_table42_103,
                4 => grib2_lookup_table42_104,
                191 => grib2_lookup_table42_10191,
                // 192-254 Reserved for Local Use
                _ => no_op,
            }
        }
        // Product Discipline 20, Health and Socioeconomic impacts
        20 => {
            match category {
                0 => grib2_lookup_table42_2000,
                1 => grib2_lookup_table42_2001,
                2 => grib2_lookup_table42_2002,
                3 => grib2_lookup_table42_2003,
                // 4-191 Reserved
                // 192-254 Reserved for Local Use
                _ => no_op,
            }
        }
        _ => no_op,
    }
}

/// # GRIB2 - CODE TABLE 4.3 - TYPE OF GENERATING PROCESS
///
/// **Details**:
/// - **Section**: 4
/// - **Octet**: 12
/// - **Revised**: 10/24/2023
///
/// **Reserved Ranges**:
/// - `22-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-3.shtml)
///
/// ## Notes
/// 1. Code figures `12` and `13` are intended for cases where code figures `0` and `2` may not sufficiently indicate significant post-processing on initial analysis or forecast output.
/// 2. Analysis increment represents "analysis minus first guess."
/// 3. Initialized analysis increment represents "initialized analysis minus analysis."
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_3 {
    Analysis = 0,
    Initialization = 1,
    Forecast = 2,
    BiasCorrectedForecast = 3,
    EnsembleForecast = 4,
    ProbabilityForecast = 5,
    ForecastError = 6,
    AnalysisError = 7,
    Observation = 8,
    Climatological = 9,
    ProbabilityWeightedForecast = 10,
    BiasCorrectedEnsembleForecast = 11,
    PostProcessedAnalysis = 12,
    PostProcessedForecast = 13,
    Nowcast = 14,
    Hindcast = 15,
    PhysicalRetrieval = 16,
    RegressionAnalysis = 17,
    DifferenceBetweenTwoForecasts = 18,
    FirstGuess = 19,
    AnalysisIncrement = 20,
    InitializationIncrementForAnalysis = 21,
    ForecastConfidenceIndicator = 192,
    ProbabilityMatchedMean = 193,
    NeighborhoodProbability = 194,
    BiasCorrectedAndDownscaledEnsembleForecast = 195,
    PerturbedAnalysisForEnsembleInitialization = 196,
    EnsembleAgreementScaleProbability = 197,
    PostProcessedDeterministicExpertWeightedForecast = 198,
    EnsembleForecastBasedOnCounting = 199,
    LocalProbabilityMatchedMean = 200,
    Missing = 255,
}
impl From<u8> for Grib2Table4_3 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Analysis,
            1 => Self::Initialization,
            2 => Self::Forecast,
            3 => Self::BiasCorrectedForecast,
            4 => Self::EnsembleForecast,
            5 => Self::ProbabilityForecast,
            6 => Self::ForecastError,
            7 => Self::AnalysisError,
            8 => Self::Observation,
            9 => Self::Climatological,
            10 => Self::ProbabilityWeightedForecast,
            11 => Self::BiasCorrectedEnsembleForecast,
            12 => Self::PostProcessedAnalysis,
            13 => Self::PostProcessedForecast,
            14 => Self::Nowcast,
            15 => Self::Hindcast,
            16 => Self::PhysicalRetrieval,
            17 => Self::RegressionAnalysis,
            18 => Self::DifferenceBetweenTwoForecasts,
            19 => Self::FirstGuess,
            20 => Self::AnalysisIncrement,
            21 => Self::InitializationIncrementForAnalysis,
            192 => Self::ForecastConfidenceIndicator,
            193 => Self::ProbabilityMatchedMean,
            194 => Self::NeighborhoodProbability,
            195 => Self::BiasCorrectedAndDownscaledEnsembleForecast,
            196 => Self::PerturbedAnalysisForEnsembleInitialization,
            197 => Self::EnsembleAgreementScaleProbability,
            198 => Self::PostProcessedDeterministicExpertWeightedForecast,
            199 => Self::EnsembleForecastBasedOnCounting,
            200 => Self::LocalProbabilityMatchedMean,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Analysis => "Analysis",
            Self::Initialization => "Initialization",
            Self::Forecast => "Forecast",
            Self::BiasCorrectedForecast => "Bias Corrected Forecast",
            Self::EnsembleForecast => "Ensemble Forecast",
            Self::ProbabilityForecast => "Probability Forecast",
            Self::ForecastError => "Forecast Error",
            Self::AnalysisError => "Analysis Error",
            Self::Observation => "Observation",
            Self::Climatological => "Climatological",
            Self::ProbabilityWeightedForecast => "Probability-Weighted Forecast",
            Self::BiasCorrectedEnsembleForecast => "Bias-Corrected Ensemble Forecast",
            Self::PostProcessedAnalysis => "Post-processed Analysis",
            Self::PostProcessedForecast => "Post-processed Forecast",
            Self::Nowcast => "Nowcast",
            Self::Hindcast => "Hindcast",
            Self::PhysicalRetrieval => "Physical Retrieval",
            Self::RegressionAnalysis => "Regression Analysis",
            Self::DifferenceBetweenTwoForecasts => "Difference Between Two Forecasts",
            Self::FirstGuess => "First guess",
            Self::AnalysisIncrement => "Analysis increment",
            Self::InitializationIncrementForAnalysis => "Initialization increment for analysis",
            Self::ForecastConfidenceIndicator => "Forecast Confidence Indicator",
            Self::ProbabilityMatchedMean => "Probability-matched Mean",
            Self::NeighborhoodProbability => "Neighborhood Probability",
            Self::BiasCorrectedAndDownscaledEnsembleForecast => {
                "Bias-Corrected and Downscaled Ensemble Forecast"
            }
            Self::PerturbedAnalysisForEnsembleInitialization => {
                "Perturbed Analysis for Ensemble Initialization"
            }
            Self::EnsembleAgreementScaleProbability => "Ensemble Agreement Scale Probability",
            Self::PostProcessedDeterministicExpertWeightedForecast => {
                "Post-Processed Deterministic-Expert-Weighted Forecast"
            }
            Self::EnsembleForecastBasedOnCounting => "Ensemble Forecast Based on Counting",
            Self::LocalProbabilityMatchedMean => "Local Probability-matched Mean",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.4 - INDICATOR OF UNIT OF TIME RANGE
///
/// **Details**:
/// - **Section**: 4
/// - **Octet**: 18
/// - **Created**: 05/12/2005
///
/// **Reserved Ranges**:
/// - `14-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-4.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_4 {
    Minute = 0,
    Hour = 1,
    Day = 2,
    Month = 3,
    Year = 4,
    Decade = 5,
    Normal = 6,
    Century = 7,
    Reserved8 = 8,
    Reserved9 = 9,
    Hours3 = 10,
    Hours6 = 11,
    Hours12 = 12,
    Second = 13,
    Missing = 255,
}
impl From<u8> for Grib2Table4_4 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Minute,
            1 => Self::Hour,
            2 => Self::Day,
            3 => Self::Month,
            4 => Self::Year,
            5 => Self::Decade,
            6 => Self::Normal,
            7 => Self::Century,
            8 => Self::Reserved8,
            9 => Self::Reserved9,
            10 => Self::Hours3,
            11 => Self::Hours6,
            12 => Self::Hours12,
            13 => Self::Second,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Minute => "Minute",
            Self::Hour => "Hour",
            Self::Day => "Day",
            Self::Month => "Month",
            Self::Year => "Year",
            Self::Decade => "Decade (10 Years)",
            Self::Normal => "Normal (30 Years)",
            Self::Century => "Century (100 Years)",
            Self::Reserved8 => "Reserved",
            Self::Reserved9 => "Reserved",
            Self::Hours3 => "3 Hours",
            Self::Hours6 => "6 Hours",
            Self::Hours12 => "12 Hours",
            Self::Second => "Second",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.5 - FIXED SURFACE TYPES AND UNITS
///
/// **Details**:
/// - **Section**: 4
/// - **Octets**: 23 and 29
/// - **Revised**: 12/07/2023
///
/// **Reserved Ranges**:
/// - `28-29`: Reserved
/// - `36-99`: Reserved
/// - `120-149`: Reserved
/// - `153-159`: Reserved
/// - `190-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml)
///
/// ## Notes
/// 1. The Eta vertical coordinate system involves normalizing the pressure at some point on a specific level by the mean sea level pressure at that point.
/// 2. Hybrid height level can be defined as: z(k)=A(k)+B(k)*orog.
/// 3. Hybrid pressure level is defined as: p(k)=A(k)+B(k)*sp.
/// 4. Sigma height level is the height-based terrain-following coordinate.
/// 5. The soil level represents a model level with varying depths provided by another GRIB message.
/// 6. The sea-ice level represents varying depths across the model domain.
/// 7. Ocean level types are defined by property differences from the near-surface.
/// 8. This level differs from entry 13, which is vertically accumulated from the surface.
///
/// This function provides a lookup for GRIB2 fixed surface types and their units
/// based on the provided code.
///
/// # Arguments
/// * `code` - The code for the fixed surface type (u8).
///
/// # Returns
/// A `TypeAndUnit` struct containing the type and unit of the fixed surface.
/// Returns "Missing" if the code is 255, "Reserved" for specified reserved ranges,
/// or "Reserved for Local Use" for local use ranges.
pub fn grib2_lookup_table4_5(code: u8) -> TypeAndUnit {
    match code {
        0 => TypeAndUnit { r#type: String::from("Reserved"), unit: String::from("") },
        1 => {
            TypeAndUnit { r#type: String::from("Ground or Water Surface"), unit: String::from("") }
        }
        2 => TypeAndUnit { r#type: String::from("Cloud Base Level"), unit: String::from("") },
        3 => TypeAndUnit { r#type: String::from("Level of Cloud Tops"), unit: String::from("") },
        4 => TypeAndUnit {
            r#type: String::from("Level of 0°C Isotherm"),
            unit: String::from("°C"),
        },
        5 => TypeAndUnit {
            r#type: String::from("Level of Adiabatic Condensation Lifted from the Surface"),
            unit: String::from(""),
        },
        6 => TypeAndUnit { r#type: String::from("Maximum Wind Level"), unit: String::from("") },
        7 => TypeAndUnit { r#type: String::from("Tropopause"), unit: String::from("") },
        8 => TypeAndUnit {
            r#type: String::from("Nominal Top of the Atmosphere"),
            unit: String::from(""),
        },
        9 => TypeAndUnit { r#type: String::from("Sea Bottom"), unit: String::from("") },
        10 => TypeAndUnit { r#type: String::from("Entire Atmosphere"), unit: String::from("") },
        11 => {
            TypeAndUnit { r#type: String::from("Cumulonimbus Base (CB)"), unit: String::from("m") }
        }
        12 => {
            TypeAndUnit { r#type: String::from("Cumulonimbus Top (CT)"), unit: String::from("m") }
        }
        13 => TypeAndUnit {
            r#type: String::from(
                "Lowest level where vertically integrated cloud cover exceeds the specified \
                 percentage",
            ),
            unit: String::from("%"),
        },
        14 => TypeAndUnit {
            r#type: String::from("Level of free convection (LFC)"),
            unit: String::from(""),
        },
        15 => TypeAndUnit {
            r#type: String::from("Convection condensation level (CCL)"),
            unit: String::from(""),
        },
        16 => TypeAndUnit {
            r#type: String::from("Level of neutral buoyancy or equilibrium (LNB)"),
            unit: String::from(""),
        },
        17 => TypeAndUnit {
            r#type: String::from("Departure level of the most unstable parcel of air (MUDL)"),
            unit: String::from(""),
        },
        18 => TypeAndUnit {
            r#type: String::from(
                "Departure level of a mixed layer parcel of air with specified layer depth",
            ),
            unit: String::from("Pa"),
        },
        19 => TypeAndUnit {
            r#type: String::from("Lowest level where cloud cover exceeds the specified percentage"),
            unit: String::from("%"),
        },
        20 => TypeAndUnit { r#type: String::from("Isothermal Level"), unit: String::from("K") },
        21 => TypeAndUnit {
            r#type: String::from("Lowest level where mass density exceeds the specified value"),
            unit: String::from("kg m-3"),
        },
        22 => TypeAndUnit {
            r#type: String::from("Highest level where mass density exceeds the specified value"),
            unit: String::from("kg m-3"),
        },
        23 => TypeAndUnit {
            r#type: String::from(
                "Lowest level where air concentration exceeds the specified value",
            ),
            unit: String::from("Bq m-3"),
        },
        24 => TypeAndUnit {
            r#type: String::from(
                "Highest level where air concentration exceeds the specified value",
            ),
            unit: String::from("Bq m-3"),
        },
        25 => TypeAndUnit {
            r#type: String::from(
                "Highest level where radar reflectivity exceeds the specified value",
            ),
            unit: String::from("dBZ"),
        },
        26 => TypeAndUnit {
            r#type: String::from("Convective cloud layer base"),
            unit: String::from("m"),
        },
        27 => TypeAndUnit {
            r#type: String::from("Convective cloud layer top"),
            unit: String::from("m"),
        },
        28..=29 => TypeAndUnit { r#type: String::from("Reserved"), unit: String::from("") },
        30 => TypeAndUnit {
            r#type: String::from("Specified radius from the centre of the Sun"),
            unit: String::from("m"),
        },
        31 => TypeAndUnit { r#type: String::from("Solar photosphere"), unit: String::from("") },
        32 => TypeAndUnit {
            r#type: String::from("Ionospheric D-region level"),
            unit: String::from(""),
        },
        33 => TypeAndUnit {
            r#type: String::from("Ionospheric E-region level"),
            unit: String::from(""),
        },
        34 => TypeAndUnit {
            r#type: String::from("Ionospheric F1-region level"),
            unit: String::from(""),
        },
        35 => TypeAndUnit {
            r#type: String::from("Ionospheric F2-region level"),
            unit: String::from(""),
        },
        36..=99 => TypeAndUnit { r#type: String::from("Reserved"), unit: String::from("") },
        100 => TypeAndUnit { r#type: String::from("Isobaric Surface"), unit: String::from("Pa") },
        101 => TypeAndUnit { r#type: String::from("Mean Sea Level"), unit: String::from("") },
        102 => TypeAndUnit {
            r#type: String::from("Specific Altitude Above Mean Sea Level"),
            unit: String::from("m"),
        },
        103 => TypeAndUnit {
            r#type: String::from("Specified Height Level Above Ground"),
            unit: String::from("m"),
        },
        104 => TypeAndUnit { r#type: String::from("Sigma Level"), unit: String::from("") },
        105 => TypeAndUnit { r#type: String::from("Hybrid Level"), unit: String::from("") },
        106 => TypeAndUnit {
            r#type: String::from("Depth Below Land Surface"),
            unit: String::from("m"),
        },
        107 => TypeAndUnit {
            r#type: String::from("Isentropic (theta) Level"),
            unit: String::from("K"),
        },
        108 => TypeAndUnit {
            r#type: String::from("Level at Specified Pressure Difference from Ground to Level"),
            unit: String::from("Pa"),
        },
        109 => TypeAndUnit {
            r#type: String::from("Potential Vorticity Surface"),
            unit: String::from("K m² kg⁻¹ s⁻¹"),
        },
        110 => TypeAndUnit { r#type: String::from("Reserved"), unit: String::from("") },
        111 => TypeAndUnit { r#type: String::from("Eta Level"), unit: String::from("") },
        112 => TypeAndUnit { r#type: String::from("Reserved"), unit: String::from("") },
        113 => {
            TypeAndUnit { r#type: String::from("Logarithmic Hybrid Level"), unit: String::from("") }
        }
        114 => TypeAndUnit { r#type: String::from("Snow Level"), unit: String::from("") },
        115 => TypeAndUnit { r#type: String::from("Sigma height level"), unit: String::from("") },
        116 => TypeAndUnit { r#type: String::from("Reserved"), unit: String::from("") },
        117 => TypeAndUnit { r#type: String::from("Mixed Layer Depth"), unit: String::from("m") },
        118 => TypeAndUnit { r#type: String::from("Hybrid Height Level"), unit: String::from("") },
        119 => {
            TypeAndUnit { r#type: String::from("Hybrid Pressure Level"), unit: String::from("") }
        }
        120..=149 => TypeAndUnit { r#type: String::from("Reserved"), unit: String::from("") },
        150 => TypeAndUnit {
            r#type: String::from("Generalized Vertical Height Coordinate"),
            unit: String::from(""),
        },
        151 => TypeAndUnit { r#type: String::from("Soil level"), unit: String::from("") },
        152 => TypeAndUnit { r#type: String::from("Sea-ice level"), unit: String::from("") },
        153..=159 => TypeAndUnit { r#type: String::from("Reserved"), unit: String::from("") },
        160 => {
            TypeAndUnit { r#type: String::from("Depth Below Sea Level"), unit: String::from("m") }
        }
        161 => TypeAndUnit {
            r#type: String::from("Depth Below Water Surface"),
            unit: String::from("m"),
        },
        162 => TypeAndUnit { r#type: String::from("Lake or River Bottom"), unit: String::from("") },
        163 => {
            TypeAndUnit { r#type: String::from("Bottom Of Sediment Layer"), unit: String::from("") }
        }
        164 => TypeAndUnit {
            r#type: String::from("Bottom Of Thermally Active Sediment Layer"),
            unit: String::from(""),
        },
        165 => TypeAndUnit {
            r#type: String::from("Bottom Of Sediment Layer Penetrated By Thermal Wave"),
            unit: String::from(""),
        },
        166 => TypeAndUnit { r#type: String::from("Mixing Layer"), unit: String::from("") },
        167 => TypeAndUnit { r#type: String::from("Bottom of Root Zone"), unit: String::from("") },
        168 => TypeAndUnit { r#type: String::from("Ocean Model Level"), unit: String::from("") },
        169 => TypeAndUnit {
            r#type: String::from(
                "Ocean level defined by water density (sigma-theta) difference from near-surface \
                 to level",
            ),
            unit: String::from("kg m-3"),
        },
        170 => TypeAndUnit {
            r#type: String::from(
                "Ocean level defined by water potential temperature difference from near-surface \
                 to level",
            ),
            unit: String::from("K"),
        },
        171 => TypeAndUnit {
            r#type: String::from(
                "Ocean level defined by vertical eddy diffusivity difference from near-surface to \
                 level",
            ),
            unit: String::from("m² s-1"),
        },
        172 => TypeAndUnit {
            r#type: String::from(
                "Ocean level defined by water density (rho) difference from near-surface to level",
            ),
            unit: String::from("m"),
        },
        173 => TypeAndUnit {
            r#type: String::from("Top of Snow Over Sea Ice on Sea, Lake or River"),
            unit: String::from(""),
        },
        174 => TypeAndUnit {
            r#type: String::from("Top Surface of Ice on Sea, Lake or River"),
            unit: String::from(""),
        },
        175 => TypeAndUnit {
            r#type: String::from("Top Surface of Ice, under Snow, on Sea, Lake or River"),
            unit: String::from(""),
        },
        176 => TypeAndUnit {
            r#type: String::from("Bottom Surface (underside) Ice on Sea, Lake or River"),
            unit: String::from(""),
        },
        177 => TypeAndUnit {
            r#type: String::from("Deep Soil (of indefinite depth)"),
            unit: String::from(""),
        },
        178 => TypeAndUnit { r#type: String::from("Reserved"), unit: String::from("") },
        179 => TypeAndUnit {
            r#type: String::from("Top Surface of Glacier Ice and Inland Ice"),
            unit: String::from(""),
        },
        180 => TypeAndUnit {
            r#type: String::from("Deep Inland or Glacier Ice (of indefinite depth)"),
            unit: String::from(""),
        },
        181 => TypeAndUnit {
            r#type: String::from("Grid Tile Land Fraction as a Model Surface"),
            unit: String::from(""),
        },
        182 => TypeAndUnit {
            r#type: String::from("Grid Tile Water Fraction as a Model Surface"),
            unit: String::from(""),
        },
        183 => TypeAndUnit {
            r#type: String::from("Grid Tile Ice Fraction on Sea, Lake or River as a Model Surface"),
            unit: String::from(""),
        },
        184 => TypeAndUnit {
            r#type: String::from(
                "Grid Tile Glacier Ice and Inland Ice Fraction as a Model Surface",
            ),
            unit: String::from(""),
        },
        185 => TypeAndUnit { r#type: String::from("Roof Level"), unit: String::from("") },
        186 => TypeAndUnit { r#type: String::from("Wall level"), unit: String::from("") },
        187 => TypeAndUnit { r#type: String::from("Road Level"), unit: String::from("") },
        188 => {
            TypeAndUnit { r#type: String::from("Melt pond Top Surface"), unit: String::from("") }
        }
        189 => {
            TypeAndUnit { r#type: String::from("Melt Pond Bottom Surface"), unit: String::from("") }
        }
        190..=191 => TypeAndUnit { r#type: String::from("Reserved"), unit: String::from("") },
        200 => TypeAndUnit {
            r#type: String::from("Entire atmosphere (considered as a single layer)"),
            unit: String::from(""),
        },
        201 => TypeAndUnit {
            r#type: String::from("Entire ocean (considered as a single layer)"),
            unit: String::from(""),
        },
        202 => {
            TypeAndUnit { r#type: String::from("Reserved for Local Use"), unit: String::from("") }
        }
        203 => {
            TypeAndUnit { r#type: String::from("Reserved for Local Use"), unit: String::from("") }
        }
        204 => TypeAndUnit {
            r#type: String::from("Highest tropospheric freezing level"),
            unit: String::from(""),
        },
        205 => {
            TypeAndUnit { r#type: String::from("Reserved for Local Use"), unit: String::from("") }
        }
        206 => TypeAndUnit {
            r#type: String::from("Grid scale cloud bottom level"),
            unit: String::from(""),
        },
        207 => TypeAndUnit {
            r#type: String::from("Grid scale cloud top level"),
            unit: String::from(""),
        },
        208 => {
            TypeAndUnit { r#type: String::from("Reserved for Local Use"), unit: String::from("") }
        }
        209 => TypeAndUnit {
            r#type: String::from("Boundary layer cloud bottom level"),
            unit: String::from(""),
        },
        210 => TypeAndUnit {
            r#type: String::from("Boundary layer cloud top level"),
            unit: String::from(""),
        },
        211 => TypeAndUnit {
            r#type: String::from("Boundary layer cloud layer"),
            unit: String::from(""),
        },
        212 => {
            TypeAndUnit { r#type: String::from("Low cloud bottom level"), unit: String::from("") }
        }
        213 => TypeAndUnit { r#type: String::from("Low cloud top level"), unit: String::from("") },
        214 => TypeAndUnit { r#type: String::from("Low cloud layer"), unit: String::from("") },
        215 => TypeAndUnit { r#type: String::from("Cloud ceiling"), unit: String::from("") },
        216 => TypeAndUnit {
            r#type: String::from("Effective Layer Top Level"),
            unit: String::from("m"),
        },
        217 => TypeAndUnit {
            r#type: String::from("Effective Layer Bottom Level"),
            unit: String::from("m"),
        },
        218 => TypeAndUnit { r#type: String::from("Effective Layer"), unit: String::from("m") },
        219 => {
            TypeAndUnit { r#type: String::from("Reserved for Local Use"), unit: String::from("") }
        }
        220 => {
            TypeAndUnit { r#type: String::from("Planetary Boundary Layer"), unit: String::from("") }
        }
        221 => TypeAndUnit {
            r#type: String::from("Layer Between Two Hybrid Levels"),
            unit: String::from(""),
        },
        222 => TypeAndUnit {
            r#type: String::from("Middle cloud bottom level"),
            unit: String::from(""),
        },
        223 => {
            TypeAndUnit { r#type: String::from("Middle cloud top level"), unit: String::from("") }
        }
        224 => TypeAndUnit { r#type: String::from("Middle cloud layer"), unit: String::from("") },
        225..=231 => {
            TypeAndUnit { r#type: String::from("Reserved for Local Use"), unit: String::from("") }
        }
        232 => {
            TypeAndUnit { r#type: String::from("High cloud bottom level"), unit: String::from("") }
        }
        233 => TypeAndUnit { r#type: String::from("High cloud top level"), unit: String::from("") },
        234 => TypeAndUnit { r#type: String::from("High cloud layer"), unit: String::from("") },
        235 => TypeAndUnit {
            r#type: String::from("Ocean Isotherm Level (1/10 °C)"),
            unit: String::from("°C"),
        },
        236 => TypeAndUnit {
            r#type: String::from("Layer between two depths below ocean surface"),
            unit: String::from(""),
        },
        237 => TypeAndUnit {
            r#type: String::from("Bottom of Ocean Mixed Layer"),
            unit: String::from("m"),
        },
        238 => TypeAndUnit {
            r#type: String::from("Bottom of Ocean Isothermal Layer"),
            unit: String::from("m"),
        },
        239 => TypeAndUnit {
            r#type: String::from("Layer Ocean Surface and 26°C Ocean Isothermal Level"),
            unit: String::from(""),
        },
        240 => TypeAndUnit { r#type: String::from("Ocean Mixed Layer"), unit: String::from("") },
        241 => {
            TypeAndUnit { r#type: String::from("Ordered Sequence of Data"), unit: String::from("") }
        }
        242 => TypeAndUnit {
            r#type: String::from("Convective cloud bottom level"),
            unit: String::from(""),
        },
        243 => TypeAndUnit {
            r#type: String::from("Convective cloud top level"),
            unit: String::from(""),
        },
        244 => {
            TypeAndUnit { r#type: String::from("Convective cloud layer"), unit: String::from("") }
        }
        245 => TypeAndUnit {
            r#type: String::from("Lowest level of the wet bulb zero"),
            unit: String::from(""),
        },
        246 => TypeAndUnit {
            r#type: String::from("Maximum equivalent potential temperature level"),
            unit: String::from(""),
        },
        247 => TypeAndUnit { r#type: String::from("Equilibrium level"), unit: String::from("") },
        248 => TypeAndUnit {
            r#type: String::from("Shallow convective cloud bottom level"),
            unit: String::from(""),
        },
        249 => TypeAndUnit {
            r#type: String::from("Shallow convective cloud top level"),
            unit: String::from(""),
        },
        250 => {
            TypeAndUnit { r#type: String::from("Reserved for Local Use"), unit: String::from("") }
        }
        251 => TypeAndUnit {
            r#type: String::from("Deep convective cloud bottom level"),
            unit: String::from(""),
        },
        252 => TypeAndUnit {
            r#type: String::from("Deep convective cloud top level"),
            unit: String::from(""),
        },
        253 => TypeAndUnit {
            r#type: String::from("Lowest bottom level of supercooled liquid water layer"),
            unit: String::from(""),
        },
        254 => TypeAndUnit {
            r#type: String::from("Highest top level of supercooled liquid water layer"),
            unit: String::from(""),
        },
        255 => TypeAndUnit { r#type: String::from("Missing"), unit: String::from("") },
        // Handle explicit reserved and local use ranges
        // The original TS had some 'Reserved' and 'Reserved for Local Use' entries
        // that fell within the general ranges. I've left those explicit in the match
        // and covered the remaining parts of the ranges here.
        192..=199 => {
            TypeAndUnit { r#type: String::from("Reserved for Local Use"), unit: String::from("") }
        }
    }
}

/// # GRIB2 - CODE TABLE 4.6 - TYPE OF ENSEMBLE FORECAST
///
/// **Details**:
/// - **Section**: 4
/// - **Octet**: 35 (for product templates 1 and 11)
/// - **Revised**: 07/22/2010
///
/// **Reserved Ranges**:
/// - `5-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-6.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_6 {
    UnperturbedHighResolutionControlForecast = 0,
    UnperturbedLowResolutionControlForecast = 1,
    NegativelyPerturbedForecast = 2,
    PositivelyPerturbedForecast = 3,
    MultiModelForecast = 4,
    PerturbedEnsembleMember = 192,
    Missing = 255,
}
impl From<u8> for Grib2Table4_6 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::UnperturbedHighResolutionControlForecast,
            1 => Self::UnperturbedLowResolutionControlForecast,
            2 => Self::NegativelyPerturbedForecast,
            3 => Self::PositivelyPerturbedForecast,
            4 => Self::MultiModelForecast,
            192 => Self::PerturbedEnsembleMember,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_6 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::UnperturbedHighResolutionControlForecast => {
                "Unperturbed High-Resolution Control Forecast"
            }
            Self::UnperturbedLowResolutionControlForecast => {
                "Unperturbed Low-Resolution Control Forecast"
            }
            Self::NegativelyPerturbedForecast => "Negatively Perturbed Forecast",
            Self::PositivelyPerturbedForecast => "Positively Perturbed Forecast",
            Self::MultiModelForecast => "Multi-Model Forecast",
            Self::PerturbedEnsembleMember => "Perturbed Ensemble Member",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.7 - DERIVED FORECAST
///
/// **Details**:
/// - **Section**: 4
/// - **Octet**: 35 (for product templates 2-4 and 12-14)
/// - **Revised**: 07/15/2024
///
/// **Reserved Ranges**:
/// - `11-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-7.shtml)
///
/// ## Notes
/// 1. Large anomaly index is defined as:
///    `{(number of members whose anomaly > 0.5 * SD) - (number of members whose anomaly < -0.5 * SD)} / (number of members)`.
///    SD is the observed climatological standard deviation.
/// 2. The reference for "minimum of all ensemble members" and "maximum of all ensemble members" is the set of ensemble members
///    and not a time interval; this differs from Product Definition Template 4.8.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_7 {
    UnweightedMeanOfAllMembers = 0,
    WeightedMeanOfAllMembers = 1,
    StandardDeviationWithRespectToClusterMean = 2,
    StandardDeviationWithRespectToClusterMeanNormalized = 3,
    SpreadOfAllMembers = 4,
    LargeAnomalyIndexOfAllMembers = 5,
    UnweightedMeanOfTheClusterMembers = 6,
    InterquartileRange = 7,
    MinimumOfAllEnsembleMembers = 8,
    MaximumOfAllEnsembleMembers = 9,
    VarianceOfAllEnsembleMembers = 10,
    UnweightedModeOfAllMembers = 192,
    PercentileValue10OfAllMembers = 193,
    PercentileValue50OfAllMembers = 194,
    PercentileValue90OfAllMembers = 195,
    StatisticallyDecidedWeightsForEachEnsembleMember = 196,
    ClimatePercentile = 197,
    DeviationOfEnsembleMeanFromDailyClimatology = 198,
    ExtremeForecastIndex = 199,
    EquallyWeightedMean = 200,
    PercentileValue5OfAllMembers = 201,
    PercentileValue25OfAllMembers = 202,
    PercentileValue75OfAllMembers = 203,
    PercentileValue95OfAllMembers = 204,
    Missing = 255,
}
impl From<u8> for Grib2Table4_7 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::UnweightedMeanOfAllMembers,
            1 => Self::WeightedMeanOfAllMembers,
            2 => Self::StandardDeviationWithRespectToClusterMean,
            3 => Self::StandardDeviationWithRespectToClusterMeanNormalized,
            4 => Self::SpreadOfAllMembers,
            5 => Self::LargeAnomalyIndexOfAllMembers,
            6 => Self::UnweightedMeanOfTheClusterMembers,
            7 => Self::InterquartileRange,
            8 => Self::MinimumOfAllEnsembleMembers,
            9 => Self::MaximumOfAllEnsembleMembers,
            10 => Self::VarianceOfAllEnsembleMembers,
            192 => Self::UnweightedModeOfAllMembers,
            193 => Self::PercentileValue10OfAllMembers,
            194 => Self::PercentileValue50OfAllMembers,
            195 => Self::PercentileValue90OfAllMembers,
            196 => Self::StatisticallyDecidedWeightsForEachEnsembleMember,
            197 => Self::ClimatePercentile,
            198 => Self::DeviationOfEnsembleMeanFromDailyClimatology,
            199 => Self::ExtremeForecastIndex,
            200 => Self::EquallyWeightedMean,
            201 => Self::PercentileValue5OfAllMembers,
            202 => Self::PercentileValue25OfAllMembers,
            203 => Self::PercentileValue75OfAllMembers,
            204 => Self::PercentileValue95OfAllMembers,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_7 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::UnweightedMeanOfAllMembers => "Unweighted Mean of All Members",
            Self::WeightedMeanOfAllMembers => "Weighted Mean of All Members",
            Self::StandardDeviationWithRespectToClusterMean => {
                "Standard Deviation with respect to Cluster Mean"
            }
            Self::StandardDeviationWithRespectToClusterMeanNormalized => {
                "Standard Deviation with respect to Cluster Mean, Normalized"
            }
            Self::SpreadOfAllMembers => "Spread of All Members",
            Self::LargeAnomalyIndexOfAllMembers => "Large Anomaly Index of All Members",
            Self::UnweightedMeanOfTheClusterMembers => "Unweighted Mean of the Cluster Members",
            Self::InterquartileRange => {
                "Interquartile Range (Range between the 25th and 75th quantile)"
            }
            Self::MinimumOfAllEnsembleMembers => "Minimum Of All Ensemble Members",
            Self::MaximumOfAllEnsembleMembers => "Maximum Of All Ensemble Members",
            Self::VarianceOfAllEnsembleMembers => "Variance of all ensemble members",
            Self::UnweightedModeOfAllMembers => "Unweighted Mode of All Members",
            Self::PercentileValue10OfAllMembers => "Percentile value (10%) of All Members",
            Self::PercentileValue50OfAllMembers => "Percentile value (50%) of All Members",
            Self::PercentileValue90OfAllMembers => "Percentile value (90%) of All Members",
            Self::StatisticallyDecidedWeightsForEachEnsembleMember => {
                "Statistically decided weights for each ensemble member"
            }
            Self::ClimatePercentile => {
                "Climate Percentile (percentile values from climate distribution)"
            }
            Self::DeviationOfEnsembleMeanFromDailyClimatology => {
                "Deviation of Ensemble Mean from Daily Climatology"
            }
            Self::ExtremeForecastIndex => "Extreme Forecast Index",
            Self::EquallyWeightedMean => "Equally Weighted Mean",
            Self::PercentileValue5OfAllMembers => "Percentile value (5%) of All Members",
            Self::PercentileValue25OfAllMembers => "Percentile value (25%) of All Members",
            Self::PercentileValue75OfAllMembers => "Percentile value (75%) of All Members",
            Self::PercentileValue95OfAllMembers => "Percentile value (95%) of All Members",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.8 - CLUSTERING METHOD
///
/// **Details**:
/// - **Section**: 4
/// - **Octet**: 41 (for product templates 3-4 and 13-14)
/// - **Created**: 05/12/2005
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-8.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_8 {
    AnomalyCorrelation = 0,
    RootMeanSquare = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table4_8 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::AnomalyCorrelation,
            1 => Self::RootMeanSquare,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::AnomalyCorrelation => "Anomoly Correlation",
            Self::RootMeanSquare => "Root Mean Square",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.9 - PROBABILITY TYPE
///
/// **Details**:
/// - **Section**: 4
/// - **Octet**: 37 (for product templates 5 and 9)
/// - **Revised**: 07/15/2024
///
/// **Reserved Ranges**:
/// - `10-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-9.shtml)
///
/// ## Notes
/// 1. Above normal, near normal, and below normal are defined as three equiprobable categories based on climatology at each point.
///    The methodology and reference climatology are unspecified and should be documented by the data producer.
/// 2. Product definition templates using this table may include octets for lower and upper limits. For categorical probabilities
///    (e.g., below, near, or above normal), these octets are set to "all ones" (missing).
/// 3. Scale Factor and Scaled Values for lower/upper limits must be set to missing for entry `9`.
///    This is primarily for categorical boolean counts.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_9 {
    ProbabilityOfEventBelowLowerLimit = 0,
    ProbabilityOfEventAboveUpperLimit = 1,
    ProbabilityOfEventBetweenUpperAndLowerLimits = 2,
    ProbabilityOfEventAboveLowerLimit = 3,
    ProbabilityOfEventBelowUpperLimit = 4,
    ProbabilityOfEventEqualToLowerLimit = 5,
    ProbabilityOfEventInAboveNormalCategory = 6,
    ProbabilityOfEventInNearNormalCategory = 7,
    ProbabilityOfEventInBelowNormalCategory = 8,
    ProbabilityBasedOnCountsOfCategoricalBoolean = 9,
    Missing = 255,
}
impl From<u8> for Grib2Table4_9 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::ProbabilityOfEventBelowLowerLimit,
            1 => Self::ProbabilityOfEventAboveUpperLimit,
            2 => Self::ProbabilityOfEventBetweenUpperAndLowerLimits,
            3 => Self::ProbabilityOfEventAboveLowerLimit,
            4 => Self::ProbabilityOfEventBelowUpperLimit,
            5 => Self::ProbabilityOfEventEqualToLowerLimit,
            6 => Self::ProbabilityOfEventInAboveNormalCategory,
            7 => Self::ProbabilityOfEventInNearNormalCategory,
            8 => Self::ProbabilityOfEventInBelowNormalCategory,
            9 => Self::ProbabilityBasedOnCountsOfCategoricalBoolean,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_9 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::ProbabilityOfEventBelowLowerLimit => "Probability of event below lower limit",
            Self::ProbabilityOfEventAboveUpperLimit => "Probability of event above upper limit",
            Self::ProbabilityOfEventBetweenUpperAndLowerLimits => {
                "Probability of event between upper and lower limits (range includes lower limit \
                 but not the upper limit)"
            }
            Self::ProbabilityOfEventAboveLowerLimit => "Probability of event above lower limit",
            Self::ProbabilityOfEventBelowUpperLimit => "Probability of event below upper limit",
            Self::ProbabilityOfEventEqualToLowerLimit => {
                "Probability of event equal to lower limit"
            }
            Self::ProbabilityOfEventInAboveNormalCategory => {
                "Probability of event in above normal category"
            }
            Self::ProbabilityOfEventInNearNormalCategory => {
                "Probability of event in near normal category"
            }
            Self::ProbabilityOfEventInBelowNormalCategory => {
                "Probability of event in below normal category"
            }
            Self::ProbabilityBasedOnCountsOfCategoricalBoolean => {
                "Probability based on counts of categorical boolean"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.10 - TYPE OF STATISTICAL PROCESSING
///
/// **Details**:
/// - **Section**: 4
/// - **Octet**: 47 (for product template 8)
/// - **Revised**: 10/24/2023
///
/// **Reserved Ranges**:
/// - `14-99`: Reserved
/// - `103-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-10.shtml)
///
/// ## Notes
/// 1. The original data value has units of Code Table 4.2 multiplied by seconds, unless otherwise specified.
/// 2. Covariance (Code 7) has squared units of Code Table 4.2.
/// 3. Ratio (Code 9) is a non-dimensional number without units.
/// 4. For code number 102, the drought index is defined by discipline 0, parameter category 22, and the corresponding parameter number.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_10 {
    Average = 0,
    Accumulation = 1,
    Maximum = 2,
    Minimum = 3,
    DifferenceEndMinusBeginning = 4,
    RootMeanSquare = 5,
    StandardDeviation = 6,
    Covariance = 7,
    DifferenceBeginningMinusEnd = 8,
    Ratio = 9,
    StandardizedAnomaly = 10,
    Summation = 11,
    ReturnPeriod = 12,
    Median = 13,
    Severity = 100,
    Mode = 101,
    IndexProcessing = 102,
    ClimatologicalMeanValue = 192,
    AverageOfNForecasts = 193,
    AverageOfNUninitializedAnalyses = 194,
    AverageOfForecastAccumulations24Hour = 195,
    AverageOfSuccessiveForecastAccumulations = 196,
    AverageOfForecastAverages24Hour = 197,
    AverageOfSuccessiveForecastAverages = 198,
    ClimatologicalAverageOfNAnalyses = 199,
    ClimatologicalAverageOfNForecasts = 200,
    ClimatologicalRootMeanSquareDifferenceBetweenNForecastsAndTheirVerifyingAnalyses = 201,
    ClimatologicalStandardDeviationOfNForecasts = 202,
    ClimatologicalStandardDeviationOfNAnalyses = 203,
    AverageOfForecastAccumulations6Hour = 204,
    AverageOfForecastAverages6Hour = 205,
    AverageOfForecastAccumulations12Hour = 206,
    AverageOfForecastAverages12Hour = 207,
    Variance = 208,
    Coefficient = 209,
    Missing = 255,
}
impl From<u8> for Grib2Table4_10 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Average,
            1 => Self::Accumulation,
            2 => Self::Maximum,
            3 => Self::Minimum,
            4 => Self::DifferenceEndMinusBeginning,
            5 => Self::RootMeanSquare,
            6 => Self::StandardDeviation,
            7 => Self::Covariance,
            8 => Self::DifferenceBeginningMinusEnd,
            9 => Self::Ratio,
            10 => Self::StandardizedAnomaly,
            11 => Self::Summation,
            12 => Self::ReturnPeriod,
            13 => Self::Median,
            100 => Self::Severity,
            101 => Self::Mode,
            102 => Self::IndexProcessing,
            192 => Self::ClimatologicalMeanValue,
            193 => Self::AverageOfNForecasts,
            194 => Self::AverageOfNUninitializedAnalyses,
            195 => Self::AverageOfForecastAccumulations24Hour,
            196 => Self::AverageOfSuccessiveForecastAccumulations,
            197 => Self::AverageOfForecastAverages24Hour,
            198 => Self::AverageOfSuccessiveForecastAverages,
            199 => Self::ClimatologicalAverageOfNAnalyses,
            200 => Self::ClimatologicalAverageOfNForecasts,
            201 => Self::ClimatologicalRootMeanSquareDifferenceBetweenNForecastsAndTheirVerifyingAnalyses,
            202 => Self::ClimatologicalStandardDeviationOfNForecasts,
            203 => Self::ClimatologicalStandardDeviationOfNAnalyses,
            204 => Self::AverageOfForecastAccumulations6Hour,
            205 => Self::AverageOfForecastAverages6Hour,
            206 => Self::AverageOfForecastAccumulations12Hour,
            207 => Self::AverageOfForecastAverages12Hour,
            208 => Self::Variance,
            209 => Self::Coefficient,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_10 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Average => "Average",
            Self::Accumulation => "Accumulation",
            Self::Maximum => "Maximum",
            Self::Minimum => "Minimum",
            Self::DifferenceEndMinusBeginning => "Difference (value at the end of the time range minus value at the beginning)",
            Self::RootMeanSquare => "Root Mean Square",
            Self::StandardDeviation => "Standard Deviation",
            Self::Covariance => "Covariance (temporal variance)",
            Self::DifferenceBeginningMinusEnd => "Difference (value at the beginning of the time range minus value at the end)",
            Self::Ratio => "Ratio",
            Self::StandardizedAnomaly => "Standardized Anomaly",
            Self::Summation => "Summation",
            Self::ReturnPeriod => "Return period",
            Self::Median => "Median",
            Self::Severity => "Severity",
            Self::Mode => "Mode",
            Self::IndexProcessing => "Index processing",
            Self::ClimatologicalMeanValue => "Climatological Mean Value",
            Self::AverageOfNForecasts => "Average of N forecasts (or initialized analyses)",
            Self::AverageOfNUninitializedAnalyses => "Average of N uninitialized analyses",
            Self::AverageOfForecastAccumulations24Hour => "Average of forecast accumulations (24-hour intervals)",
            Self::AverageOfSuccessiveForecastAccumulations => "Average of successive forecast accumulations",
            Self::AverageOfForecastAverages24Hour => "Average of forecast averages (24-hour intervals)",
            Self::AverageOfSuccessiveForecastAverages => "Average of successive forecast averages",
            Self::ClimatologicalAverageOfNAnalyses => "Climatological Average of N analyses",
            Self::ClimatologicalAverageOfNForecasts => "Climatological Average of N forecasts",
            Self::ClimatologicalRootMeanSquareDifferenceBetweenNForecastsAndTheirVerifyingAnalyses => {
                "Climatological Root Mean Square difference between N forecasts and their verifying analyses"
            }
            Self::ClimatologicalStandardDeviationOfNForecasts => {
                "Climatological Standard Deviation of N forecasts"
            }
            Self::ClimatologicalStandardDeviationOfNAnalyses => {
                "Climatological Standard Deviation of N analyses"
            }
            Self::AverageOfForecastAccumulations6Hour => "Average of forecast accumulations (6-hour intervals)",
            Self::AverageOfForecastAverages6Hour => "Average of forecast averages (6-hour intervals)",
            Self::AverageOfForecastAccumulations12Hour => "Average of forecast accumulations (12-hour intervals)",
            Self::AverageOfForecastAverages12Hour => "Average of forecast averages (12-hour intervals)",
            Self::Variance => "Variance",
            Self::Coefficient => "Coefficient",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.11 - TYPE OF TIME INTERVALS
///
/// **Details**:
/// - **Section**: 4
/// - **Octet**: 48 (for product templates 8)
/// - **Revised**: 12/21/2011
///
/// **Reserved Ranges**:
/// - `6-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-11.shtml)
///
/// ## Notes
/// 1. Code figure `5` applies when a single time subinterval is used to calculate the statistically processed field.
///    The exact starting and ending times of the subinterval are not specified but are inclusively within the overall interval.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_11 {
    Reserved = 0,
    SuccessiveTimesProcessedHaveSameForecastTimeStartOfForecastIsIncremented = 1,
    SuccessiveTimesProcessedHaveSameStartTimeOfForecastForecastTimeIsIncremented = 2,
    SuccessiveTimesProcessedHaveStartTimeOfForecastIncrementedAndForecastTimeDecrementedSoThatValidTimeRemainsConstant =
        3,
    SuccessiveTimesProcessedHaveStartTimeOfForecastDecrementedAndForecastTimeIncrementedSoThatValidTimeRemainsConstant =
        4,
    FloatingSubintervalOfTimeBetweenForecastTimeAndEndOfOverallTimeInterval = 5,
    Missing = 255,
}
impl From<u8> for Grib2Table4_11 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved,
            1 => Self::SuccessiveTimesProcessedHaveSameForecastTimeStartOfForecastIsIncremented,
            2 => Self::SuccessiveTimesProcessedHaveSameStartTimeOfForecastForecastTimeIsIncremented,
            3 => Self::SuccessiveTimesProcessedHaveStartTimeOfForecastIncrementedAndForecastTimeDecrementedSoThatValidTimeRemainsConstant,
            4 => Self::SuccessiveTimesProcessedHaveStartTimeOfForecastDecrementedAndForecastTimeIncrementedSoThatValidTimeRemainsConstant,
            5 => Self::FloatingSubintervalOfTimeBetweenForecastTimeAndEndOfOverallTimeInterval,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_11 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved => "Reserved",
            Self::SuccessiveTimesProcessedHaveSameForecastTimeStartOfForecastIsIncremented => {
                "Successive times processed have same forecast time, start time of forecast is incremented."
            }
            Self::SuccessiveTimesProcessedHaveSameStartTimeOfForecastForecastTimeIsIncremented => {
                "Successive times processed have same start time of forecast, forecast time is incremented."
            }
            Self::SuccessiveTimesProcessedHaveStartTimeOfForecastIncrementedAndForecastTimeDecrementedSoThatValidTimeRemainsConstant => {
                "Successive times processed have start time of forecast incremented and forecast time decremented so that valid time remains constant."
            }
            Self::SuccessiveTimesProcessedHaveStartTimeOfForecastDecrementedAndForecastTimeIncrementedSoThatValidTimeRemainsConstant => {
                "Successive times processed have start time of forecast decremented and forecast time incremented so that valid time remains constant."
            }
            Self::FloatingSubintervalOfTimeBetweenForecastTimeAndEndOfOverallTimeInterval => {
                "Floating subinterval of time between forecast time and end of overall time interval."
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.12 - OPERATING MODE
///
/// **Details**:
/// - **Section**: 4
/// - **Octet**: 31 (for product template 20)
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `3-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-12.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_12 {
    MaintenanceMode = 0,
    ClearAir = 1,
    Precipitation = 2,
    Missing = 255,
}
impl From<u8> for Grib2Table4_12 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::MaintenanceMode,
            1 => Self::ClearAir,
            2 => Self::Precipitation,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_12 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::MaintenanceMode => "Maintenance Mode",
            Self::ClearAir => "Clear Air",
            Self::Precipitation => "Precipitation",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.13 - QUALITY CONTROL INDICATOR
///
/// **Details**:
/// - **Section**: 4
/// - **Octet**: 33 (for Product Definition Template 20)
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-13.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_13 {
    NoQualityControlApplied = 0,
    QualityControlApplied = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table4_13 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoQualityControlApplied,
            1 => Self::QualityControlApplied,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_13 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NoQualityControlApplied => "No Quality Control Applied",
            Self::QualityControlApplied => "Quality Control Applied",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.14 - CLUTTER FILTER INDICATOR
///
/// **Details**:
/// - **Section**: 4
/// - **Octet**: 34 (for product template 20)
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-14.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_14 {
    NoClutterFilterUsed = 0,
    ClutterFilterUsed = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table4_14 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoClutterFilterUsed,
            1 => Self::ClutterFilterUsed,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_14 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NoClutterFilterUsed => "No Clutter Filter Used",
            Self::ClutterFilterUsed => "Clutter Filter Used",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.15 - TYPE OF SPATIAL PROCESSING
///
/// **Details**:
/// - **Created**: 12/10/2009
///
/// **Reserved Ranges**:
/// - `7-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-15.shtml)
///
/// ## Notes
/// 1. This method assumes that each field represents box averages/maxima/minima extending halfway to neighboring grid points.
/// 2. Budget interpolation quasi-conserves area averages, useful for budget fields such as precipitation. It averages bilinearly
///    interpolated values within a square array of points distributed in each output grid box.
/// 3. Neighbor-budget interpolation performs a budget interpolation at the grid point nearest to the nominal grid point.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_15 {
    DataCalculatedDirectlyFromSourceGridNoInterpolation = 0,
    BilinearInterpolation = 1,
    BicubicInterpolation = 2,
    NearestNeighbor = 3,
    BudgetInterpolation = 4,
    SpectralInterpolation = 5,
    NeighborBudgetInterpolation = 6,
    Missing = 255,
}
impl From<u8> for Grib2Table4_15 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::DataCalculatedDirectlyFromSourceGridNoInterpolation,
            1 => Self::BilinearInterpolation,
            2 => Self::BicubicInterpolation,
            3 => Self::NearestNeighbor,
            4 => Self::BudgetInterpolation,
            5 => Self::SpectralInterpolation,
            6 => Self::NeighborBudgetInterpolation,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_15 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::DataCalculatedDirectlyFromSourceGridNoInterpolation => {
                "Data is calculated directly from the source grid with no interpolation"
            }
            Self::BilinearInterpolation => {
                "Bilinear interpolation using the 4 source grid-point values surrounding the \
                 nominal grid-point"
            }
            Self::BicubicInterpolation => {
                "Bicubic interpolation using the 4 source grid-point values surrounding the \
                 nominal grid-point"
            }
            Self::NearestNeighbor => {
                "Using the value from the source grid-point which is nearest to the nominal \
                 grid-point"
            }
            Self::BudgetInterpolation => {
                "Budget interpolation using the 4 source grid-point values surrounding the nominal \
                 grid-point"
            }
            Self::SpectralInterpolation => {
                "Spectral interpolation using the 4 source grid-point values surrounding the \
                 nominal grid-point"
            }
            Self::NeighborBudgetInterpolation => {
                "Neighbor-budget interpolation using the 4 source grid-point values surrounding \
                 the nominal grid-point"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.16 - QUALITY VALUE ASSOCIATED WITH PARAMETER
///
/// **Details**:
/// - **Section**: 4
/// - **Octet**: 14 (for product templates 35)
/// - **Revised**: 07/01/2022
///
/// **Reserved Ranges**:
/// - `6-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-16.shtml)
///
/// ## Notes
/// 1. When a non-missing value is used, it represents a quality value associated with the parameter defined by octets 10 and 11 of the product definition template.
/// 2. For "Confidence index" (Code 0), the value ranges from 0 (no confidence) to 1 (maximal confidence) and is non-dimensional.
/// 3. "Quality indicator" (Code 1) values are defined by Code Table 4.244.
/// 4. "Correlation of Product with used Calibration Product" (Code 2) is a non-dimensional value without units.
/// 5. For "Standard deviation" (Code 3) and "Random error" (Code 4), the value uses the same units as the parameter defined by octets 10 and 11.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_16 {
    ConfidenceIndex = 0,
    QualityIndicator = 1,
    CorrelationOfProductWithUsedCalibrationProduct = 2,
    StandardDeviation = 3,
    RandomError = 4,
    Probability = 5,
    Missing = 255,
}
impl From<u8> for Grib2Table4_16 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::ConfidenceIndex,
            1 => Self::QualityIndicator,
            2 => Self::CorrelationOfProductWithUsedCalibrationProduct,
            3 => Self::StandardDeviation,
            4 => Self::RandomError,
            5 => Self::Probability,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::ConfidenceIndex => "Confidence index",
            Self::QualityIndicator => "Quality indicator",
            Self::CorrelationOfProductWithUsedCalibrationProduct => {
                "Correlation of Product with used Calibration Product"
            }
            Self::StandardDeviation => "Standard deviation",
            Self::RandomError => "Random error",
            Self::Probability => "Probability",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.91 - TYPE OF INTERVAL
///
/// **Details**:
/// - **Created**: 12/21/2011
///
/// **Reserved Ranges**:
/// - `12-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-91.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_91 {
    SmallerThanFirstLimit = 0,
    GreaterThanSecondLimit = 1,
    BetweenFirstAndSecondLimitIncludesFirstButNotSecond = 2,
    GreaterThanFirstLimit = 3,
    SmallerThanSecondLimit = 4,
    SmallerOrEqualFirstLimit = 5,
    GreaterOrEqualSecondLimit = 6,
    BetweenFirstAndSecondLimitIncludesBoth = 7,
    GreaterOrEqualFirstLimit = 8,
    SmallerOrEqualSecondLimit = 9,
    BetweenFirstAndSecondLimitIncludesSecondButNotFirst = 10,
    EqualToFirstLimit = 11,
    Missing = 255,
}
impl From<u8> for Grib2Table4_91 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::SmallerThanFirstLimit,
            1 => Self::GreaterThanSecondLimit,
            2 => Self::BetweenFirstAndSecondLimitIncludesFirstButNotSecond,
            3 => Self::GreaterThanFirstLimit,
            4 => Self::SmallerThanSecondLimit,
            5 => Self::SmallerOrEqualFirstLimit,
            6 => Self::GreaterOrEqualSecondLimit,
            7 => Self::BetweenFirstAndSecondLimitIncludesBoth,
            8 => Self::GreaterOrEqualFirstLimit,
            9 => Self::SmallerOrEqualSecondLimit,
            10 => Self::BetweenFirstAndSecondLimitIncludesSecondButNotFirst,
            11 => Self::EqualToFirstLimit,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_91 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::SmallerThanFirstLimit => "Smaller than first limit",
            Self::GreaterThanSecondLimit => "Greater than second limit",
            Self::BetweenFirstAndSecondLimitIncludesFirstButNotSecond => {
                "Between first and second limit (includes first limit but not the second)"
            }
            Self::GreaterThanFirstLimit => "Greater than first limit",
            Self::SmallerThanSecondLimit => "Smaller than second limit",
            Self::SmallerOrEqualFirstLimit => "Smaller or equal first limit",
            Self::GreaterOrEqualSecondLimit => "Greater or equal second limit",
            Self::BetweenFirstAndSecondLimitIncludesBoth => {
                "Between first and second limit (includes both first and second limits)"
            }
            Self::GreaterOrEqualFirstLimit => "Greater or equal first limit",
            Self::SmallerOrEqualSecondLimit => "Smaller or equal second limit",
            Self::BetweenFirstAndSecondLimitIncludesSecondButNotFirst => {
                "Between first and second limit (includes second limit but not the first)"
            }
            Self::EqualToFirstLimit => "Equal to first limit",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.100 - TYPE OF REFERENCE DATASET
///
/// **Details**:
/// - **Created**: 10/24/2023
///
/// **Reserved Ranges**:
/// - `6-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-100.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_100 {
    Analysis = 0,
    Forecast = 1,
    Reforecast = 2,
    Reanalysis = 3,
    ClimateProjection = 4,
    GriddedObservations = 5,
    Missing = 255,
}
impl From<u8> for Grib2Table4_100 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Analysis,
            1 => Self::Forecast,
            2 => Self::Reforecast,
            3 => Self::Reanalysis,
            4 => Self::ClimateProjection,
            5 => Self::GriddedObservations,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_100 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Analysis => "Analysis",
            Self::Forecast => "Forecast",
            Self::Reforecast => "Reforecast (Hindcast)",
            Self::Reanalysis => "Reanalysis",
            Self::ClimateProjection => "Climate Projection",
            Self::GriddedObservations => "Gridded observations",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.101 - TYPE OF RELATIONSHIP TO REFERENCE DATASET
///
/// **Details**:
/// - **Revised**: 07/12/2024
///
/// **Reserved Ranges**:
/// - `4-19`: Reserved
/// - `24-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-101.shtml)
///
/// ## Notes
/// 1. No additional parameter is needed for entries `0` and `1` (NA=0).
/// 2. Entry `2` (Significance) requires a confidence interval (NA=1).
/// 3. Entry `20` (EFI) is defined in [https://doi.org/10.1256/qj.02.152](https://doi.org/10.1256/qj.02.152). No additional parameter is needed.
/// 4. Entry `21` (SOT) requires lower and upper quantiles to be defined (NA=2).
/// 5. Entry `22` (Anomaly of probabilities) applies to templates `4.112` and `4.123`.
/// 6. Entry `23` (Standardized Drought Index) follows definitions from the WMO Handbook on Drought Indicators and Indices.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_101 {
    Anomaly = 0,
    StandardizedAnomaly = 1,
    Significance = 2,
    Climatology = 3,
    ExtremeForecastIndex = 20,
    ShiftOfTails = 21,
    AnomalyOfProbabilities = 22,
    StandardizedDroughtIndex = 23,
    Missing = 255,
}
impl From<u8> for Grib2Table4_101 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Anomaly,
            1 => Self::StandardizedAnomaly,
            2 => Self::Significance,
            3 => Self::Climatology,
            20 => Self::ExtremeForecastIndex,
            21 => Self::ShiftOfTails,
            22 => Self::AnomalyOfProbabilities,
            23 => Self::StandardizedDroughtIndex,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_101 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Anomaly => "Anomaly",
            Self::StandardizedAnomaly => "Standardized Anomaly",
            Self::Significance => "Significance (Wilcoxon-Mann-Whitney)",
            Self::Climatology => "Climatology",
            Self::ExtremeForecastIndex => "Extreme Forecast Index (EFI)",
            Self::ShiftOfTails => "Shift of Tails (SOT)",
            Self::AnomalyOfProbabilities => "Anomaly of probabilities",
            Self::StandardizedDroughtIndex => "Standardized Drought Index",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.102 - STATISTICAL PROCESSING OF REFERENCE PERIOD
///
/// **Details**:
/// - **Revised**: 07/12/2024
///
/// **Reserved Ranges**:
/// - `5-19`: Reserved
/// - `32-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-102.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_102 {
    Average = 0,
    Accumulation = 1,
    Maximum = 2,
    Minimum = 3,
    Median = 4,
    ModelClimate = 20,
    IndexBasedOnNormalDistribution = 21,
    IndexBasedOnLogNormalDistribution = 22,
    IndexBasedOnGeneralisedLogNormalDistribution = 23,
    IndexBasedOnGammaDistribution = 24,
    IndexBasedOnLogisticDistribution = 25,
    IndexBasedOnLogLogisticDistribution = 26,
    IndexBasedOnGeneralisedLogisticDistribution = 27,
    IndexBasedOnWeibullDistribution = 28,
    IndexBasedOnGeneralisedExtremeValueDistribution = 29,
    IndexBasedOnPearsonIIIDistribution = 30,
    IndexBasedOnEmpiricalDistribution = 31,
    Missing = 255,
}
impl From<u8> for Grib2Table4_102 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Average,
            1 => Self::Accumulation,
            2 => Self::Maximum,
            3 => Self::Minimum,
            4 => Self::Median,
            20 => Self::ModelClimate,
            21 => Self::IndexBasedOnNormalDistribution,
            22 => Self::IndexBasedOnLogNormalDistribution,
            23 => Self::IndexBasedOnGeneralisedLogNormalDistribution,
            24 => Self::IndexBasedOnGammaDistribution,
            25 => Self::IndexBasedOnLogisticDistribution,
            26 => Self::IndexBasedOnLogLogisticDistribution,
            27 => Self::IndexBasedOnGeneralisedLogisticDistribution,
            28 => Self::IndexBasedOnWeibullDistribution,
            29 => Self::IndexBasedOnGeneralisedExtremeValueDistribution,
            30 => Self::IndexBasedOnPearsonIIIDistribution,
            31 => Self::IndexBasedOnEmpiricalDistribution,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_102 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Average => "Average",
            Self::Accumulation => "Accumulation",
            Self::Maximum => "Maximum",
            Self::Minimum => "Minimum",
            Self::Median => "Median",
            Self::ModelClimate => "Model Climate",
            Self::IndexBasedOnNormalDistribution => "Index based on normal distribution",
            Self::IndexBasedOnLogNormalDistribution => "Index based on log-normal distribution",
            Self::IndexBasedOnGeneralisedLogNormalDistribution => {
                "Index based on generalised log-normal distribution"
            }
            Self::IndexBasedOnGammaDistribution => "Index based on gamma distribution",
            Self::IndexBasedOnLogisticDistribution => "Index based on logistic distribution",
            Self::IndexBasedOnLogLogisticDistribution => "Index based on log-logistic distribution",
            Self::IndexBasedOnGeneralisedLogisticDistribution => {
                "Index based on generalised logistic distribution"
            }
            Self::IndexBasedOnWeibullDistribution => "Index based on Weibull distribution",
            Self::IndexBasedOnGeneralisedExtremeValueDistribution => {
                "Index based on generalised extreme value distribution"
            }
            Self::IndexBasedOnPearsonIIIDistribution => "Index based on Pearson III distribution",
            Self::IndexBasedOnEmpiricalDistribution => "Index based on empirical distribution",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.103 - SPATIAL VICINITY TYPE
///
/// **Details**:
/// - **Created**: 07/12/2024
///
/// **Reserved Ranges**:
/// - `5-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-103.shtml)
///
/// ## Notes
/// 1. The following additional arguments must be specified:
///    - **Circle**: 1 argument for the radius in meters.
///    - **Rectangle**: 2 arguments for length: 1. west-east and 2. south-north, in meters.
///    - **Square**: 1 argument for the length of equal-length sides in meters.
///    - **Wedge**: 3 arguments: 1. radius in meters, 2. start angle, and 3. end angle in degrees.
///      Angles are measured counter-clockwise from 0° along the positive west-east axis.
///    - **Span of grid cells**: 2 arguments for grid cells: 1. west-east span `i +/- x`
///      and 2. south-north span `j +/- y`.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_103 {
    Circle = 0,
    Rectangle = 1,
    Square = 2,
    Wedge = 3,
    SpanOfGridBoxesCenteredAroundGridBoxIJ = 4,
    Missing = 255,
}
impl From<u8> for Grib2Table4_103 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Circle,
            1 => Self::Rectangle,
            2 => Self::Square,
            3 => Self::Wedge,
            4 => Self::SpanOfGridBoxesCenteredAroundGridBoxIJ,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_103 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Circle => "Circle [m]",
            Self::Rectangle => "Rectangle [m, m]",
            Self::Square => "Square [m]",
            Self::Wedge => "Wedge [m, degree, degree]",
            Self::SpanOfGridBoxesCenteredAroundGridBoxIJ => {
                "Span of grid boxes centered around grid box i,j [x, y]"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.104 - SPATIAL AND TEMPORAL VICINITY PROCESSING
///
/// **Details**:
/// - **Created**: 07/12/2024
///
/// **Reserved Ranges**:
/// - `1`: Reserved
/// - `5`: Reserved
/// - `7-10`: Reserved
/// - `12-189`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-104.shtml)
///
/// ## Notes
/// 1. The option **Quantile** (Code 190) requires two additional arguments:
///    - The total number of quantiles.
///    - The quantile value.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_104 {
    Average = 0,
    Reserved1 = 1,
    Maximum = 2,
    Minimum = 3,
    Range = 4,
    Reserved5 = 5,
    StandardDeviation = 6,
    Sum = 11,
    Quantile = 190,
    Categorical = 191,
    Missing = 255,
}
impl From<u8> for Grib2Table4_104 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Average,
            1 => Self::Reserved1,
            2 => Self::Maximum,
            3 => Self::Minimum,
            4 => Self::Range,
            5 => Self::Reserved5,
            6 => Self::StandardDeviation,
            11 => Self::Sum,
            190 => Self::Quantile,
            191 => Self::Categorical,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_104 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Average => "Average",
            Self::Reserved1 => "Reserved",
            Self::Maximum => "Maximum",
            Self::Minimum => "Minimum",
            Self::Range => "Range",
            Self::Reserved5 => "Reserved",
            Self::StandardDeviation => "Standard deviation",
            Self::Sum => "Sum",
            Self::Quantile => "Quantile",
            Self::Categorical => "Categorical (boolean)",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.105 - SPATIAL AND TEMPORAL VICINITY MISSING DATA
///
/// **Details**:
/// - **Created**: 07/12/2024
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-105.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_105 {
    IgnoreMissingData = 0,
    NoData = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table4_105 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::IgnoreMissingData,
            1 => Self::NoData,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_105 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::IgnoreMissingData => "Ignore missing data",
            Self::NoData => "No data",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.201 - PRECIPITATION TYPE
///
/// **Details**:
/// - **Revised**: 05/29/2019
///
/// **Reserved Ranges**:
/// - `13-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-201.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_201 {
    Reserved = 0,
    Rain = 1,
    Thunderstorm = 2,
    FreezingRain = 3,
    MixedIce = 4,
    Snow = 5,
    WetSnow = 6,
    MixtureOfRainAndSnow = 7,
    IcePellets = 8,
    Graupel = 9,
    Hail = 10,
    Drizzle = 11,
    FreezingDrizzle = 12,
    Missing = 255,
}
impl From<u8> for Grib2Table4_201 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved,
            1 => Self::Rain,
            2 => Self::Thunderstorm,
            3 => Self::FreezingRain,
            4 => Self::MixedIce,
            5 => Self::Snow,
            6 => Self::WetSnow,
            7 => Self::MixtureOfRainAndSnow,
            8 => Self::IcePellets,
            9 => Self::Graupel,
            10 => Self::Hail,
            11 => Self::Drizzle,
            12 => Self::FreezingDrizzle,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_201 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved => "Reserved",
            Self::Rain => "Rain",
            Self::Thunderstorm => "Thunderstorm",
            Self::FreezingRain => "Freezing Rain",
            Self::MixedIce => "Mixed/Ice",
            Self::Snow => "Snow",
            Self::WetSnow => "Wet Snow",
            Self::MixtureOfRainAndSnow => "Mixture of Rain and Snow",
            Self::IcePellets => "Ice Pellets",
            Self::Graupel => "Graupel",
            Self::Hail => "Hail",
            Self::Drizzle => "Drizzle",
            Self::FreezingDrizzle => "Freezing Drizzle",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.202 - PRECIPITABLE WATER CATEGORY
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `0-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-202.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_202 {
    Missing = 255,
}
impl From<u8> for Grib2Table4_202 {
    fn from(_: u8) -> Self {
        Self::Missing
    }
}
impl core::fmt::Display for Grib2Table4_202 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.203 - CLOUD TYPE
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `21-190`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `191`: Unknown
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-203.shtml)
///
/// ## Notes
/// 1. Code figures `11-20` indicate all four layers were used and ground-based fog is below the lowest layer.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_203 {
    Clear = 0,
    Cumulonimbus = 1,
    Stratus = 2,
    Stratocumulus = 3,
    Cumulus = 4,
    Altostratus = 5,
    Nimbostratus = 6,
    Altocumulus = 7,
    Cirrostratus = 8,
    Cirrocumulus = 9,
    Cirrus = 10,
    CumulonimbusGroundBasedFog = 11,
    StratusGroundBasedFog = 12,
    StratocumulusGroundBasedFog = 13,
    CumulusGroundBasedFog = 14,
    AltostratusGroundBasedFog = 15,
    NimbostratusGroundBasedFog = 16,
    AltocumulusGroundBasedFog = 17,
    CirrostratusGroundBasedFog = 18,
    CirrocumulusGroundBasedFog = 19,
    CirrusGroundBasedFog = 20,
    Unknown = 191,
    Missing = 255,
}
impl From<u8> for Grib2Table4_203 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Clear,
            1 => Self::Cumulonimbus,
            2 => Self::Stratus,
            3 => Self::Stratocumulus,
            4 => Self::Cumulus,
            5 => Self::Altostratus,
            6 => Self::Nimbostratus,
            7 => Self::Altocumulus,
            8 => Self::Cirrostratus,
            9 => Self::Cirrocumulus,
            10 => Self::Cirrus,
            11 => Self::CumulonimbusGroundBasedFog,
            12 => Self::StratusGroundBasedFog,
            13 => Self::StratocumulusGroundBasedFog,
            14 => Self::CumulusGroundBasedFog,
            15 => Self::AltostratusGroundBasedFog,
            16 => Self::NimbostratusGroundBasedFog,
            17 => Self::AltocumulusGroundBasedFog,
            18 => Self::CirrostratusGroundBasedFog,
            19 => Self::CirrocumulusGroundBasedFog,
            20 => Self::CirrusGroundBasedFog,
            191 => Self::Unknown,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_203 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Clear => "Clear",
            Self::Cumulonimbus => "Cumulonimbus",
            Self::Stratus => "Stratus",
            Self::Stratocumulus => "Stratocumulus",
            Self::Cumulus => "Cumulus",
            Self::Altostratus => "Altostratus",
            Self::Nimbostratus => "Nimbostratus",
            Self::Altocumulus => "Altocumulus",
            Self::Cirrostratus => "Cirrostratus",
            Self::Cirrocumulus => "Cirrorcumulus",
            Self::Cirrus => "Cirrus",
            Self::CumulonimbusGroundBasedFog => {
                "Cumulonimbus - ground-based fog beneath the lowest layer"
            }
            Self::StratusGroundBasedFog => "Stratus - ground-based fog beneath the lowest layer",
            Self::StratocumulusGroundBasedFog => {
                "Stratocumulus - ground-based fog beneath the lowest layer"
            }
            Self::CumulusGroundBasedFog => "Cumulus - ground-based fog beneath the lowest layer",
            Self::AltostratusGroundBasedFog => {
                "Altostratus - ground-based fog beneath the lowest layer"
            }
            Self::NimbostratusGroundBasedFog => {
                "Nimbostratus - ground-based fog beneath the lowest layer"
            }
            Self::AltocumulusGroundBasedFog => {
                "Altocumulus - ground-based fog beneath the lowest layer"
            }
            Self::CirrostratusGroundBasedFog => {
                "Cirrostratus - ground-based fog beneath the lowest layer"
            }
            Self::CirrocumulusGroundBasedFog => {
                "Cirrorcumulus - ground-based fog beneath the lowest layer"
            }
            Self::CirrusGroundBasedFog => "Cirrus - ground-based fog beneath the lowest layer",
            Self::Unknown => "Unknown",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.204 - THUNDERSTORM COVERAGE
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `5-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-204.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_204 {
    None = 0,
    Isolated = 1,
    Few = 2,
    Scattered = 3,
    Numerous = 4,
    Missing = 255,
}
impl From<u8> for Grib2Table4_204 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::None,
            1 => Self::Isolated,
            2 => Self::Few,
            3 => Self::Scattered,
            4 => Self::Numerous,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_204 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::None => "None",
            Self::Isolated => "Isolated (1-2%)",
            Self::Few => "Few (3-5%)",
            Self::Scattered => "Scattered (16-45%)",
            Self::Numerous => "Numerous (>45%)",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.205 - PRESENCE OF AEROSOL
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-205.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_205 {
    AerosolNotPresent = 0,
    AerosolPresent = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table4_205 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::AerosolNotPresent,
            1 => Self::AerosolPresent,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_205 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::AerosolNotPresent => "Aerosol not present",
            Self::AerosolPresent => "Aerosol present",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.206 - VOLCANIC ASH
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-206.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_206 {
    NotPresent = 0,
    Present = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table4_206 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NotPresent,
            1 => Self::Present,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_206 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NotPresent => "Not Present",
            Self::Present => "Present",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.207 - ICING
///
/// **Details**:
/// - **Revised**: 04/22/2009
///
/// **Reserved Ranges**:
/// - `6-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-207.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_207 {
    None = 0,
    Light = 1,
    Moderate = 2,
    Severe = 3,
    Trace = 4,
    Heavy = 5,
    Missing = 255,
}
impl From<u8> for Grib2Table4_207 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::None,
            1 => Self::Light,
            2 => Self::Moderate,
            3 => Self::Severe,
            4 => Self::Trace,
            5 => Self::Heavy,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_207 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::None => "None",
            Self::Light => "Light",
            Self::Moderate => "Moderate",
            Self::Severe => "Severe",
            Self::Trace => "Trace",
            Self::Heavy => "Data missing",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.208 - TURBULENCE
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `5-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-208.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_208 {
    None = 0,
    Light = 1,
    Moderate = 2,
    Severe = 3,
    Extreme = 4,
    Missing = 255,
}
impl From<u8> for Grib2Table4_208 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::None,
            1 => Self::Light,
            2 => Self::Moderate,
            3 => Self::Severe,
            4 => Self::Extreme,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_208 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::None => "None",
            Self::Light => "Light",
            Self::Moderate => "Moderate",
            Self::Severe => "Severe",
            Self::Extreme => "Extreme",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.209 - PLANETARY BOUNDARY-LAYER REGIME
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `5-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-209.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_209 {
    Reserved = 0,
    Stable = 1,
    MechanicallyDrivenTurbulence = 2,
    ForceConvection = 3,
    FreeConvection = 4,
    Missing = 255,
}
impl From<u8> for Grib2Table4_209 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved,
            1 => Self::Stable,
            2 => Self::MechanicallyDrivenTurbulence,
            3 => Self::ForceConvection,
            4 => Self::FreeConvection,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_209 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved => "Reserved",
            Self::Stable => "Stable",
            Self::MechanicallyDrivenTurbulence => "Mechanically-Driven Turbulence",
            Self::ForceConvection => "Force Convection",
            Self::FreeConvection => "Free Convection",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.210 - CONTRAIL INTENSITY
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-210.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_210 {
    ContrailNotPresent = 0,
    ContrailPresent = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table4_210 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::ContrailNotPresent,
            1 => Self::ContrailPresent,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_210 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::ContrailNotPresent => "Contrail Not Present",
            Self::ContrailPresent => "Contrail Present",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.211 - CONTRAIL ENGINE TYPE
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `3-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-211.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_211 {
    LowBypass = 0,
    HighBypass = 1,
    NonBypass = 2,
    Missing = 255,
}
impl From<u8> for Grib2Table4_211 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::LowBypass,
            1 => Self::HighBypass,
            2 => Self::NonBypass,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_211 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::LowBypass => "Low Bypass",
            Self::HighBypass => "High Bypass",
            Self::NonBypass => "Non-Bypass",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.212 - LAND USE
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `14-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-212.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_212 {
    Reserved = 0,
    UrbanLand = 1,
    Agricultural = 2,
    RangeLand = 3,
    DeciduousForest = 4,
    ConiferousForest = 5,
    ForestWetland = 6,
    Water = 7,
    Wetlands = 8,
    Desert = 9,
    Tundra = 10,
    Ice = 11,
    TropicalForest = 12,
    Savannah = 13,
    Missing = 255,
}
impl From<u8> for Grib2Table4_212 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved,
            1 => Self::UrbanLand,
            2 => Self::Agricultural,
            3 => Self::RangeLand,
            4 => Self::DeciduousForest,
            5 => Self::ConiferousForest,
            6 => Self::ForestWetland,
            7 => Self::Water,
            8 => Self::Wetlands,
            9 => Self::Desert,
            10 => Self::Tundra,
            11 => Self::Ice,
            12 => Self::TropicalForest,
            13 => Self::Savannah,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_212 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved => "Reserved",
            Self::UrbanLand => "Urban Land",
            Self::Agricultural => "Agricultural",
            Self::RangeLand => "Range Land",
            Self::DeciduousForest => "Deciduous Forest",
            Self::ConiferousForest => "Coniferous Forest",
            Self::ForestWetland => "Forest/Wetland",
            Self::Water => "Water",
            Self::Wetlands => "Wetlands",
            Self::Desert => "Desert",
            Self::Tundra => "Tundra",
            Self::Ice => "Ice",
            Self::TropicalForest => "Tropical Forest",
            Self::Savannah => "Savannah",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.213 - SOIL TYPE
///
/// **Details**:
/// - **Revised**: 07/16/2013
///
/// **Reserved Ranges**:
/// - `12-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-213.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_213 {
    Reserved = 0,
    Sand = 1,
    LoamySand = 2,
    SandyLoam = 3,
    SiltLoam = 4,
    Organic = 5,
    SandyClayLoam = 6,
    SiltClayLoam = 7,
    ClayLoam = 8,
    SandyClay = 9,
    SiltyClay = 10,
    Clay = 11,
    Missing = 255,
}
impl From<u8> for Grib2Table4_213 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved,
            1 => Self::Sand,
            2 => Self::LoamySand,
            3 => Self::SandyLoam,
            4 => Self::SiltLoam,
            5 => Self::Organic,
            6 => Self::SandyClayLoam,
            7 => Self::SiltClayLoam,
            8 => Self::ClayLoam,
            9 => Self::SandyClay,
            10 => Self::SiltyClay,
            11 => Self::Clay,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_213 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved => "Reserved",
            Self::Sand => "Sand",
            Self::LoamySand => "Loamy Sand",
            Self::SandyLoam => "Sandy Loam",
            Self::SiltLoam => "Silt Loam",
            Self::Organic => "Organic",
            Self::SandyClayLoam => "Sandy Clay Loam",
            Self::SiltClayLoam => "Silt Clay Loam",
            Self::ClayLoam => "Clay Loam",
            Self::SandyClay => "Sandy Clay",
            Self::SiltyClay => "Silty Clay",
            Self::Clay => "Clay",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.214 - ENVIRONMENTAL FACTOR QUALIFIER
///
/// **Details**:
/// - **Created**: 10/24/2023
///
/// **Reserved Ranges**:
/// - `6-190`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `191`: Unknown
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-214.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_214 {
    Worst = 0,
    VeryPoor = 1,
    Poor = 2,
    Average = 3,
    Good = 4,
    Excellent = 5,
    Unknown = 191,
    Missing = 255,
}
impl From<u8> for Grib2Table4_214 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Worst,
            1 => Self::VeryPoor,
            2 => Self::Poor,
            3 => Self::Average,
            4 => Self::Good,
            5 => Self::Excellent,
            191 => Self::Unknown,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_214 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Worst => "Worst",
            Self::VeryPoor => "Very poor",
            Self::Poor => "Poor",
            Self::Average => "Average",
            Self::Good => "Good",
            Self::Excellent => "Excellent",
            Self::Unknown => "Unknown",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.215 - REMOTELY-SENSED SNOW COVERAGE
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `0-49`: Reserved
/// - `51-99`: Reserved
/// - `101-249`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `50`: No-Snow/No-Cloud
/// - `100`: Clouds
/// - `250`: Snow
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-215.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_215 {
    NoSnowNoCloud = 50,
    Clouds = 100,
    Snow = 250,
    Missing = 255,
}
impl From<u8> for Grib2Table4_215 {
    fn from(val: u8) -> Self {
        match val {
            50 => Self::NoSnowNoCloud,
            100 => Self::Clouds,
            250 => Self::Snow,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_215 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NoSnowNoCloud => "No-Snow/No-Cloud",
            Self::Clouds => "Clouds",
            Self::Snow => "Snow",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.216 - ELEVATION OF SNOW COVERED TERRAIN
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `91-253`: Reserved
///
/// **Special Values**:
/// - `0-90`: Elevation in increments of 100 m
/// - `254`: Clouds
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-216.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_216 {
    Elevation100m = 0, // This represents values 0-90
    Clouds = 254,
    Missing = 255,
}
impl From<u8> for Grib2Table4_216 {
    fn from(val: u8) -> Self {
        match val {
            0..=90 => Self::Elevation100m,
            254 => Self::Clouds,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_216 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Elevation100m => "Elevation in increments of 100 m",
            Self::Clouds => "Clouds",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.217 - CLOUD MASK TYPE
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `4-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-217.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_217 {
    ClearOverWater = 0,
    ClearOverLand = 1,
    Cloud = 2,
    NoData = 3,
    Missing = 255,
}
impl From<u8> for Grib2Table4_217 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::ClearOverWater,
            1 => Self::ClearOverLand,
            2 => Self::Cloud,
            3 => Self::NoData,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_217 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::ClearOverWater => "Clear over water",
            Self::ClearOverLand => "Clear over land",
            Self::Cloud => "Cloud",
            Self::NoData => "No data",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.218 - PIXEL SCENE TYPE
///
/// **Details**:
/// - **Revised**: 05/29/2019
///
/// **Reserved Ranges**:
/// - `25-96`: Reserved
/// - `113-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-218.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_218 {
    NoSceneIdentified = 0,
    GreenNeedleLeafedForest = 1,
    GreenBroadLeafedForest = 2,
    DeciduousNeedleLeafedForest = 3,
    DeciduousBroadLeafedForest = 4,
    DeciduousMixedForest = 5,
    ClosedShrubLand = 6,
    OpenShrubLand = 7,
    WoodySavannah = 8,
    Savannah = 9,
    Grassland = 10,
    PermanentWetland = 11,
    Cropland = 12,
    Urban = 13,
    VegetationCrops = 14,
    PermanentSnowIce = 15,
    BarrenDesert = 16,
    WaterBodies = 17,
    Tundra = 18,
    WarmLiquidWaterCloud = 19,
    SupercooledLiquidWaterCloud = 20,
    MixedPhaseCloud = 21,
    OpticallyThinIceCloud = 22,
    OpticallyThickIceCloud = 23,
    MultiLayerBlackCloud = 24,
    SnowIceOnLand = 97,
    SnowIceOnWater = 98,
    SunGlint = 99,
    GeneralCloud = 100,
    LowCloudFogStratus = 101,
    LowCloudStratocumulus = 102,
    LowCloudUnknownType = 103,
    MediumCloudNimbostratus = 104,
    MediumCloudAltostratus = 105,
    MediumCloudUnknownType = 106,
    HighCloudCumulus = 107,
    HighCloudCirrus = 108,
    HighCloudUnknownType = 109,
    UnknownCloudType = 110,
    SingleLayerWaterCloud = 111,
    SingleLayerIceCloud = 112,
    Missing = 255,
}
impl From<u8> for Grib2Table4_218 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoSceneIdentified,
            1 => Self::GreenNeedleLeafedForest,
            2 => Self::GreenBroadLeafedForest,
            3 => Self::DeciduousNeedleLeafedForest,
            4 => Self::DeciduousBroadLeafedForest,
            5 => Self::DeciduousMixedForest,
            6 => Self::ClosedShrubLand,
            7 => Self::OpenShrubLand,
            8 => Self::WoodySavannah,
            9 => Self::Savannah,
            10 => Self::Grassland,
            11 => Self::PermanentWetland,
            12 => Self::Cropland,
            13 => Self::Urban,
            14 => Self::VegetationCrops,
            15 => Self::PermanentSnowIce,
            16 => Self::BarrenDesert,
            17 => Self::WaterBodies,
            18 => Self::Tundra,
            19 => Self::WarmLiquidWaterCloud,
            20 => Self::SupercooledLiquidWaterCloud,
            21 => Self::MixedPhaseCloud,
            22 => Self::OpticallyThinIceCloud,
            23 => Self::OpticallyThickIceCloud,
            24 => Self::MultiLayerBlackCloud,
            97 => Self::SnowIceOnLand,
            98 => Self::SnowIceOnWater,
            99 => Self::SunGlint,
            100 => Self::GeneralCloud,
            101 => Self::LowCloudFogStratus,
            102 => Self::LowCloudStratocumulus,
            103 => Self::LowCloudUnknownType,
            104 => Self::MediumCloudNimbostratus,
            105 => Self::MediumCloudAltostratus,
            106 => Self::MediumCloudUnknownType,
            107 => Self::HighCloudCumulus,
            108 => Self::HighCloudCirrus,
            109 => Self::HighCloudUnknownType,
            110 => Self::UnknownCloudType,
            111 => Self::SingleLayerWaterCloud,
            112 => Self::SingleLayerIceCloud,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_218 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NoSceneIdentified => "No Scene Identified",
            Self::GreenNeedleLeafedForest => "Green Needle-Leafed Forest",
            Self::GreenBroadLeafedForest => "Green Broad-Leafed Forest",
            Self::DeciduousNeedleLeafedForest => "Deciduous Needle-Leafed Forest",
            Self::DeciduousBroadLeafedForest => "Deciduous Broad-Leafed Forest",
            Self::DeciduousMixedForest => "Deciduous Mixed Forest",
            Self::ClosedShrubLand => "Closed Shrub-Land",
            Self::OpenShrubLand => "Open Shrub-Land",
            Self::WoodySavannah => "Woody Savannah",
            Self::Savannah => "Savannah",
            Self::Grassland => "Grassland",
            Self::PermanentWetland => "Permanent Wetland",
            Self::Cropland => "Cropland",
            Self::Urban => "Urban",
            Self::VegetationCrops => "Vegetation / Crops",
            Self::PermanentSnowIce => "Permanent Snow / Ice",
            Self::BarrenDesert => "Barren Desert",
            Self::WaterBodies => "Water Bodies",
            Self::Tundra => "Tundra",
            Self::WarmLiquidWaterCloud => "Warm Liquid Water Cloud",
            Self::SupercooledLiquidWaterCloud => "Supercooled Liquid Water Cloud",
            Self::MixedPhaseCloud => "Mixed Phase Cloud",
            Self::OpticallyThinIceCloud => "Optically Thin Ice Cloud",
            Self::OpticallyThickIceCloud => "Optically Thick Ice Cloud",
            Self::MultiLayerBlackCloud => "Multi-Layeblack Cloud",
            Self::SnowIceOnLand => "Snow / Ice on Land",
            Self::SnowIceOnWater => "Snow / Ice on Water",
            Self::SunGlint => "Sun-Glint",
            Self::GeneralCloud => "General Cloud",
            Self::LowCloudFogStratus => "Low Cloud / Fog / Stratus",
            Self::LowCloudStratocumulus => "Low Cloud / Stratocumulus",
            Self::LowCloudUnknownType => "Low Cloud / Unknown Type",
            Self::MediumCloudNimbostratus => "Medium Cloud / Nimbostratus",
            Self::MediumCloudAltostratus => "Medium Cloud / Altostratus",
            Self::MediumCloudUnknownType => "Medium Cloud / Unknown Type",
            Self::HighCloudCumulus => "High Cloud / Cumulus",
            Self::HighCloudCirrus => "High Cloud / Cirrus",
            Self::HighCloudUnknownType => "High Cloud / Unknown Type",
            Self::UnknownCloudType => "Unknown Cloud Type",
            Self::SingleLayerWaterCloud => "Single layer water cloud",
            Self::SingleLayerIceCloud => "Single layer ice cloud",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.219 - CLOUD TOP HEIGHT QUALITY INDICATOR
///
/// **Details**:
/// - **Created**: 12/07/2010
///
/// **Reserved Ranges**:
/// - `4-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-219.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_219 {
    NominalCloudTopHeightQuality = 0,
    FogInSegment = 1,
    PoorQualityHeightEstimation = 2,
    FogInSegmentAndPoorQualityHeightEstimation = 3,
    Missing = 255,
}
impl From<u8> for Grib2Table4_219 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NominalCloudTopHeightQuality,
            1 => Self::FogInSegment,
            2 => Self::PoorQualityHeightEstimation,
            3 => Self::FogInSegmentAndPoorQualityHeightEstimation,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_219 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NominalCloudTopHeightQuality => "Nominal Cloud Top Height Quality",
            Self::FogInSegment => "Fog In Segment",
            Self::PoorQualityHeightEstimation => "Poor Quality Height Estimation",
            Self::FogInSegmentAndPoorQualityHeightEstimation => {
                "Fog In Segment and Poor Quality Height Estimation"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.220 - HORIZONTAL DIMENSION PROCESSED
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-220.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_220 {
    Latitude = 0,
    Longitude = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table4_220 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Latitude,
            1 => Self::Longitude,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_220 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Latitude => "Latitude",
            Self::Longitude => "Longitude",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.221 - TREATMENT OF MISSING DATA
///
/// **Details**:
/// - **Created**: 05/16/2005
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-221.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_221 {
    NotIncluded = 0,
    Extrapolated = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table4_221 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NotIncluded,
            1 => Self::Extrapolated,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_221 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NotIncluded => "Not included",
            Self::Extrapolated => "Extrapolated",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.222 - CATEGORICAL RESULT
///
/// **Details**:
/// - **Revised**: 07/16/2013
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-222.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_222 {
    No = 0,
    Yes = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table4_222 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::No,
            1 => Self::Yes,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_222 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::No => "No",
            Self::Yes => "Yes",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.223 - FIRE DETECTION INDICATOR
///
/// **Details**:
/// - **Created**: 11/05/2007
///
/// **Reserved Ranges**:
/// - `4-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-223.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_223 {
    NoFireDetected = 0,
    PossibleFireDetected = 1,
    ProbableFireDetected = 2,
    MissingCode = 3, // Renamed to avoid conflict with the enum's Missing variant
    Missing = 255,
}
impl From<u8> for Grib2Table4_223 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoFireDetected,
            1 => Self::PossibleFireDetected,
            2 => Self::ProbableFireDetected,
            3 => Self::MissingCode,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_223 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NoFireDetected => "No Fire Detected",
            Self::PossibleFireDetected => "Possible Fire Detected",
            Self::ProbableFireDetected => "Probable Fire Detected",
            Self::MissingCode => "Missing",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.224 - CATEGORICAL OUTLOOK
///
/// **Details**:
/// - **Created**: 12/21/2011
///
/// **Reserved Ranges**:
/// - `1`: Reserved
/// - `3`: Reserved
/// - `5`: Reserved
/// - `7`: Reserved
/// - `9-10`: Reserved
/// - `12-13`: Reserved
/// - `15-17`: Reserved
/// - `19-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-224.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_224 {
    NoRiskArea = 0,
    GeneralThunderstormRiskArea = 2,
    SlightRiskArea = 4,
    ModerateRiskArea = 6,
    HighRiskArea = 8,
    DryThunderstormRiskArea = 11,
    CriticalRiskArea = 14,
    ExtremelyCriticalRiskArea = 18,
    Missing = 255,
}
impl From<u8> for Grib2Table4_224 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoRiskArea,
            2 => Self::GeneralThunderstormRiskArea,
            4 => Self::SlightRiskArea,
            6 => Self::ModerateRiskArea,
            8 => Self::HighRiskArea,
            11 => Self::DryThunderstormRiskArea,
            14 => Self::CriticalRiskArea,
            18 => Self::ExtremelyCriticalRiskArea,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_224 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NoRiskArea => "No Risk Area",
            Self::GeneralThunderstormRiskArea => "General Thunderstorm Risk Area",
            Self::SlightRiskArea => "Slight Risk Area",
            Self::ModerateRiskArea => "Moderate Risk Area",
            Self::HighRiskArea => "High Risk Area",
            Self::DryThunderstormRiskArea => "Dry Thunderstorm (Dry Lightning) Risk Area",
            Self::CriticalRiskArea => "Critical Risk Area",
            Self::ExtremelyCriticalRiskArea => "Extremely Critical Risk Area",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

// TODO: GRIB2 - CODE TABLE 4.225 (https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-225.shtml)

/// # GRIB2 - CODE TABLE 4.227 - ICING SCENARIO (Weather/Cloud Classification)
///
/// **Details**:
/// - **Created**: 04/09/2013
///
/// **Reserved Ranges**:
/// - `5-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-227.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_227 {
    None = 0,
    General = 1,
    Convective = 2,
    Stratiform = 3,
    Freezing = 4,
    Missing = 255,
}
impl From<u8> for Grib2Table4_227 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::None,
            1 => Self::General,
            2 => Self::Convective,
            3 => Self::Stratiform,
            4 => Self::Freezing,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_227 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::None => "None",
            Self::General => "General",
            Self::Convective => "Convective",
            Self::Stratiform => "Stratiform",
            Self::Freezing => "Freezing",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.228 - ICING SEVERITY
///
/// **Details**:
/// - **Created**: 01/19/2022
///
/// **Reserved Ranges**:
/// - `6-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-228.shtml)
///
/// ## Notes
/// None.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_228 {
    None = 0,
    Trace = 1,
    Light = 2,
    Moderate = 3,
    Severe = 4,
    Missing = 255,
}
impl From<u8> for Grib2Table4_228 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::None,
            1 => Self::Trace,
            2 => Self::Light,
            3 => Self::Moderate,
            4 => Self::Severe,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_228 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::None => "None",
            Self::Trace => "Trace",
            Self::Light => "Light",
            Self::Moderate => "Moderate",
            Self::Severe => "Severe",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # GRIB2 - CODE TABLE 4.230 - ATMOSPHERIC CHEMICAL OR PHYSICAL CONSTITUENT TYPE
///
/// **Details**:
/// - **Revised**: 04/12/2022
///
/// **Reserved Ranges**:
/// - `39-9999`: Reserved
/// - `10003`: Reserved
/// - `10024-10499`: Reserved
/// - `10501-20000`: Reserved
/// - `20022-29999`: Reserved
/// - `30001-50000`: Reserved
/// - `60017-61999`: Reserved
/// - `62035-65534`: Reserved
///
/// **Special Value**:
/// - `65535`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-230.shtml)
/// - [More data...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/WMO306_vI2_CommonTable_en_v23.0.0.pdf)
#[repr(u16)] // Use u16 for values up to 65535
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_230 {
    Ozone = 0,
    WaterVapour = 1,
    Methane = 2,
    CarbonDioxide = 3,
    CarbonMonoxide = 4,
    NitrogenDioxide = 5,
    NitrousOxide = 6,
    Formaldehyde = 7,
    SulphurDioxide = 8,
    Ammonia = 9,
    Ammonium = 10,
    NitrogenMonoxide = 11,
    AtomicOxygen = 12,
    NitrateRadical = 13,
    HydroperoxylRadical = 14,
    DinitrogenPentoxide = 15,
    NitrousAcid = 16,
    NitricAcid = 17,
    PeroxynitricAcid = 18,
    HydrogenPeroxide = 19,
    MolecularHydrogen = 20,
    AtomicNitrogen = 21,
    Sulphate = 22,
    Radon = 23,
    ElementalMercury = 24,
    DivalentMercury = 25,
    AtomicChlorine = 26,
    ChlorineMonoxide = 27,
    DichlorinePeroxide = 28,
    HypochlorousAcid = 29,
    ChlorineNitrate = 30,
    ChlorineDioxide = 31,
    AtomicBromide = 32,
    BromineMonoxide = 33,
    BromineChloride = 34,
    HydrogenBromide = 35,
    HypobromousAcid = 36,
    BromineNitrate = 37,
    Oxygen = 38,
    HydroxylRadical = 10000,
    MethylPeroxyRadical = 10001,
    MethylHydroperoxide = 10002,
    Methanol = 10004,
    FormicAcid = 10005,
    HydrogenCyanide = 10006,
    AcetoNitrile = 10007,
    Ethane = 10008,
    Ethene = 10009,
    Ethyne = 10010,
    Ethanol = 10011,
    AceticAcid = 10012,
    PeroxyacetylNitrate = 10013,
    Propane = 10014,
    Propene = 10015,
    Butanes = 10016,
    Isoprene = 10017,
    AlphaPinene = 10018,
    BetaPinene = 10019,
    Limonene = 10020,
    Benzene = 10021,
    Toluene = 10022,
    Xylene = 10023,
    DimethylSulphide = 10500,
    HydrogenChloride = 20001,
    CFC11 = 20002,
    CFC12 = 20003,
    CFC113 = 20004,
    CFC113a = 20005,
    CFC114 = 20006,
    CFC115 = 20007,
    HCFC22 = 20008,
    HCFC141b = 20009,
    HCFC142b = 20010,
    Halon1202 = 20011,
    Halon1211 = 20012,
    Halon1301 = 20013,
    Halon2402 = 20014,
    MethylChloride = 20015,
    CarbonTetrachloride = 20016,
    HCC140a = 20017,
    MethylBromide = 20018,
    Hexachlorocyclohexane = 20019,
    AlphaHexachlorocyclohexane = 20020,
    Hexachlorobiphenyl = 20021,
    RadioactivePollutant = 30000,
    HOxRadical = 60000,
    TotalInorganicAndOrganicPeroxyRadicals = 60001,
    PassiveOzone = 60002,
    NOxExpressedAsNitrogen = 60003,
    AllNitrogenOxides = 60004,
    TotalInorganicChlorine = 60005,
    TotalInorganicBromine = 60006,
    TotalInorganicChlorineExceptHClClONO2 = 60007,
    TotalInorganicBromineExceptHBrBrONO2 = 60008,
    LumpedAlkanes = 60009,
    LumpedAlkenes = 60010,
    LumpedAromaticCompounds = 60011,
    LumpedTerpenes = 60012,
    NonMethaneVolatileOrganicCompounds = 60013,
    AnthropogenicNonMethaneVolatileOrganicCompounds = 60014,
    BiogenicNonMethaneVolatileOrganicCompounds = 60015,
    LumpedOxygenatedHydrocarbons = 60016,
    TotalAerosol = 62000,
    DustDry = 62001,
    WaterInAmbient = 62002,
    AmmoniumDry = 62003,
    NitrateDry = 62004,
    NitricAcidTrihydrate = 62005,
    SulphateDry = 62006,
    MercuryDry = 62007,
    SeaSaltDry = 62008,
    BlackCarbonDry = 62009,
    ParticulateOrganicMatterDry = 62010,
    PrimaryParticulateOrganicMatterDry = 62011,
    SecondaryParticulateOrganicMatterDry = 62012,
    BrownCarbonDry = 62034,
    Missing = 65535,
}
impl From<u16> for Grib2Table4_230 {
    fn from(val: u16) -> Self {
        match val {
            0 => Self::Ozone,
            1 => Self::WaterVapour,
            2 => Self::Methane,
            3 => Self::CarbonDioxide,
            4 => Self::CarbonMonoxide,
            5 => Self::NitrogenDioxide,
            6 => Self::NitrousOxide,
            7 => Self::Formaldehyde,
            8 => Self::SulphurDioxide,
            9 => Self::Ammonia,
            10 => Self::Ammonium,
            11 => Self::NitrogenMonoxide,
            12 => Self::AtomicOxygen,
            13 => Self::NitrateRadical,
            14 => Self::HydroperoxylRadical,
            15 => Self::DinitrogenPentoxide,
            16 => Self::NitrousAcid,
            17 => Self::NitricAcid,
            18 => Self::PeroxynitricAcid,
            19 => Self::HydrogenPeroxide,
            20 => Self::MolecularHydrogen,
            21 => Self::AtomicNitrogen,
            22 => Self::Sulphate,
            23 => Self::Radon,
            24 => Self::ElementalMercury,
            25 => Self::DivalentMercury,
            26 => Self::AtomicChlorine,
            27 => Self::ChlorineMonoxide,
            28 => Self::DichlorinePeroxide,
            29 => Self::HypochlorousAcid,
            30 => Self::ChlorineNitrate,
            31 => Self::ChlorineDioxide,
            32 => Self::AtomicBromide,
            33 => Self::BromineMonoxide,
            34 => Self::BromineChloride,
            35 => Self::HydrogenBromide,
            36 => Self::HypobromousAcid,
            37 => Self::BromineNitrate,
            38 => Self::Oxygen,
            10000 => Self::HydroxylRadical,
            10001 => Self::MethylPeroxyRadical,
            10002 => Self::MethylHydroperoxide,
            10004 => Self::Methanol,
            10005 => Self::FormicAcid,
            10006 => Self::HydrogenCyanide,
            10007 => Self::AcetoNitrile,
            10008 => Self::Ethane,
            10009 => Self::Ethene,
            10010 => Self::Ethyne,
            10011 => Self::Ethanol,
            10012 => Self::AceticAcid,
            10013 => Self::PeroxyacetylNitrate,
            10014 => Self::Propane,
            10015 => Self::Propene,
            10016 => Self::Butanes,
            10017 => Self::Isoprene,
            10018 => Self::AlphaPinene,
            10019 => Self::BetaPinene,
            10020 => Self::Limonene,
            10021 => Self::Benzene,
            10022 => Self::Toluene,
            10023 => Self::Xylene,
            10500 => Self::DimethylSulphide,
            20001 => Self::HydrogenChloride,
            20002 => Self::CFC11,
            20003 => Self::CFC12,
            20004 => Self::CFC113,
            20005 => Self::CFC113a,
            20006 => Self::CFC114,
            20007 => Self::CFC115,
            20008 => Self::HCFC22,
            20009 => Self::HCFC141b,
            20010 => Self::HCFC142b,
            20011 => Self::Halon1202,
            20012 => Self::Halon1211,
            20013 => Self::Halon1301,
            20014 => Self::Halon2402,
            20015 => Self::MethylChloride,
            20016 => Self::CarbonTetrachloride,
            20017 => Self::HCC140a,
            20018 => Self::MethylBromide,
            20019 => Self::Hexachlorocyclohexane,
            20020 => Self::AlphaHexachlorocyclohexane,
            20021 => Self::Hexachlorobiphenyl,
            30000 => Self::RadioactivePollutant,
            60000 => Self::HOxRadical,
            60001 => Self::TotalInorganicAndOrganicPeroxyRadicals,
            60002 => Self::PassiveOzone,
            60003 => Self::NOxExpressedAsNitrogen,
            60004 => Self::AllNitrogenOxides,
            60005 => Self::TotalInorganicChlorine,
            60006 => Self::TotalInorganicBromine,
            60007 => Self::TotalInorganicChlorineExceptHClClONO2,
            60008 => Self::TotalInorganicBromineExceptHBrBrONO2,
            60009 => Self::LumpedAlkanes,
            60010 => Self::LumpedAlkenes,
            60011 => Self::LumpedAromaticCompounds,
            60012 => Self::LumpedTerpenes,
            60013 => Self::NonMethaneVolatileOrganicCompounds,
            60014 => Self::AnthropogenicNonMethaneVolatileOrganicCompounds,
            60015 => Self::BiogenicNonMethaneVolatileOrganicCompounds,
            60016 => Self::LumpedOxygenatedHydrocarbons,
            62000 => Self::TotalAerosol,
            62001 => Self::DustDry,
            62002 => Self::WaterInAmbient,
            62003 => Self::AmmoniumDry,
            62004 => Self::NitrateDry,
            62005 => Self::NitricAcidTrihydrate,
            62006 => Self::SulphateDry,
            62007 => Self::MercuryDry,
            62008 => Self::SeaSaltDry,
            62009 => Self::BlackCarbonDry,
            62010 => Self::ParticulateOrganicMatterDry,
            62011 => Self::PrimaryParticulateOrganicMatterDry,
            62012 => Self::SecondaryParticulateOrganicMatterDry,
            62034 => Self::BrownCarbonDry,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_230 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Ozone => "Ozone - O3",
            Self::WaterVapour => "Water Vapour - H2O",
            Self::Methane => "Methane - CH4",
            Self::CarbonDioxide => "Carbon Dioxide - CO2",
            Self::CarbonMonoxide => "Carbon Monoxide - CO",
            Self::NitrogenDioxide => "Nitrogen Dioxide - NO2",
            Self::NitrousOxide => "Nitrous Oxide - N2O",
            Self::Formaldehyde => "Formaldehyde - HCHO",
            Self::SulphurDioxide => "Sulphur Dioxide - SO2",
            Self::Ammonia => "Ammonia - NH3",
            Self::Ammonium => "Ammonium - NH4+",
            Self::NitrogenMonoxide => "Nitrogen Monoxide - NO",
            Self::AtomicOxygen => "Atomic Oxygen - O",
            Self::NitrateRadical => "Nitrate Radical - NO3",
            Self::HydroperoxylRadical => "Hydroperoxyl Radical - HO2",
            Self::DinitrogenPentoxide => "Dinitrogen Pentoxide - H2O5",
            Self::NitrousAcid => "Nitrous Acid - HONO",
            Self::NitricAcid => "Nitric Acid - HNO3",
            Self::PeroxynitricAcid => "Peroxynitric Acid - HO2NO2",
            Self::HydrogenPeroxide => "Hydrogen Peroxide - H2O2",
            Self::MolecularHydrogen => "Molecular Hydrogen - H",
            Self::AtomicNitrogen => "Atomic Nitrogen - N",
            Self::Sulphate => "Sulphate - SO42-",
            Self::Radon => "Radon - Rn",
            Self::ElementalMercury => "Elemental Mercury - Hg(O)",
            Self::DivalentMercury => "Divalent Mercury - Hg2+",
            Self::AtomicChlorine => "Atomic Chlorine - Cl",
            Self::ChlorineMonoxide => "Chlorine Monoxide - ClO",
            Self::DichlorinePeroxide => "Dichlorine Peroxide - Cl2O2",
            Self::HypochlorousAcid => "Hypochlorous Acid - HClO",
            Self::ChlorineNitrate => "Chlorine Nitrate - ClONO2",
            Self::ChlorineDioxide => "Chlorine Dioxide - ClO2",
            Self::AtomicBromide => "Atomic Bromide - Br",
            Self::BromineMonoxide => "Bromine Monoxide - BrO",
            Self::BromineChloride => "Bromine Chloride - BrCl",
            Self::HydrogenBromide => "Hydrogen Bromide - HBr",
            Self::HypobromousAcid => "Hypobromous Acid - HBrO",
            Self::BromineNitrate => "Bromine Nitrate - BrONO2",
            Self::Oxygen => "Oxygen - O2",
            Self::HydroxylRadical => "Hydroxyl Radical - OH",
            Self::MethylPeroxyRadical => "Methyl Peroxy Radical - CH3O2",
            Self::MethylHydroperoxide => "Methyl Hydroperoxide - CH3O2H",
            Self::Methanol => "Methanol - CH3OH",
            Self::FormicAcid => "Formic Acid - CH3OOH",
            Self::HydrogenCyanide => "Hydrogen Cyanide - HCN",
            Self::AcetoNitrile => "Aceto Nitrile - CH3CN",
            Self::Ethane => "Ethane - C2H6",
            Self::Ethene => "Ethene (Ethylene) - C2H4",
            Self::Ethyne => "Ethyne (Acetylene) - C2H2",
            Self::Ethanol => "Ethanol - C2H5OH",
            Self::AceticAcid => "Acetic Acid - C2H5OOH",
            Self::PeroxyacetylNitrate => "Peroxyacetyl Nitrate - CH3C(O)OONO2",
            Self::Propane => "Propane - C3H8",
            Self::Propene => "Propene - C3H6",
            Self::Butanes => "Butanes - C4H10",
            Self::Isoprene => "Isoprene - C5H10",
            Self::AlphaPinene => "Alpha Pinene - C10H16",
            Self::BetaPinene => "Beta Pinene - C10H16",
            Self::Limonene => "Limonene - C10H16",
            Self::Benzene => "Benzene - C6H6",
            Self::Toluene => "Toluene - C7H8",
            Self::Xylene => "Xylene - C8H10",
            Self::DimethylSulphide => "Dimethyl Sulphide - CH3SCH3",
            Self::HydrogenChloride => "Hydrogen Chloride - HCL",
            Self::CFC11 => "CFC-11",
            Self::CFC12 => "CFC-12",
            Self::CFC113 => "CFC-113",
            Self::CFC113a => "CFC-113a",
            Self::CFC114 => "CFC-114",
            Self::CFC115 => "CFC-115",
            Self::HCFC22 => "HCFC-22",
            Self::HCFC141b => "HCFC-141b",
            Self::HCFC142b => "HCFC-142b",
            Self::Halon1202 => "Halon-1202",
            Self::Halon1211 => "Halon-1211",
            Self::Halon1301 => "Halon-1301",
            Self::Halon2402 => "Halon-2402",
            Self::MethylChloride => "Methyl Chloride (HCC-40)",
            Self::CarbonTetrachloride => "Carbon Tetrachloride (HCC-10)",
            Self::HCC140a => "HCC-140a - CH3CCl3",
            Self::MethylBromide => "Methyl Bromide (HBC-40B1)",
            Self::Hexachlorocyclohexane => "Hexachlorocyclohexane (HCH)",
            Self::AlphaHexachlorocyclohexane => "Alpha Hexachlorocyclohexane",
            Self::Hexachlorobiphenyl => "Hexachlorobiphenyl (PCB-153)",
            Self::RadioactivePollutant => {
                "Radioactive Pollutant (Tracer, defined by originating centre)"
            }
            Self::HOxRadical => "HOx Radical (OH+HO2)",
            Self::TotalInorganicAndOrganicPeroxyRadicals => {
                "Total Inorganic and Organic Peroxy Radicals (HO2+RO2) - RO2"
            }
            Self::PassiveOzone => "Passive Ozone",
            Self::NOxExpressedAsNitrogen => "NOx Expressed As Nitrogen - NOx",
            Self::AllNitrogenOxides => "All Nitrogen Oxides (NOy) Expressed As Nitrogen - NOy",
            Self::TotalInorganicChlorine => "Total Inorganic Chlorine - Clx",
            Self::TotalInorganicBromine => "Total Inorganic Bromine - Brx",
            Self::TotalInorganicChlorineExceptHClClONO2 => {
                "Total Inorganic Chlorine Except HCl, ClONO2: ClOx"
            }
            Self::TotalInorganicBromineExceptHBrBrONO2 => {
                "Total Inorganic Bromine Except HBr, BrONO2: BrOx"
            }
            Self::LumpedAlkanes => "Lumped Alkanes",
            Self::LumpedAlkenes => "Lumped Alkenes",
            Self::LumpedAromaticCompounds => "Lumped Aromatic Compounds",
            Self::LumpedTerpenes => "Lumped Terpenes",
            Self::NonMethaneVolatileOrganicCompounds => {
                "Non-Methane Volatile Organic Compounds Expressed as Carbon - NMVOC"
            }
            Self::AnthropogenicNonMethaneVolatileOrganicCompounds => {
                "Anthropogenic Non-Methane Volatile Organic Compounds Expressed as Carbon - aNMVOC"
            }
            Self::BiogenicNonMethaneVolatileOrganicCompounds => {
                "Biogenic Non-Methane Volatile Organic Compounds Expressed as Carbon - bNMVOC"
            }
            Self::LumpedOxygenatedHydrocarbons => "Lumped Oxygenated Hydrocarbons - OVOC",
            Self::TotalAerosol => "Total Aerosol",
            Self::DustDry => "Dust Dry",
            Self::WaterInAmbient => "Water In Ambient",
            Self::AmmoniumDry => "Ammonium Dry",
            Self::NitrateDry => "Nitrate Dry",
            Self::NitricAcidTrihydrate => "Nitric Acid Trihydrate",
            Self::SulphateDry => "Sulphate Dry",
            Self::MercuryDry => "Mercury Dry",
            Self::SeaSaltDry => "Sea Salt Dry",
            Self::BlackCarbonDry => "Black Carbon Dry",
            Self::ParticulateOrganicMatterDry => "Particulate Organic Matter Dry",
            Self::PrimaryParticulateOrganicMatterDry => "Primary Particulate Organic Matter Dry",
            Self::SecondaryParticulateOrganicMatterDry => {
                "Secondary Particulate Organic Matter Dry"
            }
            Self::BrownCarbonDry => "Brown Carbon Dry",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.233: AEROSOL TYPE
///
/// **Created**: 05/16/2005
/// **Revised**: 07/18/2022
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-233.shtml)
/// - [More data...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/WMO306_vI2_CommonTable_en_v23.0.0.pdf)
///
/// ## Notes
/// Red text depicts changes made since 05/29/2019.
#[repr(u16)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_233 {
    Ozone = 0,
    WaterVapour = 1,
    Methane = 2,
    CarbonDioxide = 3,
    CarbonMonoxide = 4,
    NitrogenDioxide = 5,
    NitrousOxide = 6,
    Formaldehyde = 7,
    SulphurDioxide = 8,
    Ammonia = 9,
    Ammonium = 10,
    NitrogenMonoxide = 11,
    AtomicOxygen = 12,
    NitrateRadical = 13,
    HydroperoxylRadical = 14,
    DinitrogenPentoxide = 15,
    NitrousAcid = 16,
    NitricAcid = 17,
    PeroxynitricAcid = 18,
    HydrogenPeroxide = 19,
    MolecularHydrogen = 20,
    AtomicNitrogen = 21,
    Sulphate = 22,
    Radon = 23,
    ElementalMercury = 24,
    DivalentMercury = 25,
    AtomicChlorine = 26,
    ChlorineMonoxide = 27,
    DichlorinePeroxide = 28,
    HypochlorousAcid = 29,
    ChlorineNitrate = 30,
    ChlorineDioxide = 31,
    AtomicBromide = 32,
    BromineMonoxide = 33,
    BromineChloride = 34,
    HydrogenBromide = 35,
    HypobromousAcid = 36,
    BromineNitrate = 37,
    Oxygen = 38,
    Reserved39 = 39,
    HydroxylRadical = 10000,
    MethylPeroxyRadical = 10001,
    MethylHydroperoxide = 10002,
    Reserved10003 = 10003,
    Methanol = 10004,
    FormicAcid = 10005,
    HydrogenCyanide = 10006,
    AcetoNitrile = 10007,
    Ethane = 10008,
    Ethene = 10009,
    Ethyne = 10010,
    Ethanol = 10011,
    AceticAcid = 10012,
    PeroxyacetylNitrate = 10013,
    Propane = 10014,
    Propene = 10015,
    Butanes = 10016,
    Isoprene = 10017,
    AlphaPinene = 10018,
    BetaPinene = 10019,
    Limonene = 10020,
    Benzene = 10021,
    Toluene = 10022,
    Xylene = 10023,
    Reserved10024 = 10024,
    DimethylSulphide = 10500,
    HydrogenChloride = 20001,
    CFC11 = 20002,
    CFC12 = 20003,
    CFC113 = 20004,
    CFC113a = 20005,
    CFC114 = 20006,
    CFC115 = 20007,
    HCFC22 = 20008,
    HCFC141b = 20009,
    HCFC142b = 20010,
    Halon1202 = 20011,
    Halon1211 = 20012,
    Halon1301 = 20013,
    Halon2402 = 20014,
    MethylChloride = 20015,
    CarbonTetrachloride = 20016,
    HCC140a = 20017,
    MethylBromide = 20018,
    Hexachlorocyclohexane = 20019,
    AlphaHexachlorocyclohexane = 20020,
    Hexachlorobiphenyl = 20021,
    RadioactivePollutant = 30000,
    HOxRadical = 60000,
    TotalInorganicAndOrganicPeroxyRadicals = 60001,
    PassiveOzone = 60002,
    NOxExpressedAsNitrogen = 60003,
    AllNitrogenOxides = 60004,
    TotalInorganicChlorineExceptHClClONO2 = 60005,
    TotalInorganicBromineExceptHBrBrONO2 = 60006,
    LumpedAlkanes = 60007,
    LumpedAlkenes = 60008,
    LumpedAromaticCompounds = 60009,
    LumpedTerpenes = 60010,
    NonMethaneVolatileOrganicCompounds = 60011,
    AnthropogenicNMVOCExpressedAsCarbon = 60012,
    BiogenicNMVOCExpressedAsCarbon = 60013,
    LumpedOxygenatedHydrocarbons = 60014,
    Reserved60015 = 60015,
    TotalAerosol = 62000,
    DustDry = 62001,
    WaterInAmbient = 62002,
    AmmoniumDry = 62003,
    NitrateDry = 62004,
    NitricAcidTrihydrate = 62005,
    SulphateDry = 62006,
    MercuryDry = 62007,
    SeaSaltDry = 62008,
    BlackCarbonDry = 62009,
    ParticulateOrganicMatterDry = 62010,
    PrimaryParticulateOrganicMatterDry = 62011,
    SecondaryParticulateOrganicMatterDry = 62012,
    BlackCarbonHydrophilicDry = 62013,
    BlackCarbonHydrophobicDry = 62014,
    ParticulateOrganicMatterHydrophilicDry = 62015,
    ParticulateOrganicMatterHydrophobicDry = 62016,
    NitrateHydrophilicDry = 62017,
    NitrateHydrophobicDry = 62018,
    Reserved62019 = 62019,
    SmokeHighAbsorption = 62020,
    SmokeLowAbsorption = 62021,
    AerosolHighAbsorption = 62022,
    AerosolLowAbsorption = 62023,
    Reserved62024 = 62024,
    VolcanicAsh = 62025,
    BrownCarbonDry = 62036,
    Missing = 65535,
}
impl From<u16> for Grib2Table4_233 {
    fn from(val: u16) -> Self {
        match val {
            0 => Self::Ozone,
            1 => Self::WaterVapour,
            2 => Self::Methane,
            3 => Self::CarbonDioxide,
            4 => Self::CarbonMonoxide,
            5 => Self::NitrogenDioxide,
            6 => Self::NitrousOxide,
            7 => Self::Formaldehyde,
            8 => Self::SulphurDioxide,
            9 => Self::Ammonia,
            10 => Self::Ammonium,
            11 => Self::NitrogenMonoxide,
            12 => Self::AtomicOxygen,
            13 => Self::NitrateRadical,
            14 => Self::HydroperoxylRadical,
            15 => Self::DinitrogenPentoxide,
            16 => Self::NitrousAcid,
            17 => Self::NitricAcid,
            18 => Self::PeroxynitricAcid,
            19 => Self::HydrogenPeroxide,
            20 => Self::MolecularHydrogen,
            21 => Self::AtomicNitrogen,
            22 => Self::Sulphate,
            23 => Self::Radon,
            24 => Self::ElementalMercury,
            25 => Self::DivalentMercury,
            26 => Self::AtomicChlorine,
            27 => Self::ChlorineMonoxide,
            28 => Self::DichlorinePeroxide,
            29 => Self::HypochlorousAcid,
            30 => Self::ChlorineNitrate,
            31 => Self::ChlorineDioxide,
            32 => Self::AtomicBromide,
            33 => Self::BromineMonoxide,
            34 => Self::BromineChloride,
            35 => Self::HydrogenBromide,
            36 => Self::HypobromousAcid,
            37 => Self::BromineNitrate,
            38 => Self::Oxygen,
            39 => Self::Reserved39,
            10000 => Self::HydroxylRadical,
            10001 => Self::MethylPeroxyRadical,
            10002 => Self::MethylHydroperoxide,
            10003 => Self::Reserved10003,
            10004 => Self::Methanol,
            10005 => Self::FormicAcid,
            10006 => Self::HydrogenCyanide,
            10007 => Self::AcetoNitrile,
            10008 => Self::Ethane,
            10009 => Self::Ethene,
            10010 => Self::Ethyne,
            10011 => Self::Ethanol,
            10012 => Self::AceticAcid,
            10013 => Self::PeroxyacetylNitrate,
            10014 => Self::Propane,
            10015 => Self::Propene,
            10016 => Self::Butanes,
            10017 => Self::Isoprene,
            10018 => Self::AlphaPinene,
            10019 => Self::BetaPinene,
            10020 => Self::Limonene,
            10021 => Self::Benzene,
            10022 => Self::Toluene,
            10023 => Self::Xylene,
            10024 => Self::Reserved10024,
            10500 => Self::DimethylSulphide,
            20001 => Self::HydrogenChloride,
            20002 => Self::CFC11,
            20003 => Self::CFC12,
            20004 => Self::CFC113,
            20005 => Self::CFC113a,
            20006 => Self::CFC114,
            20007 => Self::CFC115,
            20008 => Self::HCFC22,
            20009 => Self::HCFC141b,
            20010 => Self::HCFC142b,
            20011 => Self::Halon1202,
            20012 => Self::Halon1211,
            20013 => Self::Halon1301,
            20014 => Self::Halon2402,
            20015 => Self::MethylChloride,
            20016 => Self::CarbonTetrachloride,
            20017 => Self::HCC140a,
            20018 => Self::MethylBromide,
            20019 => Self::Hexachlorocyclohexane,
            20020 => Self::AlphaHexachlorocyclohexane,
            20021 => Self::Hexachlorobiphenyl,
            30000 => Self::RadioactivePollutant,
            60000 => Self::HOxRadical,
            60001 => Self::TotalInorganicAndOrganicPeroxyRadicals,
            60002 => Self::PassiveOzone,
            60003 => Self::NOxExpressedAsNitrogen,
            60004 => Self::AllNitrogenOxides,
            60005 => Self::TotalInorganicChlorineExceptHClClONO2,
            60006 => Self::TotalInorganicBromineExceptHBrBrONO2,
            60007 => Self::LumpedAlkanes,
            60008 => Self::LumpedAlkenes,
            60009 => Self::LumpedAromaticCompounds,
            60010 => Self::LumpedTerpenes,
            60011 => Self::NonMethaneVolatileOrganicCompounds,
            60012 => Self::AnthropogenicNMVOCExpressedAsCarbon,
            60013 => Self::BiogenicNMVOCExpressedAsCarbon,
            60014 => Self::LumpedOxygenatedHydrocarbons,
            60015 => Self::Reserved60015,
            62000 => Self::TotalAerosol,
            62001 => Self::DustDry,
            62002 => Self::WaterInAmbient,
            62003 => Self::AmmoniumDry,
            62004 => Self::NitrateDry,
            62005 => Self::NitricAcidTrihydrate,
            62006 => Self::SulphateDry,
            62007 => Self::MercuryDry,
            62008 => Self::SeaSaltDry,
            62009 => Self::BlackCarbonDry,
            62010 => Self::ParticulateOrganicMatterDry,
            62011 => Self::PrimaryParticulateOrganicMatterDry,
            62012 => Self::SecondaryParticulateOrganicMatterDry,
            62013 => Self::BlackCarbonHydrophilicDry,
            62014 => Self::BlackCarbonHydrophobicDry,
            62015 => Self::ParticulateOrganicMatterHydrophilicDry,
            62016 => Self::ParticulateOrganicMatterHydrophobicDry,
            62017 => Self::NitrateHydrophilicDry,
            62018 => Self::NitrateHydrophobicDry,
            62019 => Self::Reserved62019,
            62020 => Self::SmokeHighAbsorption,
            62021 => Self::SmokeLowAbsorption,
            62022 => Self::AerosolHighAbsorption,
            62023 => Self::AerosolLowAbsorption,
            62024 => Self::Reserved62024,
            62025 => Self::VolcanicAsh,
            62036 => Self::BrownCarbonDry,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_233 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Ozone => "Ozone - O3",
            Self::WaterVapour => "Water Vapour - H2O",
            Self::Methane => "Methane - CH4",
            Self::CarbonDioxide => "Carbon Dioxide - CO2",
            Self::CarbonMonoxide => "Carbon Monoxide - CO",
            Self::NitrogenDioxide => "Nitrogen Dioxide - NO2",
            Self::NitrousOxide => "Nitrous Oxide - N2O",
            Self::Formaldehyde => "Formaldehyde - HCHO",
            Self::SulphurDioxide => "Sulphur Dioxide - SO2",
            Self::Ammonia => "Ammonia - NH3",
            Self::Ammonium => "Ammonium - NH4+",
            Self::NitrogenMonoxide => "Nitrogen Monoxide - NO",
            Self::AtomicOxygen => "Atomic Oxygen - O",
            Self::NitrateRadical => "Nitrate Radical - NO3",
            Self::HydroperoxylRadical => "Hydroperoxyl Radical - HO2",
            Self::DinitrogenPentoxide => "Dinitrogen Pentoxide - H2O5",
            Self::NitrousAcid => "Nitrous Acid - HONO",
            Self::NitricAcid => "Nitric Acid - HNO3",
            Self::PeroxynitricAcid => "Peroxynitric Acid - HO2NO2",
            Self::HydrogenPeroxide => "Hydrogen Peroxide - H2O2",
            Self::MolecularHydrogen => "Molecular Hydrogen - H",
            Self::AtomicNitrogen => "Atomic Nitrogen - N",
            Self::Sulphate => "Sulphate - SO42-",
            Self::Radon => "Radon - Rn",
            Self::ElementalMercury => "Elemental Mercury - Hg(O)",
            Self::DivalentMercury => "Divalent Mercury - Hg2+",
            Self::AtomicChlorine => "Atomic Chlorine - Cl",
            Self::ChlorineMonoxide => "Chlorine Monoxide - ClO",
            Self::DichlorinePeroxide => "Dichlorine Peroxide - Cl2O2",
            Self::HypochlorousAcid => "Hypochlorous Acid - HClO",
            Self::ChlorineNitrate => "Chlorine Nitrate - ClONO2",
            Self::ChlorineDioxide => "Chlorine Dioxide - ClO2",
            Self::AtomicBromide => "Atomic Bromide - Br",
            Self::BromineMonoxide => "Bromine Monoxide - BrO",
            Self::BromineChloride => "Bromine Chloride - BrCl",
            Self::HydrogenBromide => "Hydrogen Bromide - HBr",
            Self::HypobromousAcid => "Hypobromous Acid - HBrO",
            Self::BromineNitrate => "Bromine Nitrate - BrONO2",
            Self::Oxygen => "Oxygen - O2",
            Self::Reserved39 => "Reserved",
            Self::HydroxylRadical => "Hydroxyl Radical - OH",
            Self::MethylPeroxyRadical => "Methyl Peroxy Radical - CH3O2",
            Self::MethylHydroperoxide => "Methyl Hydroperoxide - CH3O2H",
            Self::Reserved10003 => "Reserved",
            Self::Methanol => "Methanol - CH3OH",
            Self::FormicAcid => "Formic Acid - CH3OOH",
            Self::HydrogenCyanide => "Hydrogen Cyanide - HCN",
            Self::AcetoNitrile => "Aceto Nitrile - CH3CN",
            Self::Ethane => "Ethane - C2H6",
            Self::Ethene => "Ethene (Ethylene) - C2H4",
            Self::Ethyne => "Ethyne (Acetylene) - C2H2",
            Self::Ethanol => "Ethanol - C2H5OH",
            Self::AceticAcid => "Acetic Acid - C2H5OOH",
            Self::PeroxyacetylNitrate => "Peroxyacetyl Nitrate - CH3C(O)OONO2",
            Self::Propane => "Propane - C3H8",
            Self::Propene => "Propene - C3H6",
            Self::Butanes => "Butanes - C4H10",
            Self::Isoprene => "Isoprene - C5H10",
            Self::AlphaPinene => "Alpha Pinene - C10H16",
            Self::BetaPinene => "Beta Pinene - C10H16",
            Self::Limonene => "Limonene - C10H16",
            Self::Benzene => "Benzene - C6H6",
            Self::Toluene => "Toluene - C7H8",
            Self::Xylene => "Xylene - C8H10",
            Self::Reserved10024 => "Reserved",
            Self::DimethylSulphide => "Dimethyl Sulphide - CH3SCH3",
            Self::HydrogenChloride => "Hydrogen Chloride - HCL",
            Self::CFC11 => "CFC-11",
            Self::CFC12 => "CFC-12",
            Self::CFC113 => "CFC-113",
            Self::CFC113a => "CFC-113a",
            Self::CFC114 => "CFC-114",
            Self::CFC115 => "CFC-115",
            Self::HCFC22 => "HCFC-22",
            Self::HCFC141b => "HCFC-141b",
            Self::HCFC142b => "HCFC-142b",
            Self::Halon1202 => "Halon-1202",
            Self::Halon1211 => "Halon-1211",
            Self::Halon1301 => "Halon-1301",
            Self::Halon2402 => "Halon-2402",
            Self::MethylChloride => "Methyl Chloride (HCC-40)",
            Self::CarbonTetrachloride => "Carbon Tetrachloride (HCC-10)",
            Self::HCC140a => "HCC-140a - CH3CCl3",
            Self::MethylBromide => "Methyl Bromide (HBC-40B1)",
            Self::Hexachlorocyclohexane => "Hexachlorocyclohexane (HCH)",
            Self::AlphaHexachlorocyclohexane => "Alpha Hexachlorocyclohexane",
            Self::Hexachlorobiphenyl => "Hexachlorobiphenyl (PCB-153)",
            Self::RadioactivePollutant => {
                "Radioactive Pollutant (Tracer, defined by originating centre)"
            }
            Self::HOxRadical => "HOx Radical (OH+HO2)",
            Self::TotalInorganicAndOrganicPeroxyRadicals => {
                "Total Inorganic and Organic Peroxy Radicals (HO2+RO2) - RO2"
            }
            Self::PassiveOzone => "Passive Ozone",
            Self::NOxExpressedAsNitrogen => "NOx Expressed As Nitrogen - NOx",
            Self::AllNitrogenOxides => "All Nitrogen Oxides (NOy) Expressed As Nitrogen - NOy",
            Self::TotalInorganicChlorineExceptHClClONO2 => {
                "Total Inorganic Chlorine Except HCl, ClONO2: ClOx"
            }
            Self::TotalInorganicBromineExceptHBrBrONO2 => {
                "Total Inorganic Bromine Except HBr, BrONO2: BrOx"
            }
            Self::LumpedAlkanes => "Lumped Alkanes",
            Self::LumpedAlkenes => "Lumped Alkenes",
            Self::LumpedAromaticCompounds => "Lumped Aromatic Compounds",
            Self::LumpedTerpenes => "Lumped Terpenes",
            Self::NonMethaneVolatileOrganicCompounds => {
                "Non-Methane Volatile Organic Compounds Expressed as Carbon - NMVOC"
            }
            Self::AnthropogenicNMVOCExpressedAsCarbon => {
                "Anthropogenic NMVOC Expressed as Carbon - aNMVOC"
            }
            Self::BiogenicNMVOCExpressedAsCarbon => "Biogenic NMVOC Expressed as Carbon - bNMVOC",
            Self::LumpedOxygenatedHydrocarbons => "Lumped Oxygenated Hydrocarbons - OVOC",
            Self::Reserved60015 => "Reserved",
            Self::TotalAerosol => "Total Aerosol",
            Self::DustDry => "Dust Dry",
            Self::WaterInAmbient => "Water In Ambient",
            Self::AmmoniumDry => "Ammonium Dry",
            Self::NitrateDry => "Nitrate Dry",
            Self::NitricAcidTrihydrate => "Nitric Acid Trihydrate",
            Self::SulphateDry => "Sulphate Dry",
            Self::MercuryDry => "Mercury Dry",
            Self::SeaSaltDry => "Sea Salt Dry",
            Self::BlackCarbonDry => "Black Carbon Dry",
            Self::ParticulateOrganicMatterDry => "Particulate Organic Matter Dry",
            Self::PrimaryParticulateOrganicMatterDry => "Primary Particulate Organic Matter Dry",
            Self::SecondaryParticulateOrganicMatterDry => {
                "Secondary Particulate Organic Matter Dry"
            }
            Self::BlackCarbonHydrophilicDry => "Black Carbon Hydrophilic Dry",
            Self::BlackCarbonHydrophobicDry => "Black Carbon Hydrophobic Dry",
            Self::ParticulateOrganicMatterHydrophilicDry => {
                "Particulate Organic Matter Hydrophilic Dry"
            }
            Self::ParticulateOrganicMatterHydrophobicDry => {
                "Particulate Organic Matter Hydrophobic Dry"
            }
            Self::NitrateHydrophilicDry => "Nitrate Hydrophilic Dry",
            Self::NitrateHydrophobicDry => "Nitrate Hydrophobic Dry",
            Self::Reserved62019 => "Reserved",
            Self::SmokeHighAbsorption => "Smoke - High Absorption",
            Self::SmokeLowAbsorption => "Smoke - Low Absorption",
            Self::AerosolHighAbsorption => "Aerosol - High Absorption",
            Self::AerosolLowAbsorption => "Aerosol - Low Absorption",
            Self::Reserved62024 => "Reserved",
            Self::VolcanicAsh => "Volcanic Ash",
            Self::BrownCarbonDry => "Brown Carbon Dry",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.234: CANOPY COVER FRACTION
///
/// **Created**: 07/12/2013
///
/// **Description**:
/// To be used as partitioned parameter in Product Definition Templates (PDT) 4.53 or 4.54.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-234.shtml)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_234 {
    CropsMixedFarming = 1,
    ShortGrass = 2,
    EvergreenNeedleleafTrees = 3,
    DeciduousNeedleleafTrees = 4,
    DeciduousBroadleafTrees = 5,
    EvergreenBroadleafTrees = 6,
    TallGrass = 7,
    Desert = 8,
    Tundra = 9,
    IrrigatedCrops = 10,
    Semidesert = 11,
    IceCapsAndGlaciers = 12,
    BogsAndMarshes = 13,
    InlandWater = 14,
    Ocean = 15,
    EvergreenShrubs = 16,
    DeciduousShrubs = 17,
    MixedForest = 18,
    InterruptedForest = 19,
    WaterAndLandMixtures = 20,
    Missing = 255,
}
impl From<u8> for Grib2Table4_234 {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::CropsMixedFarming,
            2 => Self::ShortGrass,
            3 => Self::EvergreenNeedleleafTrees,
            4 => Self::DeciduousNeedleleafTrees,
            5 => Self::DeciduousBroadleafTrees,
            6 => Self::EvergreenBroadleafTrees,
            7 => Self::TallGrass,
            8 => Self::Desert,
            9 => Self::Tundra,
            10 => Self::IrrigatedCrops,
            11 => Self::Semidesert,
            12 => Self::IceCapsAndGlaciers,
            13 => Self::BogsAndMarshes,
            14 => Self::InlandWater,
            15 => Self::Ocean,
            16 => Self::EvergreenShrubs,
            17 => Self::DeciduousShrubs,
            18 => Self::MixedForest,
            19 => Self::InterruptedForest,
            20 => Self::WaterAndLandMixtures,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_234 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::CropsMixedFarming => "Crops, mixed farming",
            Self::ShortGrass => "Short grass",
            Self::EvergreenNeedleleafTrees => "Evergreen needleleaf trees",
            Self::DeciduousNeedleleafTrees => "Deciduous needleleaf trees",
            Self::DeciduousBroadleafTrees => "Deciduous broadleaf trees",
            Self::EvergreenBroadleafTrees => "Evergreen broadleaf trees",
            Self::TallGrass => "Tall grass",
            Self::Desert => "Desert",
            Self::Tundra => "Tundra",
            Self::IrrigatedCrops => "Irrigated crops",
            Self::Semidesert => "Semidesert",
            Self::IceCapsAndGlaciers => "Ice caps and glaciers",
            Self::BogsAndMarshes => "Bogs and marshes",
            Self::InlandWater => "Inland water",
            Self::Ocean => "Ocean",
            Self::EvergreenShrubs => "Evergreen shrubs",
            Self::DeciduousShrubs => "Deciduous shrubs",
            Self::MixedForest => "Mixed forest",
            Self::InterruptedForest => "Interrupted forest",
            Self::WaterAndLandMixtures => "Water and land mixtures",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.235: Wind-Generated Wave Spectral Description
///
/// **Created**: 02/15/2012
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-235.shtml)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_235 {
    TotalWaveSpectrum = 0,
    GeneralizedPartition = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table4_235 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::TotalWaveSpectrum,
            1 => Self::GeneralizedPartition,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_235 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::TotalWaveSpectrum => "Total Wave Spectrum (combined wind waves and swell)",
            Self::GeneralizedPartition => "Generalized Partition",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.236: Soil Texture Fraction
/// (to be used as partitioned parameter in PDT 4.53 or 4.54)
///
/// **Created**: 07/12/2013
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-236.shtml)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_236 {
    Coarse = 1,
    Medium = 2,
    MediumFine = 3,
    Fine = 4,
    VeryFine = 5,
    Organic = 6,
    TropicalOrganic = 7,
    Missing = 255,
}
impl From<u8> for Grib2Table4_236 {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::Coarse,
            2 => Self::Medium,
            3 => Self::MediumFine,
            4 => Self::Fine,
            5 => Self::VeryFine,
            6 => Self::Organic,
            7 => Self::TropicalOrganic,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_236 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Coarse => "Coarse",
            Self::Medium => "Medium",
            Self::MediumFine => "Medium-fine",
            Self::Fine => "Fine",
            Self::VeryFine => "Very-fine",
            Self::Organic => "Organic",
            Self::TropicalOrganic => "Tropical-organic",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.238: Source or Sink
///
/// **Revised**: 07/15/2024
/// Red text depicts changes made since 07/15/2024
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-238.shtml)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_238 {
    Reserved = 0,
    Aviation = 1,
    Lightning = 2,
    BiogenicSources = 3,
    AnthropogenicSources = 4,
    WildFires = 5,
    NaturalSources = 6,
    BioFuel = 7,
    Volcanoes = 8,
    FossilFuel = 9,
    Wetlands = 10,
    Oceans = 11,
    ElevatedAnthropogenicSources = 12,
    SurfaceAnthropogenicSources = 13,
    AgricultureLivestock = 14,
    AgricultureSOils = 15,
    AgricultureWasteBurning = 16,
    AgricultureAll = 17,
    ResidentialCommercialAndOtherCombustion = 18,
    PowerGeneration = 19,
    SuperPowerStations = 20,
    Fugitives = 21,
    IndustrialProcess = 22,
    Solvents = 23,
    Ships = 24,
    Wastes = 25,
    RoadTransportation = 26,
    OffRoadTransportation = 27,
    NuclearPowerPlant = 28,
    NuclearWeapon = 29,
    Missing = 255,
}
impl From<u8> for Grib2Table4_238 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved,
            1 => Self::Aviation,
            2 => Self::Lightning,
            3 => Self::BiogenicSources,
            4 => Self::AnthropogenicSources,
            5 => Self::WildFires,
            6 => Self::NaturalSources,
            7 => Self::BioFuel,
            8 => Self::Volcanoes,
            9 => Self::FossilFuel,
            10 => Self::Wetlands,
            11 => Self::Oceans,
            12 => Self::ElevatedAnthropogenicSources,
            13 => Self::SurfaceAnthropogenicSources,
            14 => Self::AgricultureLivestock,
            15 => Self::AgricultureSOils,
            16 => Self::AgricultureWasteBurning,
            17 => Self::AgricultureAll,
            18 => Self::ResidentialCommercialAndOtherCombustion,
            19 => Self::PowerGeneration,
            20 => Self::SuperPowerStations,
            21 => Self::Fugitives,
            22 => Self::IndustrialProcess,
            23 => Self::Solvents,
            24 => Self::Ships,
            25 => Self::Wastes,
            26 => Self::RoadTransportation,
            27 => Self::OffRoadTransportation,
            28 => Self::NuclearPowerPlant,
            29 => Self::NuclearWeapon,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_238 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved => "Reserved",
            Self::Aviation => "Aviation",
            Self::Lightning => "Lightning",
            Self::BiogenicSources => "Biogenic Sources",
            Self::AnthropogenicSources => "Anthropogenic sources",
            Self::WildFires => "Wild fires",
            Self::NaturalSources => "Natural sources",
            Self::BioFuel => "Bio-fuel",
            Self::Volcanoes => "Volcanoes",
            Self::FossilFuel => "Fossil-fuel",
            Self::Wetlands => "Wetlands",
            Self::Oceans => "Oceans",
            Self::ElevatedAnthropogenicSources => "Elevated anthropogenic sources",
            Self::SurfaceAnthropogenicSources => "Surface anthropogenic sources",
            Self::AgricultureLivestock => "Agriculture livestock",
            Self::AgricultureSOils => "Agriculture soils",
            Self::AgricultureWasteBurning => "Agriculture waste burning",
            Self::AgricultureAll => "Agriculture (all)",
            Self::ResidentialCommercialAndOtherCombustion => {
                "Residential, commercial and other combustion"
            }
            Self::PowerGeneration => "Power generation",
            Self::SuperPowerStations => "Super power stations",
            Self::Fugitives => "Fugitives",
            Self::IndustrialProcess => "Industrial process",
            Self::Solvents => "Solvents",
            Self::Ships => "Ships",
            Self::Wastes => "Wastes",
            Self::RoadTransportation => "Road transportation",
            Self::OffRoadTransportation => "Off-road transportation",
            Self::NuclearPowerPlant => "Nuclear power plant",
            Self::NuclearWeapon => "Nuclear weapon",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.239: Wetland Type
///
/// **Created**: 10/24/2023
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-239.shtml)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_239 {
    Reserved = 0,
    Bog = 1,
    Drained = 2,
    Fen = 3,
    Floodplain = 4,
    Mangrove = 5,
    Marsh = 6,
    Rice = 7,
    Riverine = 8,
    SaltMarsh = 9,
    Swamp = 10,
    Upland = 11,
    WetTundra = 12,
    Missing = 255,
}
impl From<u8> for Grib2Table4_239 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved,
            1 => Self::Bog,
            2 => Self::Drained,
            3 => Self::Fen,
            4 => Self::Floodplain,
            5 => Self::Mangrove,
            6 => Self::Marsh,
            7 => Self::Rice,
            8 => Self::Riverine,
            9 => Self::SaltMarsh,
            10 => Self::Swamp,
            11 => Self::Upland,
            12 => Self::WetTundra,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_239 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved => "Reserved",
            Self::Bog => "Bog",
            Self::Drained => "Drained",
            Self::Fen => "Fen",
            Self::Floodplain => "Floodplain",
            Self::Mangrove => "Mangrove",
            Self::Marsh => "Marsh",
            Self::Rice => "Rice",
            Self::Riverine => "Riverine",
            Self::SaltMarsh => "Salt Marsh",
            Self::Swamp => "Swamp",
            Self::Upland => "Upland",
            Self::WetTundra => "Wet tundra",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.240: Type of Distribution Function
///
/// **Revised**: 07/07/2017
///
/// ## Notes
/// 1. Bin-Model or delta function with N concentration cl(r) in class (or mode) l.
///    Concentration-density function:
///    $f(r;d) = \sum_{l=1}^{N} cl(r) \delta(d-Dl)$
///    - N: Number of modes in the distribution
///    - $\delta$: Delta-Function
///    - d: Diameter
///    - Dl: Diameter of mode l(p1)
///
/// 2. Bin-Model or delta function with N concentration cl(r) in class (or mode) l.
///    Concentration-density function:
///    $f(r;m) = \sum_{l=1}^{N} cl(r) \delta(m-Ml)$
///    - N: Number of modes in the distribution
///    - $\delta$: Delta-Function
///    - m: Mass
///    - Ml: Mass of mode (p1)
///
/// 3. N-Modal concentration-density function consisting of Gaussian-functions:
///    $f(r;d) = \sum_{l=1}^{N} cl(r) (1 / \sqrt{2\pi\delta_l}) * e^{-((d-Dl)/\delta_l)^2}$
///    - N: Number of modes in the distribution
///    - d: Diameter
///    - Dl: Mean diameter of mode l(p1)
///    - $\delta_l$: Variance of Mode l (p2)
///    - cl(r): Concentration
///
/// 4. N-Modal concentration-density function consisting of Gaussian-functions:
///    $f(r;d) = \sum_{l=1}^{N} cl(r) (1 / \sqrt{2\pi\delta_l(r)}) * e^{-((d-Dl(r))/\delta_l(r))^2}$
///    - N: Fields of concentration cl(r)
///    - $\delta_l(r)$: Variance
///    - Dl(r): Mean diameter
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-240.shtml)
#[repr(u16)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_240 {
    NoSpecificDistributionFunctionGiven = 0,
    DeltaFunctionsWithFixedDiameters = 1,
    DeltaFunctionsWithFixedMasses = 2,
    GaussianDistributionFixedMeanDiameterAndVariance = 3,
    GaussianDistributionVariableParameters = 4,
    LogNormalDistributionVariableParameters = 5,
    LogNormalDistributionFixedVariance = 6,
    LogNormalDistributionFixedVarianceAndParticleDensity = 7,
    DerivedFromDistributionType7 = 8,
    Missing = 65535,
}
impl From<u16> for Grib2Table4_240 {
    fn from(val: u16) -> Self {
        match val {
            0 => Self::NoSpecificDistributionFunctionGiven,
            1 => Self::DeltaFunctionsWithFixedDiameters,
            2 => Self::DeltaFunctionsWithFixedMasses,
            3 => Self::GaussianDistributionFixedMeanDiameterAndVariance,
            4 => Self::GaussianDistributionVariableParameters,
            5 => Self::LogNormalDistributionVariableParameters,
            6 => Self::LogNormalDistributionFixedVariance,
            7 => Self::LogNormalDistributionFixedVarianceAndParticleDensity,
            8 => Self::DerivedFromDistributionType7,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_240 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NoSpecificDistributionFunctionGiven => "No specific distribution function given",
            Self::DeltaFunctionsWithFixedDiameters => {
                "Delta functions with spatially variable concentration and fixed diameters Dl(p1) \
                 in meter"
            }
            Self::DeltaFunctionsWithFixedMasses => {
                "Delta functions with spatially variable concentration and fixed masses Ml(p1) in \
                 kg"
            }
            Self::GaussianDistributionFixedMeanDiameterAndVariance => {
                "Gaussian (Normal) distribution with spatially variable concentration and fixed \
                 mean diameter Dl(p1) and variance δ(p2)"
            }
            Self::GaussianDistributionVariableParameters => {
                "Gaussian (Normal) distribution with spatially variable concentration, mean \
                 diameter and variance"
            }
            Self::LogNormalDistributionVariableParameters => {
                "Log-normal distribution with spatially variable number density, mean diameter and \
                 variance"
            }
            Self::LogNormalDistributionFixedVariance => {
                "Log-normal distribution with spatially variable number density, mean diameter and \
                 fixed variance δ(p1)"
            }
            Self::LogNormalDistributionFixedVarianceAndParticleDensity => {
                "Log-normal distribution with spatially variable number density and mass density \
                 and fixed variance δ and fixed particle density ρ(p2)"
            }
            Self::DerivedFromDistributionType7 => {
                "No distribution function. The encoded variable is derived from variables \
                 characterized by type of distribution function of type No. 7 with fixed variance \
                 σ(p1) and fixed particle density ρ(p2)"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.241: Coverage Attributes
///
/// **Updated**: 12/07/2023
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-241.shtml)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_241 {
    Undefined = 0,
    Unmodified = 1,
    SnowCovered = 2,
    Flooded = 3,
    IceCovered = 4,
    WithInterceptedWater = 5,
    WithInterceptedSnow = 6,
    Aggregated = 7,
    Missing = 255,
}
impl From<u8> for Grib2Table4_241 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Undefined,
            1 => Self::Unmodified,
            2 => Self::SnowCovered,
            3 => Self::Flooded,
            4 => Self::IceCovered,
            5 => Self::WithInterceptedWater,
            6 => Self::WithInterceptedSnow,
            7 => Self::Aggregated,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_241 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Undefined => "Undefined",
            Self::Unmodified => "Unmodified",
            Self::SnowCovered => "Snow-covered",
            Self::Flooded => "Flooded",
            Self::IceCovered => "Ice Covered",
            Self::WithInterceptedWater => "With intercepted water",
            Self::WithInterceptedSnow => "With intercepted snow",
            Self::Aggregated => "Aggregated",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.242: Tile Classification
///
/// **Updated**: 12/07/2023
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-242.shtml)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_242 {
    Reserved = 0,
    LandUseClassesESAGLOBCOVERGCV2009 = 1,
    LandUseClassesEuropeanCommissionGLC2000 = 2,
    LandUseClassesECOCLIMAP = 3,
    LandUseClassesECOCLIMAPSG = 4,
    LandUseClassesUSGSEROSGLCCV20BATsClassification = 5,
    Missing = 255,
}
impl From<u8> for Grib2Table4_242 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved,
            1 => Self::LandUseClassesESAGLOBCOVERGCV2009,
            2 => Self::LandUseClassesEuropeanCommissionGLC2000,
            3 => Self::LandUseClassesECOCLIMAP,
            4 => Self::LandUseClassesECOCLIMAPSG,
            5 => Self::LandUseClassesUSGSEROSGLCCV20BATsClassification,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_242 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved => "Reserved",
            Self::LandUseClassesESAGLOBCOVERGCV2009 => {
                "Land use classes according to ESA-GLOBCOVER GCV2009"
            }
            Self::LandUseClassesEuropeanCommissionGLC2000 => {
                "Land use classes according to European Commission-Global Land Cover Project \
                 GLC2000"
            }
            Self::LandUseClassesECOCLIMAP => "Land use classes according to ECOCLIMAP",
            Self::LandUseClassesECOCLIMAPSG => "Land use classes according to ECOCLIMAP-SG",
            Self::LandUseClassesUSGSEROSGLCCV20BATsClassification => {
                "Land use classes according to USGS EROS Global Land Cover Characterization (GLCC) \
                 v2.0 BATS Classification"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.243: Tile Class
///
/// **Created**: 04/09/2015
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-243.shtml)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_243 {
    Reserved = 0,
    EvergreenBroadleavedForest = 1,
    DeciduousBroadleavedClosedForest = 2,
    DeciduousBroadleavedOpenForest = 3,
    EvergreenNeedleLeafForest = 4,
    DeciduousNeedleLeafForest = 5,
    MixedLeafTrees = 6,
    FreshWaterFloodedTrees = 7,
    SalineWaterFloodedTrees = 8,
    MosaicTreeNaturalVegetation = 9,
    BurntTreeCover = 10,
    EvergreenShrubsClosedOpen = 11,
    DeciduousShrubsClosedOpen = 12,
    HerbaceousVegetationClosedOpen = 13,
    SparseHerbaceousOrGrass = 14,
    FloodedShrubsOrHerbaceous = 15,
    CultivatedAndManagedAreas = 16,
    MosaicCropTreeNaturalVegetation = 17,
    MosaicCropShrubGrass = 18,
    BareAreas = 19,
    Water = 20,
    SnowAndIce = 21,
    ArtificialSurface = 22,
    Ocean = 23,
    IrrigatedCroplands = 24,
    RainFedCroplands = 25,
    MosaicCropland5070Vegetation2050 = 26,
    MosaicVegetation5070Cropland2050 = 27,
    ClosedBroadleavedEvergreenForest = 28,
    ClosedNeedleLeavedEvergreenForest = 29,
    OpenNeedleLeavedDeciduousForest = 30,
    MixedBroadleavedAndNeedleLeaveForest = 31,
    MosaicShrubland5070Grassland2050 = 32,
    MosaicGrassland5070Shrubland2050 = 33,
    ClosedToOpenShrubland = 34,
    SparseVegetation = 35,
    ClosedToOpenForestRegularlyFlooded = 36,
    ClosedForestOrShrublandPermanentlyFlooded = 37,
    ClosedToOpenGrasslandRegularlyFlooded = 38,
    Undefined = 39,
    Missing = 255,
}
impl From<u8> for Grib2Table4_243 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved,
            1 => Self::EvergreenBroadleavedForest,
            2 => Self::DeciduousBroadleavedClosedForest,
            3 => Self::DeciduousBroadleavedOpenForest,
            4 => Self::EvergreenNeedleLeafForest,
            5 => Self::DeciduousNeedleLeafForest,
            6 => Self::MixedLeafTrees,
            7 => Self::FreshWaterFloodedTrees,
            8 => Self::SalineWaterFloodedTrees,
            9 => Self::MosaicTreeNaturalVegetation,
            10 => Self::BurntTreeCover,
            11 => Self::EvergreenShrubsClosedOpen,
            12 => Self::DeciduousShrubsClosedOpen,
            13 => Self::HerbaceousVegetationClosedOpen,
            14 => Self::SparseHerbaceousOrGrass,
            15 => Self::FloodedShrubsOrHerbaceous,
            16 => Self::CultivatedAndManagedAreas,
            17 => Self::MosaicCropTreeNaturalVegetation,
            18 => Self::MosaicCropShrubGrass,
            19 => Self::BareAreas,
            20 => Self::Water,
            21 => Self::SnowAndIce,
            22 => Self::ArtificialSurface,
            23 => Self::Ocean,
            24 => Self::IrrigatedCroplands,
            25 => Self::RainFedCroplands,
            26 => Self::MosaicCropland5070Vegetation2050,
            27 => Self::MosaicVegetation5070Cropland2050,
            28 => Self::ClosedBroadleavedEvergreenForest,
            29 => Self::ClosedNeedleLeavedEvergreenForest,
            30 => Self::OpenNeedleLeavedDeciduousForest,
            31 => Self::MixedBroadleavedAndNeedleLeaveForest,
            32 => Self::MosaicShrubland5070Grassland2050,
            33 => Self::MosaicGrassland5070Shrubland2050,
            34 => Self::ClosedToOpenShrubland,
            35 => Self::SparseVegetation,
            36 => Self::ClosedToOpenForestRegularlyFlooded,
            37 => Self::ClosedForestOrShrublandPermanentlyFlooded,
            38 => Self::ClosedToOpenGrasslandRegularlyFlooded,
            39 => Self::Undefined,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_243 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved => "Reserved",
            Self::EvergreenBroadleavedForest => "Evergreen broadleaved forest",
            Self::DeciduousBroadleavedClosedForest => "Deciduous broadleaved closed forest",
            Self::DeciduousBroadleavedOpenForest => "Deciduous broadleaved open forest",
            Self::EvergreenNeedleLeafForest => "Evergreen needle-leaf forest",
            Self::DeciduousNeedleLeafForest => "Deciduous needle-leaf forest",
            Self::MixedLeafTrees => "Mixed leaf trees",
            Self::FreshWaterFloodedTrees => "Fresh water flooded trees",
            Self::SalineWaterFloodedTrees => "Saline water flooded trees",
            Self::MosaicTreeNaturalVegetation => "Mosaic tree/natural vegetation",
            Self::BurntTreeCover => "Burnt tree cover",
            Self::EvergreenShrubsClosedOpen => "Evergreen shurbs closed-open",
            Self::DeciduousShrubsClosedOpen => "Deciduous shurbs closed-open",
            Self::HerbaceousVegetationClosedOpen => "Herbaceous vegetation closed-open",
            Self::SparseHerbaceousOrGrass => "Sparse herbaceous or grass",
            Self::FloodedShrubsOrHerbaceous => "Flooded shurbs or herbaceous",
            Self::CultivatedAndManagedAreas => "Cultivated and managed areas",
            Self::MosaicCropTreeNaturalVegetation => "Mosaic crop/tree/natural vegetation",
            Self::MosaicCropShrubGrass => "Mosaic crop/shrub/grass",
            Self::BareAreas => "Bare areas",
            Self::Water => "Water",
            Self::SnowAndIce => "Snow and ice",
            Self::ArtificialSurface => "Artificial surface",
            Self::Ocean => "Ocean",
            Self::IrrigatedCroplands => "Irrigated croplands",
            Self::RainFedCroplands => "Rain fed croplands",
            Self::MosaicCropland5070Vegetation2050 => {
                "Mosaic cropland (50-70%)-vegetation (20-50%)"
            }
            Self::MosaicVegetation5070Cropland2050 => {
                "Mosaic vegetation (50-70%)-cropland (20-50%)"
            }
            Self::ClosedBroadleavedEvergreenForest => "Closed broadleaved evergreen forest",
            Self::ClosedNeedleLeavedEvergreenForest => "Closed needle-leaved evergreen forest",
            Self::OpenNeedleLeavedDeciduousForest => "Open needle-leaved deciduous forest",
            Self::MixedBroadleavedAndNeedleLeaveForest => {
                "Mixed broadleaved and needle-leave forest"
            }
            Self::MosaicShrubland5070Grassland2050 => {
                "Mosaic shrubland (50-70%)-grassland (20-50%)"
            }
            Self::MosaicGrassland5070Shrubland2050 => {
                "Mosaic grassland (50-70%)-shrubland (20-50%)"
            }
            Self::ClosedToOpenShrubland => "Closed to open shrubland",
            Self::SparseVegetation => "Sparse vegetation",
            Self::ClosedToOpenForestRegularlyFlooded => "Closed to open forest regularly flooded",
            Self::ClosedForestOrShrublandPermanentlyFlooded => {
                "Closed forest or shrubland permanently flooded"
            }
            Self::ClosedToOpenGrasslandRegularlyFlooded => {
                "Closed to open grassland regularly flooded"
            }
            Self::Undefined => "Undefined",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.244: QUALITY INDICATOR
///
/// **Created**: 07/09/2018
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-244.shtml)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_244 {
    NoQualityInformationAvailable = 0,
    Failed = 1,
    Passed = 2,
    Missing = 255,
}
impl From<u8> for Grib2Table4_244 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoQualityInformationAvailable,
            1 => Self::Failed,
            2 => Self::Passed,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_244 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NoQualityInformationAvailable => "No Quality Information Available",
            Self::Failed => "Failed",
            Self::Passed => "Passed",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.246: THUNDERSTORM INTENSITY INDEX
///
/// **Created**: 06/23/2022
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-246.shtml)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_246 {
    NoThunderstormOccurrence = 0,
    WeakThunderstorm = 1,
    ModerateThunderstorm = 2,
    SevereThunderstorm = 3,
    Missing = 255,
}
impl From<u8> for Grib2Table4_246 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoThunderstormOccurrence,
            1 => Self::WeakThunderstorm,
            2 => Self::ModerateThunderstorm,
            3 => Self::SevereThunderstorm,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_246 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NoThunderstormOccurrence => "No thunderstorm occurrence",
            Self::WeakThunderstorm => "Weak thunderstorm",
            Self::ModerateThunderstorm => "Moderate thunderstorm",
            Self::SevereThunderstorm => "Severe thunderstorm",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.247: PRECIPITATION INTENSITY
///
/// **Created**: 06/23/2022
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-247.shtml)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_247 {
    NoPrecipitationOccurrence = 0,
    LightPrecipitation = 1,
    ModeratePrecipitation = 2,
    HeavyPrecipitation = 3,
    Missing = 255,
}
impl From<u8> for Grib2Table4_247 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NoPrecipitationOccurrence,
            1 => Self::LightPrecipitation,
            2 => Self::ModeratePrecipitation,
            3 => Self::HeavyPrecipitation,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_247 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NoPrecipitationOccurrence => "No precipitation occurrence",
            Self::LightPrecipitation => "Light precipitation",
            Self::ModeratePrecipitation => "Moderate precipitation",
            Self::HeavyPrecipitation => "Heavy precipitation",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.248: METHOD USED TO DERIVE DATA VALUE FOR A GIVEN LOCAL TIME
///
/// **Created**: 06/23/2022
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-248.shtml)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_248 {
    NearestForecastOrAnalysisTime = 0,
    InterpolatedToValidAtSpecifiedLocalTime = 1,
    Missing = 255,
}
impl From<u8> for Grib2Table4_248 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::NearestForecastOrAnalysisTime,
            1 => Self::InterpolatedToValidAtSpecifiedLocalTime,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_248 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::NearestForecastOrAnalysisTime => {
                "Nearest forecast or analysis time to specified local time"
            }
            Self::InterpolatedToValidAtSpecifiedLocalTime => {
                "Interpolated to be valid at the specified local time"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.249: CHARACTER OF PRECIPITATION
///
/// **Created**: 06/23/2022
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-249.shtml)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_249 {
    None = 0,
    Showers = 1,
    Intermittent = 2,
    Continuous = 3,
    Missing = 255,
}
impl From<u8> for Grib2Table4_249 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::None,
            1 => Self::Showers,
            2 => Self::Intermittent,
            3 => Self::Continuous,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_249 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::None => "None",
            Self::Showers => "Showers",
            Self::Intermittent => "Intermittent",
            Self::Continuous => "Continuous",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.250: DRAINAGE DIRECTION
///
/// **Created**: 06/23/2022
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-250.shtml)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_250 {
    Reserved = 0,
    SouthWest = 1,
    South = 2,
    SouthEast = 3,
    West = 4,
    NoDirection = 5,
    East = 6,
    NorthWest = 7,
    North = 8,
    NorthEast = 9,
    Missing = 255,
}
impl From<u8> for Grib2Table4_250 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved,
            1 => Self::SouthWest,
            2 => Self::South,
            3 => Self::SouthEast,
            4 => Self::West,
            5 => Self::NoDirection,
            6 => Self::East,
            7 => Self::NorthWest,
            8 => Self::North,
            9 => Self::NorthEast,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_250 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved => "Reserved",
            Self::SouthWest => "South-West",
            Self::South => "South",
            Self::SouthEast => "South-East",
            Self::West => "West",
            Self::NoDirection => "No direction",
            Self::East => "East",
            Self::NorthWest => "North-West",
            Self::North => "North",
            Self::NorthEast => "North-East",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.251: WAVE DIRECTION AND FREQUENCY FORMULAE
///
/// **Created**: 10/24/2023
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-251.shtml)
///
/// ## Notes
/// (1). Geometric sequence: $x_n = x_0 * r^{(n-1)}$ with 'x_0' first parameter and 'r' second parameter.
/// (2). Arithmetic sequence: $a_n = a_1 + (n-1) d$ with 'a_1' first parameter and 'd' second parameter.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_251 {
    UndefinedSequence = 0,
    GeometricSequence = 1,
    ArithmeticSequence = 2,
    Missing = 255,
}
impl From<u8> for Grib2Table4_251 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::UndefinedSequence,
            1 => Self::GeometricSequence,
            2 => Self::ArithmeticSequence,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_251 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::UndefinedSequence => "Undefined Sequence",
            Self::GeometricSequence => "Geometric sequence (see Note 1)",
            Self::ArithmeticSequence => "Arithmetic sequence (see Note 2)",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.333: Transport Dispersion Model
///
/// **Created**: 07/15/2024
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-333.shtml)
///
/// ## Notes
/// (No additional notes provided for this table)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_333 {
    Reserved = 0,
    DERMA = 1,
    EEmep = 2,
    FLEXPART = 3,
    MLDP = 4,
    MATCH = 5,
    SILAM = 6,
    SNAP = 7,
    WrfChem = 8,
    Trajectoire = 9,
    Missing = 255,
}
impl From<u8> for Grib2Table4_333 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved,
            1 => Self::DERMA,
            2 => Self::EEmep,
            3 => Self::FLEXPART,
            4 => Self::MLDP,
            5 => Self::MATCH,
            6 => Self::SILAM,
            7 => Self::SNAP,
            8 => Self::WrfChem,
            9 => Self::Trajectoire,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_333 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved => "Reserved",
            Self::DERMA => "DERMA (Danish Emergency Response Model of the Atmosphere)",
            Self::EEmep => "E-EMEP (Emergency EMEP model)",
            Self::FLEXPART => "FLEXPART (Particle dispersion model)",
            Self::MLDP => "MLDP (Modèle lagrangien de dispersion de particules)",
            Self::MATCH => "MATCH (Multi-scale Atmospheric Transport Model)",
            Self::SILAM => "SILAM (System for Integrated modeLling of Atmospheric composition)",
            Self::SNAP => "SNAP (Severe Nuclear Accident Program)",
            Self::WrfChem => "WRF-Chem (Weather Research and Forecasting Chemical model)",
            Self::Trajectoire => "Trajectoire (Trajectory model)",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.335: Emission Scenario Origin
///
/// **Created**: 07/15/2024
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-335.shtml)
///
/// ## Notes
/// (No additional notes provided for this table)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_335 {
    Reserved = 0,
    ARGOS = 1,
    JRODOS = 2,
    Assimilated = 3,
    Center = 4,
    Missing = 255,
}
impl From<u8> for Grib2Table4_335 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved,
            1 => Self::ARGOS,
            2 => Self::JRODOS,
            3 => Self::Assimilated,
            4 => Self::Center,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_335 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved => "Reserved",
            Self::ARGOS => "ARGOS (Accident Reporting and Guiding Operational System)",
            Self::JRODOS => "JRODOS (Java version of Real time Online Decision SuppOrt System)",
            Self::Assimilated => "Assimilated (Scenario retrieved from measurements)",
            Self::Center => "Center (scenario by originating center)",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.336: NWP Model
///
/// **Created**: 07/15/2024
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-336.shtml)
///
/// ## Notes
/// (No additional notes provided for this table)
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_336 {
    Reserved = 0,
    AROME = 1,
    ARPEGE = 2,
    GFS = 3,
    HARMONIE = 4,
    HIRLAM = 5,
    IFS = 6,
    GEMGDPS = 7,
    GEMRDPS = 8,
    GEMHRDPS = 9,
    WRF = 10,
    Missing = 255,
}
impl From<u8> for Grib2Table4_336 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Reserved,
            1 => Self::AROME,
            2 => Self::ARPEGE,
            3 => Self::GFS,
            4 => Self::HARMONIE,
            5 => Self::HIRLAM,
            6 => Self::IFS,
            7 => Self::GEMGDPS,
            8 => Self::GEMRDPS,
            9 => Self::GEMHRDPS,
            10 => Self::WRF,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_336 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Reserved => "Reserved",
            Self::AROME => "AROME (Meso scale NWP, Meteo-France)",
            Self::ARPEGE => "ARPEGE (Global scale NWP, Meteo-France)",
            Self::GFS => "GFS (Global forecast system, NCEP)",
            Self::HARMONIE => "HARMONIE (HIRLAM-ALADIN Research on Mesoscale Operational NWP)",
            Self::HIRLAM => "HIRLAM (HIgh resolution Limited Area Model)",
            Self::IFS => "IFS (Integrated Forecast System)",
            Self::GEMGDPS => "GEM GDPS (Canadian Global Deterministic Prediction System)",
            Self::GEMRDPS => "GEM RDPS (Canadian Regional Deterministic Prediction System)",
            Self::GEMHRDPS => {
                "GEM HRDPS (Canadian High Resolution Deterministic Prediction System)"
            }
            Self::WRF => "WRF (Weather Research and Forecasting)",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}
