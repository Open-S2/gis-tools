use crate::{
    parsers::Reader,
    readers::{
        Grib2Sections, Grib2Table4_3, Grib2Table4_4, Grib2Table4_6, Grib2Table4_7, Grib2TableA,
        TableCategory, TypeAndUnit, grib2_lookup_table4_1, grib2_lookup_table4_5,
        grib2_lookup_table42,
    },
    util::Date,
};
use alloc::string::String;

/// Returns a template generator for the given template number
///
/// @param template - the template number to generate
/// @param reader - the byte data to read
/// @param sections - the sections of the GRIB2 message that have been parsed so far
/// @returns - generated template data
#[derive(Debug, Clone, PartialEq)]
pub enum Grib2ProductDefinition {
    /// Analysis or forecast at a horizontal level or in a horizontal layer at a point in time.
    Grib2Template40(Grib2Template40),
    /// Individual ensemble forecast, control and perturbed, at a horizontal level or in a
    /// horizontal layer at a point in time.
    Grib2Template41(Grib2Template41),
    /// Derived forecast, based on all ensemble members at a horizontal level or in a horizontal
    /// layer at a point in time.
    Grib2Template42(Grib2Template42),
}
impl Grib2ProductDefinition {
    /// Create a new instance of Grib2ProductDefinition
    pub fn new<T: Reader>(template: u16, reader: &T, sections: &Grib2Sections) -> Self {
        match template {
            0 => Grib2ProductDefinition::Grib2Template40(Grib2Template40::new(reader, sections)),
            1 => Grib2ProductDefinition::Grib2Template41(Grib2Template41::new(reader, sections)),
            2 => Grib2ProductDefinition::Grib2Template42(Grib2Template42::new(reader, sections)),
            _ => panic!("Template 4.{template} not defined"),
        }
    }
}

/// PRODUCT DEFINITION TEMPLATE 4.0
///
/// Analysis or forecast at a horizontal level or in
/// a horizontal layer at a point in time.
///
/// [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-0.shtml)
///
/// @param section - the byte data to read
/// @param sections - the sections of the GRIB2 message that have been parsed so far
/// @returns - the parsed template
#[derive(Debug, Clone, PartialEq)]
pub struct Grib2Template40 {
    /// table accessed category
    category: String,
    /// Paramater
    values: TableCategory,
    /// Parameter category (see Code [Table 4.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-1.shtml))
    parameter_category: u8,
    /// Parameter number (see Code [Table 4.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2.shtml))
    parameter_number: u8,
    /// Type of generating process (see Code [Table 4.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-3.shtml))
    gen_process_type: Grib2Table4_3,
    /// Background generating process identifier (defined by originating centre)
    background_gen_process: u8,
    /// Analysis or forecast generating process identifier (see Code [ON388 Table A](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablea.html))
    forecast_gen_process: Grib2TableA,
    /// Hours after reference time data cutoff (see Notes)
    hours_after_ref_time: u16,
    /// Minutes after reference time data cutoff (see Notes)
    min_after_ref_time: u8,
    /// Indicator of unit of time range (see Code [Table 4.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-4.shtml))
    unit_of_time_range_indicator: Grib2Table4_4,
    /// Forecast time in units defined by octet 18
    forecast_time: Date,
    /// First fixed surface
    surface1: TypeAndUnit, // grib2_lookup_table4_5
    /// Type of first fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml))
    surface1_type: u8,
    /// Scale factor of first fixed surface
    surface1_scale: u8,
    /// Scaled value of first fixed surface
    surface1_value: u32,
    /// Second fixed surface
    surface2: TypeAndUnit, // grib2_lookup_table4_5
    /// Type of second fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml))
    surface2_type: u8,
    /// Scale factor of second fixed surface
    surface2_scale: u8,
    /// Scaled value of second fixed surface
    surface2_value: u32,
}
impl Grib2Template40 {
    /// Create a new instance of Grib2ProductDefinition
    pub fn new<T: Reader>(reader: &T, sections: &Grib2Sections) -> Self {
        let discipline = sections.indicator.as_ref().map(|d| u8::from(d.discipline)).unwrap_or(0);
        let ref_time =
            sections.identification.as_ref().map(|i| i.ref_time.clone()).unwrap_or_default();
        let parameter_category = reader.uint8(Some(9));
        let parameter_number = reader.uint8(Some(10));
        let gen_process_type = reader.uint8(Some(11));
        let background_gen_process = reader.uint8(Some(12));
        let forecast_gen_process = reader.uint8(Some(13));
        let hours_after_ref_time = reader.uint16_be(Some(14));
        let min_after_ref_time = reader.uint8(Some(16));
        let unit_of_time_range_indicator: Grib2Table4_4 = reader.uint8(Some(17)).into();
        let forecast_time = reader.uint32_be(Some(18));
        let surface1_type = reader.uint8(Some(22));
        let surface1_scale = reader.uint8(Some(23));
        let surface1_value = reader.uint32_be(Some(24));
        let surface2_type = reader.uint8(Some(28));
        let surface2_scale = reader.uint8(Some(29));
        let surface2_value = reader.uint32_be(Some(30));
        let category = grib2_lookup_table4_1(discipline, parameter_category);
        let values = grib2_lookup_table42(discipline, parameter_category)(parameter_number);
        let surface1 = grib2_lookup_table4_5(surface1_type);
        let surface2 = grib2_lookup_table4_5(surface2_type);

        Self {
            category,
            values,
            parameter_category,
            parameter_number,
            gen_process_type: gen_process_type.into(),
            background_gen_process,
            forecast_gen_process: forecast_gen_process.into(),
            hours_after_ref_time,
            min_after_ref_time,
            unit_of_time_range_indicator,
            forecast_time: calculate_forecast_time(
                &ref_time,
                forecast_time as i64,
                &unit_of_time_range_indicator,
            ),
            surface1,
            surface1_type,
            surface1_scale,
            surface1_value,
            surface2,
            surface2_type,
            surface2_scale,
            surface2_value,
        }
    }
}

/// PRODUCT DEFINITION TEMPLATE 4.1
///
/// Individual ensemble forecast, control and perturbed, at a horizontal
/// level or in a horizontal layer at a point in time.
///
/// [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-1.shtml)
///
/// @param section - the byte data to read
/// @param sections - the sections of the GRIB2 message that have been parsed so far
/// @returns - the parsed template
#[derive(Debug, Clone, PartialEq)]
pub struct Grib2Template41 {
    /// table accessed category
    category: String,
    /// Paramater
    values: TableCategory,
    /// Parameter category (see Code [Table 4.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-1.shtml))
    parameter_category: u8,
    /// Parameter number (see Code [Table 4.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2.shtml))
    parameter_number: u8,
    /// Type of generating process (see Code [Table 4.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-3.shtml))
    gen_process_type: Grib2Table4_3,
    /// Background generating process identifier (defined by originating centre)
    background_gen_process: u8,
    /// Forecast generating process identifier (see Code [ON388 Table A](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablea.html))
    forecast_gen_process: Grib2TableA,
    /// Hours after reference time data cutoff (see Notes)
    hours_after_ref_time: u16,
    /// Minutes after reference time data cutoff (see Notes)
    min_after_ref_time: u8,
    /// Indicator of unit of time range (see Code [Table 4.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-4.shtml))
    unit_of_time_range_indicator: Grib2Table4_4,
    /// Forecast time in units defined by octet 18
    forecast_time: Date,
    /// First fixed surface
    surface1: TypeAndUnit,
    /// Type of first fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml), result stored in `surface1`)
    surface1_type: u8,
    /// Scale factor of first fixed surface
    surface1_scale: u8,
    /// Scaled value of first fixed surface
    surface1_value: u32,
    /// Second fixed surface
    surface2: TypeAndUnit,
    /// Type of second fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml) result stored in `surface2`)
    surface2_type: u8,
    /// Scale factor of second fixed surface
    surface2_scale: u8,
    /// Scaled value of second fixed surface
    surface2_value: u32,
    /// Type of ensemble forecast (see Code [Table 4.6](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-6.shtml))
    ensemble_forecast_type: Grib2Table4_6,
    /// Perturbation number
    perturbation_number: u8,
    /// Number of forecasts in ensemble
    num_forecasts_in_ensemble: u8,
}
impl Grib2Template41 {
    /// Create a new instance of Grib2ProductDefinition
    pub fn new<T: Reader>(reader: &T, sections: &Grib2Sections) -> Self {
        let discipline = sections.indicator.as_ref().map(|d| u8::from(d.discipline)).unwrap_or(0);
        let ref_time =
            sections.identification.as_ref().map(|i| i.ref_time.clone()).unwrap_or_default();
        let parameter_category = reader.uint8(Some(9));
        let parameter_number = reader.uint8(Some(10));
        let gen_process_type = reader.uint8(Some(11));
        let background_gen_process = reader.uint8(Some(12));
        let forecast_gen_process = reader.uint8(Some(13));
        let hours_after_ref_time = reader.uint16_be(Some(14));
        let min_after_ref_time = reader.uint8(Some(16));
        let unit_of_time_range_indicator = reader.uint8(Some(17));
        let forecast_time = reader.uint32_be(Some(18));
        let surface1_type = reader.uint8(Some(22));
        let surface1_scale = reader.uint8(Some(23));
        let surface1_value = reader.uint32_be(Some(24));
        let surface2_type = reader.uint8(Some(28));
        let surface2_scale = reader.uint8(Some(29));
        let surface2_value = reader.uint32_be(Some(30));
        let ensemble_forecast_type = reader.uint8(Some(34));
        let perturbation_number = reader.uint8(Some(35));
        let num_forecasts_in_ensemble = reader.uint8(Some(36));
        let category = grib2_lookup_table4_1(discipline, parameter_category);
        let values = grib2_lookup_table42(discipline, parameter_category)(parameter_number);
        let surface1 = grib2_lookup_table4_5(surface1_type);
        let surface2 = grib2_lookup_table4_5(surface2_type);
        let unit_of_time_range_indicator = unit_of_time_range_indicator.into();

        Self {
            category,
            values,
            parameter_category,
            parameter_number,
            gen_process_type: gen_process_type.into(),
            background_gen_process,
            forecast_gen_process: forecast_gen_process.into(),
            hours_after_ref_time,
            min_after_ref_time,
            unit_of_time_range_indicator,
            forecast_time: calculate_forecast_time(
                &ref_time,
                forecast_time as i64,
                &unit_of_time_range_indicator,
            ),
            surface1,
            surface1_type,
            surface1_scale,
            surface1_value,
            surface2,
            surface2_type,
            surface2_scale,
            surface2_value,
            ensemble_forecast_type: ensemble_forecast_type.into(),
            perturbation_number,
            num_forecasts_in_ensemble,
        }
    }
}

/// PRODUCT DEFINITION TEMPLATE 4.2
///
/// Derived forecast, based on all ensemble members at a horizontal
/// level or in a horizontal layer at a point in time.
///
/// [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_temp4-2.shtml)
///
/// @param section - the byte data to read
/// @param sections - the sections of the GRIB2 message that have been parsed so far
/// @returns - the parsed template
#[derive(Debug, Clone, PartialEq)]
pub struct Grib2Template42 {
    /// table accessed category
    category: String,
    /// Paramater
    values: TableCategory,
    /// Parameter category (see Code [Table 4.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-1.shtml)) */
    parameter_category: u8,
    /// Parameter number (see Code [Table 4.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2.shtml)) */
    parameter_number: u8,
    /// Type of generating process (see Code [Table 4.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-3.shtml)) */
    gen_process_type: Grib2Table4_3,
    /// Background generating process identifier (defined by originating centre) */
    background_gen_process: u8,
    /// Forecast generating process identifier (see Code [ON388 Table A](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablea.html)) */
    forecast_gen_process: Grib2TableA,
    /// Hours after reference time data cutoff (see Notes) */
    hours_after_ref_time: u16,
    /// Minutes after reference time data cutoff */
    min_after_ref_time: u8,
    /// Indicator of unit of time range (see Code [Table 4.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-4.shtml)) */
    unit_of_time_range_indicator: Grib2Table4_4,
    /// Forecast time in units defined by octet 18 */
    forecast_time: Date,
    /// First fixed surface */
    surface1: TypeAndUnit,
    /// Type of first fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml)) */
    surface1_type: u8,
    /// Scale factor of first fixed surface */
    surface1_scale: u8,
    /// Scaled value of first fixed surface */
    surface1_value: u32,
    /// Second fixed surface */
    surface2: TypeAndUnit,
    /// Type of second fixed surface (see Code [Table 4.5](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-5.shtml)) */
    surface2_type: u8,
    /// Scale factor of second fixed surface */
    surface2_scale: u8,
    /// Scaled value of second fixed surface */
    surface2_value: u32,
    /// Derived forecast type (see Code [Table 4.7](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-7.shtml)) */
    derived_forecast_type: Grib2Table4_7,
    /// Number of forecasts in the ensemble */
    num_forecasts_in_ensemble: u8,
}
impl Grib2Template42 {
    /// Create a new instance of Grib2ProductDefinition
    pub fn new<T: Reader>(reader: &T, sections: &Grib2Sections) -> Self {
        let discipline = sections.indicator.as_ref().map(|d| u8::from(d.discipline)).unwrap_or(0);
        let ref_time =
            sections.identification.as_ref().map(|i| i.ref_time.clone()).unwrap_or_default();
        let parameter_category = reader.uint8(Some(9));
        let parameter_number = reader.uint8(Some(10));
        let gen_process_type = reader.uint8(Some(11));
        let background_gen_process = reader.uint8(Some(12));
        let forecast_gen_process = reader.uint8(Some(13));
        let hours_after_ref_time = reader.uint16_be(Some(14));
        let min_after_ref_time = reader.uint8(Some(16));
        let unit_of_time_range_indicator = reader.uint8(Some(17));
        let forecast_time = reader.uint32_be(Some(18));
        let surface1_type = reader.uint8(Some(22));
        let surface1_scale = reader.uint8(Some(23));
        let surface1_value = reader.uint32_be(Some(24));
        let surface2_type = reader.uint8(Some(28));
        let surface2_scale = reader.uint8(Some(29));
        let surface2_value = reader.uint32_be(Some(30));
        let derived_forecast_type = reader.uint8(Some(34));
        let num_forecasts_in_ensemble = reader.uint8(Some(35));
        let category = grib2_lookup_table4_1(discipline, parameter_category);
        let values = grib2_lookup_table42(discipline, parameter_category)(parameter_number);
        let surface1 = grib2_lookup_table4_5(surface1_type);
        let surface2 = grib2_lookup_table4_5(surface2_type);
        let unit_of_time_range_indicator = unit_of_time_range_indicator.into();

        Self {
            category,
            values,
            parameter_category,
            parameter_number,
            gen_process_type: gen_process_type.into(),
            background_gen_process,
            forecast_gen_process: forecast_gen_process.into(),
            hours_after_ref_time,
            min_after_ref_time,
            unit_of_time_range_indicator,
            forecast_time: calculate_forecast_time(
                &ref_time,
                forecast_time as i64,
                &unit_of_time_range_indicator,
            ),
            surface1,
            surface1_type,
            surface1_scale,
            surface1_value,
            surface2,
            surface2_type,
            surface2_scale,
            surface2_value,
            derived_forecast_type: derived_forecast_type.into(),
            num_forecasts_in_ensemble,
        }
    }
}

/// Calculate Forecast Time
///
/// @param ref_time Reference time of GRIB Packet
/// @param offset Number of units to offset the ref time by
/// @param unit_of_time unit of time of offset
/// @returns - the forecast time
pub fn calculate_forecast_time(ref_time: &Date, offset: i64, unit_of_time: &Grib2Table4_4) -> Date {
    match unit_of_time {
        Grib2Table4_4::Hour => Date::from_time(ref_time.get_time() + offset * 1000 * 60 * 60),
        _ => {
            panic!("Unable to calculate foercast time for unit: {}", unit_of_time);
        }
    }
}
