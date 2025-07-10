use alloc::string::String;

/// Categories track a name with it's units and abbreviation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCategory {
    /// The name of the category
    pub parameter: String,
    /// The units of the category
    pub units: String,
    /// The abbreviation of the category
    pub abbrev: String,
}
impl core::fmt::Display for TableCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ({}, {})", self.parameter, self.units, self.abbrev)
    }
}

/// # GRIB2 - CODE TABLE 4.0 - PRODUCT DEFINITION TEMPLATE NUMBER
///
/// **Details**:
/// - **Section**: 4
/// - **Octets**: 8-9
/// - **Revised**: 07/12/2024
///
/// **Reserved Ranges**:
/// - `16-19`: Reserved
/// - `21-29`: Reserved
/// - `36-39`: Reserved
/// - `50`: Reserved
/// - `52`: Reserved
/// - `64-66`: Reserved
/// - `69`: Reserved
/// - `74-75`: Reserved
/// - `128-253`: Reserved
/// - `255-999`: Reserved
/// - `1003-1099`: Reserved
/// - `1102-32767`: Reserved
/// - `32768-65534`: Reserved for Local Use
///
/// **Special Value**:
/// - `65535`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-0.shtml)
///
/// ## Notes
/// - Red text depicts changes made since 08/23/2023.
#[repr(u16)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table4_0 {
    AnalysisOrForecastAtHorizontalLevelOrLayerPointInTime = 0,
    IndividualEnsembleForecastAtHorizontalLevelOrLayerPointInTime = 1,
    DerivedForecastsBasedOnAllEnsembleMembersAtHorizontalLevelOrLayerPointInTime = 2,
    DerivedForecastsBasedOnClusterRectangularAreaAtHorizontalLevelOrLayerPointInTime = 3,
    DerivedForecastsBasedOnClusterCircularAreaAtHorizontalLevelOrLayerPointInTime = 4,
    ProbabilityForecastsAtHorizontalLevelOrLayerPointInTime = 5,
    PercentileForecastsAtHorizontalLevelOrLayerPointInTime = 6,
    AnalysisOrForecastErrorAtHorizontalLevelOrLayerPointInTime = 7,
    AverageAccumulationExtremeValuesStatisticallyProcessedContinuousNonContinuousTimeInterval = 8,
    ProbabilityForecastsAtHorizontalLevelOrLayerContinuousNonContinuousTimeInterval = 9,
    PercentileForecastsAtHorizontalLevelOrLayerContinuousNonContinuousTimeInterval = 10,
    IndividualEnsembleForecastContinuousNonContinuousTimeInterval = 11,
    DerivedForecastsAllEnsembleMembersContinuousNonContinuousTimeInterval = 12,
    DerivedForecastsClusterRectangularAreaContinuousNonContinuousTimeInterval = 13,
    DerivedForecastsClusterCircularAreaContinuousNonContinuousTimeInterval = 14,
    AverageAccumulationExtremeValuesStatisticallyProcessedSpatialAreaPointInTime = 15,
    AnalysisOrForecastSimulatedSatelliteData = 32,
    IndividualEnsembleForecastSimulatedSatelliteData = 33,
    IndividualEnsembleForecastContinuousNonContinuousIntervalSimulatedSatelliteData = 34,
    SatelliteProductWithOrWithoutQualityValues = 35,
    AnalysisOrForecastAtmosphericChemicalConstituents = 40,
    IndividualEnsembleForecastAtmosphericChemicalConstituents = 41,
    AverageAccumulationExtremeValuesStatisticallyProcessedAtmosphericChemicalConstituents = 42,
    IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAtmosphericChemicalConstituents =
        43,
    AnalysisOrForecastAerosol = 44,
    IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosol = 45,
    AverageAccumulationExtremeValuesStatisticallyProcessedAerosol = 46,
    IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosol2 = 47, // Note: Duplicate description, distinguished by value
    AnalysisOrForecastOpticalPropertiesAerosol = 48,
    IndividualEnsembleForecastOpticalPropertiesAerosol = 49,
    CategoricalForecastAtHorizontalLevelOrLayerPointInTime = 51,
    PartitionedParametersAtHorizontalLevelOrLayerPointInTime = 53,
    IndividualEnsembleForecastPartitionedParameters = 54,
    SpatioTemporalChangingTilesAtHorizontalLevelOrLayerPointInTime = 55,
    IndividualEnsembleForecastSpatioTemporalChangingTileParametersDeprecated = 56,
    AnalysisOrForecastAtmosphericChemicalConstituentsDistributionFunction = 57,
    IndividualEnsembleForecastAtmosphericChemicalConstituentsDistributionFunction = 58,
    IndividualEnsembleForecastSpatioTemporalChangingTileParametersCorrected = 59,
    IndividualEnsembleReforecastPointInTime = 60,
    IndividualEnsembleReforecastContinuousNonContinuousTimeInterval = 61,
    AverageAccumulationExtremeValuesStatisticallyProcessedSpatioTemporalChangingTiles = 62,
    IndividualEnsembleForecastSpatioTemporalChangingTiles = 63,
    AverageAccumulationExtremeValuesStatisticallyProcessedAtmosphericChemicalConstituentsDistributionFunction =
        67,
    IndividualEnsembleForecastAtmosphericChemicalConstituentsDistributionFunction2 = 68, // Note: Duplicate description, distinguished by value
    PostProcessingAnalysisOrForecastPointInTime = 70,
    PostProcessingIndividualEnsembleForecastPointInTime = 71,
    PostProcessingAverageAccumulationExtremeValuesStatisticallyProcessed = 72,
    PostProcessingIndividualEnsembleForecastContinuousNonContinuousTimeInterval = 73,
    AnalysisOrForecastAtmosphericChemicalConstituentsSourceSink = 76,
    IndividualEnsembleForecastAtmosphericChemicalConstituentsSourceSink = 77,
    AverageAccumulationExtremeValuesStatisticallyProcessedAtmosphericChemicalConstituentsSourceSink =
        78,
    IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAtmosphericChemicalConstituentsSourceSink =
        79,
    AnalysisOrForecastOpticalPropertiesAerosolSourceSink = 80,
    IndividualEnsembleForecastOpticalPropertiesAerosolSourceSink = 81,
    AverageAccumulationExtremeValuesStatisticallyProcessedAerosolSourceSink = 82,
    IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosolSourceSink = 83,
    IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosol3 = 84, // Note: Duplicate description, distinguished by value
    IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosolSourceSink2 = 85, // Note: Duplicate description, distinguished by value
    QuantileForecastsAtHorizontalLevelOrLayerPointInTime = 86,
    QuantileForecastsAtHorizontalLevelOrLayerContinuousNonContinuousTimeInterval = 87,
    AnalysisOrForecastAtHorizontalLevelOrLayerSpecifiedLocalTime = 88,
    PostProcessedQuantileForecastsPointInTime = 89,
    PostProcessedQuantileForecastsContinuousNonContinuousTimeInterval = 90,
    CategoricalForecastAtHorizontalLevelOrLayerContinuousNonContinuousTimeInterval = 91,
    IndividualEnsembleForecastSpecifiedLocalTime = 92,
    PostProcessingAnalysisOrForecastSpecifiedLocalTime = 93,
    PostProcessingIndividualEnsembleForecastSpecifiedLocalTime = 94,
    AverageAccumulationExtremeValuesStatisticallyProcessedSpecifiedLocalTime = 95,
    AverageAccumulationExtremeValuesStatisticallyProcessedIndividualEnsembleForecastSpecifiedLocalTime =
        96,
    AverageAccumulationExtremeValuesStatisticallyProcessedPostProcessingAnalysisOrForecastSpecifiedLocalTime =
        97,
    AverageAccumulationExtremeValuesStatisticallyProcessedPostProcessingIndividualEnsembleForecastSpecifiedLocalTime =
        98,
    AnalysisOrForecastWave2DSpectraExplicitList = 99,
    IndividualEnsembleForecastWave2DSpectraExplicitList = 100,
    AnalysisOrForecastWave2DSpectraFormulae = 101,
    IndividualEnsembleForecastWave2DSpectraFormulae = 102,
    AnalysisOrForecastWavesSelectedByPeriodRange = 103,
    IndividualEnsembleForecastWavesSelectedByPeriodRange = 104,
    AnomaliesSignificanceDerivedProductsAnalysisForecastReferencePeriod = 105,
    AnomaliesSignificanceDerivedProductsIndividualEnsembleForecastReferencePeriod = 106,
    AnomaliesSignificanceDerivedProductsDerivedForecastsAllEnsembleMembersReferencePeriod = 107,
    AnalysisOrForecastGenericOpticalProducts = 108,
    IndividualEnsembleForecastGenericOpticalProducts = 109,
    AverageAccumulationExtremeValuesStatisticallyProcessedGenericOpticalProducts = 110,
    IndividualEnsembleForecastContinuousNonContinuousIntervalGenericOpticalProducts = 111,
    AnomaliesSignificanceDerivedProductsProbabilityForecastsReferencePeriod = 112,
    GeneralizedTilesAtHorizontalLevelOrLayerPointInTime = 113,
    AverageAccumulationExtremeValuesStatisticallyProcessedGeneralizedTiles = 114,
    IndividualEnsembleForecastGeneralizedTiles = 115,
    IndividualEnsembleForecastGeneralizedTilesContinuousNonContinuousTimeInterval = 116,
    IndividualLargeEnsembleForecastPointInTime = 117,
    IndividualLargeEnsembleForecastContinuousNonContinuousInterval = 118,
    ProbabilityForecastsFromLargeEnsemblesPointInTime = 119,
    ProbabilityForecastsFromLargeEnsemblesContinuousNonContinuousTimeInterval = 120,
    ProbabilityForecastsLargeEnsemblesSpatiotemporalProcessingFocalStatisticsPointInTime = 121,
    ProbabilityForecastsLargeEnsemblesSpatiotemporalProcessingFocalStatisticsContinuousNonContinuousTimeInterval =
        122,
    ProbabilityForecastsLargeEnsemblesSpatiotemporalProcessingFocalStatisticsReferencePeriod = 123,
    AnalysisOrForecastRadionuclides = 124,
    IndividualEnsembleForecastRadionuclides = 125,
    AverageAccumulationExtremeValuesStatisticallyProcessedRadionuclides = 126,
    IndividualEnsembleForecastContinuousNonContinuousTimeIntervalRadionuclides = 127,
    CcittIa5CharacterString = 254,
    CrossSectionAnalysisAndForecastPointInTime = 1000,
    CrossSectionAveragedStatisticallyProcessedAnalysisOrForecastTimeRange = 1001,
    CrossSectionAnalysisAndForecastAveragedStatisticallyProcessedLatitudeLongitude = 1002,
    HovmollerTypeGridNoAveragingStatisticalProcessing = 1100,
    HovmollerTypeGridAveragingStatisticalProcessing = 1101,
    Missing = 65535,
}
impl From<u16> for Grib2Table4_0 {
    fn from(val: u16) -> Self {
        match val {
            0 => Self::AnalysisOrForecastAtHorizontalLevelOrLayerPointInTime,
            1 => Self::IndividualEnsembleForecastAtHorizontalLevelOrLayerPointInTime,
            2 => Self::DerivedForecastsBasedOnAllEnsembleMembersAtHorizontalLevelOrLayerPointInTime,
            3 => Self::DerivedForecastsBasedOnClusterRectangularAreaAtHorizontalLevelOrLayerPointInTime,
            4 => Self::DerivedForecastsBasedOnClusterCircularAreaAtHorizontalLevelOrLayerPointInTime,
            5 => Self::ProbabilityForecastsAtHorizontalLevelOrLayerPointInTime,
            6 => Self::PercentileForecastsAtHorizontalLevelOrLayerPointInTime,
            7 => Self::AnalysisOrForecastErrorAtHorizontalLevelOrLayerPointInTime,
            8 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedContinuousNonContinuousTimeInterval,
            9 => Self::ProbabilityForecastsAtHorizontalLevelOrLayerContinuousNonContinuousTimeInterval,
            10 => Self::PercentileForecastsAtHorizontalLevelOrLayerContinuousNonContinuousTimeInterval,
            11 => Self::IndividualEnsembleForecastContinuousNonContinuousTimeInterval,
            12 => Self::DerivedForecastsAllEnsembleMembersContinuousNonContinuousTimeInterval,
            13 => Self::DerivedForecastsClusterRectangularAreaContinuousNonContinuousTimeInterval,
            14 => Self::DerivedForecastsClusterCircularAreaContinuousNonContinuousTimeInterval,
            15 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedSpatialAreaPointInTime,
            32 => Self::AnalysisOrForecastSimulatedSatelliteData,
            33 => Self::IndividualEnsembleForecastSimulatedSatelliteData,
            34 => Self::IndividualEnsembleForecastContinuousNonContinuousIntervalSimulatedSatelliteData,
            35 => Self::SatelliteProductWithOrWithoutQualityValues,
            40 => Self::AnalysisOrForecastAtmosphericChemicalConstituents,
            41 => Self::IndividualEnsembleForecastAtmosphericChemicalConstituents,
            42 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedAtmosphericChemicalConstituents,
            43 => Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAtmosphericChemicalConstituents,
            44 => Self::AnalysisOrForecastAerosol,
            45 => Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosol,
            46 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedAerosol,
            47 => Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosol2,
            48 => Self::AnalysisOrForecastOpticalPropertiesAerosol,
            49 => Self::IndividualEnsembleForecastOpticalPropertiesAerosol,
            51 => Self::CategoricalForecastAtHorizontalLevelOrLayerPointInTime,
            53 => Self::PartitionedParametersAtHorizontalLevelOrLayerPointInTime,
            54 => Self::IndividualEnsembleForecastPartitionedParameters,
            55 => Self::SpatioTemporalChangingTilesAtHorizontalLevelOrLayerPointInTime,
            56 => Self::IndividualEnsembleForecastSpatioTemporalChangingTileParametersDeprecated,
            57 => Self::AnalysisOrForecastAtmosphericChemicalConstituentsDistributionFunction,
            58 => Self::IndividualEnsembleForecastAtmosphericChemicalConstituentsDistributionFunction,
            59 => Self::IndividualEnsembleForecastSpatioTemporalChangingTileParametersCorrected,
            60 => Self::IndividualEnsembleReforecastPointInTime,
            61 => Self::IndividualEnsembleReforecastContinuousNonContinuousTimeInterval,
            62 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedSpatioTemporalChangingTiles,
            63 => Self::IndividualEnsembleForecastSpatioTemporalChangingTiles,
            67 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedAtmosphericChemicalConstituentsDistributionFunction,
            68 => Self::IndividualEnsembleForecastAtmosphericChemicalConstituentsDistributionFunction2,
            70 => Self::PostProcessingAnalysisOrForecastPointInTime,
            71 => Self::PostProcessingIndividualEnsembleForecastPointInTime,
            72 => Self::PostProcessingAverageAccumulationExtremeValuesStatisticallyProcessed,
            73 => Self::PostProcessingIndividualEnsembleForecastContinuousNonContinuousTimeInterval,
            76 => Self::AnalysisOrForecastAtmosphericChemicalConstituentsSourceSink,
            77 => Self::IndividualEnsembleForecastAtmosphericChemicalConstituentsSourceSink,
            78 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedAtmosphericChemicalConstituentsSourceSink,
            79 => Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAtmosphericChemicalConstituentsSourceSink,
            80 => Self::AnalysisOrForecastOpticalPropertiesAerosolSourceSink,
            81 => Self::IndividualEnsembleForecastOpticalPropertiesAerosolSourceSink,
            82 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedAerosolSourceSink,
            83 => Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosolSourceSink,
            84 => Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosol3,
            85 => Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosolSourceSink2,
            86 => Self::QuantileForecastsAtHorizontalLevelOrLayerPointInTime,
            87 => Self::QuantileForecastsAtHorizontalLevelOrLayerContinuousNonContinuousTimeInterval,
            88 => Self::AnalysisOrForecastAtHorizontalLevelOrLayerSpecifiedLocalTime,
            89 => Self::PostProcessedQuantileForecastsPointInTime,
            90 => Self::PostProcessedQuantileForecastsContinuousNonContinuousTimeInterval,
            91 => Self::CategoricalForecastAtHorizontalLevelOrLayerContinuousNonContinuousTimeInterval,
            92 => Self::IndividualEnsembleForecastSpecifiedLocalTime,
            93 => Self::PostProcessingAnalysisOrForecastSpecifiedLocalTime,
            94 => Self::PostProcessingIndividualEnsembleForecastSpecifiedLocalTime,
            95 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedSpecifiedLocalTime,
            96 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedIndividualEnsembleForecastSpecifiedLocalTime,
            97 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedPostProcessingAnalysisOrForecastSpecifiedLocalTime,
            98 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedPostProcessingIndividualEnsembleForecastSpecifiedLocalTime,
            99 => Self::AnalysisOrForecastWave2DSpectraExplicitList,
            100 => Self::IndividualEnsembleForecastWave2DSpectraExplicitList,
            101 => Self::AnalysisOrForecastWave2DSpectraFormulae,
            102 => Self::IndividualEnsembleForecastWave2DSpectraFormulae,
            103 => Self::AnalysisOrForecastWavesSelectedByPeriodRange,
            104 => Self::IndividualEnsembleForecastWavesSelectedByPeriodRange,
            105 => Self::AnomaliesSignificanceDerivedProductsAnalysisForecastReferencePeriod,
            106 => Self::AnomaliesSignificanceDerivedProductsIndividualEnsembleForecastReferencePeriod,
            107 => Self::AnomaliesSignificanceDerivedProductsDerivedForecastsAllEnsembleMembersReferencePeriod,
            108 => Self::AnalysisOrForecastGenericOpticalProducts,
            109 => Self::IndividualEnsembleForecastGenericOpticalProducts,
            110 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedGenericOpticalProducts,
            111 => Self::IndividualEnsembleForecastContinuousNonContinuousIntervalGenericOpticalProducts,
            112 => Self::AnomaliesSignificanceDerivedProductsProbabilityForecastsReferencePeriod,
            113 => Self::GeneralizedTilesAtHorizontalLevelOrLayerPointInTime,
            114 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedGeneralizedTiles,
            115 => Self::IndividualEnsembleForecastGeneralizedTiles,
            116 => Self::IndividualEnsembleForecastGeneralizedTilesContinuousNonContinuousTimeInterval,
            117 => Self::IndividualLargeEnsembleForecastPointInTime,
            118 => Self::IndividualLargeEnsembleForecastContinuousNonContinuousInterval,
            119 => Self::ProbabilityForecastsFromLargeEnsemblesPointInTime,
            120 => Self::ProbabilityForecastsFromLargeEnsemblesContinuousNonContinuousTimeInterval,
            121 => Self::ProbabilityForecastsLargeEnsemblesSpatiotemporalProcessingFocalStatisticsPointInTime,
            122 => Self::ProbabilityForecastsLargeEnsemblesSpatiotemporalProcessingFocalStatisticsContinuousNonContinuousTimeInterval,
            123 => Self::ProbabilityForecastsLargeEnsemblesSpatiotemporalProcessingFocalStatisticsReferencePeriod,
            124 => Self::AnalysisOrForecastRadionuclides,
            125 => Self::IndividualEnsembleForecastRadionuclides,
            126 => Self::AverageAccumulationExtremeValuesStatisticallyProcessedRadionuclides,
            127 => Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalRadionuclides,
            254 => Self::CcittIa5CharacterString,
            1000 => Self::CrossSectionAnalysisAndForecastPointInTime,
            1001 => Self::CrossSectionAveragedStatisticallyProcessedAnalysisOrForecastTimeRange,
            1002 => Self::CrossSectionAnalysisAndForecastAveragedStatisticallyProcessedLatitudeLongitude,
            1100 => Self::HovmollerTypeGridNoAveragingStatisticalProcessing,
            1101 => Self::HovmollerTypeGridAveragingStatisticalProcessing,
            _ => Self::Missing,
        }
    }
}
impl core::fmt::Display for Grib2Table4_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::AnalysisOrForecastAtHorizontalLevelOrLayerPointInTime => "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time.",
            Self::IndividualEnsembleForecastAtHorizontalLevelOrLayerPointInTime => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time.",
            Self::DerivedForecastsBasedOnAllEnsembleMembersAtHorizontalLevelOrLayerPointInTime => "Derived forecasts based on all ensemble members at a horizontal level or in a horizontal layer at a point in time.",
            Self::DerivedForecastsBasedOnClusterRectangularAreaAtHorizontalLevelOrLayerPointInTime => "Derived forecasts based on a cluster of ensemble members over a rectangular area at a horizontal level or in a horizontal layer at a point in time.",
            Self::DerivedForecastsBasedOnClusterCircularAreaAtHorizontalLevelOrLayerPointInTime => "Derived forecasts based on a cluster of ensemble members over a circular area at a horizontal level or in a horizontal layer at a point in time.",
            Self::ProbabilityForecastsAtHorizontalLevelOrLayerPointInTime => "Probability forecasts at a horizontal level or in a horizontal layer at a point in time.",
            Self::PercentileForecastsAtHorizontalLevelOrLayerPointInTime => "Percentile forecasts at a horizontal level or in a horizontal layer at a point in time.",
            Self::AnalysisOrForecastErrorAtHorizontalLevelOrLayerPointInTime => "Analysis or forecast error at a horizontal level or in a horizontal layer at a point in time.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedContinuousNonContinuousTimeInterval => "Average, accumulation, extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::ProbabilityForecastsAtHorizontalLevelOrLayerContinuousNonContinuousTimeInterval => "Probability forecasts at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::PercentileForecastsAtHorizontalLevelOrLayerContinuousNonContinuousTimeInterval => "Percentile forecasts at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::IndividualEnsembleForecastContinuousNonContinuousTimeInterval => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval.",
            Self::DerivedForecastsAllEnsembleMembersContinuousNonContinuousTimeInterval => "Derived forecasts based on all ensemble members at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval.",
            Self::DerivedForecastsClusterRectangularAreaContinuousNonContinuousTimeInterval => "Derived forecasts based on a cluster of ensemble members over a rectangular area at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval.",
            Self::DerivedForecastsClusterCircularAreaContinuousNonContinuousTimeInterval => "Derived forecasts based on a cluster of ensemble members over a circular area at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedSpatialAreaPointInTime => "Average, accumulation, extreme values or other statistically-processed values over a spatial area at a horizontal level or in a horizontal layer at a point in time.",
            Self::AnalysisOrForecastSimulatedSatelliteData => "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for simulate (synthetic) satellite data.",
            Self::IndividualEnsembleForecastSimulatedSatelliteData => "Individual Ensemble Forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for simulated (synthetic) satellite data.",
            Self::IndividualEnsembleForecastContinuousNonContinuousIntervalSimulatedSatelliteData => "Individual Ensemble Forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous interval for simulated (synthetic) satellite data.",
            Self::SatelliteProductWithOrWithoutQualityValues => "Satellite product with or without associated quality values.",
            Self::AnalysisOrForecastAtmosphericChemicalConstituents => "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for atmospheric chemical constituents.",
            Self::IndividualEnsembleForecastAtmosphericChemicalConstituents => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for atmospheric chemical constituents.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedAtmosphericChemicalConstituents => "Average, accumulation, and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for atmospheric chemical constituents.",
            Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAtmosphericChemicalConstituents => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval for atmospheric chemical constituents.",
            Self::AnalysisOrForecastAerosol => "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for aerosol.",
            Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosol => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval for aerosol.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedAerosol => "Average, accumulation, and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for aerosol.",
            Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosol2 => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval for aerosol.",
            Self::AnalysisOrForecastOpticalPropertiesAerosol => "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for optical properties of aerosol.",
            Self::IndividualEnsembleForecastOpticalPropertiesAerosol => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for optical properties of aerosol.",
            Self::CategoricalForecastAtHorizontalLevelOrLayerPointInTime => "Categorical forecast at a horizontal level or in a horizontal layer at a point in time.",
            Self::PartitionedParametersAtHorizontalLevelOrLayerPointInTime => "Partitioned parameters at a horizontal level or horizontal layer at a point in time.",
            Self::IndividualEnsembleForecastPartitionedParameters => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for partitioned parameters.",
            Self::SpatioTemporalChangingTilesAtHorizontalLevelOrLayerPointInTime => "Spatio-temporal changing tiles at a horizontal level or horizontal layer at a point in time.",
            Self::IndividualEnsembleForecastSpatioTemporalChangingTileParametersDeprecated => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for spatio-temporal changing tile parameters (DEPRECATED).",
            Self::AnalysisOrForecastAtmosphericChemicalConstituentsDistributionFunction => "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for atmospheric chemical constituents based on a distribution function.",
            Self::IndividualEnsembleForecastAtmosphericChemicalConstituentsDistributionFunction => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time interval for atmospheric chemical constituents based on a distribution function.",
            Self::IndividualEnsembleForecastSpatioTemporalChangingTileParametersCorrected => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for spatio-temporal changing tile parameters (corrected version of template 4.56).",
            Self::IndividualEnsembleReforecastPointInTime => "Individual Ensemble Reforecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time.",
            Self::IndividualEnsembleReforecastContinuousNonContinuousTimeInterval => "Individual Ensemble Reforecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedSpatioTemporalChangingTiles => "Average, accumulation and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for spatio-temporal changing tiles at a horizontal level or horizontal layer at a point in time.",
            Self::IndividualEnsembleForecastSpatioTemporalChangingTiles => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for spatio-temporal changing tiles.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedAtmosphericChemicalConstituentsDistributionFunction => "Average, accumulation and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for atmospheric chemical constituents based on a distribution function.",
            Self::IndividualEnsembleForecastAtmosphericChemicalConstituentsDistributionFunction2 => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for atmospheric chemical constituents based on a distribution function.",
            Self::PostProcessingAnalysisOrForecastPointInTime => "Post-processing analysis or forecast at a horizontal level or in a horizontal layer at a point in time.",
            Self::PostProcessingIndividualEnsembleForecastPointInTime => "Post-processing individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time.",
            Self::PostProcessingAverageAccumulationExtremeValuesStatisticallyProcessed => "Post-processing average, accumulation, extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::PostProcessingIndividualEnsembleForecastContinuousNonContinuousTimeInterval => "Post-processing individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous time interval.",
            Self::AnalysisOrForecastAtmosphericChemicalConstituentsSourceSink => "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for atmospheric chemical constituents with source or sink.",
            Self::IndividualEnsembleForecastAtmosphericChemicalConstituentsSourceSink => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for atmospheric chemical constituents with source or sink.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedAtmosphericChemicalConstituentsSourceSink => "Average, accumulation, and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for atmospheric chemical constituents with source or sink.",
            Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAtmosphericChemicalConstituentsSourceSink => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for atmospheric chemical constituents with source or sink.",
            Self::AnalysisOrForecastOpticalPropertiesAerosolSourceSink => "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for optical properties of aerosol with source or sink.",
            Self::IndividualEnsembleForecastOpticalPropertiesAerosolSourceSink => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for optical properties of aerosol with source or sink.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedAerosolSourceSink => "Average, accumulation, and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for aerosol with source or sink.",
            Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosolSourceSink => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for aerosol with source or sink.",
            Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosol3 => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for aerosol.",
            Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalAerosolSourceSink2 => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for aerosol with source or sink.",
            Self::QuantileForecastsAtHorizontalLevelOrLayerPointInTime => "Quantile forecasts at a horizontal level or in a horizontal layer at a point in time.",
            Self::QuantileForecastsAtHorizontalLevelOrLayerContinuousNonContinuousTimeInterval => "Quantile forecasts at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::AnalysisOrForecastAtHorizontalLevelOrLayerSpecifiedLocalTime => "Analysis or forecast at a horizontal level or in a horizontal layer at a specified local time.",
            Self::PostProcessedQuantileForecastsPointInTime => "Post-processed quantile forecasts at a horizontal level or in a horizontal layer at a point in time.",
            Self::PostProcessedQuantileForecastsContinuousNonContinuousTimeInterval => "Post-processed quantile forecasts at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::CategoricalForecastAtHorizontalLevelOrLayerContinuousNonContinuousTimeInterval => "Categorical forecast at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::IndividualEnsembleForecastSpecifiedLocalTime => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a specified local time.",
            Self::PostProcessingAnalysisOrForecastSpecifiedLocalTime => "Post-processing analysis or forecast at a horizontal level or in a horizontal layer at a specified local time.",
            Self::PostProcessingIndividualEnsembleForecastSpecifiedLocalTime => "Post-processing individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a specified local time.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedSpecifiedLocalTime => "Average, accumulation, extreme values or other statistically processed values at a horizontal level or in a horizontal layer at a specified local time.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedIndividualEnsembleForecastSpecifiedLocalTime => "Average, accumulation, extreme values or other statistically processed values of an individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a specified local time.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedPostProcessingAnalysisOrForecastSpecifiedLocalTime => "Average, accumulation, extreme values or other statistically processed values of post-processing analysis or forecast at a horizontal level or in a horizontal layer at a specified local time.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedPostProcessingIndividualEnsembleForecastSpecifiedLocalTime => "Average, accumulation, extreme values or other statistically processed values of a post-processing individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a specified local time.",
            Self::AnalysisOrForecastWave2DSpectraExplicitList => "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for wave 2D spectra with explicit list of frequencies and directions.",
            Self::IndividualEnsembleForecastWave2DSpectraExplicitList => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for wave 2D spectra with explicit list of frequencies and directions.",
            Self::AnalysisOrForecastWave2DSpectraFormulae => "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for wave 2D spectra with frequencies and directions defined by formulae.",
            Self::IndividualEnsembleForecastWave2DSpectraFormulae => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for wave 2D spectra with frequencies and directions defined by formulae.",
            Self::AnalysisOrForecastWavesSelectedByPeriodRange => "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for waves selected by period range.",
            Self::IndividualEnsembleForecastWavesSelectedByPeriodRange => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for waves selected by period range.",
            Self::AnomaliesSignificanceDerivedProductsAnalysisForecastReferencePeriod => "Anomalies, significance and other derived products from an analysis or forecast in relation to a reference period at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::AnomaliesSignificanceDerivedProductsIndividualEnsembleForecastReferencePeriod => "Anomalies, significance and other derived products from an individual ensemble forecast, control and perturbed in relation to a reference period at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::AnomaliesSignificanceDerivedProductsDerivedForecastsAllEnsembleMembersReferencePeriod => "Anomalies, significance and other derived products from derived forecasts based on all ensemble members in relation to a reference period at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::AnalysisOrForecastGenericOpticalProducts => "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for generic optical products.",
            Self::IndividualEnsembleForecastGenericOpticalProducts => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for generic optical products.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedGenericOpticalProducts => "Average, accumulation, extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for generic optical products.",
            Self::IndividualEnsembleForecastContinuousNonContinuousIntervalGenericOpticalProducts => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous interval for generic optical products.",
            Self::AnomaliesSignificanceDerivedProductsProbabilityForecastsReferencePeriod => "Anomalies, significance and other derived products as probability forecasts in relation to a reference period at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::GeneralizedTilesAtHorizontalLevelOrLayerPointInTime => "Generalized tiles at a horizontal level or horizontal layer at a point in time.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedGeneralizedTiles => "Average, accumulation, and/or extreme values or other statistically processed values on generalized tiles at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::IndividualEnsembleForecastGeneralizedTiles => "Individual ensemble forecast, control and perturbed on generalized tiles at a horizontal level or in a horizontal layer at a point in time.",
            Self::IndividualEnsembleForecastGeneralizedTilesContinuousNonContinuousTimeInterval => "Individual ensemble forecast, control and perturbed on generalized tiles at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::IndividualLargeEnsembleForecastPointInTime => "Individual large ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time.",
            Self::IndividualLargeEnsembleForecastContinuousNonContinuousInterval => "Individual large ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer, in a continuous or non-continuous interval.",
            Self::ProbabilityForecastsFromLargeEnsemblesPointInTime => "Probability forecasts from large ensembles at a horizontal level or in a horizontal layer at a point in time.",
            Self::ProbabilityForecastsFromLargeEnsemblesContinuousNonContinuousTimeInterval => "Probability forecasts from large ensembles at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::ProbabilityForecastsLargeEnsemblesSpatiotemporalProcessingFocalStatisticsPointInTime => "Probability forecasts from large ensembles with spatiotemporal processing based on focal (moving window) statistics at a horizontal level or in a horizontal layer at a point in time.",
            Self::ProbabilityForecastsLargeEnsemblesSpatiotemporalProcessingFocalStatisticsContinuousNonContinuousTimeInterval => "Probability forecasts from large ensembles with spatiotemporal processing based on focal (moving window) statistics at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::ProbabilityForecastsLargeEnsemblesSpatiotemporalProcessingFocalStatisticsReferencePeriod => "Probability forecasts from large ensembles with spatiotemporal processing based on focal (moving window) statistics in relation to a reference period at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval.",
            Self::AnalysisOrForecastRadionuclides => "Analysis or forecast at a horizontal level or in a horizontal layer at a point in time for radionuclides.",
            Self::IndividualEnsembleForecastRadionuclides => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer at a point in time for radionuclides.",
            Self::AverageAccumulationExtremeValuesStatisticallyProcessedRadionuclides => "Average, accumulation, and/or extreme values or other statistically processed values at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for radionuclides.",
            Self::IndividualEnsembleForecastContinuousNonContinuousTimeIntervalRadionuclides => "Individual ensemble forecast, control and perturbed, at a horizontal level or in a horizontal layer in a continuous or non-continuous time interval for radionuclides.",
            Self::CcittIa5CharacterString => "CCITT IA5 character string.",
            Self::CrossSectionAnalysisAndForecastPointInTime => "Cross-section of analysis and forecast at a point in time.",
            Self::CrossSectionAveragedStatisticallyProcessedAnalysisOrForecastTimeRange => "Cross-section of averaged or otherwise statistically processed analysis or forecast over a range of time.",
            Self::CrossSectionAnalysisAndForecastAveragedStatisticallyProcessedLatitudeLongitude => "Cross-section of analysis and forecast, averaged or otherwise statistically-processed over latitude or longitude.",
            Self::HovmollerTypeGridNoAveragingStatisticalProcessing => "Hovmoller-type grid with no averaging or other statistical processing.",
            Self::HovmollerTypeGridAveragingStatisticalProcessing => "Hovmoller-type grid with averaging or other statistical processing.",
            Self::Missing => "Missing",
        };
        f.write_str(desc)
    }
}

/// GRIB2 - CODE TABLE 4.1: PARAMETER CATEGORY BY PRODUCT DISCIPLINE
///
/// **Created**: 12/07/2023
/// **Revised**: 12/07/2023 (Red text depicts changes made since 10/30/2023)
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-1.shtml)
///
/// ## Notes
/// - The disciplines are given in Section 0, Octet 7 of the GRIB2 message and are defined in Table 0.0.
/// - When a new category is to be added to Code table 4.1 and more than one discipline applies,
///   the choice of discipline should be made based on the intended use of the product.
///
/// This function provides a lookup for GRIB2 parameter categories based on the product
/// discipline and category codes.
///
/// # Arguments
/// * `discipline` - The product discipline (u8).
/// * `category` - The parameter category within the discipline (u8).
///
/// # Returns
/// A `String` containing the description of the parameter category.
/// Returns "Missing" if the discipline or category is not found or is a special value.
pub fn grib2_lookup_table4_1(discipline: u8, category: u8) -> String {
    match discipline {
        // Product Discipline 0 - Meteorological Products
        0 => match category {
            0 => String::from("Temperature (see Table 4.2-0-0)"),
            1 => String::from("Moisture (see Table 4.2-0-1)"),
            2 => String::from("Momentum (see Table 4.2-0-2)"),
            3 => String::from("Mass (see Table 4.2-0-3)"),
            4 => String::from("Short-wave radiation (see Table 4.2-0-4)"),
            5 => String::from("Long-wave radiation (see Table 4.2-0-5)"),
            6 => String::from("Cloud (see Table 4.2-0-6)"),
            7 => String::from("Thermodynamic Stability indices (see Table 4.2-0-7)"),
            8 => String::from("Kinematic Stability indices"),
            9 => String::from("Temperature Probabilities*"),
            10 => String::from("Moisture Probabilities*"),
            11 => String::from("Momentum Probabilities*"),
            12 => String::from("Mass Probabilities*"),
            13 => String::from("Aerosols (see Table 4.2-0-13)"),
            14 => String::from("Trace gases (e.g. Ozone, CO2) (see Table 4.2-0-14)"),
            15 => String::from("Radar (see Table 4.2-0-15)"),
            16 => String::from("Forecast Radar Imagery (see Table 4.2-0-16)"),
            17 => String::from("Electrodynamics (see Table 4.2-0-17)"),
            18 => String::from("Nuclear/radiology (see Table 4.2-0-18)"),
            19 => String::from("Physical atmospheric properties (see Table 4.2-0-19)"),
            20 => String::from("Atmospheric chemical Constituents (see Table 4.2-0-20)"),
            21 => String::from("Thermodynamic Properties (see Table 4.2-0-21)"),
            22 => String::from("Drought Indices (see Table 4.2-0-22)"),
            // 23-189 Reserved
            190 => String::from("CCITT IA5 string (see Table 4.2-0-190)"),
            191 => String::from("Miscellaneous (see Table 4.2-0-191)"),
            // 192-254 Reserved for Local Use
            192 => String::from("Covariance (see Table 4.2-0-192)"),
            255 => String::from("Missing"),
            23..=189 => String::from("Reserved"),
            193..=254 => String::from("Reserved for Local Use"),
        },
        // Product Discipline 1 - Hydrological Products
        1 => match category {
            0 => String::from("Hydrology basic products (see Table 4.2-1-0)"),
            1 => String::from("Hydrology probabilities (see Table 4.2-1-1)"),
            2 => String::from("Inland water and sediment properties (see Table 4.2-1-2)"),
            255 => String::from("Missing"),
            3..=191 => String::from("Reserved"),
            192..=254 => String::from("Reserved for Local Use"),
        },
        // Product Discipline 2 - Land Surface Products
        2 => match category {
            0 => String::from("Vegetation/Biomass (see Table 4.2-2-0)"),
            1 => String::from("Agricultural/Aquacultural Special Products (see Table 4.2-2-1)"),
            2 => String::from("Transportation-related Products"),
            3 => String::from("Soil Products (see Table 4.2-2-3)"),
            4 => String::from("Fire Weather Products (see Table 4.2-2-4)"),
            5 => String::from("Land Surface Products (see Table 4.2-2-5)"),
            6 => String::from("Urban areas (see Table 4.2-2-6)"),
            255 => String::from("Missing"),
            7..=191 => String::from("Reserved"),
            192..=254 => String::from("Reserved for Local Use"),
        },
        // Product Discipline 3 - Satellite Remote Sensing Products
        3 => match category {
            0 => String::from("Image format products (See note 1) (see Table 4.2-3-0)"),
            1 => String::from("Quantitative products (See note 2) (see Table 4.2-3-1)"),
            2 => String::from("Cloud Properties (see Table 4.2-3-2)"),
            3 => String::from("Flight Rules Conditions (see Table 4.2-3-3)"),
            4 => String::from("Volcanic Ash (see Table 4.2-3-4)"),
            5 => String::from("Sea-surface Temperature (see Table 4.2-3-5)"),
            6 => String::from("Solar Radiation (see Table 4.2-3-6)"),
            192 => String::from("Forecast Satellite Imagery (See note 2) (see Table 4.2-3-192)"),
            255 => String::from("Missing"),
            7..=191 => String::from("Reserved"),
            193..=254 => String::from("Reserved for Local Use"),
        },
        // Product Discipline 4 - Space Weather Products
        4 => match category {
            0 => String::from("Temperature (see Table 4.2-4-0)"),
            1 => String::from("Momentum (see Table 4.2-4-1)"),
            2 => String::from("Charged Particle Mass and Number (see Table 4.2-4-2)"),
            3 => String::from("Electric and Magnetic Fields (see Table 4.2-4-3)"),
            4 => String::from("Energetic Particles (see Table 4.2-4-4)"),
            5 => String::from("Waves (see Table 4.2-4-5)"),
            6 => String::from("Solar Electromagnetic Emissions (see Table 4.2-4-6)"),
            7 => String::from("Terrestrial Electromagnetic Emissions (see Table 4.2-4-7)"),
            8 => String::from("Imagery (see Table 4.2-4-8)"),
            9 => String::from("Ion-Neutral Coupling (see Table 4.2-4-9)"),
            10 => String::from("Space Weather Indices (see Table 4.2-4-10)"),
            255 => String::from("Missing"),
            11..=191 => String::from("Reserved"),
            192..=254 => String::from("Reserved for Local Use"),
        },
        // Product Discipline 10 - Oceanographic Products
        10 => match category {
            0 => String::from("Waves (see Table 4.2-10-0)"),
            1 => String::from("Currents (see Table 4.2-10-1)"),
            2 => String::from("Ice (see Table 4.2-10-2)"),
            3 => String::from("Surface Properties (see Table 4.2-10-3)"),
            4 => String::from("Sub-surface Properties (see Table 4.2-10-4)"),
            191 => String::from("Miscellaneous (see Table 4.2-10-191)"),
            255 => String::from("Missing"),
            5..=190 => String::from("Reserved"),
            192..=254 => String::from("Reserved for Local Use"),
        },
        // Product Discipline 20 - Health and Socioeconomic impacts
        20 => match category {
            0 => String::from("Health Indicators (see Table 4.2-20-0)"),
            1 => String::from("Epidemiology (see Table 4.2-20-1)"),
            2 => String::from("Socioeconomic indicators (see Table 4.2-20-2)"),
            3 => String::from("Renewable energy sector (see Table 4.2-20-3)"),
            255 => String::from("Missing"),
            4..=191 => String::from("Reserved"),
            192..=254 => String::from("Reserved for Local Use"),
        },
        _ => String::from("Missing"), // Default for unknown disciplines
    }
}

/// # GRIB2 - CODE TABLE 4.2-0-0
///
/// **Classification**: Meteorological products, Temperature category
///
/// **Available forms**: GRIB2
///
/// **Defined area**: Meteorological parameters
///
/// **Alias**: N/A
///
/// **Domain**: Global
///
/// **Input type**: Numeric (GRIB2 octets)
///
/// **Output type**: Numeric (GRIB2 octets)
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2.shtml)
///
/// ## Required Parameters
/// - `Parameter`: The name of the meteorological parameter.
/// - `Units`: The units of measurement for the parameter.
/// - `Abbrev`: The abbreviation for the parameter.
///
/// ## Optional Parameters
/// - `Reserved`: Reserved values are placeholders for future use or non-used categories.
///
/// ## Notes
/// (1) Parameter deprecated. See Regulation 92.6.2 and use another parameter instead.
/// (2) Apparent temperature is the perceived outdoor temperature, caused by a combination of phenomena, such as air temperature, relative humidity, and wind speed.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 0.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 0 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_00(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Temperature"),
            units: String::from("K"),
            abbrev: String::from("TMP"),
        },
        1 => TableCategory {
            parameter: String::from("Virtual Temperature"),
            units: String::from("K"),
            abbrev: String::from("VTMP"),
        },
        2 => TableCategory {
            parameter: String::from("Potential Temperature"),
            units: String::from("K"),
            abbrev: String::from("POT"),
        },
        3 => TableCategory {
            parameter: String::from(
                "Pseudo-Adiabatic Potential Temperature (or Equivalent Potential Temperature)",
            ),
            units: String::from("K"),
            abbrev: String::from("EPOT"),
        },
        4 => TableCategory {
            parameter: String::from("Maximum Temperature"),
            units: String::from("K"),
            abbrev: String::from("TMAX"),
        },
        5 => TableCategory {
            parameter: String::from("Minimum Temperature"),
            units: String::from("K"),
            abbrev: String::from("TMIN"),
        },
        6 => TableCategory {
            parameter: String::from("Dew Point Temperature"),
            units: String::from("K"),
            abbrev: String::from("DPT"),
        },
        7 => TableCategory {
            parameter: String::from("Dew Point Depression (or Deficit)"),
            units: String::from("K"),
            abbrev: String::from("DEPR"),
        },
        8 => TableCategory {
            parameter: String::from("Lapse Rate"),
            units: String::from("K m-1"),
            abbrev: String::from("LAPR"),
        },
        9 => TableCategory {
            parameter: String::from("Temperature Anomaly"),
            units: String::from("K"),
            abbrev: String::from("TMPA"),
        },
        10 => TableCategory {
            parameter: String::from("Latent Heat Net Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("LHTFL"),
        },
        11 => TableCategory {
            parameter: String::from("Sensible Heat Net Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("SHTFL"),
        },
        12 => TableCategory {
            parameter: String::from("Heat Index"),
            units: String::from("K"),
            abbrev: String::from("HEATX"),
        },
        13 => TableCategory {
            parameter: String::from("Wind Chill Factor"),
            units: String::from("K"),
            abbrev: String::from("WCF"),
        },
        14 => TableCategory {
            parameter: String::from("Minimum Dew Point Depression"),
            units: String::from("K"),
            abbrev: String::from("MINDPD"),
        },
        15 => TableCategory {
            parameter: String::from("Virtual Potential Temperature"),
            units: String::from("K"),
            abbrev: String::from("VPTMP"),
        },
        16 => TableCategory {
            parameter: String::from("Snow Phase Change Heat Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("SNOHF"),
        },
        17 => TableCategory {
            parameter: String::from("Skin Temperature"),
            units: String::from("K"),
            abbrev: String::from("SKINT"),
        },
        18 => TableCategory {
            parameter: String::from("Snow Temperature (top of snow)"),
            units: String::from("K"),
            abbrev: String::from("SNOT"),
        },
        19 => TableCategory {
            parameter: String::from("Turbulent Transfer Coefficient for Heat"),
            units: String::from("Numeric"),
            abbrev: String::from("TTCHT"),
        },
        20 => TableCategory {
            parameter: String::from("Turbulent Diffusion Coefficient for Heat"),
            units: String::from("m2s-1"),
            abbrev: String::from("TDCHT"),
        },
        21 => TableCategory {
            parameter: String::from("Apparent Temperature"),
            units: String::from("K"),
            abbrev: String::from("APTMP"),
        },
        22 => TableCategory {
            parameter: String::from("Temperature Tendency due to Short-Wave Radiation"),
            units: String::from("K s-1"),
            abbrev: String::from("TTSWR"),
        },
        23 => TableCategory {
            parameter: String::from("Temperature Tendency due to Long-Wave Radiation"),
            units: String::from("K s-1"),
            abbrev: String::from("TTLWR"),
        },
        24 => TableCategory {
            parameter: String::from("Temperature Tendency due to Short-Wave Radiation, Clear Sky"),
            units: String::from("K s-1"),
            abbrev: String::from("TTSWRCS"),
        },
        25 => TableCategory {
            parameter: String::from("Temperature Tendency due to Long-Wave Radiation, Clear Sky"),
            units: String::from("K s-1"),
            abbrev: String::from("TTLWRCS"),
        },
        26 => TableCategory {
            parameter: String::from("Temperature Tendency due to parameterizations"),
            units: String::from("K s-1"),
            abbrev: String::from("TTPARM"),
        },
        27 => TableCategory {
            parameter: String::from("Wet Bulb Temperature"),
            units: String::from("K"),
            abbrev: String::from("WETBT"),
        },
        28 => TableCategory {
            parameter: String::from("Unbalanced Component of Temperature"),
            units: String::from("K"),
            abbrev: String::from("UCTMP"),
        },
        29 => TableCategory {
            parameter: String::from("Temperature Advection"),
            units: String::from("K s-1"),
            abbrev: String::from("TMPADV"),
        },
        30 => TableCategory {
            parameter: String::from("Latent Heat Net Flux Due to Evaporation"),
            units: String::from("W m-2"),
            abbrev: String::from("LHFLXE"),
        },
        31 => TableCategory {
            parameter: String::from("Latent Heat Net Flux Due to Sublimation"),
            units: String::from("W m-2"),
            abbrev: String::from("LHFLXS"),
        },
        32 => TableCategory {
            parameter: String::from("Wet-Bulb Potential Temperature"),
            units: String::from("K"),
            abbrev: String::from("WETBPT"),
        },
        192 => TableCategory {
            parameter: String::from("Snow Phase Change Heat Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("SNOHF"),
        },
        193 => TableCategory {
            parameter: String::from("Temperature Tendency by All Radiation"),
            units: String::from("K s-1"),
            abbrev: String::from("TTRAD"),
        },
        194 => TableCategory {
            parameter: String::from("Relative Error Variance"),
            units: String::from(""),
            abbrev: String::from("REV"),
        },
        195 => TableCategory {
            parameter: String::from("Large Scale Condensate Heating Rate"),
            units: String::from("K s-1"),
            abbrev: String::from("LRGHR"),
        },
        196 => TableCategory {
            parameter: String::from("Deep Convective Heating Rate"),
            units: String::from("K s-1"),
            abbrev: String::from("CNVHR"),
        },
        197 => TableCategory {
            parameter: String::from("Total Downward Heat Flux at Surface"),
            units: String::from("W m-2"),
            abbrev: String::from("THFLX"),
        },
        198 => TableCategory {
            parameter: String::from("Temperature Tendency by All Physics"),
            units: String::from("K s-1"),
            abbrev: String::from("TTDIA"),
        },
        199 => TableCategory {
            parameter: String::from("Temperature Tendency by Non-radiation Physics"),
            units: String::from("K s-1"),
            abbrev: String::from("TTPHY"),
        },
        200 => TableCategory {
            parameter: String::from("Standard Dev. of IR Temp. over 1x1 deg. area"),
            units: String::from("K"),
            abbrev: String::from("TSD1D"),
        },
        201 => TableCategory {
            parameter: String::from("Shallow Convective Heating Rate"),
            units: String::from("K s-1"),
            abbrev: String::from("SHAHR"),
        },
        202 => TableCategory {
            parameter: String::from("Vertical Diffusion Heating rate"),
            units: String::from("K s-1"),
            abbrev: String::from("VDFHR"),
        },
        203 => TableCategory {
            parameter: String::from("Potential Temperature at Top of Viscous Sublayer"),
            units: String::from("K"),
            abbrev: String::from("THZ0"),
        },
        204 => TableCategory {
            parameter: String::from("Tropical Cyclone Heat Potential"),
            units: String::from("J m-2 K"),
            abbrev: String::from("TCHP"),
        },
        205 => TableCategory {
            parameter: String::from("Effective Layer (EL) Temperature"),
            units: String::from("C"),
            abbrev: String::from("ELMELT"),
        },
        206 => TableCategory {
            parameter: String::from("Wet Bulb Globe Temperature"),
            units: String::from("K"),
            abbrev: String::from("WETGLBT"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        33..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - CODE TABLE 4.2-0-1
///
/// **Classification**: Meteorological products, Moisture category
///
/// **Available forms**: GRIB2
///
/// **Defined area**: Meteorological parameters
///
/// **Alias**: N/A
///
/// **Domain**: Global
///
/// **Input type**: Numeric (GRIB2 octets)
///
/// **Output type**: Numeric (GRIB2 octets)
///
/// **Used by**:
/// - Section 0, Octet 7 = 0
/// - Section 4, Octet 10 = 1
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2.shtml)
///
/// ## Required Parameters
/// - `Parameter`: The name of the meteorological parameter.
/// - `Units`: The units of measurement for the parameter.
/// - `Abbrev`: The abbreviation for the parameter.
///
/// ## Optional Parameters
/// - `Reserved`: Reserved values are placeholders for future use or non-used categories.
///
/// ## Notes
/// (1) Parameter deprecated - See Regulation 92.6.2 and use another parameter instead.
/// (2) Total precipitation/snowfall rate stands for the sum of convective and large-scale precipitation/snowfall rate.
/// (3) Statistical process 1 (Accumulation) does not change units. It is recommended to use another parameter with "rate" in its name and accumulation in PDT.
/// (4) The listed units for this parameter appear to be inappropriate for the potential evaporation rate. Instead, it is recommended to use parameter 143.
/// (5) Total solid precipitation includes the sum of all types of solid water, e.g. graupel, snow, and hail.
/// (6) Assuming a cloud containing a bi-modal ice particle distribution, "cloud ice" refers to the small particle mode, whereas the large mode is usually called "snow". ("Ice pellets", in contrast, may refer to the precipitation of sleet, formed from freezing raindrops or refreezing (partially) melted snowflakes, or the precipitation of small hail.)
/// (7) It is recommended to use Snow melt rate instead (discipline 2, category 0, number 41).
/// (8) It is recommended to use parameter 148.
/// (9) Snow evaporation is the accumulated amount of water that has evaporated from snow from within the snow-covered area of a grid-box.
///
/// Local Use Notes:
/// (A) The numeric value is a reference to a weather string and key table stored in the Local Use Section (Section 2) of the same GRIB2 message. See MDL Template 2.1 page and this page for more details.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 1.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 1 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_01(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Specific Humidity"),
            units: String::from("kg kg-1"),
            abbrev: String::from("SPFH"),
        },
        1 => TableCategory {
            parameter: String::from("Relative Humidity"),
            units: String::from("%"),
            abbrev: String::from("RH"),
        },
        2 => TableCategory {
            parameter: String::from("Humidity Mixing Ratio"),
            units: String::from("kg kg-1"),
            abbrev: String::from("MIXR"),
        },
        3 => TableCategory {
            parameter: String::from("Precipitable Water"),
            units: String::from("kg m-2"),
            abbrev: String::from("PWAT"),
        },
        4 => TableCategory {
            parameter: String::from("Vapour Pressure"),
            units: String::from("Pa"),
            abbrev: String::from("VAPP"),
        },
        5 => TableCategory {
            parameter: String::from("Saturation Deficit"),
            units: String::from("Pa"),
            abbrev: String::from("SATD"),
        },
        6 => TableCategory {
            parameter: String::from("Evaporation"),
            units: String::from("kg m-2"),
            abbrev: String::from("EVP"),
        },
        7 => TableCategory {
            parameter: String::from("Precipitation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("PRATE"),
        },
        8 => TableCategory {
            parameter: String::from("Total Precipitation"),
            units: String::from("kg m-2"),
            abbrev: String::from("APCP"),
        },
        9 => TableCategory {
            parameter: String::from("Large-Scale Precipitation (non-convective)"),
            units: String::from("kg m-2"),
            abbrev: String::from("NCPCP"),
        },
        10 => TableCategory {
            parameter: String::from("Convective Precipitation"),
            units: String::from("kg m-2"),
            abbrev: String::from("ACPCP"),
        },
        11 => TableCategory {
            parameter: String::from("Snow Depth"),
            units: String::from("m"),
            abbrev: String::from("SNOD"),
        },
        12 => TableCategory {
            parameter: String::from("Snowfall Rate Water Equivalent"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("SRWEQ"),
        },
        13 => TableCategory {
            parameter: String::from("Water Equivalent of Accumulated Snow Depth"),
            units: String::from("kg m-2"),
            abbrev: String::from("WEASD"),
        },
        14 => TableCategory {
            parameter: String::from("Convective Snow"),
            units: String::from("kg m-2"),
            abbrev: String::from("SNOC"),
        },
        15 => TableCategory {
            parameter: String::from("Large-Scale Snow"),
            units: String::from("kg m-2"),
            abbrev: String::from("SNOL"),
        },
        16 => TableCategory {
            parameter: String::from("Snow Melt"),
            units: String::from("kg m-2"),
            abbrev: String::from("SNOM"),
        },
        17 => TableCategory {
            parameter: String::from("Snow Age"),
            units: String::from("day"),
            abbrev: String::from("SNOAG"),
        },
        18 => TableCategory {
            parameter: String::from("Absolute Humidity"),
            units: String::from("kg m-3"),
            abbrev: String::from("ABSH"),
        },
        19 => TableCategory {
            parameter: String::from("Precipitation Type"),
            units: String::from("See Table 4.201"),
            abbrev: String::from("PTYPE"),
        },
        20 => TableCategory {
            parameter: String::from("Integrated Liquid Water"),
            units: String::from("kg m-2"),
            abbrev: String::from("ILIQW"),
        },
        21 => TableCategory {
            parameter: String::from("Condensate"),
            units: String::from("kg kg-1"),
            abbrev: String::from("TCOND"),
        },
        22 => TableCategory {
            parameter: String::from("Cloud Mixing Ratio"),
            units: String::from("kg kg-1"),
            abbrev: String::from("CLMR"),
        },
        23 => TableCategory {
            parameter: String::from("Ice Water Mixing Ratio"),
            units: String::from("kg kg-1"),
            abbrev: String::from("ICMR"),
        },
        24 => TableCategory {
            parameter: String::from("Rain Mixing Ratio"),
            units: String::from("kg kg-1"),
            abbrev: String::from("RWMR"),
        },
        25 => TableCategory {
            parameter: String::from("Snow Mixing Ratio"),
            units: String::from("kg kg-1"),
            abbrev: String::from("SNMR"),
        },
        26 => TableCategory {
            parameter: String::from("Horizontal Moisture Convergence"),
            units: String::from("kg kg-1 s-1"),
            abbrev: String::from("MCONV"),
        },
        27 => TableCategory {
            parameter: String::from("Maximum Relative Humidity"),
            units: String::from("%"),
            abbrev: String::from("MAXRH"),
        },
        28 => TableCategory {
            parameter: String::from("Maximum Absolute Humidity"),
            units: String::from("kg m-3"),
            abbrev: String::from("MAXAH"),
        },
        29 => TableCategory {
            parameter: String::from("Total Snowfall"),
            units: String::from("m"),
            abbrev: String::from("ASNOW"),
        },
        30 => TableCategory {
            parameter: String::from("Precipitable Water Category"),
            units: String::from("See Table 4.202"),
            abbrev: String::from("PWCAT"),
        },
        31 => TableCategory {
            parameter: String::from("Hail"),
            units: String::from("m"),
            abbrev: String::from("HAIL"),
        },
        32 => TableCategory {
            parameter: String::from("Graupel"),
            units: String::from("kg kg-1"),
            abbrev: String::from("GRLE"),
        },
        33 => TableCategory {
            parameter: String::from("Categorical Rain"),
            units: String::from("Code table 4.222"),
            abbrev: String::from("CRAIN"),
        },
        34 => TableCategory {
            parameter: String::from("Categorical Freezing Rain"),
            units: String::from("Code table 4.222"),
            abbrev: String::from("CFRZR"),
        },
        35 => TableCategory {
            parameter: String::from("Categorical Ice Pellets"),
            units: String::from("Code table 4.222"),
            abbrev: String::from("CICEP"),
        },
        36 => TableCategory {
            parameter: String::from("Categorical Snow"),
            units: String::from("Code table 4.222"),
            abbrev: String::from("CSNOW"),
        },
        37 => TableCategory {
            parameter: String::from("Convective Precipitation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CPRAT"),
        },
        38 => TableCategory {
            parameter: String::from("Horizontal Moisture Divergence"),
            units: String::from("kg kg-1 s-1"),
            abbrev: String::from("MDIVER"),
        },
        39 => TableCategory {
            parameter: String::from("Percent frozen precipitation"),
            units: String::from("%"),
            abbrev: String::from("CPOFP"),
        },
        40 => TableCategory {
            parameter: String::from("Potential Evaporation"),
            units: String::from("kg m-2"),
            abbrev: String::from("PEVAP"),
        },
        41 => TableCategory {
            parameter: String::from("Potential Evaporation Rate"),
            units: String::from("W m-2"),
            abbrev: String::from("PEVPR"),
        },
        42 => TableCategory {
            parameter: String::from("Snow Cover"),
            units: String::from("%"),
            abbrev: String::from("SNOWC"),
        },
        43 => TableCategory {
            parameter: String::from("Rain Fraction of Total Cloud Water"),
            units: String::from("Proportion"),
            abbrev: String::from("FRAIN"),
        },
        44 => TableCategory {
            parameter: String::from("Rime Factor"),
            units: String::from("Numeric"),
            abbrev: String::from("RIME"),
        },
        45 => TableCategory {
            parameter: String::from("Total Column Integrated Rain"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLR"),
        },
        46 => TableCategory {
            parameter: String::from("Total Column Integrated Snow"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLS"),
        },
        47 => TableCategory {
            parameter: String::from("Large Scale Water Precipitation"),
            units: String::from("kg m-2"),
            abbrev: String::from("LSWP"),
        },
        48 => TableCategory {
            parameter: String::from("Convective Water Precipitation"),
            units: String::from("kg m-2"),
            abbrev: String::from("CWP"),
        },
        49 => TableCategory {
            parameter: String::from("Total Water Precipitation"),
            units: String::from("kg m-2"),
            abbrev: String::from("TWATP"),
        },
        50 => TableCategory {
            parameter: String::from("Total Snow Precipitation"),
            units: String::from("kg m-2"),
            abbrev: String::from("TSNOWP"),
        },
        51 => TableCategory {
            parameter: String::from("Total Column Water"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCWAT"),
        },
        52 => TableCategory {
            parameter: String::from("Total Precipitation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("TPRATE"),
        },
        53 => TableCategory {
            parameter: String::from("Total Snowfall Rate Water Equivalent"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("TSRWE"),
        },
        54 => TableCategory {
            parameter: String::from("Large Scale Precipitation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("LSPRATE"),
        },
        55 => TableCategory {
            parameter: String::from("Convective Snowfall Rate Water Equivalent"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CSRWE"),
        },
        56 => TableCategory {
            parameter: String::from("Large Scale Snowfall Rate Water Equivalent"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("LSSRWE"),
        },
        57 => TableCategory {
            parameter: String::from("Total Snowfall Rate"),
            units: String::from("m s-1"),
            abbrev: String::from("TSRATE"),
        },
        58 => TableCategory {
            parameter: String::from("Convective Snowfall Rate"),
            units: String::from("m s-1"),
            abbrev: String::from("CSRATE"),
        },
        59 => TableCategory {
            parameter: String::from("Large Scale Snowfall Rate"),
            units: String::from("m s-1"),
            abbrev: String::from("LSSRATE"),
        },
        60 => TableCategory {
            parameter: String::from("Snow Depth Water Equivalent"),
            units: String::from("kg m-2"),
            abbrev: String::from("SDWE"),
        },
        61 => TableCategory {
            parameter: String::from("Snow Density"),
            units: String::from("kg m-3"),
            abbrev: String::from("SDEN"),
        },
        62 => TableCategory {
            parameter: String::from("Snow Evaporation"),
            units: String::from("kg m-2"),
            abbrev: String::from("SEVAP"),
        },
        63 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        }, // Reserved
        64 => TableCategory {
            parameter: String::from("Total Column Integrated Water Vapour"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCIWV"),
        },
        65 => TableCategory {
            parameter: String::from("Rain Precipitation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("RPRATE"),
        },
        66 => TableCategory {
            parameter: String::from("Snow Precipitation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("SPRATE"),
        },
        67 => TableCategory {
            parameter: String::from("Freezing Rain Precipitation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("FPRATE"),
        },
        68 => TableCategory {
            parameter: String::from("Ice Pellets Precipitation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("IPRATE"),
        },
        69 => TableCategory {
            parameter: String::from("Total Column Integrated Cloud Water"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLW"),
        },
        70 => TableCategory {
            parameter: String::from("Total Column Integrated Cloud Ice"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLI"),
        },
        71 => TableCategory {
            parameter: String::from("Hail Mixing Ratio"),
            units: String::from("kg kg-1"),
            abbrev: String::from("HAILMXR"),
        },
        72 => TableCategory {
            parameter: String::from("Total Column Integrated Hail"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLH"),
        },
        73 => TableCategory {
            parameter: String::from("Hail Precipitation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("HAILPR"),
        },
        74 => TableCategory {
            parameter: String::from("Total Column Integrated Graupel"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLG"),
        },
        75 => TableCategory {
            parameter: String::from("Graupel (Snow Pellets) Precipitation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("GPRATE"),
        },
        76 => TableCategory {
            parameter: String::from("Convective Rain Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CRRATE"),
        },
        77 => TableCategory {
            parameter: String::from("Large Scale Rain Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("LSRRATE"),
        },
        78 => TableCategory {
            parameter: String::from(
                "Total Column Integrated Water (All components including precipitation)",
            ),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLWA"),
        },
        79 => TableCategory {
            parameter: String::from("Evaporation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("EVARATE"),
        },
        80 => TableCategory {
            parameter: String::from("Total Condensate"),
            units: String::from("kg kg-1"),
            abbrev: String::from("TOTCON"),
        },
        81 => TableCategory {
            parameter: String::from("Total Column-Integrated Condensate"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCICON"),
        },
        82 => TableCategory {
            parameter: String::from("Cloud Ice Mixing Ratio"),
            units: String::from("kg kg-1"),
            abbrev: String::from("CIMIXR"),
        },
        83 => TableCategory {
            parameter: String::from("Specific Cloud Liquid Water Content"),
            units: String::from("kg kg-1"),
            abbrev: String::from("SCLLWC"),
        },
        84 => TableCategory {
            parameter: String::from("Specific Cloud Ice Water Content"),
            units: String::from("kg kg-1"),
            abbrev: String::from("SCLIWC"),
        },
        85 => TableCategory {
            parameter: String::from("Specific Rain Water Content"),
            units: String::from("kg kg-1"),
            abbrev: String::from("SRAINW"),
        },
        86 => TableCategory {
            parameter: String::from("Specific Snow Water Content"),
            units: String::from("kg kg-1"),
            abbrev: String::from("SSNOWW"),
        },
        87 => TableCategory {
            parameter: String::from("Stratiform Precipitation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("STRPRATE"),
        },
        88 => TableCategory {
            parameter: String::from("Categorical Convective Precipitation"),
            units: String::from("Code table 4.222"),
            abbrev: String::from("CATCP"),
        },
        89 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        }, // Reserved
        90 => TableCategory {
            parameter: String::from("Total Kinematic Moisture Flux"),
            units: String::from("kg kg-1 m s-1"),
            abbrev: String::from("TKMFLX"),
        },
        91 => TableCategory {
            parameter: String::from("U-component (zonal) Kinematic Moisture Flux"),
            units: String::from("kg kg-1 m s-1"),
            abbrev: String::from("UKMFLX"),
        },
        92 => TableCategory {
            parameter: String::from("V-component (meridional) Kinematic Moisture Flux"),
            units: String::from("kg kg-1 m s-1"),
            abbrev: String::from("VKMFLX"),
        },
        93 => TableCategory {
            parameter: String::from("Relative Humidity With Respect to Water"),
            units: String::from("%"),
            abbrev: String::from("RHWATER"),
        },
        94 => TableCategory {
            parameter: String::from("Relative Humidity With Respect to Ice"),
            units: String::from("%"),
            abbrev: String::from("RHICE"),
        },
        95 => TableCategory {
            parameter: String::from("Freezing or Frozen Precipitation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("FZPRATE"),
        },
        96 => TableCategory {
            parameter: String::from("Mass Density of Rain"),
            units: String::from("kg m-3"),
            abbrev: String::from("MASSDR"),
        },
        97 => TableCategory {
            parameter: String::from("Mass Density of Snow"),
            units: String::from("kg m-3"),
            abbrev: String::from("MASSDS"),
        },
        98 => TableCategory {
            parameter: String::from("Mass Density of Graupel"),
            units: String::from("kg m-3"),
            abbrev: String::from("MASSDG"),
        },
        99 => TableCategory {
            parameter: String::from("Mass Density of Hail"),
            units: String::from("kg m-3"),
            abbrev: String::from("MASSDH"),
        },
        100 => TableCategory {
            parameter: String::from("Specific Number Concentration of Rain"),
            units: String::from("kg-1"),
            abbrev: String::from("SPNCR"),
        },
        101 => TableCategory {
            parameter: String::from("Specific Number Concentration of Snow"),
            units: String::from("kg-1"),
            abbrev: String::from("SPNCS"),
        },
        102 => TableCategory {
            parameter: String::from("Specific Number Concentration of Graupel"),
            units: String::from("kg-1"),
            abbrev: String::from("SPNCG"),
        },
        103 => TableCategory {
            parameter: String::from("Specific Number Concentration of Hail"),
            units: String::from("kg-1"),
            abbrev: String::from("SPNCH"),
        },
        104 => TableCategory {
            parameter: String::from("Number Density of Rain"),
            units: String::from("m-3"),
            abbrev: String::from("NUMDR"),
        },
        105 => TableCategory {
            parameter: String::from("Number Density of Snow"),
            units: String::from("m-3"),
            abbrev: String::from("NUMDS"),
        },
        106 => TableCategory {
            parameter: String::from("Number Density of Graupel"),
            units: String::from("m-3"),
            abbrev: String::from("NUMDG"),
        },
        107 => TableCategory {
            parameter: String::from("Number Density of Hail"),
            units: String::from("m-3"),
            abbrev: String::from("NUMDH"),
        },
        108 => TableCategory {
            parameter: String::from("Specific Humidity Tendency due to Parameterizations"),
            units: String::from("kg kg-1 s-1"),
            abbrev: String::from("SHTPRM"),
        },
        109 => TableCategory {
            parameter: String::from(
                "Mass Density of Liquid Water Coating on Hail Expressed as Mass of Liquid Water per Unit Volume of Air",
            ),
            units: String::from("kg m-3"),
            abbrev: String::from("MDLWHVA"),
        },
        110 => TableCategory {
            parameter: String::from(
                "Specific Mass of Liquid Water Coating on Hail Expressed as Mass of Liquid Water per Unit Mass of Moist Air",
            ),
            units: String::from("kg kg-1"),
            abbrev: String::from("SMLWHMA"),
        },
        111 => TableCategory {
            parameter: String::from(
                "Mass Mixing Ratio of Liquid Water Coating on Hail Expressed as Mass of Liquid Water per Unit Mass of Dry Air",
            ),
            units: String::from("kg kg-1"),
            abbrev: String::from("MMLWHDA"),
        },
        112 => TableCategory {
            parameter: String::from(
                "Mass Density of Liquid Water Coating on Graupel Expressed as Mass of Liquid Water per Unit Volume of Air",
            ),
            units: String::from("kg m-3"),
            abbrev: String::from("MDLWGVA"),
        },
        113 => TableCategory {
            parameter: String::from(
                "Specific Mass of Liquid Water Coating on Graupel Expressed as Mass of Liquid Water per Unit Mass of Moist Air",
            ),
            units: String::from("kg kg-1"),
            abbrev: String::from("SMLWGMA"),
        },
        114 => TableCategory {
            parameter: String::from(
                "Mass Mixing Ratio of Liquid Water Coating on Graupel Expressed as Mass of Liquid Water per Unit Mass of Dry Air",
            ),
            units: String::from("kg kg-1"),
            abbrev: String::from("MMLWGDA"),
        },
        115 => TableCategory {
            parameter: String::from(
                "Mass Density of Liquid Water Coating on Snow Expressed as Mass of Liquid Water per Unit Volume of Air",
            ),
            units: String::from("kg m-3"),
            abbrev: String::from("MDLWSVA"),
        },
        116 => TableCategory {
            parameter: String::from(
                "Specific Mass of Liquid Water Coating on Snow Expressed as Mass of Liquid Water per Unit Mass of Moist Air",
            ),
            units: String::from("kg kg-1"),
            abbrev: String::from("SMLWSMA"),
        },
        117 => TableCategory {
            parameter: String::from(
                "Mass Mixing Ratio of Liquid Water Coating on Snow Expressed as Mass of Liquid Water per Unit Mass of Dry Air",
            ),
            units: String::from("kg kg-1"),
            abbrev: String::from("MMLWSDA"),
        },
        118 => TableCategory {
            parameter: String::from("Unbalanced Component of Specific Humidity"),
            units: String::from("kg kg-1"),
            abbrev: String::from("UNCSH"),
        },
        119 => TableCategory {
            parameter: String::from("Unbalanced Component of Specific Cloud Liquid Water content"),
            units: String::from("kg kg-1"),
            abbrev: String::from("UCSCLW"),
        },
        120 => TableCategory {
            parameter: String::from("Unbalanced Component of Specific Cloud Ice Water content"),
            units: String::from("kg kg-1"),
            abbrev: String::from("UCSCIW"),
        },
        121 => TableCategory {
            parameter: String::from("Fraction of Snow Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("FSNOWC"),
        },
        122 => TableCategory {
            parameter: String::from("Precipitation intensity index"),
            units: String::from("See Table 4.247"),
            abbrev: String::from("PIIDX"),
        },
        123 => TableCategory {
            parameter: String::from("Dominant precipitation type"),
            units: String::from("See Table 4.201"),
            abbrev: String::from("DPTYPE"),
        },
        124 => TableCategory {
            parameter: String::from("Presence of showers"),
            units: String::from("See Table 4.222"),
            abbrev: String::from("PSHOW"),
        },
        125 => TableCategory {
            parameter: String::from("Presence of blowing snow"),
            units: String::from("See Table 4.222"),
            abbrev: String::from("PBSNOW"),
        },
        126 => TableCategory {
            parameter: String::from("Presence of blizzard"),
            units: String::from("See Table 4.222"),
            abbrev: String::from("PBLIZZ"),
        },
        127 => TableCategory {
            parameter: String::from("Ice pellets (non-water equivalent) precipitation rate"),
            units: String::from("m s-1"),
            abbrev: String::from("ICEP"),
        },
        128 => TableCategory {
            parameter: String::from("Total solid precipitation rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("TSPRATE"),
        },
        129 => TableCategory {
            parameter: String::from("Effective Radius of Cloud Water"),
            units: String::from("m"),
            abbrev: String::from("EFRCWAT"),
        },
        130 => TableCategory {
            parameter: String::from("Effective Radius of Rain"),
            units: String::from("m"),
            abbrev: String::from("EFRRAIN"),
        },
        131 => TableCategory {
            parameter: String::from("Effective Radius of Cloud Ice"),
            units: String::from("m"),
            abbrev: String::from("EFRCICE"),
        },
        132 => TableCategory {
            parameter: String::from("Effective Radius of Snow"),
            units: String::from("m"),
            abbrev: String::from("EFRSNOW"),
        },
        133 => TableCategory {
            parameter: String::from("Effective Radius of Graupel"),
            units: String::from("m"),
            abbrev: String::from("EFRGRL"),
        },
        134 => TableCategory {
            parameter: String::from("Effective Radius of Hail"),
            units: String::from("m"),
            abbrev: String::from("EFRHAIL"),
        },
        135 => TableCategory {
            parameter: String::from("Effective Radius of Subgrid Liquid Clouds"),
            units: String::from("m"),
            abbrev: String::from("EFRSLC"),
        },
        136 => TableCategory {
            parameter: String::from("Effective Radius of Subgrid Ice Clouds"),
            units: String::from("m"),
            abbrev: String::from("EFRSICEC"),
        },
        137 => TableCategory {
            parameter: String::from("Effective Aspect Ratio of Rain"),
            units: String::from(""),
            abbrev: String::from("EFARRAIN"),
        },
        138 => TableCategory {
            parameter: String::from("Effective Aspect Ratio of Cloud Ice"),
            units: String::from(""),
            abbrev: String::from("EFARCICE"),
        },
        139 => TableCategory {
            parameter: String::from("Effective Aspect Ratio of Snow"),
            units: String::from(""),
            abbrev: String::from("EFARSNOW"),
        },
        140 => TableCategory {
            parameter: String::from("Effective Aspect Ratio of Graupel"),
            units: String::from(""),
            abbrev: String::from("EFARGRL"),
        },
        141 => TableCategory {
            parameter: String::from("Effective Aspect Ratio of Hail"),
            units: String::from(""),
            abbrev: String::from("EFARHAIL"),
        },
        142 => TableCategory {
            parameter: String::from("Effective Aspect Ratio of Subgrid Ice Clouds"),
            units: String::from(""),
            abbrev: String::from("EFARSIC"),
        },
        143 => TableCategory {
            parameter: String::from("Potential evaporation rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("PERATE"),
        },
        144 => TableCategory {
            parameter: String::from("Specific rain water content (convective)"),
            units: String::from("kg kg-1"),
            abbrev: String::from("SRWATERC"),
        },
        145 => TableCategory {
            parameter: String::from("Specific snow water content (convective)"),
            units: String::from("kg kg-1"),
            abbrev: String::from("SSNOWWC"),
        },
        146 => TableCategory {
            parameter: String::from("Cloud ice precipitation rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CICEPR"),
        },
        147 => TableCategory {
            parameter: String::from("Character of precipitation"),
            units: String::from("See Table 4.249"),
            abbrev: String::from("CHPRECIP"),
        },
        148 => TableCategory {
            parameter: String::from("Snow evaporation rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("SNOWERAT"),
        },
        149 => TableCategory {
            parameter: String::from("Cloud water mixing ratio"),
            units: String::from("kg kg-1"),
            abbrev: String::from("CWATERMR"),
        },
        150 => TableCategory {
            parameter: String::from("Column integrated eastward water vapour mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CEWVMF"),
        },
        151 => TableCategory {
            parameter: String::from("Column integrated northward water vapour mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CNWVMF"),
        },
        152 => TableCategory {
            parameter: String::from("Column integrated eastward cloud liquid water mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CECLWMF"),
        },
        153 => TableCategory {
            parameter: String::from("Column integrated northward cloud liquid water mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CNCLWMF"),
        },
        154 => TableCategory {
            parameter: String::from("Column integrated eastward cloud ice mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CECIMF"),
        },
        155 => TableCategory {
            parameter: String::from("Column integrated northward cloud ice mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CNCIMF"),
        },
        156 => TableCategory {
            parameter: String::from("Column integrated eastward rain mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CERMF"),
        },
        157 => TableCategory {
            parameter: String::from("Column integrated northward rain mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CNRMF"),
        },
        158 => TableCategory {
            parameter: String::from("Column integrated eastward snow mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CEFMF"),
        },
        159 => TableCategory {
            parameter: String::from("Column integrated northward snow mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CNSMF"),
        },
        160 => TableCategory {
            parameter: String::from("Column integrated divergence of water vapour mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CDWFMF"),
        },
        161 => TableCategory {
            parameter: String::from("Column integrated divergence of cloud liquid water mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CDCLWMF"),
        },
        162 => TableCategory {
            parameter: String::from("Column integrated divergence of cloud ice mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CDCIMF"),
        },
        163 => TableCategory {
            parameter: String::from("Column integrated divergence of rain mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CDRMF"),
        },
        164 => TableCategory {
            parameter: String::from("Column integrated divergence of snow mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CDSMF"),
        },
        165 => TableCategory {
            parameter: String::from("Column integrated divergence of total water mass flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CDTWMF"),
        },
        166 => TableCategory {
            parameter: String::from("Column integrated water vapour flux"),
            units: String::from("kg m-1s-1"),
            abbrev: String::from("CWVF"),
        },
        167 => TableCategory {
            parameter: String::from("Total column supercooled liquid water"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCSLW"),
        },
        168 => TableCategory {
            parameter: String::from("Saturation specific humidity with respect to water"),
            units: String::from("kg m-3"),
            abbrev: String::from("SSPFHW"),
        },
        169 => TableCategory {
            parameter: String::from(
                "Total column integrated saturation specific humidity with respect to water",
            ),
            units: String::from("kg m-2"),
            abbrev: String::from("TCISSPFHW"),
        },
        192 => TableCategory {
            parameter: String::from("Categorical Rain"),
            units: String::from("Code table 4.222"),
            abbrev: String::from("CRAIN"),
        },
        193 => TableCategory {
            parameter: String::from("Categorical Freezing Rain"),
            units: String::from("Code table 4.222"),
            abbrev: String::from("CFRZR"),
        },
        194 => TableCategory {
            parameter: String::from("Categorical Ice Pellets"),
            units: String::from("Code table 4.222"),
            abbrev: String::from("CICEP"),
        },
        195 => TableCategory {
            parameter: String::from("Categorical Snow"),
            units: String::from("Code table 4.222"),
            abbrev: String::from("CSNOW"),
        },
        196 => TableCategory {
            parameter: String::from("Convective Precipitation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CPRAT"),
        },
        197 => TableCategory {
            parameter: String::from("Horizontal Moisture Divergence"),
            units: String::from("kg kg-1 s-1"),
            abbrev: String::from("MDIV"),
        },
        198 => TableCategory {
            parameter: String::from("Minimum Relative Humidity"),
            units: String::from("%"),
            abbrev: String::from("MINRH"),
        },
        199 => TableCategory {
            parameter: String::from("Potential Evaporation"),
            units: String::from("kg m-2"),
            abbrev: String::from("PEVAP"),
        },
        200 => TableCategory {
            parameter: String::from("Potential Evaporation Rate"),
            units: String::from("W m-2"),
            abbrev: String::from("PEVPR"),
        },
        201 => TableCategory {
            parameter: String::from("Snow Cover"),
            units: String::from("%"),
            abbrev: String::from("SNOWC"),
        },
        202 => TableCategory {
            parameter: String::from("Rain Fraction of Total Liquid Water"),
            units: String::from("non-dim"),
            abbrev: String::from("FRAIN"),
        },
        203 => TableCategory {
            parameter: String::from("Rime Factor"),
            units: String::from("non-dim"),
            abbrev: String::from("RIME"),
        },
        204 => TableCategory {
            parameter: String::from("Total Column Integrated Rain"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLR"),
        },
        205 => TableCategory {
            parameter: String::from("Total Column Integrated Snow"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLS"),
        },
        206 => TableCategory {
            parameter: String::from("Total Icing Potential Diagnostic"),
            units: String::from("non-dim"),
            abbrev: String::from("TIPD"),
        },
        207 => TableCategory {
            parameter: String::from("Number concentration for ice particles"),
            units: String::from("non-dim"),
            abbrev: String::from("NCIP"),
        },
        208 => TableCategory {
            parameter: String::from("Snow temperature"),
            units: String::from("K"),
            abbrev: String::from("SNOT"),
        },
        209 => TableCategory {
            parameter: String::from("Total column-integrated supercooled liquid water"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCLSW"),
        },
        210 => TableCategory {
            parameter: String::from("Total column-integrated melting ice"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLM"),
        },
        211 => TableCategory {
            parameter: String::from("Evaporation - Precipitation"),
            units: String::from("cm/day"),
            abbrev: String::from("EMNP"),
        },
        212 => TableCategory {
            parameter: String::from("Sublimation (evaporation from snow)"),
            units: String::from("W m-2"),
            abbrev: String::from("SBSNO"),
        },
        213 => TableCategory {
            parameter: String::from("Deep Convective Moistening Rate"),
            units: String::from("kg kg-1 s-1"),
            abbrev: String::from("CNVMR"),
        },
        214 => TableCategory {
            parameter: String::from("Shallow Convective Moistening Rate"),
            units: String::from("kg kg-1 s-1"),
            abbrev: String::from("SHAMR"),
        },
        215 => TableCategory {
            parameter: String::from("Vertical Diffusion Moistening Rate"),
            units: String::from("kg kg-1 s-1"),
            abbrev: String::from("VDFMR"),
        },
        216 => TableCategory {
            parameter: String::from("Condensation Pressure of Parcali"),
            units: String::from("Pa"),
            abbrev: String::from("CONDP"),
        },
        217 => TableCategory {
            parameter: String::from("Large scale moistening rate"),
            units: String::from("kg kg-1 s-1"),
            abbrev: String::from("LRGMR"),
        },
        218 => TableCategory {
            parameter: String::from("Specific humidity at top of viscous sublayer"),
            units: String::from("kg kg-1"),
            abbrev: String::from("QZ0"),
        },
        219 => TableCategory {
            parameter: String::from("Maximum specific humidity at 2m"),
            units: String::from("kg kg-1"),
            abbrev: String::from("QMAX"),
        },
        220 => TableCategory {
            parameter: String::from("Minimum specific humidity at 2m"),
            units: String::from("kg kg-1"),
            abbrev: String::from("QMIN"),
        },
        221 => TableCategory {
            parameter: String::from("Liquid precipitation (Rainfall)"),
            units: String::from("kg m-2"),
            abbrev: String::from("ARAIN"),
        },
        222 => TableCategory {
            parameter: String::from("Snow temperature, depth-avg"),
            units: String::from("K"),
            abbrev: String::from("SNOWT"),
        },
        223 => TableCategory {
            parameter: String::from("Total precipitation (nearest grid point)"),
            units: String::from("kg m-2"),
            abbrev: String::from("APCPN"),
        },
        224 => TableCategory {
            parameter: String::from("Convective precipitation (nearest grid point)"),
            units: String::from("kg m-2"),
            abbrev: String::from("ACPCPN"),
        },
        225 => TableCategory {
            parameter: String::from("Freezing Rain"),
            units: String::from("kg m-2"),
            abbrev: String::from("FRZR"),
        },
        226 => TableCategory {
            parameter: String::from("Dominant Weather"),
            units: String::from("Numeric"),
            abbrev: String::from("PWTHER"),
        },
        227 => TableCategory {
            parameter: String::from("Frozen Rain"),
            units: String::from("kg m-2"),
            abbrev: String::from("FROZR"),
        },
        228 => TableCategory {
            parameter: String::from("Flat Ice Accumulation (FRAM)"),
            units: String::from("kg m-2"),
            abbrev: String::from("FICEAC"),
        },
        229 => TableCategory {
            parameter: String::from("Line Ice Accumulation (FRAM)"),
            units: String::from("kg m-2"),
            abbrev: String::from("LICEAC"),
        },
        230 => TableCategory {
            parameter: String::from("Sleet Accumulation"),
            units: String::from("kg m-2"),
            abbrev: String::from("SLACC"),
        },
        231 => TableCategory {
            parameter: String::from("Precipitation Potential Index"),
            units: String::from("%"),
            abbrev: String::from("PPINDX"),
        },
        232 => TableCategory {
            parameter: String::from("Probability Cloud Ice Present"),
            units: String::from("%"),
            abbrev: String::from("PROBCIP"),
        },
        233 => TableCategory {
            parameter: String::from("Snow Liquid Ratio"),
            units: String::from("kg kg-1"),
            abbrev: String::from("SNOWLR"),
        },
        234 => TableCategory {
            parameter: String::from("Precipitation Duration"),
            units: String::from("hour"),
            abbrev: String::from("PCPDUR"),
        },
        235 => TableCategory {
            parameter: String::from("Cloud Liquid Mixing Ratio"),
            units: String::from("kg kg-1"),
            abbrev: String::from("CLLMR"),
        },
        241 => TableCategory {
            parameter: String::from("Total Snow"),
            units: String::from("kg m-2"),
            abbrev: String::from("TSNOW"),
        },
        242 => TableCategory {
            parameter: String::from("Relative Humidity with Respect to Precipitable Water"),
            units: String::from("%"),
            abbrev: String::from("RHPW"),
        },
        245 => TableCategory {
            parameter: String::from("Hourly Maximum of Column Vertical Integrated Graupel"),
            units: String::from("kg m-2"),
            abbrev: String::from("MAXVIG"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        170..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        236..=240 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-0-2
///
/// **Classification**: Meteorological products, Momentum category
///
/// **Available forms**: Numerical values
///
/// **Defined area**: Meteorological domain
///
/// **Alias**: Momentum, wind
///
/// **Domain**: Meteorological
///
/// **Input type**: Numerical
///
/// **Output type**: Numerical
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 2.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 2 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_02(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Wind Direction (from which blowing)"),
            units: String::from("°"),
            abbrev: String::from("WDIR"),
        },
        1 => TableCategory {
            parameter: String::from("Wind Speed"),
            units: String::from("m s-1"),
            abbrev: String::from("WIND"),
        },
        2 => TableCategory {
            parameter: String::from("U-Component of Wind"),
            units: String::from("m s-1"),
            abbrev: String::from("UGRD"),
        },
        3 => TableCategory {
            parameter: String::from("V-Component of Wind"),
            units: String::from("m s-1"),
            abbrev: String::from("VGRD"),
        },
        4 => TableCategory {
            parameter: String::from("Stream Function"),
            units: String::from("m2 s-1"),
            abbrev: String::from("STRM"),
        },
        5 => TableCategory {
            parameter: String::from("Velocity Potential"),
            units: String::from("m2 s-1"),
            abbrev: String::from("VPOT"),
        },
        6 => TableCategory {
            parameter: String::from("Montgomery Stream Function"),
            units: String::from("m2 s-2"),
            abbrev: String::from("MNTSF"),
        },
        7 => TableCategory {
            parameter: String::from("Sigma Coordinate Vertical Velocity"),
            units: String::from("s-1"),
            abbrev: String::from("SGCVV"),
        },
        8 => TableCategory {
            parameter: String::from("Vertical Velocity (Pressure)"),
            units: String::from("Pa s-1"),
            abbrev: String::from("VVEL"),
        },
        9 => TableCategory {
            parameter: String::from("Vertical Velocity (Geometric)"),
            units: String::from("m s-1"),
            abbrev: String::from("DZDT"),
        },
        10 => TableCategory {
            parameter: String::from("Absolute Vorticity"),
            units: String::from("s-1"),
            abbrev: String::from("ABSV"),
        },
        11 => TableCategory {
            parameter: String::from("Absolute Divergence"),
            units: String::from("s-1"),
            abbrev: String::from("ABSD"),
        },
        12 => TableCategory {
            parameter: String::from("Relative Vorticity"),
            units: String::from("s-1"),
            abbrev: String::from("RELV"),
        },
        13 => TableCategory {
            parameter: String::from("Relative Divergence"),
            units: String::from("s-1"),
            abbrev: String::from("RELD"),
        },
        14 => TableCategory {
            parameter: String::from("Potential Vorticity"),
            units: String::from("K m2 kg-1 s-1"),
            abbrev: String::from("PVORT"),
        },
        15 => TableCategory {
            parameter: String::from("Vertical U-Component Shear"),
            units: String::from("s-1"),
            abbrev: String::from("VUCSH"),
        },
        16 => TableCategory {
            parameter: String::from("Vertical V-Component Shear"),
            units: String::from("s-1"),
            abbrev: String::from("VVCSH"),
        },
        17 => TableCategory {
            parameter: String::from("Momentum Flux, U-Component"),
            units: String::from("N m-2"),
            abbrev: String::from("UFLX"),
        },
        18 => TableCategory {
            parameter: String::from("Momentum Flux, V-Component"),
            units: String::from("N m-2"),
            abbrev: String::from("VFLX"),
        },
        19 => TableCategory {
            parameter: String::from("Wind Mixing Energy"),
            units: String::from("J"),
            abbrev: String::from("WMIXE"),
        },
        20 => TableCategory {
            parameter: String::from("Boundary Layer Dissipation"),
            units: String::from("W m-2"),
            abbrev: String::from("BLYDP"),
        },
        21 => TableCategory {
            parameter: String::from("Maximum Wind Speed"),
            units: String::from("m s-1"),
            abbrev: String::from("MAXGUST"),
        },
        22 => TableCategory {
            parameter: String::from("Wind Speed (Gust)"),
            units: String::from("m s-1"),
            abbrev: String::from("GUST"),
        },
        23 => TableCategory {
            parameter: String::from("U-Component of Wind (Gust)"),
            units: String::from("m s-1"),
            abbrev: String::from("UGUST"),
        },
        24 => TableCategory {
            parameter: String::from("V-Component of Wind (Gust)"),
            units: String::from("m s-1"),
            abbrev: String::from("VGUST"),
        },
        25 => TableCategory {
            parameter: String::from("Vertical Speed Shear"),
            units: String::from("s-1"),
            abbrev: String::from("VWSH"),
        },
        26 => TableCategory {
            parameter: String::from("Horizontal Momentum Flux"),
            units: String::from("N m-2"),
            abbrev: String::from("MFLX"),
        },
        27 => TableCategory {
            parameter: String::from("U-Component Storm Motion"),
            units: String::from("m s-1"),
            abbrev: String::from("USTM"),
        },
        28 => TableCategory {
            parameter: String::from("V-Component Storm Motion"),
            units: String::from("m s-1"),
            abbrev: String::from("VSTM"),
        },
        29 => TableCategory {
            parameter: String::from("Drag Coefficient"),
            units: String::from("Numeric"),
            abbrev: String::from("CD"),
        },
        30 => TableCategory {
            parameter: String::from("Frictional Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("FRICV"),
        },
        31 => TableCategory {
            parameter: String::from("Turbulent Diffusion Coefficient for Momentum"),
            units: String::from("m2 s-1"),
            abbrev: String::from("TDCMOM"),
        },
        32 => TableCategory {
            parameter: String::from("Eta Coordinate Vertical Velocity"),
            units: String::from("s-1"),
            abbrev: String::from("ETACVV"),
        },
        33 => TableCategory {
            parameter: String::from("Wind Fetch"),
            units: String::from("m"),
            abbrev: String::from("WINDF"),
        },
        34 => TableCategory {
            parameter: String::from("Normal Wind Component"),
            units: String::from("m s-1"),
            abbrev: String::from("NWIND"),
        },
        35 => TableCategory {
            parameter: String::from("Tangential Wind Component"),
            units: String::from("m s-1"),
            abbrev: String::from("TWIND"),
        },
        36 => TableCategory {
            parameter: String::from("Amplitude Function for Rossby Wave Envelope"),
            units: String::from("m s-1"),
            abbrev: String::from("AFRWE"),
        },
        37 => TableCategory {
            parameter: String::from("Northward Turbulent Surface Stress"),
            units: String::from("N m-2 s"),
            abbrev: String::from("NTSS"),
        },
        38 => TableCategory {
            parameter: String::from("Eastward Turbulent Surface Stress"),
            units: String::from("N m-2 s"),
            abbrev: String::from("ETSS"),
        },
        39 => TableCategory {
            parameter: String::from("Eastward Wind Tendency Due to Parameterizations"),
            units: String::from("m s-2"),
            abbrev: String::from("EWTPARM"),
        },
        40 => TableCategory {
            parameter: String::from("Northward Wind Tendency Due to Parameterizations"),
            units: String::from("m s-2"),
            abbrev: String::from("NWTPARM"),
        },
        41 => TableCategory {
            parameter: String::from("U-Component of Geostrophic Wind"),
            units: String::from("m s-1"),
            abbrev: String::from("UGWIND"),
        },
        42 => TableCategory {
            parameter: String::from("V-Component of Geostrophic Wind"),
            units: String::from("m s-1"),
            abbrev: String::from("VGWIND"),
        },
        43 => TableCategory {
            parameter: String::from("Geostrophic Wind Direction"),
            units: String::from("°"),
            abbrev: String::from("GEOWD"),
        },
        44 => TableCategory {
            parameter: String::from("Geostrophic Wind Speed"),
            units: String::from("m s-1"),
            abbrev: String::from("GEOWS"),
        },
        45 => TableCategory {
            parameter: String::from("Unbalanced Component of Divergence"),
            units: String::from("s-1"),
            abbrev: String::from("UNDIV"),
        },
        46 => TableCategory {
            parameter: String::from("Vorticity Advection"),
            units: String::from("s-2"),
            abbrev: String::from("VORTADV"),
        },
        47 => TableCategory {
            parameter: String::from("Surface Roughness for Heat"),
            units: String::from("m"),
            abbrev: String::from("SFRHEAT"),
        },
        48 => TableCategory {
            parameter: String::from("Surface Roughness for Moisture"),
            units: String::from("m"),
            abbrev: String::from("SFRMOIST"),
        },
        49 => TableCategory {
            parameter: String::from("Wind Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("WINDSTR"),
        },
        50 => TableCategory {
            parameter: String::from("Eastward Wind Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("EWINDSTR"),
        },
        51 => TableCategory {
            parameter: String::from("Northward Wind Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("NWINDSTR"),
        },
        52 => TableCategory {
            parameter: String::from("U-Component of Wind Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("UWINDSTR"),
        },
        53 => TableCategory {
            parameter: String::from("V-Component of Wind Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("VWINDSTR"),
        },
        54 => TableCategory {
            parameter: String::from("Natural Logarithm of Surface Roughness Length for Heat"),
            units: String::from("m"),
            abbrev: String::from("NLSRLH"),
        },
        55 => TableCategory {
            parameter: String::from("Natural Logarithm of Surface Roughness Length for Moisture"),
            units: String::from("m"),
            abbrev: String::from("NLSRLM"),
        },
        56 => TableCategory {
            parameter: String::from("U-Component of Neutral Wind"),
            units: String::from("m s-1"),
            abbrev: String::from("UNWIND"),
        },
        57 => TableCategory {
            parameter: String::from("V-Component of Neutral Wind"),
            units: String::from("m s-1"),
            abbrev: String::from("VNWIND"),
        },
        58 => TableCategory {
            parameter: String::from("Magnitude of Turbulent Surface Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("TSFCSTR"),
        },
        59 => TableCategory {
            parameter: String::from("Vertical Divergence"),
            units: String::from("s-1"),
            abbrev: String::from("VDIV"),
        },
        60 => TableCategory {
            parameter: String::from("Drag Thermal Coefficient"),
            units: String::from("Numeric"),
            abbrev: String::from("DTC"),
        },
        61 => TableCategory {
            parameter: String::from("Drag Evaporation Coefficient"),
            units: String::from("Numeric"),
            abbrev: String::from("DEC"),
        },
        62 => TableCategory {
            parameter: String::from("Eastward Turbulent Surface Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("EASTTSS"),
        },
        63 => TableCategory {
            parameter: String::from("Northward Turbulent Surface Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("NRTHTSS"),
        },
        192 => TableCategory {
            parameter: String::from("Vertical Speed Shear"),
            units: String::from("s-1"),
            abbrev: String::from("VWSH"),
        },
        193 => TableCategory {
            parameter: String::from("Horizontal Momentum Flux"),
            units: String::from("N m-2"),
            abbrev: String::from("MFLX"),
        },
        194 => TableCategory {
            parameter: String::from("U-Component Storm Motion"),
            units: String::from("m s-1"),
            abbrev: String::from("USTM"),
        },
        195 => TableCategory {
            parameter: String::from("V-Component Storm Motion"),
            units: String::from("m s-1"),
            abbrev: String::from("VSTM"),
        },
        196 => TableCategory {
            parameter: String::from("Drag Coefficient"),
            units: String::from("non-dim"),
            abbrev: String::from("CD"),
        },
        197 => TableCategory {
            parameter: String::from("Frictional Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("FRICV"),
        },
        198 => TableCategory {
            parameter: String::from("Latitude of U Wind Component of Velocity"),
            units: String::from("deg"),
            abbrev: String::from("LAUV"),
        },
        199 => TableCategory {
            parameter: String::from("Longitude of U Wind Component of Velocity"),
            units: String::from("deg"),
            abbrev: String::from("LOUV"),
        },
        200 => TableCategory {
            parameter: String::from("Latitude of V Wind Component of Velocity"),
            units: String::from("deg"),
            abbrev: String::from("LAVV"),
        },
        201 => TableCategory {
            parameter: String::from("Longitude of V Wind Component of Velocity"),
            units: String::from("deg"),
            abbrev: String::from("LOVV"),
        },
        202 => TableCategory {
            parameter: String::from("Latitude of Pressure Point"),
            units: String::from("deg"),
            abbrev: String::from("LAPP"),
        },
        203 => TableCategory {
            parameter: String::from("Longitude of Pressure Point"),
            units: String::from("deg"),
            abbrev: String::from("LOPP"),
        },
        204 => TableCategory {
            parameter: String::from("Vertical Eddy Diffusivity Heat Exchange"),
            units: String::from("m2 s-1"),
            abbrev: String::from("VEDH"),
        },
        205 => TableCategory {
            parameter: String::from(
                "Covariance between Meridional and Zonal Components of the Wind",
            ),
            units: String::from("m2 s-2"),
            abbrev: String::from("COVMZ"),
        },
        206 => TableCategory {
            parameter: String::from(
                "Covariance between Temperature and Zonal Components of the Wind",
            ),
            units: String::from("K*m s-1"),
            abbrev: String::from("COVTZ"),
        },
        207 => TableCategory {
            parameter: String::from(
                "Covariance between Temperature and Meridional Components of the Wind",
            ),
            units: String::from("K*m s-1"),
            abbrev: String::from("COVTM"),
        },
        208 => TableCategory {
            parameter: String::from("Vertical Diffusion Zonal Acceleration"),
            units: String::from("m s-2"),
            abbrev: String::from("VDFUA"),
        },
        209 => TableCategory {
            parameter: String::from("Vertical Diffusion Meridional Acceleration"),
            units: String::from("m s-2"),
            abbrev: String::from("VDFVA"),
        },
        210 => TableCategory {
            parameter: String::from("Gravity Wave Drag Zonal Acceleration"),
            units: String::from("m s-2"),
            abbrev: String::from("GWDU"),
        },
        211 => TableCategory {
            parameter: String::from("Gravity Wave Drag Meridional Acceleration"),
            units: String::from("m s-2"),
            abbrev: String::from("GWDV"),
        },
        212 => TableCategory {
            parameter: String::from("Convective Zonal Momentum Mixing Acceleration"),
            units: String::from("m s-2"),
            abbrev: String::from("CNVU"),
        },
        213 => TableCategory {
            parameter: String::from("Convective Meridional Momentum Mixing Acceleration"),
            units: String::from("m s-2"),
            abbrev: String::from("CNVV"),
        },
        214 => TableCategory {
            parameter: String::from("Tendency of Vertical Velocity"),
            units: String::from("m s-2"),
            abbrev: String::from("WTEND"),
        },
        215 => TableCategory {
            parameter: String::from("Omega (Dp/Dt) Divide by Density"),
            units: String::from("K"),
            abbrev: String::from("OMGALF"),
        },
        216 => TableCategory {
            parameter: String::from("Convective Gravity Wave Drag Zonal Acceleration"),
            units: String::from("m s-2"),
            abbrev: String::from("CNGWDU"),
        },
        217 => TableCategory {
            parameter: String::from("Convective Gravity Wave Drag Meridional Acceleration"),
            units: String::from("m s-2"),
            abbrev: String::from("CNGWDV"),
        },
        218 => TableCategory {
            parameter: String::from("Velocity Point Model Surface"),
            units: String::from("m"),
            abbrev: String::from("LMV"),
        },
        219 => TableCategory {
            parameter: String::from("Potential Vorticity (Mass-Weighted)"),
            units: String::from("1/s/m"),
            abbrev: String::from("PVMWW"),
        },
        220 => TableCategory {
            parameter: String::from("Hourly Maximum of Upward Vertical Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("MAXUVV"),
        },
        221 => TableCategory {
            parameter: String::from("Hourly Maximum of Downward Vertical Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("MAXDVV"),
        },
        222 => TableCategory {
            parameter: String::from("U Component of Hourly Maximum 10m Wind Speed"),
            units: String::from("m s-1"),
            abbrev: String::from("MAXUW"),
        },
        223 => TableCategory {
            parameter: String::from("V Component of Hourly Maximum 10m Wind Speed"),
            units: String::from("m s-1"),
            abbrev: String::from("MAXVW"),
        },
        224 => TableCategory {
            parameter: String::from("Ventilation Rate"),
            units: String::from("m2 s-1"),
            abbrev: String::from("VRATE"),
        },
        225 => TableCategory {
            parameter: String::from("Transport Wind Speed"),
            units: String::from("m s-1"),
            abbrev: String::from("TRWSPD"),
        },
        226 => TableCategory {
            parameter: String::from("Transport Wind Direction"),
            units: String::from("°"),
            abbrev: String::from("TRWDIR"),
        },
        227 => TableCategory {
            parameter: String::from("Earliest Reasonable Arrival Time (10% Exceedance)"),
            units: String::from("s"),
            abbrev: String::from("TOA10"),
        },
        228 => TableCategory {
            parameter: String::from("Most Likely Arrival Time (50% Exceedance)"),
            units: String::from("s"),
            abbrev: String::from("TOA50"),
        },
        229 => TableCategory {
            parameter: String::from("Most Likely Departure Time (50% Exceedance)"),
            units: String::from("s"),
            abbrev: String::from("TOD50"),
        },
        230 => TableCategory {
            parameter: String::from("Latest Reasonable Departure Time (90% Exceedance)"),
            units: String::from("s"),
            abbrev: String::from("TOD90"),
        },
        231 => TableCategory {
            parameter: String::from("Tropical Wind Direction"),
            units: String::from("°"),
            abbrev: String::from("TPWDIR"),
        },
        232 => TableCategory {
            parameter: String::from("Tropical Wind Speed"),
            units: String::from("m s-1"),
            abbrev: String::from("TPWSPD"),
        },
        233 => TableCategory {
            parameter: String::from("Inflow Based (ESFC) to 50% EL Shear Magnitude"),
            units: String::from("kt"),
            abbrev: String::from("ESHR"),
        },
        234 => TableCategory {
            parameter: String::from("U Component Inflow Based to 50% EL Shear Vector"),
            units: String::from("kt"),
            abbrev: String::from("UESH"),
        },
        235 => TableCategory {
            parameter: String::from("V Component Inflow Based to 50% EL Shear Vector"),
            units: String::from("kt"),
            abbrev: String::from("VESH"),
        },
        236 => TableCategory {
            parameter: String::from("U Component Bunkers Effective Right Motion"),
            units: String::from("kt"),
            abbrev: String::from("UEID"),
        },
        237 => TableCategory {
            parameter: String::from("V Component Bunkers Effective Right Motion"),
            units: String::from("kt"),
            abbrev: String::from("VEID"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from("Missing"),
        },
        64..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-0-3
/// PARAMETERS FOR DISCIPLINE 0 - CATEGORY 3
/// (Meteorological products, Mass category)
/// In Section 0, Octet 7 = 0
/// In Section 4, Octet 10 = 3
/// Revised 12/07/2023
/// Red text depicts changes made since 06/23/2022
/// @see [GRIB2 - Table 4.2-0-3: Parameters for Discipline 0 Category 3 (Mass category)](https://www.example.com)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 3.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 3 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_03(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Pressure"),
            units: String::from("Pa"),
            abbrev: String::from("PRES"),
        },
        1 => TableCategory {
            parameter: String::from("Pressure Reduced to MSL"),
            units: String::from("Pa"),
            abbrev: String::from("PRMSL"),
        },
        2 => TableCategory {
            parameter: String::from("Pressure Tendency"),
            units: String::from("Pa s-1"),
            abbrev: String::from("PTEND"),
        },
        3 => TableCategory {
            parameter: String::from("ICAO Standard Atmosphere Reference Height"),
            units: String::from("m"),
            abbrev: String::from("ICAHT"),
        },
        4 => TableCategory {
            parameter: String::from("Geopotential"),
            units: String::from("m2 s-2"),
            abbrev: String::from("GP"),
        },
        5 => TableCategory {
            parameter: String::from("Geopotential Height"),
            units: String::from("gpm"),
            abbrev: String::from("HGT"),
        },
        6 => TableCategory {
            parameter: String::from("Geometric Height"),
            units: String::from("m"),
            abbrev: String::from("DIST"),
        },
        7 => TableCategory {
            parameter: String::from("Standard Deviation of Height"),
            units: String::from("m"),
            abbrev: String::from("HSTDV"),
        },
        8 => TableCategory {
            parameter: String::from("Pressure Anomaly"),
            units: String::from("Pa"),
            abbrev: String::from("PRESA"),
        },
        9 => TableCategory {
            parameter: String::from("Geopotential Height Anomaly"),
            units: String::from("gpm"),
            abbrev: String::from("GPA"),
        },
        10 => TableCategory {
            parameter: String::from("Density"),
            units: String::from("kg m-3"),
            abbrev: String::from("DEN"),
        },
        11 => TableCategory {
            parameter: String::from("Altimeter Setting"),
            units: String::from("Pa"),
            abbrev: String::from("ALTS"),
        },
        12 => TableCategory {
            parameter: String::from("Thickness"),
            units: String::from("m"),
            abbrev: String::from("THICK"),
        },
        13 => TableCategory {
            parameter: String::from("Pressure Altitude"),
            units: String::from("m"),
            abbrev: String::from("PRESALT"),
        },
        14 => TableCategory {
            parameter: String::from("Density Altitude"),
            units: String::from("m"),
            abbrev: String::from("DENALT"),
        },
        15 => TableCategory {
            parameter: String::from("5-Wave Geopotential Height"),
            units: String::from("gpm"),
            abbrev: String::from("5WAVH"),
        },
        16 => TableCategory {
            parameter: String::from("Zonal Flux of Gravity Wave Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("U-GWD"),
        },
        17 => TableCategory {
            parameter: String::from("Meridional Flux of Gravity Wave Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("V-GWD"),
        },
        18 => TableCategory {
            parameter: String::from("Planetary Boundary Layer Height"),
            units: String::from("m"),
            abbrev: String::from("HPBL"),
        },
        19 => TableCategory {
            parameter: String::from("5-Wave Geopotential Height Anomaly"),
            units: String::from("gpm"),
            abbrev: String::from("5WAVA"),
        },
        20 => TableCategory {
            parameter: String::from("Standard Deviation of Sub-Grid Scale Orography"),
            units: String::from("m"),
            abbrev: String::from("SDSGSO"),
        },
        21 => TableCategory {
            parameter: String::from("Angle of Sub-Grid Scale Orography"),
            units: String::from("rad"),
            abbrev: String::from("AOSGSO"),
        },
        22 => TableCategory {
            parameter: String::from("Slope of Sub-Grid Scale Orography"),
            units: String::from("Numeric"),
            abbrev: String::from("SSGSO"),
        },
        23 => TableCategory {
            parameter: String::from("Gravity Wave Dissipation"),
            units: String::from("W m-2"),
            abbrev: String::from("GWD"),
        },
        24 => TableCategory {
            parameter: String::from("Anisotropy of Sub-Grid Scale Orography"),
            units: String::from("Numeric"),
            abbrev: String::from("ASGSO"),
        },
        25 => TableCategory {
            parameter: String::from("Natural Logarithm of Pressure in Pa"),
            units: String::from("Numeric"),
            abbrev: String::from("NLPRES"),
        },
        26 => TableCategory {
            parameter: String::from("Exner Pressure"),
            units: String::from("Numeric"),
            abbrev: String::from("EXPRES"),
        },
        27 => TableCategory {
            parameter: String::from("Updraught Mass Flux"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("UMFLX"),
        },
        28 => TableCategory {
            parameter: String::from("Downdraught Mass Flux"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("DMFLX"),
        },
        29 => TableCategory {
            parameter: String::from("Updraught Detrainment Rate"),
            units: String::from("kg m-3 s-1"),
            abbrev: String::from("UDRATE"),
        },
        30 => TableCategory {
            parameter: String::from("Downdraught Detrainment Rate"),
            units: String::from("kg m-3 s-1"),
            abbrev: String::from("DDRATE"),
        },
        31 => TableCategory {
            parameter: String::from("Unbalanced Component of Logarithm of Surface Pressure"),
            units: String::from("-"),
            abbrev: String::from("UCLSPRS"),
        },
        32 => TableCategory {
            parameter: String::from("Saturation water vapour pressure"),
            units: String::from("Pa"),
            abbrev: String::from("SWATERVP"),
        },
        33 => TableCategory {
            parameter: String::from("Geometric altitude above mean sea level"),
            units: String::from("m"),
            abbrev: String::from("GAMSL"),
        },
        34 => TableCategory {
            parameter: String::from("Geometric height above ground level"),
            units: String::from("m"),
            abbrev: String::from("GHAGRD"),
        },
        35 => TableCategory {
            parameter: String::from("Column integrated divergence of total mass flux"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CDTMF"),
        },
        36 => TableCategory {
            parameter: String::from("Column integrated eastward total mass flux"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CETMF"),
        },
        37 => TableCategory {
            parameter: String::from("Column integrated northward total mass flux"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CNTMF"),
        },
        38 => TableCategory {
            parameter: String::from("Standard deviation of filtered subgrid orography"),
            units: String::from("m"),
            abbrev: String::from("SDFSO"),
        },
        39 => TableCategory {
            parameter: String::from("Column integrated mass of atmosphere"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CMATMOS"),
        },
        40 => TableCategory {
            parameter: String::from("Column integrated eastward geopotential flux"),
            units: String::from("W m-1"),
            abbrev: String::from("CEGFLUX"),
        },
        41 => TableCategory {
            parameter: String::from("Column integrated northward geopotential flux"),
            units: String::from("W m-1"),
            abbrev: String::from("CNGFLUX"),
        },
        42 => TableCategory {
            parameter: String::from("Column integrated divergence of water geopotential flux"),
            units: String::from("W m-2"),
            abbrev: String::from("CDWGFLUX"),
        },
        43 => TableCategory {
            parameter: String::from("Column integrated divergence of geopotential flux"),
            units: String::from("W m-2"),
            abbrev: String::from("CDGFLUX"),
        },
        44 => TableCategory {
            parameter: String::from("Height of zero-degree wet-bulb temperature"),
            units: String::from("m"),
            abbrev: String::from("HWBT"),
        },
        45 => TableCategory {
            parameter: String::from("Height of one-degree wet-bulb temperature"),
            units: String::from("m"),
            abbrev: String::from("WOBT"),
        },
        46 => TableCategory {
            parameter: String::from("Pressure departure from hydrostatic state"),
            units: String::from("Pa"),
            abbrev: String::from("PRESDHS"),
        },
        192 => TableCategory {
            parameter: String::from("MSLP (Eta model reduction)"),
            units: String::from("Pa"),
            abbrev: String::from("MSLET"),
        },
        193 => TableCategory {
            parameter: String::from("5-Wave Geopotential Height"),
            units: String::from("gpm"),
            abbrev: String::from("5WAVH"),
        },
        194 => TableCategory {
            parameter: String::from("Zonal Flux of Gravity Wave Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("U-GWD"),
        },
        195 => TableCategory {
            parameter: String::from("Meridional Flux of Gravity Wave Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("V-GWD"),
        },
        196 => TableCategory {
            parameter: String::from("Planetary Boundary Layer Height"),
            units: String::from("m"),
            abbrev: String::from("HPBL"),
        },
        197 => TableCategory {
            parameter: String::from("5-Wave Geopotential Height Anomaly"),
            units: String::from("gpm"),
            abbrev: String::from("5WAVA"),
        },
        198 => TableCategory {
            parameter: String::from("MSLP (MAPS System Reduction)"),
            units: String::from("Pa"),
            abbrev: String::from("MSLMA"),
        },
        199 => TableCategory {
            parameter: String::from("3-hr pressure tendency (Std. Atmos. Reduction)"),
            units: String::from("Pa s-1"),
            abbrev: String::from("TSLSA"),
        },
        200 => TableCategory {
            parameter: String::from("Pressure of level from which parcel was lifted"),
            units: String::from("Pa"),
            abbrev: String::from("PLPL"),
        },
        201 => TableCategory {
            parameter: String::from("X-gradient of Log Pressure"),
            units: String::from("m-1"),
            abbrev: String::from("LPSX"),
        },
        202 => TableCategory {
            parameter: String::from("Y-gradient of Log Pressure"),
            units: String::from("m-1"),
            abbrev: String::from("LPSY"),
        },
        203 => TableCategory {
            parameter: String::from("X-gradient of Height"),
            units: String::from("m-1"),
            abbrev: String::from("HGTX"),
        },
        204 => TableCategory {
            parameter: String::from("Y-gradient of Height"),
            units: String::from("m-1"),
            abbrev: String::from("HGTY"),
        },
        205 => TableCategory {
            parameter: String::from("Layer Thickness"),
            units: String::from("m"),
            abbrev: String::from("LAYTH"),
        },
        206 => TableCategory {
            parameter: String::from("Natural Log of Surface Pressure"),
            units: String::from("ln (kPa)"),
            abbrev: String::from("NLGSP"),
        },
        207 => TableCategory {
            parameter: String::from("Convective updraft mass flux"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CNVUMF"),
        },
        208 => TableCategory {
            parameter: String::from("Convective downdraft mass flux"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CNVDMF"),
        },
        209 => TableCategory {
            parameter: String::from("Convective detrainment mass flux"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CNVDEMF"),
        },
        210 => TableCategory {
            parameter: String::from("Mass Point Model Surface"),
            units: String::from(""),
            abbrev: String::from("LMH"),
        },
        211 => TableCategory {
            parameter: String::from("Geopotential Height (nearest grid point)"),
            units: String::from("gpm"),
            abbrev: String::from("HGTN"),
        },
        212 => TableCategory {
            parameter: String::from("Pressure (nearest grid point)"),
            units: String::from("Pa"),
            abbrev: String::from("PRESN"),
        },
        213 => TableCategory {
            parameter: String::from("Orographic Convexity"),
            units: String::from(""),
            abbrev: String::from("ORCONV"),
        },
        214 => TableCategory {
            parameter: String::from("Orographic Asymmetry, W Component"),
            units: String::from(""),
            abbrev: String::from("ORASW"),
        },
        215 => TableCategory {
            parameter: String::from("Orographic Asymmetry, S Component"),
            units: String::from(""),
            abbrev: String::from("ORASS"),
        },
        216 => TableCategory {
            parameter: String::from("Orographic Asymmetry, SW Component"),
            units: String::from(""),
            abbrev: String::from("ORASSW"),
        },
        217 => TableCategory {
            parameter: String::from("Orographic Asymmetry, NW Component"),
            units: String::from(""),
            abbrev: String::from("ORASNW"),
        },
        218 => TableCategory {
            parameter: String::from("Orographic Length Scale, W Component"),
            units: String::from(""),
            abbrev: String::from("ORLSW"),
        },
        219 => TableCategory {
            parameter: String::from("Orographic Length Scale, S Component"),
            units: String::from(""),
            abbrev: String::from("ORLSS"),
        },
        220 => TableCategory {
            parameter: String::from("Orographic Length Scale, SW Component"),
            units: String::from(""),
            abbrev: String::from("ORLSSW"),
        },
        221 => TableCategory {
            parameter: String::from("Orographic Length Scale, NW Component"),
            units: String::from(""),
            abbrev: String::from("ORLSNW"),
        },
        222 => TableCategory {
            parameter: String::from("Effective Surface Height"),
            units: String::from("m"),
            abbrev: String::from("EFSH"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from("Missing"),
        },
        47..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-0-4
/// PARAMETERS FOR DISCIPLINE 0 - CATEGORY 4
/// (Meteorological products, Short-wave radiation category)
/// In Section 0, Octet 7 = 0
/// In Section 4, Octet 10 = 4
/// Revised 11/02/2023
/// Red text depicts changes made since 06/23/2022
/// @see [GRIB2 - Table 4.2-0-4: Parameters for Discipline 0 Category 4 (Short-wave radiation category)](https://www.example.com)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 4.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 4 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_04(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Net Short-Wave Radiation Flux (Surface)"),
            units: String::from("W m-2"),
            abbrev: String::from("NSWRS"),
        },
        1 => TableCategory {
            parameter: String::from("Net Short-Wave Radiation Flux (Top of Atmosphere)"),
            units: String::from("W m-2"),
            abbrev: String::from("NSWRT"),
        },
        2 => TableCategory {
            parameter: String::from("Short-Wave Radiation Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("SWAVR"),
        },
        3 => TableCategory {
            parameter: String::from("Global Radiation Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("GRAD"),
        },
        4 => TableCategory {
            parameter: String::from("Brightness Temperature"),
            units: String::from("K"),
            abbrev: String::from("BRTMP"),
        },
        5 => TableCategory {
            parameter: String::from("Radiance (with respect to wave number)"),
            units: String::from("W m-1 sr-1"),
            abbrev: String::from("LWRAD"),
        },
        6 => TableCategory {
            parameter: String::from("Radiance (with respect to wavelength)"),
            units: String::from("W m-3 sr-1"),
            abbrev: String::from("SWRAD"),
        },
        7 => TableCategory {
            parameter: String::from("Downward Short-Wave Radiation Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("DSWRF"),
        },
        8 => TableCategory {
            parameter: String::from("Upward Short-Wave Radiation Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("USWRF"),
        },
        9 => TableCategory {
            parameter: String::from("Net Short Wave Radiation Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("NSWRF"),
        },
        10 => TableCategory {
            parameter: String::from("Photosynthetically Active Radiation"),
            units: String::from("W m-2"),
            abbrev: String::from("PHOTAR"),
        },
        11 => TableCategory {
            parameter: String::from("Net Short-Wave Radiation Flux, Clear Sky"),
            units: String::from("W m-2"),
            abbrev: String::from("NSWRFCS"),
        },
        12 => TableCategory {
            parameter: String::from("Downward UV Radiation"),
            units: String::from("W m-2"),
            abbrev: String::from("DWUVR"),
        },
        13 => TableCategory {
            parameter: String::from("Direct Short Wave Radiation Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("DSWRFLX"),
        },
        14 => TableCategory {
            parameter: String::from("Diffuse Short Wave Radiation Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("DIFSWRF"),
        },
        15 => TableCategory {
            parameter: String::from(
                "Upward UV radiation emitted/reflected from the Earth's surface",
            ),
            units: String::from("W m-2"),
            abbrev: String::from("UVVEARTH"),
        },
        50 => TableCategory {
            parameter: String::from("UV Index (Under Clear Sky)"),
            units: String::from("Numeric"),
            abbrev: String::from("UVIUCS"),
        },
        51 => TableCategory {
            parameter: String::from("UV Index"),
            units: String::from("Numeric"),
            abbrev: String::from("UVI"),
        },
        52 => TableCategory {
            parameter: String::from("Downward Short-Wave Radiation Flux, Clear Sky"),
            units: String::from("W m-2"),
            abbrev: String::from("DSWRFCS"),
        },
        53 => TableCategory {
            parameter: String::from("Upward Short-Wave Radiation Flux, Clear Sky"),
            units: String::from("W m-2"),
            abbrev: String::from("USWRFCS"),
        },
        54 => TableCategory {
            parameter: String::from("Direct normal short-wave radiation flux"),
            units: String::from("W m-2"),
            abbrev: String::from("DNSWRFLX"),
        },
        55 => TableCategory {
            parameter: String::from("UV visible albedo for diffuse radiation"),
            units: String::from("%"),
            abbrev: String::from("UVALBDIF"),
        },
        56 => TableCategory {
            parameter: String::from("UV visible albedo for direct radiation"),
            units: String::from("%"),
            abbrev: String::from("UVALBDIR"),
        },
        57 => TableCategory {
            parameter: String::from("UV visible albedo for direct radiation, geometric component"),
            units: String::from("%"),
            abbrev: String::from("UBALBDIRG"),
        },
        58 => TableCategory {
            parameter: String::from("UV visible albedo for direct radiation, isotropic component"),
            units: String::from("%"),
            abbrev: String::from("UVALBDIRI"),
        },
        59 => TableCategory {
            parameter: String::from("UV visible albedo for direct radiation, volumetric component"),
            units: String::from("%"),
            abbrev: String::from("UVBDIRV"),
        },
        60 => TableCategory {
            parameter: String::from("Photosynthetically active radiation flux, clear sky"),
            units: String::from("W m-2"),
            abbrev: String::from("PHOARFCS"),
        },
        61 => TableCategory {
            parameter: String::from("Direct short-wave radiation flux, clear sky"),
            units: String::from("W m-2"),
            abbrev: String::from("DSWRFLXCS"),
        },
        192 => TableCategory {
            parameter: String::from("Downward Short-Wave Radiation Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("DSWRF"),
        },
        193 => TableCategory {
            parameter: String::from("Upward Short-Wave Radiation Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("USWRF"),
        },
        194 => TableCategory {
            parameter: String::from("UV-B Downward Solar Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("DUVB"),
        },
        195 => TableCategory {
            parameter: String::from("Clear sky UV-B Downward Solar Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("CDUVB"),
        },
        196 => TableCategory {
            parameter: String::from("Clear Sky Downward Solar Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("CSDSF"),
        },
        197 => TableCategory {
            parameter: String::from("Solar Radiative Heating Rate"),
            units: String::from("K s-1"),
            abbrev: String::from("SWHR"),
        },
        198 => TableCategory {
            parameter: String::from("Clear Sky Upward Solar Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("CSUSF"),
        },
        199 => TableCategory {
            parameter: String::from("Cloud Forcing Net Solar Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("CFNSF"),
        },
        200 => TableCategory {
            parameter: String::from("Visible Beam Downward Solar Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("VBDSF"),
        },
        201 => TableCategory {
            parameter: String::from("Visible Diffuse Downward Solar Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("VDDSF"),
        },
        202 => TableCategory {
            parameter: String::from("Near IR Beam Downward Solar Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("NBDSF"),
        },
        203 => TableCategory {
            parameter: String::from("Near IR Diffuse Downward Solar Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("NDDSF"),
        },
        204 => TableCategory {
            parameter: String::from("Downward Total Radiation Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("DTRF"),
        },
        205 => TableCategory {
            parameter: String::from("Upward Total Radiation Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("UTRF"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from("Missing"),
        },
        16..=49 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        62..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-0-5
/// PARAMETERS FOR DISCIPLINE 0 - CATEGORY 5
/// (Meteorological products, Long-wave radiation category)
/// In Section 0, Octet 7 = 0
/// In Section 4, Octet 10 = 5
/// Revised 11/02/2023
/// Red text depicts changes made since 11/02/2023
/// @see [GRIB2 - Table 4.2-0-5: Parameters for Discipline 0 Category 5 (Long-wave radiation category)](https://www.example.com)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 5.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 5 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_05(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Net Long-Wave Radiation Flux (Surface)"),
            units: String::from("W m-2"),
            abbrev: String::from("NLWRS"),
        },
        1 => TableCategory {
            parameter: String::from("Net Long-Wave Radiation Flux (Top of Atmosphere)"),
            units: String::from("W m-2"),
            abbrev: String::from("NLWRT"),
        },
        2 => TableCategory {
            parameter: String::from("Long-Wave Radiation Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("LWAVR"),
        },
        3 => TableCategory {
            parameter: String::from("Downward Long-Wave Rad. Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("DLWRF"),
        },
        4 => TableCategory {
            parameter: String::from("Upward Long-Wave Rad. Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("ULWRF"),
        },
        5 => TableCategory {
            parameter: String::from("Net Long-Wave Radiation Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("NLWRF"),
        },
        6 => TableCategory {
            parameter: String::from("Net Long-Wave Radiation Flux, Clear Sky"),
            units: String::from("W m-2"),
            abbrev: String::from("NLWRCS"),
        },
        7 => TableCategory {
            parameter: String::from("Brightness Temperature"),
            units: String::from("K"),
            abbrev: String::from("BRTEMP"),
        },
        8 => TableCategory {
            parameter: String::from("Downward Long-Wave Radiation Flux, Clear Sky"),
            units: String::from("W m-2"),
            abbrev: String::from("DLWRFCS"),
        },
        9 => TableCategory {
            parameter: String::from("Near IR albedo for diffuse radiation"),
            units: String::from("%"),
            abbrev: String::from("NIRALBDIF"),
        },
        10 => TableCategory {
            parameter: String::from("Near IR albedo for direct radiation"),
            units: String::from("%"),
            abbrev: String::from("NIRALBDIR"),
        },
        11 => TableCategory {
            parameter: String::from("Near IR albedo for direct radiation, geometric component"),
            units: String::from("%"),
            abbrev: String::from("NIRALBDIRG"),
        },
        12 => TableCategory {
            parameter: String::from("Near IR albedo for direct radiation, isotropic component"),
            units: String::from("%"),
            abbrev: String::from("NIRALBDIRI"),
        },
        13 => TableCategory {
            parameter: String::from("Near IR albedo for direct radiation, volumetric component"),
            units: String::from("%"),
            abbrev: String::from("NIRALBDIRV"),
        },
        192 => TableCategory {
            parameter: String::from("Downward Long-Wave Rad. Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("DLWRF"),
        },
        193 => TableCategory {
            parameter: String::from("Upward Long-Wave Rad. Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("ULWRF"),
        },
        194 => TableCategory {
            parameter: String::from("Long-Wave Radiative Heating Rate"),
            units: String::from("K s-1"),
            abbrev: String::from("LWHR"),
        },
        195 => TableCategory {
            parameter: String::from("Clear Sky Upward Long Wave Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("CSULF"),
        },
        196 => TableCategory {
            parameter: String::from("Clear Sky Downward Long Wave Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("CSDLF"),
        },
        197 => TableCategory {
            parameter: String::from("Cloud Forcing Net Long Wave Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("CFNLF"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from("Missing"),
        },
        14..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-0-6
/// PARAMETERS FOR DISCIPLINE 0 - CATEGORY 6
/// (Meteorological products, Cloud category)
/// In Section 0, Octet 7 = 0
/// In Section 4, Octet 10 = 6
/// Revised 10/24/2023
/// Red text depicts changes made since 10/24/2023
///
/// **Notes:**
/// - Parameter deprecated - Use another parameter in parameter category 1: moisture instead.
/// - The sum of the water and ice fractions may exceed the total due to overlap between the volumes containing ice and those containing liquid water.
/// - Fog is defined as cloud cover in the lowest model level.
/// - This parameter is the amount of sunshine in seconds over a given length of time in seconds. Sunshine is defined as a radiation intensity above 120 W m-2.
///   @see [GRIB2 - Table 4.2-0-6: Parameters for Discipline 0 Category 6 (Cloud category)](https://www.example.com)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 6.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 6 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_06(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Cloud Ice"),
            units: String::from("kg m-2"),
            abbrev: String::from("CICE"),
        },
        1 => TableCategory {
            parameter: String::from("Total Cloud Cover"),
            units: String::from("%"),
            abbrev: String::from("TCDC"),
        },
        2 => TableCategory {
            parameter: String::from("Convective Cloud Cover"),
            units: String::from("%"),
            abbrev: String::from("CDCON"),
        },
        3 => TableCategory {
            parameter: String::from("Low Cloud Cover"),
            units: String::from("%"),
            abbrev: String::from("LCDC"),
        },
        4 => TableCategory {
            parameter: String::from("Medium Cloud Cover"),
            units: String::from("%"),
            abbrev: String::from("MCDC"),
        },
        5 => TableCategory {
            parameter: String::from("High Cloud Cover"),
            units: String::from("%"),
            abbrev: String::from("HCDC"),
        },
        6 => TableCategory {
            parameter: String::from("Cloud Water"),
            units: String::from("kg m-2"),
            abbrev: String::from("CWAT"),
        },
        7 => TableCategory {
            parameter: String::from("Cloud Amount"),
            units: String::from("%"),
            abbrev: String::from("CDCA"),
        },
        8 => TableCategory {
            parameter: String::from("Cloud Type"),
            units: String::from("See Table 4.203"),
            abbrev: String::from("CDCT"),
        },
        9 => TableCategory {
            parameter: String::from("Thunderstorm Maximum Tops"),
            units: String::from("m"),
            abbrev: String::from("TMAXT"),
        },
        10 => TableCategory {
            parameter: String::from("Thunderstorm Coverage"),
            units: String::from("See Table 4.204"),
            abbrev: String::from("THUNC"),
        },
        11 => TableCategory {
            parameter: String::from("Cloud Base"),
            units: String::from("m"),
            abbrev: String::from("CDCB"),
        },
        12 => TableCategory {
            parameter: String::from("Cloud Top"),
            units: String::from("m"),
            abbrev: String::from("CDCTOP"),
        },
        13 => TableCategory {
            parameter: String::from("Ceiling"),
            units: String::from("m"),
            abbrev: String::from("CEIL"),
        },
        14 => TableCategory {
            parameter: String::from("Non-Convective Cloud Cover"),
            units: String::from("%"),
            abbrev: String::from("CDLYR"),
        },
        15 => TableCategory {
            parameter: String::from("Cloud Work Function"),
            units: String::from("J kg-1"),
            abbrev: String::from("CWORK"),
        },
        16 => TableCategory {
            parameter: String::from("Convective Cloud Efficiency"),
            units: String::from("Proportion"),
            abbrev: String::from("CUEFI"),
        },
        17 => TableCategory {
            parameter: String::from("Total Condensate"),
            units: String::from("kg kg-1"),
            abbrev: String::from("TCONDO"),
        }, // Deprecated
        18 => TableCategory {
            parameter: String::from("Total Column-Integrated Cloud Water"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLWO"),
        }, // Deprecated
        19 => TableCategory {
            parameter: String::from("Total Column-Integrated Cloud Ice"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLIO"),
        }, // Deprecated
        20 => TableCategory {
            parameter: String::from("Total Column-Integrated Condensate"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLC"),
        }, // Deprecated
        21 => TableCategory {
            parameter: String::from("Ice fraction of total condensate"),
            units: String::from("Proportion"),
            abbrev: String::from("FICE"),
        },
        22 => TableCategory {
            parameter: String::from("Cloud Cover"),
            units: String::from("%"),
            abbrev: String::from("CDCC"),
        },
        23 => TableCategory {
            parameter: String::from("Cloud Ice Mixing Ratio"),
            units: String::from("kg kg-1"),
            abbrev: String::from("CDCIMR"),
        }, // Deprecated
        24 => TableCategory {
            parameter: String::from("Sunshine"),
            units: String::from("Numeric"),
            abbrev: String::from("SUNS"),
        },
        25 => TableCategory {
            parameter: String::from("Horizontal Extent of Cumulonimbus (CB)"),
            units: String::from("%"),
            abbrev: String::from("CBHE"),
        },
        26 => TableCategory {
            parameter: String::from("Height of Convective Cloud Base"),
            units: String::from("m"),
            abbrev: String::from("HCONCB"),
        },
        27 => TableCategory {
            parameter: String::from("Height of Convective Cloud Top"),
            units: String::from("m"),
            abbrev: String::from("HCONCT"),
        },
        28 => TableCategory {
            parameter: String::from("Number Concentration of Cloud Droplets"),
            units: String::from("kg-1"),
            abbrev: String::from("NCONCD"),
        },
        29 => TableCategory {
            parameter: String::from("Number Concentration of Cloud Ice"),
            units: String::from("kg-1"),
            abbrev: String::from("NCCICE"),
        },
        30 => TableCategory {
            parameter: String::from("Number Density of Cloud Droplets"),
            units: String::from("m-3"),
            abbrev: String::from("NDENCD"),
        },
        31 => TableCategory {
            parameter: String::from("Number Density of Cloud Ice"),
            units: String::from("m-3"),
            abbrev: String::from("NDCICE"),
        },
        32 => TableCategory {
            parameter: String::from("Fraction of Cloud Cover"),
            units: String::from("Numeric"),
            abbrev: String::from("FRACCC"),
        },
        33 => TableCategory {
            parameter: String::from("Sunshine Duration"),
            units: String::from("s"),
            abbrev: String::from("SUNSD"),
        },
        34 => TableCategory {
            parameter: String::from("Surface Long Wave Effective Total Cloudiness"),
            units: String::from("Numeric"),
            abbrev: String::from("SLWTC"),
        },
        35 => TableCategory {
            parameter: String::from("Surface Short Wave Effective Total Cloudiness"),
            units: String::from("Numeric"),
            abbrev: String::from("SSWTC"),
        },
        36 => TableCategory {
            parameter: String::from("Fraction of Stratiform Precipitation Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("FSTRPC"),
        },
        37 => TableCategory {
            parameter: String::from("Fraction of Convective Precipitation Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("FCONPC"),
        },
        38 => TableCategory {
            parameter: String::from("Mass Density of Cloud Droplets"),
            units: String::from("kg m-3"),
            abbrev: String::from("MASSDCD"),
        },
        39 => TableCategory {
            parameter: String::from("Mass Density of Cloud Ice"),
            units: String::from("kg m-3"),
            abbrev: String::from("MASSDCI"),
        },
        40 => TableCategory {
            parameter: String::from("Mass Density of Convective Cloud Water Droplets"),
            units: String::from("kg m-3"),
            abbrev: String::from("MDCCWD"),
        },
        47 => TableCategory {
            parameter: String::from("Volume Fraction of Cloud Water Droplets"),
            units: String::from("Numeric"),
            abbrev: String::from("VFRCWD"),
        }, // Note 2
        48 => TableCategory {
            parameter: String::from("Volume Fraction of Cloud Ice Particles"),
            units: String::from("Numeric"),
            abbrev: String::from("VFRCICE"),
        }, // Note 2
        49 => TableCategory {
            parameter: String::from("Volume Fraction of Cloud (Ice and/or Water)"),
            units: String::from("Numeric"),
            abbrev: String::from("VFRCIW"),
        }, // Note 2
        50 => TableCategory {
            parameter: String::from("Fog"),
            units: String::from("%"),
            abbrev: String::from("FOG"),
        }, // Note 3
        51 => TableCategory {
            parameter: String::from("Sunshine Duration Fraction"),
            units: String::from("Proportion"),
            abbrev: String::from("SUNFRAC"),
        }, // Note 4
        192 => TableCategory {
            parameter: String::from("Non-Convective Cloud Cover"),
            units: String::from("%"),
            abbrev: String::from("CDLYR"),
        },
        193 => TableCategory {
            parameter: String::from("Cloud Work Function"),
            units: String::from("J kg-1"),
            abbrev: String::from("CWORK"),
        },
        194 => TableCategory {
            parameter: String::from("Convective Cloud Efficiency"),
            units: String::from("non-dim"),
            abbrev: String::from("CUEFI"),
        },
        195 => TableCategory {
            parameter: String::from("Total Condensate"),
            units: String::from("kg kg-1"),
            abbrev: String::from("TCOND"),
        },
        196 => TableCategory {
            parameter: String::from("Total Column-Integrated Cloud Water"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLW"),
        },
        197 => TableCategory {
            parameter: String::from("Total Column-Integrated Cloud Ice"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLI"),
        },
        198 => TableCategory {
            parameter: String::from("Total Column-Integrated Condensate"),
            units: String::from("kg m-2"),
            abbrev: String::from("TCOLC"),
        },
        199 => TableCategory {
            parameter: String::from("Ice fraction of total condensate"),
            units: String::from("non-dim"),
            abbrev: String::from("FICE"),
        },
        200 => TableCategory {
            parameter: String::from("Convective Cloud Mass Flux"),
            units: String::from("Pa s-1"),
            abbrev: String::from("MFLUX"),
        },
        201 => TableCategory {
            parameter: String::from("Sunshine Duration"),
            units: String::from("s"),
            abbrev: String::from("SUNSD"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from("Missing"),
        },
        41..=46 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        52..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-0-7
/// PARAMETERS FOR DISCIPLINE 0 - CATEGORY 7
/// (Meteorological products, Thermodynamic Stability category)
/// In Section 0, Octet 7 = 0
/// In Section 4, Octet 10 = 7
/// Revised 06/23/2022
/// Red text depicts changes made since 01/25/2021
///
/// **Notes:**
/// - Parameter deprecated - Use another parameter in parameter category 1: moisture instead.
///   @see [GRIB2 - Table 4.2-0-7: Parameters for Discipline 0 Category 7 (Thermodynamic Stability category)](https://www.example.com)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 7.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 7 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_07(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Parcel Lifted Index (to 500 hPa)"),
            units: String::from("K"),
            abbrev: String::from("PLI"),
        },
        1 => TableCategory {
            parameter: String::from("Best Lifted Index (to 500 hPa)"),
            units: String::from("K"),
            abbrev: String::from("BLI"),
        },
        2 => TableCategory {
            parameter: String::from("K Index"),
            units: String::from("K"),
            abbrev: String::from("KX"),
        },
        3 => TableCategory {
            parameter: String::from("KO Index"),
            units: String::from("K"),
            abbrev: String::from("KOX"),
        },
        4 => TableCategory {
            parameter: String::from("Total Totals Index"),
            units: String::from("K"),
            abbrev: String::from("TOTALX"),
        },
        5 => TableCategory {
            parameter: String::from("Sweat Index"),
            units: String::from("Numeric"),
            abbrev: String::from("SX"),
        },
        6 => TableCategory {
            parameter: String::from("Convective Available Potential Energy"),
            units: String::from("J kg-1"),
            abbrev: String::from("CAPE"),
        },
        7 => TableCategory {
            parameter: String::from("Convective Inhibition"),
            units: String::from("J kg-1"),
            abbrev: String::from("CIN"),
        },
        8 => TableCategory {
            parameter: String::from("Storm Relative Helicity"),
            units: String::from("m2 s-2"),
            abbrev: String::from("HLCY"),
        },
        9 => TableCategory {
            parameter: String::from("Energy Helicity Index"),
            units: String::from("Numeric"),
            abbrev: String::from("EHLX"),
        },
        10 => TableCategory {
            parameter: String::from("Surface Lifted Index"),
            units: String::from("K"),
            abbrev: String::from("LFT X"),
        },
        11 => TableCategory {
            parameter: String::from("Best (4 layer) Lifted Index"),
            units: String::from("K"),
            abbrev: String::from("4LFTX"),
        },
        12 => TableCategory {
            parameter: String::from("Richardson Number"),
            units: String::from("Numeric"),
            abbrev: String::from("RI"),
        },
        13 => TableCategory {
            parameter: String::from("Showalter Index"),
            units: String::from("K"),
            abbrev: String::from("SHWINX"),
        },
        15 => TableCategory {
            parameter: String::from("Updraft Helicity"),
            units: String::from("m2 s-2"),
            abbrev: String::from("UPHL"),
        },
        16 => TableCategory {
            parameter: String::from("Bulk Richardson Number"),
            units: String::from("Numeric"),
            abbrev: String::from("BLKRN"),
        },
        17 => TableCategory {
            parameter: String::from("Gradient Richardson Number"),
            units: String::from("Numeric"),
            abbrev: String::from("GRDRN"),
        },
        18 => TableCategory {
            parameter: String::from("Flux Richardson Number"),
            units: String::from("Numeric"),
            abbrev: String::from("FLXRN"),
        },
        19 => TableCategory {
            parameter: String::from("Convective Available Potential Energy Shear"),
            units: String::from("m2 s-2"),
            abbrev: String::from("CONAPES"),
        },
        20 => TableCategory {
            parameter: String::from("Thunderstorm intensity index"),
            units: String::from("See Table 4.246"),
            abbrev: String::from("TIIDEX"),
        },
        192 => TableCategory {
            parameter: String::from("Surface Lifted Index"),
            units: String::from("K"),
            abbrev: String::from("LFT X"),
        },
        193 => TableCategory {
            parameter: String::from("Best (4 layer) Lifted Index"),
            units: String::from("K"),
            abbrev: String::from("4LFTX"),
        },
        194 => TableCategory {
            parameter: String::from("Richardson Number"),
            units: String::from("Numeric"),
            abbrev: String::from("RI"),
        },
        195 => TableCategory {
            parameter: String::from("Convective Weather Detection Index"),
            units: String::from(""),
            abbrev: String::from("CWDI"),
        },
        196 => TableCategory {
            parameter: String::from("Ultra Violet Index"),
            units: String::from("W m-2"),
            abbrev: String::from("UVI"),
        },
        197 => TableCategory {
            parameter: String::from("Updraft Helicity"),
            units: String::from("m2 s-2"),
            abbrev: String::from("UPHL"),
        },
        198 => TableCategory {
            parameter: String::from("Leaf Area Index"),
            units: String::from("Numeric"),
            abbrev: String::from("LAI"),
        },
        199 => TableCategory {
            parameter: String::from("Hourly Maximum of Updraft Helicity"),
            units: String::from("m2 s-2"),
            abbrev: String::from("MXUPHL"),
        },
        200 => TableCategory {
            parameter: String::from("Hourly Minimum of Updraft Helicity"),
            units: String::from("m2 s-2"),
            abbrev: String::from("MNUPHL"),
        },
        201 => TableCategory {
            parameter: String::from("Bourgoiun Negative Energy Layer (surface to freezing level)"),
            units: String::from("J kg-1"),
            abbrev: String::from("BNEGELAY"),
        },
        202 => TableCategory {
            parameter: String::from("Bourgoiun Positive Energy Layer (2k ft AGL to 400 hPa)"),
            units: String::from("J kg-1"),
            abbrev: String::from("BPOSELAY"),
        },
        203 => TableCategory {
            parameter: String::from("Downdraft CAPE"),
            units: String::from("J kg-1"),
            abbrev: String::from("DCAPE"),
        },
        204 => TableCategory {
            parameter: String::from("Effective Storm Relative Helicity"),
            units: String::from("m2 s-2"),
            abbrev: String::from("EFHL"),
        },
        205 => TableCategory {
            parameter: String::from("Enhanced Stretching Potential"),
            units: String::from("Numeric"),
            abbrev: String::from("ESP"),
        },
        206 => TableCategory {
            parameter: String::from("Critical Angle"),
            units: String::from("Degree"),
            abbrev: String::from("CANGLE"),
        },
        207 => TableCategory {
            parameter: String::from("Effective Surface Helicity"),
            units: String::from("m2 s-2"),
            abbrev: String::from("E3KH"),
        },
        208 => TableCategory {
            parameter: String::from("Significant Tornado Parameter with CIN-Effective Layer"),
            units: String::from("numeric"),
            abbrev: String::from("STPC"),
        },
        209 => TableCategory {
            parameter: String::from("Significant Hail Parameter"),
            units: String::from("numeric"),
            abbrev: String::from("SIGH"),
        },
        210 => TableCategory {
            parameter: String::from("Supercell Composite Parameter-Effective Layer"),
            units: String::from("numeric"),
            abbrev: String::from("SCCP"),
        },
        211 => TableCategory {
            parameter: String::from("Significant Tornado parameter-Fixed Layer"),
            units: String::from("numeric"),
            abbrev: String::from("SIGT"),
        },
        212 => TableCategory {
            parameter: String::from("Mixed Layer (100 mb) Virtual LFC"),
            units: String::from("numeric"),
            abbrev: String::from("MLFC"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from("Missing"),
        },
        14 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        21..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-0-13
/// PARAMETERS FOR DISCIPLINE 0 - CATEGORY 13
/// (Meteorological products, Aerosols category)
/// In Section 0, Octet 7 = 0
/// In Section 4, Octet 10 = 13
/// Revised 02/13/2012
/// Red text depicts changes made since 01/26/2006
///
/// **Notes:**
/// - Aerosol Type is described in Table 4.205.
///   @see [GRIB2 - Table 4.2-0-13: Aerosols Category](https://www.example.com)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 13.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 13 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_013(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Aerosol Type"),
            units: String::from("See Table 4.205"),
            abbrev: String::from("AEROT"),
        },
        192 => TableCategory {
            parameter: String::from("Particulate matter (coarse)"),
            units: String::from("µg m-3"),
            abbrev: String::from("PMTC"),
        },
        193 => TableCategory {
            parameter: String::from("Particulate matter (fine)"),
            units: String::from("µg m-3"),
            abbrev: String::from("PMTF"),
        },
        194 => TableCategory {
            parameter: String::from("Particulate matter (fine)"),
            units: String::from("log10 (µg m-3)"),
            abbrev: String::from("LPMTF"),
        },
        195 => TableCategory {
            parameter: String::from("Integrated column particulate matter (fine)"),
            units: String::from("log10 (µg m-3)"),
            abbrev: String::from("LIPMF"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from("Missing"),
        },
        1..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-0-14
/// PARAMETERS FOR DISCIPLINE 0 - CATEGORY 14
/// (Meteorological products, Trace Gases category)
/// In Section 0, Octet 7 = 0
/// In Section 4, Octet 10 = 14
/// Revised 12/04/2020
/// Red text depicts changes made since 02/13/2012
///
/// **Notes:**
/// - Trace gases parameters, including Ozone and PM2.5 related metrics.
///   @see [GRIB2 - Table 4.2-0-14: Trace Gases Category](https://www.example.com)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 14.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 14 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_014(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Total Ozone"),
            units: String::from("DU"),
            abbrev: String::from("TOZNE"),
        },
        1 => TableCategory {
            parameter: String::from("Ozone Mixing Ratio"),
            units: String::from("kg kg-1"),
            abbrev: String::from("O3MR"),
        },
        2 => TableCategory {
            parameter: String::from("Total Column Integrated Ozone"),
            units: String::from("DU"),
            abbrev: String::from("TCIOZ"),
        },
        192 => TableCategory {
            parameter: String::from("Ozone Mixing Ratio"),
            units: String::from("kg kg-1"),
            abbrev: String::from("O3MR"),
        },
        193 => TableCategory {
            parameter: String::from("Ozone Concentration"),
            units: String::from("ppb"),
            abbrev: String::from("OZCON"),
        },
        194 => TableCategory {
            parameter: String::from("Categorical Ozone Concentration"),
            units: String::from("Non-Dim"),
            abbrev: String::from("OZCAT"),
        },
        195 => TableCategory {
            parameter: String::from("Ozone Vertical Diffusion"),
            units: String::from("kg kg-1 s-1"),
            abbrev: String::from("VDFOZ"),
        },
        196 => TableCategory {
            parameter: String::from("Ozone Production"),
            units: String::from("kg kg-1 s-1"),
            abbrev: String::from("POZ"),
        },
        197 => TableCategory {
            parameter: String::from("Ozone Tendency"),
            units: String::from("kg kg-1 s-1"),
            abbrev: String::from("TOZ"),
        },
        198 => TableCategory {
            parameter: String::from("Ozone Production from Temperature Term"),
            units: String::from("kg kg-1 s-1"),
            abbrev: String::from("POZT"),
        },
        199 => TableCategory {
            parameter: String::from("Ozone Production from Column Ozone Term"),
            units: String::from("kg kg-1 s-1"),
            abbrev: String::from("POZO"),
        },
        200 => TableCategory {
            parameter: String::from("Ozone Daily Max from 1-hour Average"),
            units: String::from("ppbV"),
            abbrev: String::from("OZMAX1"),
        },
        201 => TableCategory {
            parameter: String::from("Ozone Daily Max from 8-hour Average"),
            units: String::from("ppbV"),
            abbrev: String::from("OZMAX8"),
        },
        202 => TableCategory {
            parameter: String::from("PM 2.5 Daily Max from 1-hour Average"),
            units: String::from("μg m-3"),
            abbrev: String::from("PDMAX1"),
        },
        203 => TableCategory {
            parameter: String::from("PM 2.5 Daily Max from 24-hour Average"),
            units: String::from("μg m-3"),
            abbrev: String::from("PDMAX24"),
        },
        204 => TableCategory {
            parameter: String::from("Acetaldehyde & Higher Aldehydes"),
            units: String::from("ppbV"),
            abbrev: String::from("ALD2"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from("Missing"),
        },
        3..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-0-15
/// PARAMETERS FOR DISCIPLINE 0 - CATEGORY 15
/// (Meteorological products, Radar category)
/// In Section 0, Octet 7 = 0
/// In Section 4, Octet 10 = 15
/// Revised 12/05/2014
/// Red text depicts changes made since 04/08/2013
///
/// **Notes:**
/// - Radar-related parameters such as reflectivity, velocity, and precipitation.
///   @see [GRIB2 - Table 4.2-0-15: Radar Category](https://www.example.com)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 15.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 15 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_015(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Base Spectrum Width"),
            units: String::from("m s-1"),
            abbrev: String::from("BSWID"),
        },
        1 => TableCategory {
            parameter: String::from("Base Reflectivity"),
            units: String::from("dB"),
            abbrev: String::from("BREF"),
        },
        2 => TableCategory {
            parameter: String::from("Base Radial Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("BRVEL"),
        },
        3 => TableCategory {
            parameter: String::from("Vertically-Integrated Liquid Water"),
            units: String::from("kg m-2"),
            abbrev: String::from("VIL"),
        },
        4 => TableCategory {
            parameter: String::from("Layer Maximum Base Reflectivity"),
            units: String::from("dB"),
            abbrev: String::from("LMAXBR"),
        },
        5 => TableCategory {
            parameter: String::from("Precipitation"),
            units: String::from("kg m-2"),
            abbrev: String::from("PREC"),
        },
        6 => TableCategory {
            parameter: String::from("Radar Spectra (1)"),
            units: String::from(""),
            abbrev: String::from("RDSP1"),
        },
        7 => TableCategory {
            parameter: String::from("Radar Spectra (2)"),
            units: String::from(""),
            abbrev: String::from("RDSP2"),
        },
        8 => TableCategory {
            parameter: String::from("Radar Spectra (3)"),
            units: String::from(""),
            abbrev: String::from("RDSP3"),
        },
        9 => TableCategory {
            parameter: String::from("Reflectivity of Cloud Droplets"),
            units: String::from("dB"),
            abbrev: String::from("RFCD"),
        },
        10 => TableCategory {
            parameter: String::from("Reflectivity of Cloud Ice"),
            units: String::from("dB"),
            abbrev: String::from("RFCI"),
        },
        11 => TableCategory {
            parameter: String::from("Reflectivity of Snow"),
            units: String::from("dB"),
            abbrev: String::from("RFSNOW"),
        },
        12 => TableCategory {
            parameter: String::from("Reflectivity of Rain"),
            units: String::from("dB"),
            abbrev: String::from("RFRAIN"),
        },
        13 => TableCategory {
            parameter: String::from("Reflectivity of Graupel"),
            units: String::from("dB"),
            abbrev: String::from("RFGRPL"),
        },
        14 => TableCategory {
            parameter: String::from("Reflectivity of Hail"),
            units: String::from("dB"),
            abbrev: String::from("RFHAIL"),
        },
        15 => TableCategory {
            parameter: String::from("Hybrid Scan Reflectivity"),
            units: String::from("dB"),
            abbrev: String::from("HSR"),
        },
        16 => TableCategory {
            parameter: String::from("Hybrid Scan Reflectivity Height"),
            units: String::from("m"),
            abbrev: String::from("HSRHT"),
        },
        192 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from("Missing"),
        },
        17..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-0-16
/// PARAMETERS FOR DISCIPLINE 0 - CATEGORY 16
/// (Meteorological products, Forecast Radar Imagery category)
/// In Section 0, Octet 7 = 0
/// In Section 4, Octet 10 = 16
/// Revised 02/10/2021
/// Red text depicts changes made since 12/06/2011
///
/// **Notes:**
/// - Radar reflectivity and Echo Top products.
/// - For Echo Top product, Use octet number 38 to store threshold value (e.g., 18.3 dB) in Product Definition Template 4.20.
/// - Decibel (dB) is a logarithmic measure of the relative power or radar reflectivity.
///   @see [GRIB2 - Table 4.2-0-16: Forecast Radar Imagery](https://www.example.com)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 16.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 16 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_016(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Equivalent radar reflectivity factor for rain"),
            units: String::from("m m6 m-3"),
            abbrev: String::from("REFZR"),
        },
        1 => TableCategory {
            parameter: String::from("Equivalent radar reflectivity factor for snow"),
            units: String::from("m m6 m-3"),
            abbrev: String::from("REFZI"),
        },
        2 => TableCategory {
            parameter: String::from(
                "Equivalent radar reflectivity factor for parameterized convection",
            ),
            units: String::from("m m6 m-3"),
            abbrev: String::from("REFZC"),
        },
        3 => TableCategory {
            parameter: String::from("Echo Top"),
            units: String::from("m"),
            abbrev: String::from("RETOP"),
        },
        4 => TableCategory {
            parameter: String::from("Reflectivity"),
            units: String::from("dB"),
            abbrev: String::from("REFD"),
        },
        5 => TableCategory {
            parameter: String::from("Composite reflectivity"),
            units: String::from("dB"),
            abbrev: String::from("REFC"),
        },
        192 => TableCategory {
            parameter: String::from("Equivalent radar reflectivity factor for rain"),
            units: String::from("m m6 m-3"),
            abbrev: String::from("REFZR"),
        },
        193 => TableCategory {
            parameter: String::from("Equivalent radar reflectivity factor for snow"),
            units: String::from("m m6 m-3"),
            abbrev: String::from("REFZI"),
        },
        194 => TableCategory {
            parameter: String::from(
                "Equivalent radar reflectivity factor for parameterized convection",
            ),
            units: String::from("m m6 m-3"),
            abbrev: String::from("REFZC"),
        },
        195 => TableCategory {
            parameter: String::from("Reflectivity"),
            units: String::from("dB"),
            abbrev: String::from("REFD"),
        },
        196 => TableCategory {
            parameter: String::from("Composite reflectivity"),
            units: String::from("dB"),
            abbrev: String::from("REFC"),
        },
        197 => TableCategory {
            parameter: String::from("Echo Top"),
            units: String::from("m"),
            abbrev: String::from("RETOP"),
        },
        198 => TableCategory {
            parameter: String::from("Hourly Maximum of Simulated Reflectivity"),
            units: String::from("dB"),
            abbrev: String::from("MAXREF"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from("Missing"),
        },
        6..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-0-17
/// PARAMETERS FOR DISCIPLINE 0 - CATEGORY 17
/// (Meteorological products, Electrodynamics category)
/// In Section 0, Octet 7 = 0
/// In Section 4, Octet 10 = 17
/// Revised 02/23/2021
/// Red text depicts changes made since 05/28/2019
///
/// **Notes:**
/// 1. Definition of LPI after Lynn et. al. (2010): Prediction of lightning flash density with the WRF model, Adv. Geosci., 23, 11-16.
/// 2. The total lightning flash density is the sum of cloud-to-ground and cloud-to-cloud lightning flash densities.
/// 3. The subgrid-scale lightning potential index is derived from subgrid-scale information for models with coarser resolution.
///    @see [GRIB2 - Table 4.2-0-17: Electrodynamics](https://www.example.com)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 17.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 17 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_017(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Lightning Strike Density"),
            units: String::from("m-2 s-1"),
            abbrev: String::from("LTNGSD"),
        },
        1 => TableCategory {
            parameter: String::from("Lightning Potential Index (LPI)"),
            units: String::from("J kg-1"),
            abbrev: String::from("LTPINX"),
        },
        2 => TableCategory {
            parameter: String::from("Cloud-to-Ground Lightning Flash Density"),
            units: String::from("km-2 day-1"),
            abbrev: String::from("CDGDLTFD"),
        },
        3 => TableCategory {
            parameter: String::from("Cloud-to-Cloud Lightning Flash Density"),
            units: String::from("km-2 day-1"),
            abbrev: String::from("CDCDLTFD"),
        },
        4 => TableCategory {
            parameter: String::from("Total Lightning Flash Density"),
            units: String::from("km-2 day-1"),
            abbrev: String::from("TLGTFD"),
        },
        5 => TableCategory {
            parameter: String::from("Subgrid-scale lightning potential index"),
            units: String::from("J kg-1"),
            abbrev: String::from("SLNGPIDX"),
        },
        192 => TableCategory {
            parameter: String::from("Lightning"),
            units: String::from("non-dim"),
            abbrev: String::from("LTNG"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from("Missing"),
        },
        6..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-0-18
/// PARAMETERS FOR DISCIPLINE 0 - CATEGORY 18
/// (Meteorological products, Nuclear/Radiology Imagery category)
/// In Section 0, Octet 7 = 0
/// In Section 4, Octet 10 = 18
/// Revised 07/15/2024
/// Red text depicts changes made since 07/15/2024
///
/// **Notes:**
/// 1. Statistical process 1 (Accumulation) does not change units. It is recommended to use another parameter
///    without the word "time-integrated" in its name and accumulation in PDT.
/// 2. Conversion factor between "Specific Activity Concentration" (14) and "Air Concentration" (10) is "Mass Density" [kg m-3].
/// 3. Use the radionuclide release start date as baseline to determine activity arrival or activity end.
///    @see [GRIB2 - Table 4.2-0-18: Nuclear/Radiology Imagery](https://www.example.com)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 18.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 18 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_018(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Air Concentration of Caesium 137"),
            units: String::from("Bq m-3"),
            abbrev: String::from("ACCES"),
        },
        1 => TableCategory {
            parameter: String::from("Air Concentration of Iodine 131"),
            units: String::from("Bq m-3"),
            abbrev: String::from("ACIOD"),
        },
        2 => TableCategory {
            parameter: String::from("Air Concentration of Radioactive Pollutant"),
            units: String::from("Bq m-3"),
            abbrev: String::from("ACRADP"),
        },
        3 => TableCategory {
            parameter: String::from("Ground Deposition of Caesium 137"),
            units: String::from("Bq m-2"),
            abbrev: String::from("GDCES"),
        },
        4 => TableCategory {
            parameter: String::from("Ground Deposition of Iodine 131"),
            units: String::from("Bq m-2"),
            abbrev: String::from("GDIOD"),
        },
        5 => TableCategory {
            parameter: String::from("Ground Deposition of Radioactive Pollutant"),
            units: String::from("Bq m-2"),
            abbrev: String::from("GDRADP"),
        },
        6 => TableCategory {
            parameter: String::from("Time Integrated Air Concentration of Cesium Pollutant"),
            units: String::from("Bq s m-3"),
            abbrev: String::from("TIACCP"),
        },
        7 => TableCategory {
            parameter: String::from("Time Integrated Air Concentration of Iodine Pollutant"),
            units: String::from("Bq s m-3"),
            abbrev: String::from("TIACIP"),
        },
        8 => TableCategory {
            parameter: String::from("Time Integrated Air Concentration of Radioactive Pollutant"),
            units: String::from("Bq s m-3"),
            abbrev: String::from("TIACRP"),
        },
        9 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        10 => TableCategory {
            parameter: String::from("Air Concentration"),
            units: String::from("Bq m-3"),
            abbrev: String::from("AIRCON"),
        },
        11 => TableCategory {
            parameter: String::from("Wet Deposition"),
            units: String::from("Bq m-2"),
            abbrev: String::from("WETDEP"),
        },
        12 => TableCategory {
            parameter: String::from("Dry Deposition"),
            units: String::from("Bq m-2"),
            abbrev: String::from("DRYDEP"),
        },
        13 => TableCategory {
            parameter: String::from("Total Deposition (Wet + Dry)"),
            units: String::from("Bq m-2"),
            abbrev: String::from("TOTLWD"),
        },
        14 => TableCategory {
            parameter: String::from("Specific Activity Concentration"),
            units: String::from("Bq kg-1"),
            abbrev: String::from("SACON"),
        },
        15 => TableCategory {
            parameter: String::from("Maximum of Air Concentration in Layer"),
            units: String::from("Bq m-3"),
            abbrev: String::from("MAXACON"),
        },
        16 => TableCategory {
            parameter: String::from("Height of Maximum of Air Concentration"),
            units: String::from("m"),
            abbrev: String::from("HMXACON"),
        },
        17 => TableCategory {
            parameter: String::from("Column-Integrated Air Concentration"),
            units: String::from("Bq m-2"),
            abbrev: String::from("CIAIRC"),
        },
        18 => TableCategory {
            parameter: String::from("Column-Averaged Air Concentration in Layer"),
            units: String::from("Bq m-3"),
            abbrev: String::from("CAACL"),
        },
        19 => TableCategory {
            parameter: String::from("Deposition activity arrival"),
            units: String::from("s"),
            abbrev: String::from("DEPACTA"),
        },
        20 => TableCategory {
            parameter: String::from("Deposition activity ended"),
            units: String::from("s"),
            abbrev: String::from("DEPACTE"),
        },
        21 => TableCategory {
            parameter: String::from("Cloud activity arrival"),
            units: String::from("s"),
            abbrev: String::from("CLDACTA"),
        },
        22 => TableCategory {
            parameter: String::from("Cloud activity ended"),
            units: String::from("s"),
            abbrev: String::from("CLDACTE"),
        },
        23 => TableCategory {
            parameter: String::from("Effective dose rate"),
            units: String::from("nSv h-1"),
            abbrev: String::from("EFFDOSER"),
        },
        24 => TableCategory {
            parameter: String::from("Thyroid dose rate (adult)"),
            units: String::from("nSv h-1"),
            abbrev: String::from("THYDOSER"),
        },
        25 => TableCategory {
            parameter: String::from("Gamma dose rate (adult)"),
            units: String::from("nSv h-1"),
            abbrev: String::from("GAMDOSER"),
        },
        26 => TableCategory {
            parameter: String::from("Activity emission"),
            units: String::from("Bq s-1"),
            abbrev: String::from("ACTEMM"),
        },
        192 => TableCategory {
            parameter: String::from("Lightning"),
            units: String::from("non-dim"),
            abbrev: String::from("LTNG"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from("Missing"),
        },
        27..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-0-19
/// PARAMETERS FOR DISCIPLINE 0 - CATEGORY 19
/// (Meteorological products, Physical Atmospheric category)
/// In Section 0, Octet 7 = 0
/// In Section 4, Octet 10 = 19
/// Revised 12/07/2023
/// Red text depicts changes made since 01/19/2022
///
/// **Notes:**
/// 1. Parameter deprecated - See Regulation 92.6.2 and use another parameter instead.
/// 2. Supercooled large droplets (SLD) are defined as those with a diameter greater than 50 microns.
/// 3. Eddy Dissipation parameter is third root of eddy dissipation rate [m2 s-3].
/// 4. In astronomy, sky transparency means the effect on the viewing experience caused by the scattering of light through atmospheric water vapour, aerosols or other constituents. Ideal transparency conditions produce a black night sky conducive to viewing faint astronomical objects, almost like being in outer space. In poor transparency conditions, which may occur even in cloud-free conditions, the deep sky background is greyish (not black), faint details are washed out and contrast is reduced.
/// 5. Seeing means the steadiness or turbulence of the atmosphere in the context of astronomical observation. Turbulence causes rapid random fluctuations of the optical path through the atmosphere. The twinkling of stars, for example, occurs in poor seeing conditions.
/// 6. A duct layer is an atmospheric layer with a refractivity which leads to a trapping of electromagnetic waves. In a trapping layer the refractivity leads to a bending of EM waves, which is stronger than the Earth's curvature.
///    @see [GRIB2 - Table 4.2-0-19: Physical Atmospheric](https://www.example.com)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 19.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 19 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_019(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Visibility"),
            units: String::from("m"),
            abbrev: String::from("VIS"),
        },
        1 => TableCategory {
            parameter: String::from("Albedo"),
            units: String::from("%"),
            abbrev: String::from("ALBDO"),
        },
        2 => TableCategory {
            parameter: String::from("Thunderstorm Probability"),
            units: String::from("%"),
            abbrev: String::from("TSTM"),
        },
        3 => TableCategory {
            parameter: String::from("Mixed Layer Depth"),
            units: String::from("m"),
            abbrev: String::from("MIXHT"),
        },
        4 => TableCategory {
            parameter: String::from("Volcanic Ash"),
            units: String::from("See Table 4.206"),
            abbrev: String::from("VOLASH"),
        },
        5 => TableCategory {
            parameter: String::from("Icing Top"),
            units: String::from("m"),
            abbrev: String::from("ICIT"),
        },
        6 => TableCategory {
            parameter: String::from("Icing Base"),
            units: String::from("m"),
            abbrev: String::from("ICIB"),
        },
        7 => TableCategory {
            parameter: String::from("Icing"),
            units: String::from("See Table 4.207"),
            abbrev: String::from("ICI"),
        },
        8 => TableCategory {
            parameter: String::from("Turbulence Top"),
            units: String::from("m"),
            abbrev: String::from("TURBT"),
        },
        9 => TableCategory {
            parameter: String::from("Turbulence Base"),
            units: String::from("m"),
            abbrev: String::from("TURBB"),
        },
        10 => TableCategory {
            parameter: String::from("Turbulence"),
            units: String::from("See Table 4.208"),
            abbrev: String::from("TURB"),
        },
        11 => TableCategory {
            parameter: String::from("Turbulent Kinetic Energy"),
            units: String::from("J kg-1"),
            abbrev: String::from("TKE"),
        },
        12 => TableCategory {
            parameter: String::from("Planetary Boundary Layer Regime"),
            units: String::from("See Table 4.209"),
            abbrev: String::from("PBLREG"),
        },
        13 => TableCategory {
            parameter: String::from("Contrail Intensity"),
            units: String::from("See Table 4.210"),
            abbrev: String::from("CONTI"),
        },
        14 => TableCategory {
            parameter: String::from("Contrail Engine Type"),
            units: String::from("See Table 4.211"),
            abbrev: String::from("CONTET"),
        },
        15 => TableCategory {
            parameter: String::from("Contrail Top"),
            units: String::from("m"),
            abbrev: String::from("CONTT"),
        },
        16 => TableCategory {
            parameter: String::from("Contrail Base"),
            units: String::from("m"),
            abbrev: String::from("CONTB"),
        },
        17 => TableCategory {
            parameter: String::from("Maximum Snow Albedo"),
            units: String::from("%"),
            abbrev: String::from("MXSALB"),
        },
        18 => TableCategory {
            parameter: String::from("Snow-Free Albedo"),
            units: String::from("%"),
            abbrev: String::from("SNFALB"),
        },
        19 => TableCategory {
            parameter: String::from("Snow Albedo"),
            units: String::from("%"),
            abbrev: String::from("SALBD"),
        },
        20 => TableCategory {
            parameter: String::from("Icing"),
            units: String::from("%"),
            abbrev: String::from("ICIP"),
        },
        21 => TableCategory {
            parameter: String::from("In-Cloud Turbulence"),
            units: String::from("%"),
            abbrev: String::from("CTP"),
        },
        22 => TableCategory {
            parameter: String::from("Clear Air Turbulence (CAT)"),
            units: String::from("%"),
            abbrev: String::from("CAT"),
        },
        23 => TableCategory {
            parameter: String::from("Supercooled Large Droplet Probability"),
            units: String::from("%"),
            abbrev: String::from("SLDP"),
        },
        24 => TableCategory {
            parameter: String::from("Convective Turbulent Kinetic Energy"),
            units: String::from("J kg-1"),
            abbrev: String::from("CONTKE"),
        },
        25 => TableCategory {
            parameter: String::from("Weather"),
            units: String::from("See Table 4.225"),
            abbrev: String::from("WIWW"),
        },
        26 => TableCategory {
            parameter: String::from("Convective Outlook"),
            units: String::from("See Table 4.224"),
            abbrev: String::from("CONVO"),
        },
        27 => TableCategory {
            parameter: String::from("Icing Scenario"),
            units: String::from("See Table 4.227"),
            abbrev: String::from("ICESC"),
        },
        28 => TableCategory {
            parameter: String::from("Mountain Wave Turbulence (Eddy Dissipation Rate)"),
            units: String::from("m2/3 s-1"),
            abbrev: String::from("MWTURB"),
        },
        29 => TableCategory {
            parameter: String::from("Clear Air Turbulence (CAT) (Eddy Dissipation Rate)"),
            units: String::from("m2/3 s-1"),
            abbrev: String::from("CATEDR"),
        },
        30 => TableCategory {
            parameter: String::from("Eddy Dissipation Parameter"),
            units: String::from("m2/3 s-1"),
            abbrev: String::from("EDPARM"),
        },
        31 => TableCategory {
            parameter: String::from("Maximum of Eddy Dissipation Parameter in Layer"),
            units: String::from("m2/3 s-1"),
            abbrev: String::from("MXEDPRM"),
        },
        32 => TableCategory {
            parameter: String::from("Highest Freezing Level"),
            units: String::from("m"),
            abbrev: String::from("HIFREL"),
        },
        33 => TableCategory {
            parameter: String::from("Visibility Through Liquid Fog"),
            units: String::from("m"),
            abbrev: String::from("VISLFOG"),
        },
        34 => TableCategory {
            parameter: String::from("Visibility Through Ice Fog"),
            units: String::from("m"),
            abbrev: String::from("VISIFOG"),
        },
        35 => TableCategory {
            parameter: String::from("Visibility Through Blowing Snow"),
            units: String::from("m"),
            abbrev: String::from("VISBSN"),
        },
        36 => TableCategory {
            parameter: String::from("Presence of Snow Squalls"),
            units: String::from("See Table 4.222"),
            abbrev: String::from("PSNOWS"),
        },
        37 => TableCategory {
            parameter: String::from("Icing Severity"),
            units: String::from("See Table 4.228"),
            abbrev: String::from("ICESEV"),
        },
        38 => TableCategory {
            parameter: String::from("Sky transparency index"),
            units: String::from("See Table 4.214"),
            abbrev: String::from("SKYIDX"),
        },
        39 => TableCategory {
            parameter: String::from("Seeing index"),
            units: String::from("See Table 4.214"),
            abbrev: String::from("SEEINDEX"),
        },
        40 => TableCategory {
            parameter: String::from("Snow level"),
            units: String::from("m"),
            abbrev: String::from("SNOWLVL"),
        },
        41 => TableCategory {
            parameter: String::from("Duct base height"),
            units: String::from("m"),
            abbrev: String::from("DBHEIGHT"),
        },
        42 => TableCategory {
            parameter: String::from("Trapping layer base height"),
            units: String::from("m"),
            abbrev: String::from("TLBHEIGHT"),
        },
        43 => TableCategory {
            parameter: String::from("Trapping layer top height"),
            units: String::from("m"),
            abbrev: String::from("TLTHEIGHT"),
        },
        44 => TableCategory {
            parameter: String::from("Mean vertical gradient of refractivity inside trapping layer"),
            units: String::from("m-1"),
            abbrev: String::from("MEANVGRTL"),
        },
        45 => TableCategory {
            parameter: String::from(
                "Minimum vertical gradient of refractivity inside trapping layer",
            ),
            units: String::from("m-1"),
            abbrev: String::from("MINVGRTL"),
        },
        46 => TableCategory {
            parameter: String::from("Net radiation flux"),
            units: String::from("W m-2"),
            abbrev: String::from("NETRADFLUX"),
        },
        47 => TableCategory {
            parameter: String::from("Global irradiance on tilted surfaces"),
            units: String::from("W m-2"),
            abbrev: String::from("GLIRRTS"),
        },
        48 => TableCategory {
            parameter: String::from("Top of persistent contrails"),
            units: String::from("m"),
            abbrev: String::from("PCONTT"),
        },
        49 => TableCategory {
            parameter: String::from("Base of persistent contrails"),
            units: String::from("m"),
            abbrev: String::from("PCONTB"),
        },
        50 => TableCategory {
            parameter: String::from(
                "Convectively-induced turbulence (CIT) (eddy dissipation rate)",
            ),
            units: String::from("m2/3 s-1"),
            abbrev: String::from("CITEDR"),
        },
        192 => TableCategory {
            parameter: String::from("Maximum Snow Albedo"),
            units: String::from("%"),
            abbrev: String::from("MXSALB"),
        },
        193 => TableCategory {
            parameter: String::from("Snow-Free Albedo"),
            units: String::from("%"),
            abbrev: String::from("SNFALB"),
        },
        194 => TableCategory {
            parameter: String::from("Slight risk convective outlook"),
            units: String::from("categorical"),
            abbrev: String::from("SRCONO"),
        },
        195 => TableCategory {
            parameter: String::from("Moderate risk convective outlook"),
            units: String::from("categorical"),
            abbrev: String::from("MRCONO"),
        },
        196 => TableCategory {
            parameter: String::from("High risk convective outlook"),
            units: String::from("categorical"),
            abbrev: String::from("HRCONO"),
        },
        197 => TableCategory {
            parameter: String::from("Tornado probability"),
            units: String::from("%"),
            abbrev: String::from("TORPROB"),
        },
        198 => TableCategory {
            parameter: String::from("Hail probability"),
            units: String::from("%"),
            abbrev: String::from("HAILPROB"),
        },
        199 => TableCategory {
            parameter: String::from("Wind probability"),
            units: String::from("%"),
            abbrev: String::from("WINDPROB"),
        },
        200 => TableCategory {
            parameter: String::from("Significant Tornado probability"),
            units: String::from("%"),
            abbrev: String::from("STORPROB"),
        },
        201 => TableCategory {
            parameter: String::from("Significant Hail probability"),
            units: String::from("%"),
            abbrev: String::from("SHAILPRO"),
        },
        202 => TableCategory {
            parameter: String::from("Significant Wind probability"),
            units: String::from("%"),
            abbrev: String::from("SWINDPRO"),
        },
        203 => TableCategory {
            parameter: String::from("Categorical Thunderstorm"),
            units: String::from("Code table 4.222"),
            abbrev: String::from("TSTMC"),
        },
        204 => TableCategory {
            parameter: String::from("Number of mixed layers next to surface"),
            units: String::from("integer"),
            abbrev: String::from("MIXLY"),
        },
        205 => TableCategory {
            parameter: String::from("Flight Category"),
            units: String::from(""),
            abbrev: String::from("FLGHT"),
        },
        206 => TableCategory {
            parameter: String::from("Confidence - Ceiling"),
            units: String::from(""),
            abbrev: String::from("CICEL"),
        },
        207 => TableCategory {
            parameter: String::from("Confidence - Visibility"),
            units: String::from(""),
            abbrev: String::from("CIVIS"),
        },
        208 => TableCategory {
            parameter: String::from("Confidence - Flight Category"),
            units: String::from(""),
            abbrev: String::from("CIFLT"),
        },
        209 => TableCategory {
            parameter: String::from("Low-Level aviation interest"),
            units: String::from(""),
            abbrev: String::from("LAVNI"),
        },
        210 => TableCategory {
            parameter: String::from("High-Level aviation interest"),
            units: String::from(""),
            abbrev: String::from("HAVNI"),
        },
        211 => TableCategory {
            parameter: String::from("Visible, Black Sky Albedo"),
            units: String::from("%"),
            abbrev: String::from("SBSALB"),
        },
        212 => TableCategory {
            parameter: String::from("Visible, White Sky Albedo"),
            units: String::from("%"),
            abbrev: String::from("SWSALB"),
        },
        213 => TableCategory {
            parameter: String::from("Near IR, Black Sky Albedo"),
            units: String::from("%"),
            abbrev: String::from("NBSALB"),
        },
        214 => TableCategory {
            parameter: String::from("Near IR, White Sky Albedo"),
            units: String::from("%"),
            abbrev: String::from("NWSALB"),
        },
        215 => TableCategory {
            parameter: String::from("Total Probability of Severe Thunderstorms (Days 2,3)"),
            units: String::from("%"),
            abbrev: String::from("PRSVR"),
        },
        216 => TableCategory {
            parameter: String::from("Total Probability of Extreme Severe Thunderstorms (Days 2,3)"),
            units: String::from("%"),
            abbrev: String::from("PRSIGSVR"),
        },
        217 => TableCategory {
            parameter: String::from("Supercooled Large Droplet (SLD) Icing"),
            units: String::from("See Table 4.207"),
            abbrev: String::from("SIPD"),
        },
        218 => TableCategory {
            parameter: String::from("Radiative emissivity"),
            units: String::from(""),
            abbrev: String::from("EPSR"),
        },
        219 => TableCategory {
            parameter: String::from("Turbulence Potential Forecast Index"),
            units: String::from(""),
            abbrev: String::from("TPFI"),
        },
        220 => TableCategory {
            parameter: String::from("Categorical Severe Thunderstorm"),
            units: String::from("Code table 4.222"),
            abbrev: String::from("SVRTS"),
        },
        221 => TableCategory {
            parameter: String::from("Probability of Convection"),
            units: String::from("%"),
            abbrev: String::from("PROCON"),
        },
        222 => TableCategory {
            parameter: String::from("Convection Potential"),
            units: String::from("Code table 4.222"),
            abbrev: String::from("CONVP"),
        },
        232 => TableCategory {
            parameter: String::from("Volcanic Ash Forecast Transport and Dispersion"),
            units: String::from("log10 (kg m-3)"),
            abbrev: String::from("VAFTD"),
        },
        233 => TableCategory {
            parameter: String::from("Icing probability"),
            units: String::from("non-dim"),
            abbrev: String::from("ICPRB"),
        },
        234 => TableCategory {
            parameter: String::from("Icing Severity"),
            units: String::from("non-dim"),
            abbrev: String::from("ICSEV"),
        },
        235 => TableCategory {
            parameter: String::from("Joint Fire Weather Probability"),
            units: String::from("%"),
            abbrev: String::from("JFWPRB"),
        },
        236 => TableCategory {
            parameter: String::from("Snow Level"),
            units: String::from("m"),
            abbrev: String::from("SNOWLVL"),
        },
        237 => TableCategory {
            parameter: String::from("Dry Thunderstorm Probability"),
            units: String::from("%"),
            abbrev: String::from("DRYTPROB"),
        },
        238 => TableCategory {
            parameter: String::from("Ellrod Index"),
            units: String::from(""),
            abbrev: String::from("ELLINX"),
        },
        239 => TableCategory {
            parameter: String::from("Craven-Wiedenfeld Aggregate Severe Parameter"),
            units: String::from("Numeric"),
            abbrev: String::from("CWASP"),
        },
        240 => TableCategory {
            parameter: String::from("Continuous Icing Severity"),
            units: String::from("non-dim"),
            abbrev: String::from("ICESEVCON"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        51..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        223..=231 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        _ => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}
