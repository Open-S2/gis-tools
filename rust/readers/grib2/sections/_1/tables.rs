/// # Table 1.0 - GRIB Master Tables
///
/// **Details**:
/// - **Section**: 1
/// - **Octet**: 10 (index 9)
///
/// **Reserved Ranges**:
/// - `34-254`: Future Version
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the version numbers used in GRIB2 Master Tables,
/// providing context for interpreting the data's versioning information.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-0.shtml)
///
/// ## Notes
/// - Revised 12/07/2023
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table1_0 {
    Experimental = 0,
    Version20011107 = 1,
    Version20031104 = 2,
    Version20051102 = 3,
    Version20071107 = 4,
    Version20091104 = 5,
    Version20100915 = 6,
    Version20110504 = 7,
    Version20111108 = 8,
    Version20120502 = 9,
    Version20121107 = 10,
    Version20130508 = 11,
    Version20131114 = 12,
    Version20140507 = 13,
    Version20141105 = 14,
    Version20150506 = 15,
    Version20151111 = 16,
    Version20160504 = 17,
    Version20161102 = 18,
    Version20170503 = 19,
    Version20171108 = 20,
    Version20180502 = 21,
    Version20181107 = 22,
    Version20190515 = 23,
    Version20191106 = 24,
    Version20200506 = 25,
    Version20201116 = 26,
    Version20210616 = 27,
    Version20211115 = 28,
    Version20220515 = 29,
    Version20221115 = 30,
    Version20230615 = 31,
    Version20231130 = 32,
    PreOperationalNextAmendment = 33,
    Missing = 255,
}
impl From<u8> for Grib2Table1_0 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Experimental,
            1 => Self::Version20011107,
            2 => Self::Version20031104,
            3 => Self::Version20051102,
            4 => Self::Version20071107,
            5 => Self::Version20091104,
            6 => Self::Version20100915,
            7 => Self::Version20110504,
            8 => Self::Version20111108,
            9 => Self::Version20120502,
            10 => Self::Version20121107,
            11 => Self::Version20130508,
            12 => Self::Version20131114,
            13 => Self::Version20140507,
            14 => Self::Version20141105,
            15 => Self::Version20150506,
            16 => Self::Version20151111,
            17 => Self::Version20160504,
            18 => Self::Version20161102,
            19 => Self::Version20170503,
            20 => Self::Version20171108,
            21 => Self::Version20180502,
            22 => Self::Version20181107,
            23 => Self::Version20190515,
            24 => Self::Version20191106,
            25 => Self::Version20200506,
            26 => Self::Version20201116,
            27 => Self::Version20210616,
            28 => Self::Version20211115,
            29 => Self::Version20220515,
            30 => Self::Version20221115,
            31 => Self::Version20230615,
            32 => Self::Version20231130,
            33 => Self::PreOperationalNextAmendment,
            255 | _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table1_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Experimental => "Experimental",
            Self::Version20011107 => "Version Implemented on 7 November 2001",
            Self::Version20031104 => "Version Implemented on 4 November 2003",
            Self::Version20051102 => "Version Implemented on 2 November 2005",
            Self::Version20071107 => "Version Implemented on 7 November 2007",
            Self::Version20091104 => "Version Implemented on 4 November 2009",
            Self::Version20100915 => "Version Implemented on 15 September 2010",
            Self::Version20110504 => "Version Implemented on 4 May 2011",
            Self::Version20111108 => "Version Implemented on 8 November 2011",
            Self::Version20120502 => "Version Implemented on 2 May 2012",
            Self::Version20121107 => "Version Implemented on 7 November 2012",
            Self::Version20130508 => "Version Implemented on 8 May 2013",
            Self::Version20131114 => "Version Implemented on 14 November 2013",
            Self::Version20140507 => "Version Implemented on 7 May 2014",
            Self::Version20141105 => "Version Implemented on 5 November 2014",
            Self::Version20150506 => "Version Implemented on 6 May 2015",
            Self::Version20151111 => "Version Implemented on 11 November 2015",
            Self::Version20160504 => "Version Implemented on 4 May 2016",
            Self::Version20161102 => "Version Implemented on 2 November 2016",
            Self::Version20170503 => "Version Implemented on 3 May 2017",
            Self::Version20171108 => "Version Implemented on 8 November 2017",
            Self::Version20180502 => "Version Implemented on 2 May 2018",
            Self::Version20181107 => "Version Implemented on 7 November 2018",
            Self::Version20190515 => "Version Implemented on 15 May 2019",
            Self::Version20191106 => "Version Implemented on 06 November 2019",
            Self::Version20200506 => "Version Implemented on 06 May 2020",
            Self::Version20201116 => "Version Implemented on 16 November 2020",
            Self::Version20210616 => "Version Implemented on 16 June 2021",
            Self::Version20211115 => "Version Implemented on 15 November 2021",
            Self::Version20220515 => "Version Implemented on 15 May 2022",
            Self::Version20221115 => "Version Implemented on 15 November 2022",
            Self::Version20230615 => "Version Implemented on 15 June 2023",
            Self::Version20231130 => "Version Implemented on 30 November 2023",
            Self::PreOperationalNextAmendment => {
                "Pre-operational to be implemented by next amendment"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 1.1 - GRIB Local Tables Version Number
///
/// **Details**:
/// - **Section**: 1
/// - **Octet**: 11 (index 10)
///
/// **Used Ranges**:
/// - `1-254`: Number of local table versions used
///
/// **Special Values**:
/// - `0`: Local tables not used. Only table entries and templates from the current master table are valid.
/// - `255`: Missing
///
/// ## Description
/// This table defines the version numbers used in GRIB2 Local Tables,
/// providing context for interpreting the data's versioning information.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-1.shtml)
///
/// ## Notes
/// - Created 05/11/2005
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table1_1 {
    LocalTablesNotUsed = 0,
    Missing = 255,
}
impl From<u8> for Grib2Table1_1 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::LocalTablesNotUsed,
            255 | _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table1_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::LocalTablesNotUsed => {
                "Local tables not used. Only table entries and templates from the current master table are valid."
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 1.2 - Significance of Reference Time
///
/// **Details**:
/// - **Section**: 1
/// - **Octet**: 12 (index 11)
///
/// **Reserved Ranges**:
/// - `6-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the significance of the reference time in GRIB2 files,
/// providing context for interpreting the data's temporal meaning.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-2.shtml)
///
/// ## Notes
/// - Revised 06/16/2022
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table1_2 {
    Analysis = 0,
    StartOfForecast = 1,
    VerifyingTimeOfForecast = 2,
    ObservationTime = 3,
    LocalTime = 4,
    SimulationStart = 5,
    Missing = 255,
}
impl From<u8> for Grib2Table1_2 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Analysis,
            1 => Self::StartOfForecast,
            2 => Self::VerifyingTimeOfForecast,
            3 => Self::ObservationTime,
            4 => Self::LocalTime,
            5 => Self::SimulationStart,
            255 | _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table1_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Analysis => "Analysis",
            Self::StartOfForecast => "Start of Forecast",
            Self::VerifyingTimeOfForecast => "Verifying Time of Forecast",
            Self::ObservationTime => "Observation Time",
            Self::LocalTime => "Local Time",
            Self::SimulationStart => "Simulation start",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 1.3 - Production Status of Data
///
/// **Details**:
/// - **Section**: 1
/// - **Octet**: 20 (index 19)
///
/// **Reserved Ranges**:
/// - `14-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the production status of data in GRIB2 files,
/// providing context for interpreting the data's operational and research status.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-3.shtml)
///
/// ## Notes
/// - Revised 07/12/2024
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table1_3 {
    OperationalProducts = 0,
    OperationalTestProducts = 1,
    ResearchProducts = 2,
    ReAnalysisProducts = 3,
    ThorpexInteractiveGrandGlobalEnsembleTigge = 4,
    ThorpexInteractiveGrandGlobalEnsembleTiggeTest = 5,
    S2sOperationalProducts = 6,
    S2sTestProducts = 7,
    UncertaintiesInEnsemblesOfRegionalReanalysisProjectUerra = 8,
    UncertaintiesInEnsemblesOfRegionalReanalysisProjectUerraTest = 9,
    CopernicusRegionalReanalysis = 10,
    CopernicusRegionalReanalysisTest = 11,
    DestinationEarth = 12,
    DestinationEarthTest = 13,
    Missing = 255,
}
impl From<u8> for Grib2Table1_3 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::OperationalProducts,
            1 => Self::OperationalTestProducts,
            2 => Self::ResearchProducts,
            3 => Self::ReAnalysisProducts,
            4 => Self::ThorpexInteractiveGrandGlobalEnsembleTigge,
            5 => Self::ThorpexInteractiveGrandGlobalEnsembleTiggeTest,
            6 => Self::S2sOperationalProducts,
            7 => Self::S2sTestProducts,
            8 => Self::UncertaintiesInEnsemblesOfRegionalReanalysisProjectUerra,
            9 => Self::UncertaintiesInEnsemblesOfRegionalReanalysisProjectUerraTest,
            10 => Self::CopernicusRegionalReanalysis,
            11 => Self::CopernicusRegionalReanalysisTest,
            12 => Self::DestinationEarth,
            13 => Self::DestinationEarthTest,
            255 | _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table1_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::OperationalProducts => "Operational Products",
            Self::OperationalTestProducts => "Operational Test Products",
            Self::ResearchProducts => "Research Products",
            Self::ReAnalysisProducts => "Re-Analysis Products",
            Self::ThorpexInteractiveGrandGlobalEnsembleTigge => {
                "THORPEX Interactive Grand Global Ensemble (TIGGE)"
            }
            Self::ThorpexInteractiveGrandGlobalEnsembleTiggeTest => {
                "THORPEX Interactive Grand Global Ensemble (TIGGE) test"
            }
            Self::S2sOperationalProducts => "S2S Operational Products",
            Self::S2sTestProducts => "S2S Test Products",
            Self::UncertaintiesInEnsemblesOfRegionalReanalysisProjectUerra => {
                "Uncertainties in ensembles of regional reanalysis project (UERRA)"
            }
            Self::UncertaintiesInEnsemblesOfRegionalReanalysisProjectUerraTest => {
                "Uncertainties in ensembles of regional reanalysis project (UERRA) Test"
            }
            Self::CopernicusRegionalReanalysis => "Copernicus Regional Reanalysis",
            Self::CopernicusRegionalReanalysisTest => "Copernicus Regional Reanalysis Test",
            Self::DestinationEarth => "Destination Earth",
            Self::DestinationEarthTest => "Destination Earth test",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 1.4 - TYPE OF DATA
///
/// **Details**:
/// - **Section**: 1
/// - **Octet**: 21 (index 20)
///
/// **Reserved Ranges**:
/// - `9-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Values**:
/// - `192`: Experimental Products
/// - `255`: Missing
///
/// ## Description
/// This table defines the types of data in GRIB2 files,
/// providing context for interpreting the data's nature, whether operational, research, or experimental.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-4.shtml)
///
/// ## Notes
/// - Revised 08/23/2023
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table1_4 {
    AnalysisProducts = 0,
    ForecastProducts = 1,
    AnalysisAndForecastProducts = 2,
    ControlForecastProducts = 3,
    PerturbedForecastProducts = 4,
    ControlAndPerturbedForecastProducts = 5,
    ProcessedSatelliteObservations = 6,
    ProcessedRadarObservations = 7,
    EventProbability = 8,
    ExperimentalProducts = 192,
    Missing = 255,
}
impl From<u8> for Grib2Table1_4 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::AnalysisProducts,
            1 => Self::ForecastProducts,
            2 => Self::AnalysisAndForecastProducts,
            3 => Self::ControlForecastProducts,
            4 => Self::PerturbedForecastProducts,
            5 => Self::ControlAndPerturbedForecastProducts,
            6 => Self::ProcessedSatelliteObservations,
            7 => Self::ProcessedRadarObservations,
            8 => Self::EventProbability,
            192 => Self::ExperimentalProducts,
            255 | _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table1_4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::AnalysisProducts => "Analysis Products",
            Self::ForecastProducts => "Forecast Products",
            Self::AnalysisAndForecastProducts => "Analysis and Forecast Products",
            Self::ControlForecastProducts => "Control Forecast Products",
            Self::PerturbedForecastProducts => "Perturbed Forecast Products",
            Self::ControlAndPerturbedForecastProducts => "Control and Perturbed Forecast Products",
            Self::ProcessedSatelliteObservations => "Processed Satellite Observations",
            Self::ProcessedRadarObservations => "Processed Radar Observations",
            Self::EventProbability => "Event Probability",
            Self::ExperimentalProducts => "Experimental Products",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 1.5 - IDENTIFICATION TEMPLATE NUMBER
///
/// **Details**:
/// - **Section**: 1
/// - **Octet**: 21 (index 20)
///
/// **Reserved Ranges**:
/// - `3-32767`: Reserved
/// - `32768-65534`: Reserved for Local Use
///
/// **Special Value**:
/// - `65535`: Missing
///
/// ## Description
/// This table defines the identification template numbers in GRIB2 files,
/// providing context for interpreting the data's template classifications.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-5.shtml)
///
/// ## Notes
/// - Created 07/01/2014
#[repr(u16)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table1_5 {
    CalendarDefinition = 0,
    PaleontologicalOffset = 1,
    CalendarDefinitionAndPaleontologicalOffset = 2,
    Missing = 65535,
}
impl From<u16> for Grib2Table1_5 {
    fn from(val: u16) -> Self {
        match val {
            0 => Self::CalendarDefinition,
            1 => Self::PaleontologicalOffset,
            2 => Self::CalendarDefinitionAndPaleontologicalOffset,
            65535 | _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table1_5 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::CalendarDefinition => "Calendar Definition",
            Self::PaleontologicalOffset => "Paleontological Offset",
            Self::CalendarDefinitionAndPaleontologicalOffset => {
                "Calendar Definition and Paleontological Offset"
            }
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// # Table 1.6 - TYPE OF CALENDAR
///
/// **Details**:
/// - **Section**: 1
/// - **Octet**: 21 (index 20)
///
/// **Reserved Ranges**:
/// - `4-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Description
/// This table defines the types of calendars in GRIB2 files,
/// providing context for interpreting the data's calendar classifications.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-6.shtml)
///
/// ## Notes
/// - (1). Essentially a non-leap year
/// - (2). Extends the Gregorian calendar indefinitely in the past
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table1_6 {
    Gregorian = 0,
    Day360 = 1,
    Day365 = 2,             // (see Note 1)
    ProlepticGregorian = 3, // (see Note 2)
    Missing = 255,
}
impl From<u8> for Grib2Table1_6 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Gregorian,
            1 => Self::Day360,
            2 => Self::Day365,
            3 => Self::ProlepticGregorian,
            255 | _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table1_6 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::Gregorian => "Gregorian",
            Self::Day360 => "360-day",
            Self::Day365 => "365-day (see Note 1)",
            Self::ProlepticGregorian => "Proleptic Gregorian (see Note 2)",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}
