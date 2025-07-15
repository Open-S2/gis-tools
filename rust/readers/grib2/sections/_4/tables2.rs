use crate::readers::TableCategory;
use alloc::string::String;

/// # GRIB2 - TABLE 4.2-0-20
/// PARAMETERS FOR DISCIPLINE 0, CATEGORY 20
/// **(Meteorological products, Atmospheric Chemical Constituents category)**
///
/// **Details**:
/// - **Discipline**: 0 (Meteorological products)
/// - **Category**: 20 (Atmospheric Chemical Constituents)
/// - **Section**: 4
/// - **Octet 10**: 20
/// - **Revised**: 11/02/2023
///
/// **Reserved Ranges**:
/// - `19-49`: Reserved
/// - `82-99`: Reserved
/// - `113-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2-0-20.shtml)
///
/// ## Notes
/// 1. First Fixed Surface and Second Fixed Surface of Code Table 4.5 define the vertical extent.
/// 2. The term "Number Density" is synonymous with "Number Concentration" (Code 59).
/// 3. Net source represents the sum of all atmospheric processes creating and destroying constituents or aerosols.
/// 4. Use Snow melt rate instead for certain processes (Discipline 2, Category 0, Number 41).
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 20.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 20 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_020(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Mass Density (Concentration)"),
            units: String::from("kg m-3"),
            abbrev: String::from("MASSDEN"),
        },
        1 => TableCategory {
            parameter: String::from("Column-Integrated Mass Density"),
            units: String::from("kg m-2"),
            abbrev: String::from("COLMD"),
        },
        2 => TableCategory {
            parameter: String::from("Mass Mixing Ratio (Mass Fraction in Air)"),
            units: String::from("kg kg-1"),
            abbrev: String::from("MASSMR"),
        },
        3 => TableCategory {
            parameter: String::from("Atmosphere Emission Mass Flux"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("AEMFLX"),
        },
        4 => TableCategory {
            parameter: String::from("Atmosphere Net Production Mass Flux"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("ANPMFLX"),
        },
        5 => TableCategory {
            parameter: String::from("Atmosphere Net Production And Emission Mass Flux"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("ANPEMFLX"),
        },
        6 => TableCategory {
            parameter: String::from("Surface Dry Deposition Mass Flux"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("SDDMFLX"),
        },
        7 => TableCategory {
            parameter: String::from("Surface Wet Deposition Mass Flux"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("SWDMFLX"),
        },
        8 => TableCategory {
            parameter: String::from("Atmosphere Re-Emission Mass Flux"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("AREMFLX"),
        },
        9 => TableCategory {
            parameter: String::from("Wet Deposition by Large-Scale Precipitation Mass Flux"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("WLSMFLX"),
        },
        10 => TableCategory {
            parameter: String::from("Wet Deposition by Convective Precipitation Mass Flux"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("WDCPMFLX"),
        },
        11 => TableCategory {
            parameter: String::from("Sedimentation Mass Flux"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("SEDMFLX"),
        },
        12 => TableCategory {
            parameter: String::from("Dry Deposition Mass Flux"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("DDMFLX"),
        },
        13 => TableCategory {
            parameter: String::from("Transfer From Hydrophobic to Hydrophilic"),
            units: String::from("kg kg-1s-1"),
            abbrev: String::from("TRANHH"),
        },
        14 => TableCategory {
            parameter: String::from("Transfer From SO2 to SO4"),
            units: String::from("kg kg-1s-1"),
            abbrev: String::from("TRSDS"),
        },
        15 => TableCategory {
            parameter: String::from("Dry deposition velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("DDVEL"),
        },
        16 => TableCategory {
            parameter: String::from("Mass mixing ratio with respect to dry air"),
            units: String::from("kg kg-1"),
            abbrev: String::from("MSSRDRYA"),
        },
        17 => TableCategory {
            parameter: String::from("Mass mixing ratio with respect to wet air"),
            units: String::from("kg kg-1"),
            abbrev: String::from("MSSRWETA"),
        },
        18 => TableCategory {
            parameter: String::from("Potential of hydrogen (pH)"),
            units: String::from("pH"),
            abbrev: String::from("POTHPH"),
        },
        50 => TableCategory {
            parameter: String::from("Amount in Atmosphere"),
            units: String::from("mol"),
            abbrev: String::from("AIA"),
        },
        51 => TableCategory {
            parameter: String::from("Concentration In Air"),
            units: String::from("mol m-3"),
            abbrev: String::from("CONAIR"),
        },
        52 => TableCategory {
            parameter: String::from("Volume Mixing Ratio"),
            units: String::from("mol mol-1"),
            abbrev: String::from("VMXR"),
        },
        53 => TableCategory {
            parameter: String::from("Chemical Gross Production Rate of Concentration"),
            units: String::from("mol m-3s-1"),
            abbrev: String::from("CGPRC"),
        },
        54 => TableCategory {
            parameter: String::from("Chemical Gross Destruction Rate of Concentration"),
            units: String::from("mol m-3s-1"),
            abbrev: String::from("CGDRC"),
        },
        55 => TableCategory {
            parameter: String::from("Surface Flux"),
            units: String::from("mol m-2s-1"),
            abbrev: String::from("SFLUX"),
        },
        56 => TableCategory {
            parameter: String::from("Changes Of Amount in Atmosphere"),
            units: String::from("mol s-1"),
            abbrev: String::from("COAIA"),
        },
        57 => TableCategory {
            parameter: String::from("Total Yearly Average Burden of The Atmosphere"),
            units: String::from("mol"),
            abbrev: String::from("TYABA"),
        },
        58 => TableCategory {
            parameter: String::from("Total Yearly Average Atmospheric Loss"),
            units: String::from("mol s-1"),
            abbrev: String::from("TYAAL"),
        },
        59 => TableCategory {
            parameter: String::from("Aerosol Number Concentration"),
            units: String::from("m-3"),
            abbrev: String::from("ANCON"),
        },
        60 => TableCategory {
            parameter: String::from("Aerosol Specific Number Concentration"),
            units: String::from("kg-1"),
            abbrev: String::from("ASNCON"),
        },
        61 => TableCategory {
            parameter: String::from("Maximum of Mass Density"),
            units: String::from("kg m-3"),
            abbrev: String::from("MXMASSD"),
        },
        62 => TableCategory {
            parameter: String::from("Height of Mass Density"),
            units: String::from("m"),
            abbrev: String::from("HGTMD"),
        },
        63 => TableCategory {
            parameter: String::from("Column-Averaged Mass Density in Layer"),
            units: String::from("kg m-3"),
            abbrev: String::from("CAVEMDL"),
        },
        64 => TableCategory {
            parameter: String::from("Mole fraction with respect to dry air"),
            units: String::from("mol mol-1"),
            abbrev: String::from("MOLRDRYA"),
        },
        65 => TableCategory {
            parameter: String::from("Mole fraction with respect to wet air"),
            units: String::from("mol mol-1"),
            abbrev: String::from("MOLRWETA"),
        },
        66 => TableCategory {
            parameter: String::from("Column-integrated in-cloud scavenging rate by precipitation"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CINCLDSP"),
        },
        67 => TableCategory {
            parameter: String::from(
                "Column-integrated below-cloud scavenging rate by precipitation",
            ),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CBLCLDSP"),
        },
        68 => TableCategory {
            parameter: String::from(
                "Column-integrated release rate from evaporating precipitation",
            ),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CIRELREP"),
        },
        69 => TableCategory {
            parameter: String::from(
                "Column-integrated in-cloud scavenging rate by large-scale precipitation",
            ),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CINCSLSP"),
        },
        70 => TableCategory {
            parameter: String::from(
                "Column-integrated below-cloud scavenging rate by large-scale precipitation",
            ),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CBECSLSP"),
        },
        71 => TableCategory {
            parameter: String::from(
                "Column-integrated release rate from evaporating large-scale precipitation",
            ),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CRERELSP"),
        },
        72 => TableCategory {
            parameter: String::from(
                "Column-integrated in-cloud scavenging rate by convective precipitation",
            ),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CINCSRCP"),
        },
        73 => TableCategory {
            parameter: String::from(
                "Column-integrated below-cloud scavenging rate by convective precipitation",
            ),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CBLCSRCP"),
        },
        74 => TableCategory {
            parameter: String::from(
                "Column-integrated release rate from evaporating convective precipitation",
            ),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CIRERECP"),
        },
        75 => TableCategory {
            parameter: String::from("Wildfire flux"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("WFIREFLX"),
        },
        76 => TableCategory {
            parameter: String::from("Emission Rate"),
            units: String::from("kg kg-1 s-1"),
            abbrev: String::from("EMISFLX"),
        },
        77 => TableCategory {
            parameter: String::from("Surface Emission flux"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("SFCEFLX"),
        },
        78 => TableCategory {
            parameter: String::from("Column integrated eastward mass flux"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CEMF"),
        },
        79 => TableCategory {
            parameter: String::from("Column integrated northward mass flux"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CNMF"),
        },
        80 => TableCategory {
            parameter: String::from("Column integrated divergence of mass flux"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CDIVMF"),
        },
        81 => TableCategory {
            parameter: String::from("Column integrated net source"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("CNETS"),
        },
        100 => TableCategory {
            parameter: String::from("Surface Area Density (Aerosol)"),
            units: String::from("m-1"),
            abbrev: String::from("SADEN"),
        },
        101 => TableCategory {
            parameter: String::from("Vertical Visual Range"),
            units: String::from("m"),
            abbrev: String::from("ATMTK"),
        },
        102 => TableCategory {
            parameter: String::from("Aerosol Optical Thickness"),
            units: String::from("Numeric"),
            abbrev: String::from("AOTK"),
        },
        103 => TableCategory {
            parameter: String::from("Single Scattering Albedo"),
            units: String::from("Numeric"),
            abbrev: String::from("SSALBK"),
        },
        104 => TableCategory {
            parameter: String::from("Asymmetry Factor"),
            units: String::from("Numeric"),
            abbrev: String::from("ASYSFK"),
        },
        105 => TableCategory {
            parameter: String::from("Aerosol Extinction Coefficient"),
            units: String::from("m-1"),
            abbrev: String::from("AECOEF"),
        },
        106 => TableCategory {
            parameter: String::from("Aerosol Absorption Coefficient"),
            units: String::from("m-1"),
            abbrev: String::from("AACOEF"),
        },
        107 => TableCategory {
            parameter: String::from("Aerosol Lidar Backscatter from Satellite"),
            units: String::from("m-1sr-1"),
            abbrev: String::from("ALBSAT"),
        },
        108 => TableCategory {
            parameter: String::from("Aerosol Lidar Backscatter from the Ground"),
            units: String::from("m-1sr-1"),
            abbrev: String::from("ALBGRD"),
        },
        109 => TableCategory {
            parameter: String::from("Aerosol Lidar Extinction from Satellite"),
            units: String::from("m-1"),
            abbrev: String::from("ALESAT"),
        },
        110 => TableCategory {
            parameter: String::from("Aerosol Lidar Extinction from the Ground"),
            units: String::from("m-1"),
            abbrev: String::from("ALEGRD"),
        },
        111 => TableCategory {
            parameter: String::from("Angstrom Exponent"),
            units: String::from("Numeric"),
            abbrev: String::from("ANGSTEXP"),
        },
        112 => TableCategory {
            parameter: String::from("Scattering Aerosol Optical Thickness"),
            units: String::from("Numeric"),
            abbrev: String::from("SCTAOTK"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        19..=49 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        82..=99 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        113..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-0-21
/// PARAMETERS FOR DISCIPLINE 0, CATEGORY 21
/// **(Meteorological products, Thermodynamic Properties category)**
///
/// **Details**:
/// - **Discipline**: 0 (Meteorological products)
/// - **Category**: 21 (Thermodynamic Properties)
/// - **Section**: 4
/// - **Octet 10**: 21
/// - **Revised**: 12/07/2023
///
/// **Reserved Ranges**:
/// - `23-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// 1. Total energy is the sum of internal energy, potential energy, kinetic energy, and latent heat. The same applies to energy fluxes.
/// 2. Water enthalpy (flux) is associated with the temperature of the water mass.
/// 3. Water potential energy flux is the flux of potential energy associated with the water mass.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table4-2-0-21.shtml)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 21.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 21 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_021(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Column Integrated Potential + Internal Energy"),
            units: String::from("J m-2"),
            abbrev: String::from("POTINTENG"),
        },
        1 => TableCategory {
            parameter: String::from("Column Integrated Kinetic Energy"),
            units: String::from("J m-2"),
            abbrev: String::from("KINENG"),
        },
        2 => TableCategory {
            parameter: String::from("Column Integrated Total Energy"),
            units: String::from("J m-2"),
            abbrev: String::from("TOTENG"),
        },
        3 => TableCategory {
            parameter: String::from("Column Integrated Enthalpy"),
            units: String::from("J m-2"),
            abbrev: String::from("ENTHALPY"),
        },
        4 => TableCategory {
            parameter: String::from("Column Integrated Water Enthalpy"),
            units: String::from("J m-2"),
            abbrev: String::from("WATENTHALPY"),
        },
        5 => TableCategory {
            parameter: String::from("Column Integrated Eastward Enthalpy Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("EASTENTFLUX"),
        },
        6 => TableCategory {
            parameter: String::from("Column Integrated Northward Enthalpy Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("NRTHENTFLUX"),
        },
        7 => TableCategory {
            parameter: String::from("Column Integrated Eastward Potential Energy Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("EASTPOTFLUX"),
        },
        8 => TableCategory {
            parameter: String::from("Column Integrated Northward Potential Energy Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("NRTHPOTFLUX"),
        },
        9 => TableCategory {
            parameter: String::from("Column Integrated Eastward Kinetic Energy Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("EASTKINFLUX"),
        },
        10 => TableCategory {
            parameter: String::from("Column Integrated Northward Kinetic Energy Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("NRTHKINFLUX"),
        },
        11 => TableCategory {
            parameter: String::from("Column Integrated Eastward Total Energy Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("EASTTOTFLUX"),
        },
        12 => TableCategory {
            parameter: String::from("Column Integrated Northward Total Energy Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("NRTHTOTFLUX"),
        },
        13 => TableCategory {
            parameter: String::from("Divergence of Column Integrated Enthalpy Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("DIVENTFLUX"),
        },
        14 => TableCategory {
            parameter: String::from("Divergence of Column Integrated Potential Energy Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("DIVPOTFLUX"),
        },
        15 => TableCategory {
            parameter: String::from("Divergence of Column Integrated Water Potential Energy Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("DIVWPOTFLUX"),
        },
        16 => TableCategory {
            parameter: String::from("Divergence of Column Integrated Kinetic Energy Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("DIVKENGFLUX"),
        },
        17 => TableCategory {
            parameter: String::from("Divergence of Column Integrated Total Energy Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("DIVTOTFLUX"),
        },
        18 => TableCategory {
            parameter: String::from("Divergence of Column Integrated Water Enthalpy Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("DIVWENTFLUX"),
        },
        19 => TableCategory {
            parameter: String::from("Column Integrated Eastward Heat Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("EASTHFLUX"),
        },
        20 => TableCategory {
            parameter: String::from("Column Integrated Northward Heat Flux"),
            units: String::from("W m-1"),
            abbrev: String::from("NRTHHFLUX"),
        },
        21 => TableCategory {
            parameter: String::from("Column Integrated Potential + Internal + Latent Energy"),
            units: String::from("J m-2"),
            abbrev: String::from("PILENERGY"),
        },
        22 => TableCategory {
            parameter: String::from("Eady Growth Rate"),
            units: String::from("day-1"),
            abbrev: String::from("EADYGR"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        23..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-0-22
/// PARAMETERS FOR DISCIPLINE 0, CATEGORY 22
/// **(Meteorological products, Drought Indices category)**
///
/// **Details**:
/// - **Discipline**: 0 (Meteorological products)
/// - **Category**: 22 (Drought Indices)
/// - **Section**: 4
/// - **Octet 10**: 22
/// - **Created**: 07/15/2024
///
/// **Reserved Ranges**:
/// - `7-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// 1. Descriptions of the drought indices are available in the Handbook of Drought Indicators and Indices (WMO-No. 1173).
/// 2. Indices are calculated over different time ranges specified in templates 4.107, 4.108, 4.109, and 4.112.
/// 3. All standardized drought indices follow the Standardized Precipitation Index User Guide (WMO-No. 1090).
///
/// ## Links
/// - [Handbook of Drought Indicators and Indices](https://library.wmo.int/idurl/4/55169)
/// - [Standardized Precipitation Index User Guide](https://library.wmo.int/idurl/4/39629)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 22.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 22 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_022(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Standard Precipitation Index (SPI)"),
            units: String::from("dimensionless"),
            abbrev: String::from("SPI"),
        },
        1 => TableCategory {
            parameter: String::from("Standardized Precipitation Evapotranspiration Index (SPEI)"),
            units: String::from("dimensionless"),
            abbrev: String::from("SPEI"),
        },
        2 => TableCategory {
            parameter: String::from("Standardized Streamflow Index (SSFI)"),
            units: String::from("dimensionless"),
            abbrev: String::from("SSFI"),
        },
        3 => TableCategory {
            parameter: String::from("Standardized Reservoir Supply Index (SRSI)"),
            units: String::from("dimensionless"),
            abbrev: String::from("SRSI"),
        },
        4 => TableCategory {
            parameter: String::from("Standardized Water-level Index (SWI)"),
            units: String::from("dimensionless"),
            abbrev: String::from("SWI"),
        },
        5 => TableCategory {
            parameter: String::from("Standardized Snowmelt and Rain Index (SMRI)"),
            units: String::from("dimensionless"),
            abbrev: String::from("SMRI"),
        },
        6 => TableCategory {
            parameter: String::from("Streamflow Drought Index (SDI)"),
            units: String::from("dimensionless"),
            abbrev: String::from("SDI"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        7..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-0-190
/// PARAMETERS FOR DISCIPLINE 0, CATEGORY 190
/// **(Meteorological products, ASCII IA5 String category)**
///
/// **Details**:
/// - **Discipline**: 0 (Meteorological products)
/// - **Category**: 190 (ASCII IA5 String)
/// - **Section**: 4
/// - **Octet 10**: 190
/// - **Revised**: 12/14/2011
///
/// **Reserved Ranges**:
/// - `1-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// 1. This table defines ASCII IA5 strings used for arbitrary text.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/)
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 190.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 190 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_0190(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Arbitrary Text String"),
            units: String::from("CCITTIA5"),
            abbrev: String::from("ATEXT"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
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

/// # GRIB2 - TABLE 4.2-0-191
/// PARAMETERS FOR DISCIPLINE 0, CATEGORY 191
/// **(Meteorological products, Miscellaneous category)**
///
/// **Details**:
/// - **Discipline**: 0 (Meteorological products)
/// - **Category**: 191 (Miscellaneous)
/// - **Section**: 4
/// - **Octet 10**: 191
/// - **Revised**: 07/15/2024
///
/// **Reserved Ranges**:
/// - `8-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// 1. Hurricane, tropical storm, and tropical depression tracks use spatiotemporal vicinity logic.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 191.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 191 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_0191(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from(
                "Seconds prior to initial reference time (defined in Section 1)",
            ),
            units: String::from("s"),
            abbrev: String::from("TSEC"),
        },
        1 => TableCategory {
            parameter: String::from("Geographical Latitude"),
            units: String::from("° N"),
            abbrev: String::from("GEOLAT"),
        },
        2 => TableCategory {
            parameter: String::from("Geographical Longitude"),
            units: String::from("° E"),
            abbrev: String::from("GEOLON"),
        },
        3 => TableCategory {
            parameter: String::from("Days Since Last Observation"),
            units: String::from("d"),
            abbrev: String::from("DSLOBS"),
        },
        4 => TableCategory {
            parameter: String::from("Tropical cyclone density track"),
            units: String::from("Numeric"),
            abbrev: String::from("TCDTRACK"),
        },
        5 => TableCategory {
            parameter: String::from("Hurricane track in spatiotemporal vicinity"),
            units: String::from("boolean"),
            abbrev: String::from("HURTSV"),
        },
        6 => TableCategory {
            parameter: String::from("Tropical storm track in spatiotemporal vicinity"),
            units: String::from("boolean"),
            abbrev: String::from("TSTSV"),
        },
        7 => TableCategory {
            parameter: String::from("Tropical depression track in spatiotemporal vicinity"),
            units: String::from("boolean"),
            abbrev: String::from("TDTSV"),
        },
        192 => TableCategory {
            parameter: String::from("Latitude (-90 to 90)"),
            units: String::from("°"),
            abbrev: String::from("NLAT"),
        },
        193 => TableCategory {
            parameter: String::from("East Longitude (0 to 360)"),
            units: String::from("°"),
            abbrev: String::from("ELON"),
        },
        194 => TableCategory {
            parameter: String::from("Seconds prior to initial reference time"),
            units: String::from("s"),
            abbrev: String::from("RTSEC"),
        },
        195 => TableCategory {
            parameter: String::from("Model Layer number (From bottom up)"),
            units: String::from(""),
            abbrev: String::from("MLYNO"),
        },
        196 => TableCategory {
            parameter: String::from("Latitude (nearest neighbor) (-90 to 90)"),
            units: String::from("°"),
            abbrev: String::from("NLATN"),
        },
        197 => TableCategory {
            parameter: String::from("East Longitude (nearest neighbor) (0 to 360)"),
            units: String::from("°"),
            abbrev: String::from("ELONN"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        8..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-0-192
/// PARAMETERS FOR DISCIPLINE 0, CATEGORY 192
/// **(Meteorological products, Covariance category)**
///
/// **Details**:
/// - **Discipline**: 0 (Meteorological products)
/// - **Category**: 192 (Covariance)
/// - **Section**: 4
/// - **Octet 10**: 192
/// - **Created**: 03/12/2008
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// Covariances are defined as `[XY]-[X][Y]`, where `[]` indicates the mean over the specified time span.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 0, Category 192.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 0, Category 192 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_0192(category: u8) -> TableCategory {
    match category {
        1 => TableCategory {
            parameter: String::from(
                "Covariance between zonal and meridional components of the wind",
            ),
            units: String::from("m2/s2"),
            abbrev: String::from("COVMZ"),
        },
        2 => TableCategory {
            parameter: String::from(
                "Covariance between zonal component of the wind and temperature",
            ),
            units: String::from("K*m/s"),
            abbrev: String::from("COVTZ"),
        },
        3 => TableCategory {
            parameter: String::from(
                "Covariance between meridional component of the wind and temperature",
            ),
            units: String::from("K*m/s"),
            abbrev: String::from("COVTM"),
        },
        4 => TableCategory {
            parameter: String::from(
                "Covariance between temperature and vertical component of the wind",
            ),
            units: String::from("K*m/s"),
            abbrev: String::from("COVTW"),
        },
        5 => TableCategory {
            parameter: String::from("Covariance between zonal and zonal components of the wind"),
            units: String::from("m2/s2"),
            abbrev: String::from("COVZZ"),
        },
        6 => TableCategory {
            parameter: String::from(
                "Covariance between meridional and meridional components of the wind",
            ),
            units: String::from("m2/s2"),
            abbrev: String::from("COVMM"),
        },
        7 => TableCategory {
            parameter: String::from(
                "Covariance between specific humidity and zonal components of the wind",
            ),
            units: String::from("kg/kg*m/s"),
            abbrev: String::from("COVQZ"),
        },
        8 => TableCategory {
            parameter: String::from(
                "Covariance between specific humidity and meridional components of the wind",
            ),
            units: String::from("kg/kg*m/s"),
            abbrev: String::from("COVQM"),
        },
        9 => TableCategory {
            parameter: String::from(
                "Covariance between temperature and vertical components of the wind",
            ),
            units: String::from("K*Pa/s"),
            abbrev: String::from("COVTVV"),
        },
        10 => TableCategory {
            parameter: String::from(
                "Covariance between specific humidity and vertical components of the wind",
            ),
            units: String::from("kg/kg*Pa/s"),
            abbrev: String::from("COVQVV"),
        },
        11 => TableCategory {
            parameter: String::from("Covariance between surface pressure and surface pressure"),
            units: String::from("Pa*Pa"),
            abbrev: String::from("COVPSPS"),
        },
        12 => TableCategory {
            parameter: String::from("Covariance between specific humidity and specific humidity"),
            units: String::from("kg/kg*kg/kg"),
            abbrev: String::from("COVQQ"),
        },
        13 => TableCategory {
            parameter: String::from(
                "Covariance between vertical and vertical components of the wind",
            ),
            units: String::from("Pa2/s2"),
            abbrev: String::from("COVVVVV"),
        },
        14 => TableCategory {
            parameter: String::from("Covariance between temperature and temperature"),
            units: String::from("K*K"),
            abbrev: String::from("COVTT"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        0 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        }, // Category 0 is reserved according to the source
        15..=254 => TableCategory {
            parameter: String::from("Reserved for Local Use"),
            units: String::from(""),
            abbrev: String::from("Reserved for Local Use"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-1-0
/// PARAMETERS FOR DISCIPLINE 1, CATEGORY 0
/// **(Hydrological products, Hydrology Basic category)**
///
/// **Details**:
/// - **Discipline**: 1 (Hydrological products)
/// - **Category**: 0 (Hydrology Basic)
/// - **Section**: 4
/// - **Octet 10**: 0
/// - **Revised**: 07/15/2024
///
/// **Reserved Ranges**:
/// - `21-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// 1. Remotely sensed snow cover uses dimensionless thematic values (e.g., 50: no-snow/no-cloud, 100: clouds, 250: snow). See Table 4.215.
/// 2. Snow coverage by elevation portrays elevations with snow packs; see Table 4.216.
/// 3. Snow water equivalent percent of normal is stored in percent units (e.g., 110 = 110% of normal snow water equivalent).
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 1, Category 0.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 1, Category 0 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_10(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from(
                "Flash Flood Guidance (Encoded as an accumulation over a floating subinterval of \
                 time between the reference time and valid time)",
            ),
            units: String::from("kg m-2"),
            abbrev: String::from("FFLDG"),
        },
        1 => TableCategory {
            parameter: String::from(
                "Flash Flood Runoff (Encoded as an accumulation over a floating subinterval of \
                 time)",
            ),
            units: String::from("kg m-2"),
            abbrev: String::from("FFLDRO"),
        },
        2 => TableCategory {
            parameter: String::from("Remotely Sensed Snow Cover"),
            units: String::from("See Table 4.215"),
            abbrev: String::from("RSSC"),
        },
        3 => TableCategory {
            parameter: String::from("Elevation of Snow Covered Terrain"),
            units: String::from("See Table 4.216"),
            abbrev: String::from("ESCT"),
        },
        4 => TableCategory {
            parameter: String::from("Snow Water Equivalent Percent of Normal"),
            units: String::from("%"),
            abbrev: String::from("SWEPON"),
        },
        5 => TableCategory {
            parameter: String::from("Baseflow-Groundwater Runoff"),
            units: String::from("kg m-2"),
            abbrev: String::from("BGRUN"),
        },
        6 => TableCategory {
            parameter: String::from("Storm Surface Runoff"),
            units: String::from("kg m-2"),
            abbrev: String::from("SSRUN"),
        },
        7 => TableCategory {
            parameter: String::from("Discharge from Rivers or Streams"),
            units: String::from("m3 s-1"),
            abbrev: String::from("DISRS"),
        },
        8 => TableCategory {
            parameter: String::from("Group Water Upper Storage"),
            units: String::from("kg m-2"),
            abbrev: String::from("GWUPS"),
        },
        9 => TableCategory {
            parameter: String::from("Group Water Lower Storage"),
            units: String::from("kg m-2"),
            abbrev: String::from("GWLOWS"),
        },
        10 => TableCategory {
            parameter: String::from("Side Flow into River Channel"),
            units: String::from("m3 s-1 m-1"),
            abbrev: String::from("SFLORC"),
        },
        11 => TableCategory {
            parameter: String::from("River Storage of Water"),
            units: String::from("m3"),
            abbrev: String::from("RVERSW"),
        },
        12 => TableCategory {
            parameter: String::from("Flood Plain Storage of Water"),
            units: String::from("m3"),
            abbrev: String::from("FLDPSW"),
        },
        13 => TableCategory {
            parameter: String::from("Depth of Water on Soil Surface"),
            units: String::from("kg m-2"),
            abbrev: String::from("DEPWSS"),
        },
        14 => TableCategory {
            parameter: String::from("Upstream Accumulated Precipitation"),
            units: String::from("kg m-2"),
            abbrev: String::from("UPAPCP"),
        },
        15 => TableCategory {
            parameter: String::from("Upstream Accumulated Snow Melt"),
            units: String::from("kg m-2"),
            abbrev: String::from("UPASM"),
        },
        16 => TableCategory {
            parameter: String::from("Percolation Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("PERRATE"),
        },
        17 => TableCategory {
            parameter: String::from("River Outflow of Water"),
            units: String::from("m3 s-1"),
            abbrev: String::from("RVEROW"),
        },
        18 => TableCategory {
            parameter: String::from("Floodplain Outflow of Water"),
            units: String::from("m3 s-1"),
            abbrev: String::from("FLDPOW"),
        },
        19 => TableCategory {
            parameter: String::from("Floodpath Outflow of Water"),
            units: String::from("m3 s-1"),
            abbrev: String::from("FLDPATHOW"),
        },
        20 => TableCategory {
            parameter: String::from("Water on Surface"),
            units: String::from("kg m-2"),
            abbrev: String::from("WATSURF"),
        },
        192 => TableCategory {
            parameter: String::from("Baseflow-Groundwater Runoff"),
            units: String::from("kg m-2"),
            abbrev: String::from("BGRUN"),
        },
        193 => TableCategory {
            parameter: String::from("Storm Surface Runoff"),
            units: String::from("kg m-2"),
            abbrev: String::from("SSRUN"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
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

/// # GRIB2 - TABLE 4.2-1-1
/// PARAMETERS FOR DISCIPLINE 1, CATEGORY 1
/// **(Hydrological products, Hydrology Probabilities category)**
///
/// **Details**:
/// - **Discipline**: 1 (Hydrological products)
/// - **Category**: 1 (Hydrology Probabilities)
/// - **Section**: 4
/// - **Octet 10**: 1
/// - **Revised**: 11/02/2023
///
/// **Reserved Ranges**:
/// - `3-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 1, Category 1.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 1, Category 1 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_11(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from(
                "Conditional percent precipitation amount fractile for an overall period (encoded \
                 as an accumulation)",
            ),
            units: String::from("kg m-2"),
            abbrev: String::from("CPPOP"),
        },
        1 => TableCategory {
            parameter: String::from(
                "Percent Precipitation in a sub-period of an overall period (encoded as a percent \
                 accumulation over the sub-period)",
            ),
            units: String::from("%"),
            abbrev: String::from("PPOSP"),
        },
        2 => TableCategory {
            parameter: String::from("Probability of 0.01 inch of precipitation (POP)"),
            units: String::from("%"),
            abbrev: String::from("POP"),
        },
        192 => TableCategory {
            parameter: String::from("Probability of Freezing Precipitation"),
            units: String::from("%"),
            abbrev: String::from("CPOZP"),
        },
        193 => TableCategory {
            parameter: String::from("Percent of Frozen Precipitation"),
            units: String::from("%"),
            abbrev: String::from("CPOFP"),
        },
        194 => TableCategory {
            parameter: String::from(
                "Probability of precipitation exceeding flash flood guidance values",
            ),
            units: String::from("%"),
            abbrev: String::from("PPFFG"),
        },
        195 => TableCategory {
            parameter: String::from(
                "Probability of Wetting Rain, exceeding in 0.10\" in a given time period",
            ),
            units: String::from("%"),
            abbrev: String::from("CWR"),
        },
        196 => TableCategory {
            parameter: String::from(
                "Binary Probability of precipitation exceeding average recurrence intervals (ARI)",
            ),
            units: String::from("see Code table 4.222"),
            abbrev: String::from("QPFARI"),
        },
        197 => TableCategory {
            parameter: String::from(
                "Binary Probability of precipitation exceeding flash flood guidance values",
            ),
            units: String::from("see Code table 4.222"),
            abbrev: String::from("QPFFFG"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
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

/// # GRIB2 - TABLE 4.2-1-2
/// PARAMETERS FOR DISCIPLINE 1, CATEGORY 2
/// **(Hydrological products, Inland water and sediment properties category)**
///
/// **Details**:
/// - **Discipline**: 1 (Hydrological products)
/// - **Category**: 2 (Inland water and sediment properties)
/// - **Section**: 4
/// - **Octet 10**: 2
/// - **Revised**: 07/15/2024
///
/// **Reserved Ranges**:
/// - `24-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// 1. The same parameter name may exist in multiple categories based on its intended use. For example, "Water Temperature" in this table applies to freshwater lakes and rivers, unlike its counterpart in oceanographic products.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 1, Category 2.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 1, Category 2 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_12(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Water Depth"),
            units: String::from("m"),
            abbrev: String::from("WDPTHIL"),
        },
        1 => TableCategory {
            parameter: String::from("Water Temperature"),
            units: String::from("K"),
            abbrev: String::from("WTMPIL"),
        },
        2 => TableCategory {
            parameter: String::from("Water Fraction"),
            units: String::from("Proportion"),
            abbrev: String::from("WFRACT"),
        },
        3 => TableCategory {
            parameter: String::from("Sediment Thickness"),
            units: String::from("m"),
            abbrev: String::from("SEDTK"),
        },
        4 => TableCategory {
            parameter: String::from("Sediment Temperature"),
            units: String::from("K"),
            abbrev: String::from("SEDTMP"),
        },
        5 => TableCategory {
            parameter: String::from("Ice Thickness"),
            units: String::from("m"),
            abbrev: String::from("ICTKIL"),
        },
        6 => TableCategory {
            parameter: String::from("Ice Temperature"),
            units: String::from("K"),
            abbrev: String::from("ICETIL"),
        },
        7 => TableCategory {
            parameter: String::from("Ice Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("ICECIL"),
        },
        8 => TableCategory {
            parameter: String::from("Land Cover (0=water, 1=land)"),
            units: String::from("Proportion"),
            abbrev: String::from("LANDIL"),
        },
        9 => TableCategory {
            parameter: String::from("Shape Factor with Respect to Salinity Profile"),
            units: String::from(""),
            abbrev: String::from("SFSAL"),
        },
        10 => TableCategory {
            parameter: String::from(
                "Shape Factor with Respect to Temperature Profile in Thermocline",
            ),
            units: String::from(""),
            abbrev: String::from("SFTMP"),
        },
        11 => TableCategory {
            parameter: String::from(
                "Attenuation Coefficient of Water with Respect to Solar Radiation",
            ),
            units: String::from("m-1"),
            abbrev: String::from("ACWSR"),
        },
        12 => TableCategory {
            parameter: String::from("Salinity"),
            units: String::from("kg kg-1"),
            abbrev: String::from("SALTIL"),
        },
        13 => TableCategory {
            parameter: String::from("Cross Sectional Area of Flow in Channel"),
            units: String::from("m2"),
            abbrev: String::from("CSAFC"),
        },
        14 => TableCategory {
            parameter: String::from("Snow Temperature"),
            units: String::from("K"),
            abbrev: String::from("LNDSNOWT"),
        },
        15 => TableCategory {
            parameter: String::from("Lake Depth"),
            units: String::from("m"),
            abbrev: String::from("LDEPTH"),
        },
        16 => TableCategory {
            parameter: String::from("River Depth"),
            units: String::from("m"),
            abbrev: String::from("RDEPTH"),
        },
        17 => TableCategory {
            parameter: String::from("Floodplain Depth"),
            units: String::from("m"),
            abbrev: String::from("FLDPDEPTH"),
        },
        18 => TableCategory {
            parameter: String::from("Floodplain Flooded Fraction"),
            units: String::from("Proportion"),
            abbrev: String::from("FLDPFLFR"),
        },
        19 => TableCategory {
            parameter: String::from("Floodplain Flooded Area"),
            units: String::from("m2"),
            abbrev: String::from("FLDPFLAR"),
        },
        20 => TableCategory {
            parameter: String::from("River Fraction"),
            units: String::from("Proportion"),
            abbrev: String::from("RVERFR"),
        },
        21 => TableCategory {
            parameter: String::from("River Area"),
            units: String::from("m2"),
            abbrev: String::from("RVERAR"),
        },
        22 => TableCategory {
            parameter: String::from("Fraction of River Coverage Plus River Related Flooding"),
            units: String::from("Proportion"),
            abbrev: String::from("FRCRF"),
        },
        23 => TableCategory {
            parameter: String::from("Area of River Coverage Plus River Related Flooding"),
            units: String::from("m2"),
            abbrev: String::from("ARCRF"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        24..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-2-0
/// PARAMETERS FOR DISCIPLINE 2, CATEGORY 0
/// **(Land Surface products, Vegetation/Biomass category)**
///
/// **Details**:
/// - **Discipline**: 2 (Land Surface products)
/// - **Category**: 0 (Vegetation/Biomass)
/// - **Section**: 4
/// - **Octet 10**: 0
/// - **Revised**: 12/07/2023
///
/// **Reserved Ranges**:
/// - `64-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// 1. Deprecated parameters are marked and alternative parameters are recommended (e.g., categories for Soil Products).
/// 2. Statistical process 1 (Accumulation) does not alter units.
/// 3. C4 plants use a specific photosynthesis mechanism to avoid photorespiration.
/// 4. Net ecosystem fluxes can specify chemical species (e.g., CO₂ or CH₄) with chemical constituent templates.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 2, Category 0.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 2, Category 0 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_20(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Land Cover (0=sea, 1=land)"),
            units: String::from("Proportion"),
            abbrev: String::from("LAND"),
        },
        1 => TableCategory {
            parameter: String::from("Surface Roughness"),
            units: String::from("m"),
            abbrev: String::from("SFCR"),
        },
        2 => TableCategory {
            parameter: String::from("Soil Temperature (Parameter Deprecated)"),
            units: String::from("K"),
            abbrev: String::from("TSOIL"),
        },
        3 => TableCategory {
            parameter: String::from("Soil Moisture Content (Parameter Deprecated)"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        4 => TableCategory {
            parameter: String::from("Vegetation"),
            units: String::from("%"),
            abbrev: String::from("VEG"),
        },
        5 => TableCategory {
            parameter: String::from("Water Runoff"),
            units: String::from("kg m-2"),
            abbrev: String::from("WATR"),
        },
        6 => TableCategory {
            parameter: String::from("Evapotranspiration"),
            units: String::from("kg-2 s-1"),
            abbrev: String::from("EVAPT"),
        },
        7 => TableCategory {
            parameter: String::from("Model Terrain Height"),
            units: String::from("m"),
            abbrev: String::from("MTERH"),
        },
        8 => TableCategory {
            parameter: String::from("Land Use"),
            units: String::from("See Table 4.212"),
            abbrev: String::from("LANDU"),
        },
        9 => TableCategory {
            parameter: String::from("Volumetric Soil Moisture Content"),
            units: String::from("Proportion"),
            abbrev: String::from("SOILW"),
        },
        10 => TableCategory {
            parameter: String::from("Ground Heat Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("GFLUX"),
        },
        11 => TableCategory {
            parameter: String::from("Moisture Availability"),
            units: String::from("%"),
            abbrev: String::from("MSTAV"),
        },
        12 => TableCategory {
            parameter: String::from("Exchange Coefficient"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("SFEXC"),
        },
        13 => TableCategory {
            parameter: String::from("Plant Canopy Surface Water"),
            units: String::from("kg m-2"),
            abbrev: String::from("CNWAT"),
        },
        14 => TableCategory {
            parameter: String::from("Blackadar's Mixing Length Scale"),
            units: String::from("m"),
            abbrev: String::from("BMIXL"),
        },
        15 => TableCategory {
            parameter: String::from("Canopy Conductance"),
            units: String::from("m s-1"),
            abbrev: String::from("CCOND"),
        },
        16 => TableCategory {
            parameter: String::from("Minimal Stomatal Resistance"),
            units: String::from("s m-1"),
            abbrev: String::from("RSMIN"),
        },
        17 => TableCategory {
            parameter: String::from("Wilting Point (Parameter Deprecated)"),
            units: String::from("Proportion"),
            abbrev: String::from("WILT"),
        },
        18 => TableCategory {
            parameter: String::from("Solar parameter in canopy conductance"),
            units: String::from("Proportion"),
            abbrev: String::from("RCS"),
        },
        19 => TableCategory {
            parameter: String::from("Temperature parameter in canopy"),
            units: String::from("Proportion"),
            abbrev: String::from("RCT"),
        },
        20 => TableCategory {
            parameter: String::from("Humidity parameter in canopy conductance"),
            units: String::from("Proportion"),
            abbrev: String::from("RCQ"),
        },
        21 => TableCategory {
            parameter: String::from("Soil moisture parameter in canopy conductance"),
            units: String::from("Proportion"),
            abbrev: String::from("RCSOL"),
        },
        22 => TableCategory {
            parameter: String::from("Soil Moisture (Parameter Deprecated)"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        23 => TableCategory {
            parameter: String::from("Column-Integrated Soil Water (Parameter Deprecated)"),
            units: String::from("kg m-2"),
            abbrev: String::from("CISOILW"),
        },
        24 => TableCategory {
            parameter: String::from("Heat Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("HFLUX"),
        },
        25 => TableCategory {
            parameter: String::from("Volumetric Soil Moisture"),
            units: String::from("m3 m-3"),
            abbrev: String::from("VSOILM"),
        },
        26 => TableCategory {
            parameter: String::from("Wilting Point"),
            units: String::from("kg m-3"),
            abbrev: String::from("WILT"),
        },
        27 => TableCategory {
            parameter: String::from("Volumetric Wilting Point"),
            units: String::from("m3 m-3"),
            abbrev: String::from("VWILTP"),
        },
        28 => TableCategory {
            parameter: String::from("Leaf Area Index"),
            units: String::from("Numeric"),
            abbrev: String::from("LEAINX"),
        },
        29 => TableCategory {
            parameter: String::from("Evergreen Forest Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("EVGFC"),
        },
        30 => TableCategory {
            parameter: String::from("Deciduous Forest Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("DECFC"),
        },
        31 => TableCategory {
            parameter: String::from("Normalized Differential Vegetation Index (NDVI)"),
            units: String::from("Numeric"),
            abbrev: String::from("NDVINX"),
        },
        32 => TableCategory {
            parameter: String::from("Root Depth of Vegetation"),
            units: String::from("m"),
            abbrev: String::from("RDVEG"),
        },
        33 => TableCategory {
            parameter: String::from("Water Runoff and Drainage"),
            units: String::from("kg m-2"),
            abbrev: String::from("WROD"),
        },
        34 => TableCategory {
            parameter: String::from("Surface Water Runoff"),
            units: String::from("kg m-2"),
            abbrev: String::from("SFCWRO"),
        },
        35 => TableCategory {
            parameter: String::from("Tile Class"),
            units: String::from("See Table 4.243"),
            abbrev: String::from("TCLASS"),
        },
        36 => TableCategory {
            parameter: String::from("Tile Fraction"),
            units: String::from("Proportion"),
            abbrev: String::from("TFRCT"),
        },
        37 => TableCategory {
            parameter: String::from("Tile Percentage"),
            units: String::from("%"),
            abbrev: String::from("TPERCT"),
        },
        38 => TableCategory {
            parameter: String::from("Soil Volumetric Ice Content (Water Equivalent)"),
            units: String::from("m3 m-3"),
            abbrev: String::from("SOILVIC"),
        },
        39 => TableCategory {
            parameter: String::from("Evapotranspiration Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("EVAPTRAT"),
        },
        40 => TableCategory {
            parameter: String::from("Potential Evapotranspiration Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("PEVAPTRAT"),
        },
        41 => TableCategory {
            parameter: String::from("Snow Melt Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("SMRATE"),
        },
        42 => TableCategory {
            parameter: String::from("Water Runoff and Drainage Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("WRDRATE"),
        },
        43 => TableCategory {
            parameter: String::from("Drainage direction"),
            units: String::from("See Table 4.250"),
            abbrev: String::from("DRAINDIR"),
        },
        44 => TableCategory {
            parameter: String::from("Upstream Area"),
            units: String::from("m2"),
            abbrev: String::from("UPSAREA"),
        },
        45 => TableCategory {
            parameter: String::from("Wetland Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("WETCOV"),
        },
        46 => TableCategory {
            parameter: String::from("Wetland Type"),
            units: String::from("See Table 4.239"),
            abbrev: String::from("WETTYPE"),
        },
        47 => TableCategory {
            parameter: String::from("Irrigation Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("IRRCOV"),
        },
        48 => TableCategory {
            parameter: String::from("C4 Crop Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("CROPCOV"),
        },
        49 => TableCategory {
            parameter: String::from("C4 Grass Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("GRASSCOV"),
        },
        50 => TableCategory {
            parameter: String::from("Skin Reservoir Content"),
            units: String::from("kg m-2"),
            abbrev: String::from("SKINRC"),
        },
        51 => TableCategory {
            parameter: String::from("Surface Runoff Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("SURFRATE"),
        },
        52 => TableCategory {
            parameter: String::from("Subsurface Runoff Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("SUBSRATE"),
        },
        53 => TableCategory {
            parameter: String::from("Low-Vegetation Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("LOVEGCOV"),
        },
        54 => TableCategory {
            parameter: String::from("High-Vegetation Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("HIVEGCOV"),
        },
        55 => TableCategory {
            parameter: String::from("Leaf Area Index (Low-Vegetation)"),
            units: String::from("m2 m-2"),
            abbrev: String::from("LAILO"),
        },
        56 => TableCategory {
            parameter: String::from("Leaf Area Index (High-Vegetation)"),
            units: String::from("m2 m-2"),
            abbrev: String::from("LAIHI"),
        },
        57 => TableCategory {
            parameter: String::from("Type of Low-Vegetation"),
            units: String::from("See Table 4.234"),
            abbrev: String::from("TYPLOVEG"),
        },
        58 => TableCategory {
            parameter: String::from("Type of High-Vegetation"),
            units: String::from("See Table 4.234"),
            abbrev: String::from("TYPHIVEG"),
        },
        59 => TableCategory {
            parameter: String::from("Net Ecosystem Exchange Flux"),
            units: String::from("kg-2 s-1"),
            abbrev: String::from("NECOFLUX"),
        },
        60 => TableCategory {
            parameter: String::from("Gross Primary Production Flux"),
            units: String::from("kg-2 s-1"),
            abbrev: String::from("GROSSFLUX"),
        },
        61 => TableCategory {
            parameter: String::from("Ecosystem Respiration Flux"),
            units: String::from("kg-2 s-1"),
            abbrev: String::from("ECORFLUX"),
        },
        62 => TableCategory {
            parameter: String::from("Emissivity"),
            units: String::from("Proportion"),
            abbrev: String::from("EMISS"),
        },
        63 => TableCategory {
            parameter: String::from("Canopy Air Temperature"),
            units: String::from("K"),
            abbrev: String::from("CANTMP"),
        },
        192 => TableCategory {
            parameter: String::from("Volumetric Soil Moisture Content"),
            units: String::from("Fraction"),
            abbrev: String::from("SOILW"),
        },
        193 => TableCategory {
            parameter: String::from("Ground Heat Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("GFLUX"),
        },
        194 => TableCategory {
            parameter: String::from("Moisture Availability"),
            units: String::from("%"),
            abbrev: String::from("MSTAV"),
        },
        195 => TableCategory {
            parameter: String::from("Exchange Coefficient"),
            units: String::from("(kg m-3) (m s-1)"),
            abbrev: String::from("SFEXC"),
        },
        196 => TableCategory {
            parameter: String::from("Plant Canopy Surface Water"),
            units: String::from("kg m-2"),
            abbrev: String::from("CNWAT"),
        },
        197 => TableCategory {
            parameter: String::from("Blackadar's Mixing Length Scale"),
            units: String::from("m"),
            abbrev: String::from("BMIXL"),
        },
        198 => TableCategory {
            parameter: String::from("Vegetation Type"),
            units: String::from("Integer (0-13)"),
            abbrev: String::from("VGTYP"),
        },
        199 => TableCategory {
            parameter: String::from("Canopy Conductance"),
            units: String::from("m s-1"),
            abbrev: String::from("CCOND"),
        },
        200 => TableCategory {
            parameter: String::from("Minimal Stomatal Resistance"),
            units: String::from("s m-1"),
            abbrev: String::from("RSMIN"),
        },
        201 => TableCategory {
            parameter: String::from("Wilting Point"),
            units: String::from("Fraction"),
            abbrev: String::from("WILT"),
        },
        202 => TableCategory {
            parameter: String::from("Solar parameter in canopy conductance"),
            units: String::from("Fraction"),
            abbrev: String::from("RCS"),
        },
        203 => TableCategory {
            parameter: String::from("Temperature parameter in canopy conductance"),
            units: String::from("Fraction"),
            abbrev: String::from("RCT"),
        },
        204 => TableCategory {
            parameter: String::from("Humidity parameter in canopy conductance"),
            units: String::from("Fraction"),
            abbrev: String::from("RCQ"),
        },
        205 => TableCategory {
            parameter: String::from("Soil moisture parameter in canopy conductance"),
            units: String::from("Fraction"),
            abbrev: String::from("RCSOL"),
        },
        206 => TableCategory {
            parameter: String::from("Rate of water dropping from canopy to ground"),
            units: String::from(""),
            abbrev: String::from("RDRIP"),
        },
        207 => TableCategory {
            parameter: String::from("Ice-free water surface"),
            units: String::from("%"),
            abbrev: String::from("ICWAT"),
        },
        208 => TableCategory {
            parameter: String::from("Surface exchange coefficients for T and Q divided by delta z"),
            units: String::from("m s-1"),
            abbrev: String::from("AKHS"),
        },
        209 => TableCategory {
            parameter: String::from("Surface exchange coefficients for U and V divided by delta z"),
            units: String::from("m s-1"),
            abbrev: String::from("AKMS"),
        },
        210 => TableCategory {
            parameter: String::from("Vegetation Canopy Temperature"),
            units: String::from("K"),
            abbrev: String::from("VEGT"),
        },
        211 => TableCategory {
            parameter: String::from("Surface Water Storage"),
            units: String::from("kg m-2"),
            abbrev: String::from("SSTOR"),
        },
        212 => TableCategory {
            parameter: String::from("Liquid Soil Moisture Content (non-frozen)"),
            units: String::from("kg m-2"),
            abbrev: String::from("LSOIL"),
        },
        213 => TableCategory {
            parameter: String::from("Open Water Evaporation (standing water)"),
            units: String::from("W m-2"),
            abbrev: String::from("EWATR"),
        },
        214 => TableCategory {
            parameter: String::from("Groundwater Recharge"),
            units: String::from("kg m-2"),
            abbrev: String::from("GWREC"),
        },
        215 => TableCategory {
            parameter: String::from("Flood Plain Recharge"),
            units: String::from("kg m-2"),
            abbrev: String::from("QREC"),
        },
        216 => TableCategory {
            parameter: String::from("Roughness Length for Heat"),
            units: String::from("m"),
            abbrev: String::from("SFCRH"),
        },
        217 => TableCategory {
            parameter: String::from("Normalized Difference Vegetation Index"),
            units: String::from(""),
            abbrev: String::from("NDVI"),
        },
        218 => TableCategory {
            parameter: String::from("Land-Sea Coverage (nearest neighbor) [land=1, sea=0]"),
            units: String::from(""),
            abbrev: String::from("LANDN"),
        },
        219 => TableCategory {
            parameter: String::from("Asymptotic Mixing Length Scale"),
            units: String::from("m"),
            abbrev: String::from("AMIXL"),
        },
        220 => TableCategory {
            parameter: String::from("Water Vapor Added by Precip Assimilation"),
            units: String::from("kg m-2"),
            abbrev: String::from("WVINC"),
        },
        221 => TableCategory {
            parameter: String::from("Water Condensate Added by Precip Assimilation"),
            units: String::from("kg m-2"),
            abbrev: String::from("WCINC"),
        },
        222 => TableCategory {
            parameter: String::from("Water Vapor Flux Convergence (Vertical Int)"),
            units: String::from("kg m-2"),
            abbrev: String::from("WVCONV"),
        },
        223 => TableCategory {
            parameter: String::from("Water Condensate Flux Convergence (Vertical Int)"),
            units: String::from("kg m-2"),
            abbrev: String::from("WCCONV"),
        },
        224 => TableCategory {
            parameter: String::from("Water Vapor Zonal Flux (Vertical Int)"),
            units: String::from("kg m-2"),
            abbrev: String::from("WVUFLX"),
        },
        225 => TableCategory {
            parameter: String::from("Water Vapor Meridional Flux (Vertical Int)"),
            units: String::from("kg m-2"),
            abbrev: String::from("WVVFLX"),
        },
        226 => TableCategory {
            parameter: String::from("Water Condensate Zonal Flux (Vertical Int)"),
            units: String::from("kg m-2"),
            abbrev: String::from("WCUFLX"),
        },
        227 => TableCategory {
            parameter: String::from("Water Condensate Meridional Flux (Vertical Int)"),
            units: String::from("kg m-2"),
            abbrev: String::from("WCVFLX"),
        },
        228 => TableCategory {
            parameter: String::from("Aerodynamic Conductance"),
            units: String::from("m s-1"),
            abbrev: String::from("ACOND"),
        },
        229 => TableCategory {
            parameter: String::from("Canopy Water Evaporation"),
            units: String::from("W m-2"),
            abbrev: String::from("EVCW"),
        },
        230 => TableCategory {
            parameter: String::from("Transpiration"),
            units: String::from("W m-2"),
            abbrev: String::from("TRANS"),
        },
        231 => TableCategory {
            parameter: String::from(
                "Seasonally Minimum Green Vegetation Fraction (over 1-year period)",
            ),
            units: String::from("%"),
            abbrev: String::from("VEGMIN"),
        },
        232 => TableCategory {
            parameter: String::from(
                "Seasonally Maximum Green Vegetation Fraction (over 1-year period)",
            ),
            units: String::from("%"),
            abbrev: String::from("VEGMAX"),
        },
        233 => TableCategory {
            parameter: String::from("Land Fraction"),
            units: String::from("Fraction"),
            abbrev: String::from("LANDFRC"),
        },
        234 => TableCategory {
            parameter: String::from("Lake Fraction"),
            units: String::from("Fraction"),
            abbrev: String::from("LAKEFRC"),
        },
        235 => TableCategory {
            parameter: String::from("Precipitation Advected Heat Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("PAHFLX"),
        },
        236 => TableCategory {
            parameter: String::from("Water Storage in Aquifer"),
            units: String::from("kg m-2"),
            abbrev: String::from("WATERSA"),
        },
        237 => TableCategory {
            parameter: String::from("Evaporation of Intercepted Water"),
            units: String::from("kg m-2"),
            abbrev: String::from("EIWATER"),
        },
        238 => TableCategory {
            parameter: String::from("Plant Transpiration"),
            units: String::from("kg m-2"),
            abbrev: String::from("PLANTTR"),
        },
        239 => TableCategory {
            parameter: String::from("Soil Surface Evaporation"),
            units: String::from("kg m-2"),
            abbrev: String::from("SOILSE"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
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

/// # GRIB2 - TABLE 4.2-2-1
/// PARAMETERS FOR DISCIPLINE 2, CATEGORY 1
/// **(Land Surface products, Agricultural Special Products category)**
///
/// **Details**:
/// - **Discipline**: 2 (Land Surface products)
/// - **Category**: 1 (Agricultural Special Products)
/// - **Section**: 4
/// - **Octet 10**: 1
/// - **Created**: 12/21/2012
///
/// **Reserved Ranges**:
/// - `0-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 2, Category 1.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 2, Category 1 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_21(category: u8) -> TableCategory {
    match category {
        192 => TableCategory {
            parameter: String::from("Cold Advisory for Newborn Livestock"),
            units: String::from(""),
            abbrev: String::from("CANL"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        0..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-2-3
/// PARAMETERS FOR DISCIPLINE 2, CATEGORY 3
/// **(Land Surface products, Soil category)**
///
/// **Details**:
/// - **Discipline**: 2 (Land Surface products)
/// - **Category**: 3 (Soil)
/// - **Section**: 4
/// - **Octet 10**: 3
/// - **Revised**: 12/07/2023
///
/// **Reserved Ranges**:
/// - `31-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// 1. Deprecated parameters are marked. Refer to Regulation 92.6.2 for alternatives.
/// 2. It is recommended to avoid parameters flagged as less descriptive; alternatives are preferred.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 2, Category 3.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 2, Category 3 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_23(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Soil Type"),
            units: String::from("See Table 4.213"),
            abbrev: String::from("SOTYP"),
        },
        1 => TableCategory {
            parameter: String::from("Upper Layer Soil Temperature (Deprecated)"),
            units: String::from("K"),
            abbrev: String::from("UPLST"),
        },
        2 => TableCategory {
            parameter: String::from("Upper Layer Soil Moisture (Deprecated)"),
            units: String::from("kg m-3"),
            abbrev: String::from("UPLSM"),
        },
        3 => TableCategory {
            parameter: String::from("Lower Layer Soil Moisture (Deprecated)"),
            units: String::from("kg m-3"),
            abbrev: String::from("LOWLSM"),
        },
        4 => TableCategory {
            parameter: String::from("Bottom Layer Soil Temperature (Deprecated)"),
            units: String::from("K"),
            abbrev: String::from("BOTLST"),
        },
        5 => TableCategory {
            parameter: String::from("Liquid Volumetric Soil Moisture (non-frozen)"),
            units: String::from("Proportion"),
            abbrev: String::from("SOILL"),
        },
        6 => TableCategory {
            parameter: String::from("Number of Soil Layers in Root Zone"),
            units: String::from("Numeric"),
            abbrev: String::from("RLYRS"),
        },
        7 => TableCategory {
            parameter: String::from("Transpiration Stress-onset (soil moisture)"),
            units: String::from("Proportion"),
            abbrev: String::from("SMREF"),
        },
        8 => TableCategory {
            parameter: String::from("Direct Evaporation Cease (soil moisture)"),
            units: String::from("Proportion"),
            abbrev: String::from("SMDRY"),
        },
        9 => TableCategory {
            parameter: String::from("Soil Porosity"),
            units: String::from("Proportion"),
            abbrev: String::from("POROS"),
        },
        10 => TableCategory {
            parameter: String::from("Liquid Volumetric Soil Moisture (Non-Frozen)"),
            units: String::from("m3 m-3"),
            abbrev: String::from("LIQVSM"),
        },
        11 => TableCategory {
            parameter: String::from("Volumetric Transpiration Stress-Onset (Soil Moisture)"),
            units: String::from("m3 m-3"),
            abbrev: String::from("VOLTSO"),
        },
        12 => TableCategory {
            parameter: String::from("Transpiration Stress-Onset (Soil Moisture)"),
            units: String::from("kg m-3"),
            abbrev: String::from("TRANSO"),
        },
        13 => TableCategory {
            parameter: String::from("Volumetric Direct Evaporation Cease (Soil Moisture)"),
            units: String::from("m3 m-3"),
            abbrev: String::from("VOLDEC"),
        },
        14 => TableCategory {
            parameter: String::from("Direct Evaporation Cease (Soil Moisture)"),
            units: String::from("kg m-3"),
            abbrev: String::from("DIREC"),
        },
        15 => TableCategory {
            parameter: String::from("Soil Porosity"),
            units: String::from("m3 m-3"),
            abbrev: String::from("SOILP"),
        },
        16 => TableCategory {
            parameter: String::from("Volumetric Saturation Of Soil Moisture"),
            units: String::from("m3 m-3"),
            abbrev: String::from("VSOSM"),
        },
        17 => TableCategory {
            parameter: String::from("Saturation Of Soil Moisture"),
            units: String::from("kg m-3"),
            abbrev: String::from("SATOSM"),
        },
        18 => TableCategory {
            parameter: String::from("Soil Temperature"),
            units: String::from("K"),
            abbrev: String::from("SOILTMP"),
        },
        19 => TableCategory {
            parameter: String::from("Soil Moisture"),
            units: String::from("kg m-3"),
            abbrev: String::from("SOILMOI"),
        },
        20 => TableCategory {
            parameter: String::from("Column-Integrated Soil Moisture"),
            units: String::from("kg m-2"),
            abbrev: String::from("CISOILM"),
        },
        21 => TableCategory {
            parameter: String::from("Soil Ice"),
            units: String::from("kg m-3"),
            abbrev: String::from("SOILICE"),
        },
        22 => TableCategory {
            parameter: String::from("Column-Integrated Soil Ice"),
            units: String::from("kg m-2"),
            abbrev: String::from("CISICE"),
        },
        23 => TableCategory {
            parameter: String::from("Liquid Water in Snow Pack"),
            units: String::from("kg m-2"),
            abbrev: String::from("LWSNWP"),
        },
        24 => TableCategory {
            parameter: String::from("Frost Index"),
            units: String::from("kg day-1"),
            abbrev: String::from("FRSTINX"),
        },
        25 => TableCategory {
            parameter: String::from("Snow Depth at Elevation Bands"),
            units: String::from("kg m-2"),
            abbrev: String::from("SNWDEB"),
        },
        26 => TableCategory {
            parameter: String::from("Soil Heat Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("SHFLX"),
        },
        27 => TableCategory {
            parameter: String::from("Soil Depth"),
            units: String::from("m"),
            abbrev: String::from("SOILDEP"),
        },
        28 => TableCategory {
            parameter: String::from("Snow Temperature"),
            units: String::from("K"),
            abbrev: String::from("SNOWTMP"),
        },
        29 => TableCategory {
            parameter: String::from("Ice Temperature"),
            units: String::from("K"),
            abbrev: String::from("ICETEMP"),
        },
        30 => TableCategory {
            parameter: String::from("Soil Wetness Index"),
            units: String::from("Numeric"),
            abbrev: String::from("SWET"),
        },
        192 => TableCategory {
            parameter: String::from("Liquid Volumetric Soil Moisture (non Frozen)"),
            units: String::from("Proportion"),
            abbrev: String::from("SOILL"),
        },
        193 => TableCategory {
            parameter: String::from("Number of Soil Layers in Root Zone"),
            units: String::from("non-dim"),
            abbrev: String::from("RLYRS"),
        },
        194 => TableCategory {
            parameter: String::from("Surface Slope Type"),
            units: String::from("Index"),
            abbrev: String::from("SLTYP"),
        },
        195 => TableCategory {
            parameter: String::from("Transpiration Stress-onset (soil moisture)"),
            units: String::from("Proportion"),
            abbrev: String::from("SMREF"),
        },
        196 => TableCategory {
            parameter: String::from("Direct Evaporation Cease (soil moisture)"),
            units: String::from("Proportion"),
            abbrev: String::from("SMDRY"),
        },
        197 => TableCategory {
            parameter: String::from("Soil Porosity"),
            units: String::from("Proportion"),
            abbrev: String::from("POROS"),
        },
        198 => TableCategory {
            parameter: String::from("Direct Evaporation from Bare Soil"),
            units: String::from("W m-2"),
            abbrev: String::from("EVBS"),
        },
        199 => TableCategory {
            parameter: String::from("Land Surface Precipitation Accumulation"),
            units: String::from("kg m-2"),
            abbrev: String::from("LSPA"),
        },
        200 => TableCategory {
            parameter: String::from("Bare Soil Surface Skin Temperature"),
            units: String::from("K"),
            abbrev: String::from("BARET"),
        },
        201 => TableCategory {
            parameter: String::from("Average Surface Skin Temperature"),
            units: String::from("K"),
            abbrev: String::from("AVSFT"),
        },
        202 => TableCategory {
            parameter: String::from("Effective Radiative Skin Temperature"),
            units: String::from("K"),
            abbrev: String::from("RADT"),
        },
        203 => TableCategory {
            parameter: String::from("Field Capacity"),
            units: String::from("Fraction"),
            abbrev: String::from("FLDCP"),
        },
        204 => TableCategory {
            parameter: String::from("Soil Moisture Availability In The Top Soil Layer"),
            units: String::from("%"),
            abbrev: String::from("MSTAV"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        31..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-2-4
/// PARAMETERS FOR DISCIPLINE 2, CATEGORY 4
/// **(Land Surface products, Fire Weather category)**
///
/// **Details**:
/// - **Discipline**: 2 (Land Surface products)
/// - **Category**: 4 (Fire Weather)
/// - **Section**: 4
/// - **Octet 10**: 4
/// - **Revised**: 10/30/2023
///
/// **Reserved Ranges**:
/// - `37-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// 1. The Fosberg index denotes the potential influence of weather on wildfire, factoring temperature, wind speed, relative humidity, and precipitation. Higher values indicate a greater potential impact.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 2, Category 4.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 2, Category 4 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_24(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Fire Outlook"),
            units: String::from("See Table 4.224"),
            abbrev: String::from("FIREOLK"),
        },
        1 => TableCategory {
            parameter: String::from("Fire Outlook Due to Dry Thunderstorm"),
            units: String::from("See Table 4.224"),
            abbrev: String::from("FIREODT"),
        },
        2 => TableCategory {
            parameter: String::from("Haines Index"),
            units: String::from("Numeric"),
            abbrev: String::from("HINDEX"),
        },
        3 => TableCategory {
            parameter: String::from("Fire Burned Area"),
            units: String::from("%"),
            abbrev: String::from("FBAREA"),
        },
        4 => TableCategory {
            parameter: String::from("Fosberg Index"),
            units: String::from("Numeric"),
            abbrev: String::from("FOSINDX"),
        },
        5 => TableCategory {
            parameter: String::from("Fire Weather Index (Canadian Forest Service)"),
            units: String::from("Numeric"),
            abbrev: String::from("FWINX"),
        },
        6 => TableCategory {
            parameter: String::from("Fine Fuel Moisture Code (Canadian Forest Service)"),
            units: String::from("Numeric"),
            abbrev: String::from("FFMCODE"),
        },
        7 => TableCategory {
            parameter: String::from("Duff Moisture Code (Canadian Forest Service)"),
            units: String::from("Numeric"),
            abbrev: String::from("DUFMCODE"),
        },
        8 => TableCategory {
            parameter: String::from("Drought Code (Canadian Forest Service)"),
            units: String::from("Numeric"),
            abbrev: String::from("DRTCODE"),
        },
        9 => TableCategory {
            parameter: String::from("Initial Fire Spread Index (Canadian Forest Service)"),
            units: String::from("Numeric"),
            abbrev: String::from("INFSINX"),
        },
        10 => TableCategory {
            parameter: String::from("Fire Build Up Index (Canadian Forest Service)"),
            units: String::from("Numeric"),
            abbrev: String::from("FBUPINX"),
        },
        11 => TableCategory {
            parameter: String::from("Fire Daily Severity Rating (Canadian Forest Service)"),
            units: String::from("Numeric"),
            abbrev: String::from("FDSRTE"),
        },
        12 => TableCategory {
            parameter: String::from("Keetch-Byram Drought Index"),
            units: String::from("Numeric"),
            abbrev: String::from("KRIDX"),
        },
        13 => TableCategory {
            parameter: String::from("Drought Factor (Australian forest service)"),
            units: String::from("Numeric"),
            abbrev: String::from("DRFACT"),
        },
        14 => TableCategory {
            parameter: String::from("Rate of Spread (Australian forest service)"),
            units: String::from("m s-1"),
            abbrev: String::from("RATESPRD"),
        },
        15 => TableCategory {
            parameter: String::from("Fire Danger Index (Australian forest service)"),
            units: String::from("Numeric"),
            abbrev: String::from("FIREDIDX"),
        },
        16 => TableCategory {
            parameter: String::from("Spread Component (US Forest Service NFDRS)"),
            units: String::from("Numeric"),
            abbrev: String::from("SPRDCOMP"),
        },
        17 => TableCategory {
            parameter: String::from("Burning Index (Australian forest service)"),
            units: String::from("Numeric"),
            abbrev: String::from("BURNIDX"),
        },
        18 => TableCategory {
            parameter: String::from("Ignition Component (Australian forest service)"),
            units: String::from("%"),
            abbrev: String::from("IGNCOMP"),
        },
        19 => TableCategory {
            parameter: String::from("Energy Release Component (Australian forest service)"),
            units: String::from("J m-2"),
            abbrev: String::from("ENRELCOM"),
        },
        20 => TableCategory {
            parameter: String::from("Burning Area"),
            units: String::from("%"),
            abbrev: String::from("BURNAREA"),
        },
        21 => TableCategory {
            parameter: String::from("Burnable Area"),
            units: String::from("%"),
            abbrev: String::from("BURNABAREA"),
        },
        22 => TableCategory {
            parameter: String::from("Unburnable Area"),
            units: String::from("%"),
            abbrev: String::from("UNBURNAREA"),
        },
        23 => TableCategory {
            parameter: String::from("Fuel Load"),
            units: String::from("kg m-2"),
            abbrev: String::from("FUELLOAD"),
        },
        24 => TableCategory {
            parameter: String::from("Combustion Completeness"),
            units: String::from("%"),
            abbrev: String::from("COMBCO"),
        },
        25 => TableCategory {
            parameter: String::from("Fuel Moisture Content"),
            units: String::from("kg kg-1"),
            abbrev: String::from("FUELMC"),
        },
        26 => TableCategory {
            parameter: String::from("Wildfire Potential (NOAA GSL)"),
            units: String::from("Numeric"),
            abbrev: String::from("WFIREPOT"),
        },
        27 => TableCategory {
            parameter: String::from("Live Leaf Fuel Load"),
            units: String::from("kg m-2"),
            abbrev: String::from("LLFL"),
        },
        28 => TableCategory {
            parameter: String::from("Live Wood Fuel Load"),
            units: String::from("kg m-2"),
            abbrev: String::from("LWFL"),
        },
        29 => TableCategory {
            parameter: String::from("Dead Leaf Fuel Load"),
            units: String::from("kg m-2"),
            abbrev: String::from("DLFL"),
        },
        30 => TableCategory {
            parameter: String::from("Dead Wood Fuel Load"),
            units: String::from("kg m-2"),
            abbrev: String::from("DWFL"),
        },
        31 => TableCategory {
            parameter: String::from("Live Fuel Moisture Content"),
            units: String::from("kg kg-1"),
            abbrev: String::from("LFMC"),
        },
        32 => TableCategory {
            parameter: String::from("Fine Dead Leaf Moisture Content"),
            units: String::from("kg kg-1"),
            abbrev: String::from("FDLMC"),
        },
        33 => TableCategory {
            parameter: String::from("Dense Dead Leaf Moisture Content"),
            units: String::from("kg kg-1"),
            abbrev: String::from("DDLMC"),
        },
        34 => TableCategory {
            parameter: String::from("Fine Dead Wood Moisture Content"),
            units: String::from("kg kg-1"),
            abbrev: String::from("FDWMC"),
        },
        35 => TableCategory {
            parameter: String::from("Dense Dead Wood Moisture Content"),
            units: String::from("kg kg-1"),
            abbrev: String::from("DDWMC"),
        },
        36 => TableCategory {
            parameter: String::from("Fire Radiative Power"),
            units: String::from("W"),
            abbrev: String::from("FRADPOW"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        37..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-2-5
/// PARAMETERS FOR DISCIPLINE 2, CATEGORY 5
/// **(Land Surface products, Glaciers and Inland Ice category)**
///
/// **Details**:
/// - **Discipline**: 2 (Land Surface products)
/// - **Category**: 5 (Glaciers and Inland Ice)
/// - **Section**: 4
/// - **Octet 10**: 5
/// - **Revised**: 10/30/2023
///
/// **Reserved Ranges**:
/// - `2-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// 1. A value strictly above 0.5 for Glacier Cover is treated as glacier. A value equal to or below 0.5 is treated as land without glacier.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 2, Category 5.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 2, Category 5 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_25(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Glacier Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("GLACCOV"),
        },
        1 => TableCategory {
            parameter: String::from("Glacier Temperature"),
            units: String::from("K"),
            abbrev: String::from("GLACTMP"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        2..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-2-6
/// PARAMETERS FOR DISCIPLINE 2, CATEGORY 6
/// **(Land Surface products, Urban Areas category)**
///
/// **Details**:
/// - **Discipline**: 2 (Land Surface products)
/// - **Category**: 6 (Urban Areas)
/// - **Section**: 4
/// - **Octet 10**: 6
/// - **Revised**: 12/07/2023
///
/// **Reserved Ranges**:
/// - `9-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 2, Category 6.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 2, Category 6 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_26(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Urban Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("URBCOVER"),
        },
        1 => TableCategory {
            parameter: String::from("Road Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("ROADCOVER"),
        },
        2 => TableCategory {
            parameter: String::from("Building Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("BUILDCOVER"),
        },
        3 => TableCategory {
            parameter: String::from("Building Height"),
            units: String::from("m"),
            abbrev: String::from("BUILDHGT"),
        },
        4 => TableCategory {
            parameter: String::from("Vertical-to-Horizontal Area Fraction"),
            units: String::from("m2 m-2"),
            abbrev: String::from("VZAFRAC"),
        },
        5 => TableCategory {
            parameter: String::from("Standard Deviation of Building Height"),
            units: String::from("m"),
            abbrev: String::from("SDBUILDHGT"),
        },
        6 => TableCategory {
            parameter: String::from("Distance downward from roof surface"),
            units: String::from("m"),
            abbrev: String::from("DDROOF"),
        },
        7 => TableCategory {
            parameter: String::from("Distance inward from outer wall surface"),
            units: String::from("m"),
            abbrev: String::from("DIOWALL"),
        },
        8 => TableCategory {
            parameter: String::from("Distance downward from road surface"),
            units: String::from("m"),
            abbrev: String::from("DDROAD"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        9..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-3-0
/// PARAMETERS FOR DISCIPLINE 3, CATEGORY 0
/// **(Space products, Image Format category)**
///
/// **Details**:
/// - **Discipline**: 3 (Space products)
/// - **Category**: 0 (Image Format)
/// - **Section**: 4
/// - **Octet 10**: 0
/// - **Revised**: 06/27/2008
///
/// **Reserved Ranges**:
/// - `10-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 3, Category 0.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 3, Category 0 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_30(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Scaled Radiance"),
            units: String::from("Numeric"),
            abbrev: String::from("SRAD"),
        },
        1 => TableCategory {
            parameter: String::from("Scaled Albedo"),
            units: String::from("Numeric"),
            abbrev: String::from("SALBEDO"),
        },
        2 => TableCategory {
            parameter: String::from("Scaled Brightness Temperature"),
            units: String::from("Numeric"),
            abbrev: String::from("SBTMP"),
        },
        3 => TableCategory {
            parameter: String::from("Scaled Precipitable Water"),
            units: String::from("Numeric"),
            abbrev: String::from("SPWAT"),
        },
        4 => TableCategory {
            parameter: String::from("Scaled Lifted Index"),
            units: String::from("Numeric"),
            abbrev: String::from("SLFTI"),
        },
        5 => TableCategory {
            parameter: String::from("Scaled Cloud Top Pressure"),
            units: String::from("Numeric"),
            abbrev: String::from("SCTPRES"),
        },
        6 => TableCategory {
            parameter: String::from("Scaled Skin Temperature"),
            units: String::from("Numeric"),
            abbrev: String::from("SSTMP"),
        },
        7 => TableCategory {
            parameter: String::from("Cloud Mask"),
            units: String::from("See Table 4.217"),
            abbrev: String::from("CLOUDM"),
        },
        8 => TableCategory {
            parameter: String::from("Pixel Scene Type"),
            units: String::from("See Table 4.218"),
            abbrev: String::from("PIXST"),
        },
        9 => TableCategory {
            parameter: String::from("Fire Detection Indicator"),
            units: String::from("See Table 4.223"),
            abbrev: String::from("FIREDI"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        10..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-3-1
/// PARAMETERS FOR DISCIPLINE 3, CATEGORY 1
/// **(Space products, Quantitative category)**
///
/// **Details**:
/// - **Discipline**: 3 (Space products)
/// - **Category**: 1 (Quantitative)
/// - **Section**: 4
/// - **Octet 10**: 1
/// - **Revised**: 12/07/2023
///
/// **Reserved Ranges**:
/// - `18`: Reserved
/// - `24-26`: Reserved
/// - `33-97`: Reserved
/// - `100-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// 1. Bidirectional Reflectance Factor is the ratio of the radiant flux reflected by a surface to that reflected by an ideal, diffuse Lambertian standard surface under identical conditions.
/// 2. Scaled Radiance is the top-of-atmosphere radiance observed by a sensor, multiplied by π, divided by the in-band solar irradiance.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 3, Category 1.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 3, Category 1 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_31(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Estimated Precipitation"),
            units: String::from("kg m-2"),
            abbrev: String::from("ESTP"),
        },
        1 => TableCategory {
            parameter: String::from("Instantaneous Rain Rate"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("IRRATE"),
        },
        2 => TableCategory {
            parameter: String::from("Cloud Top Height"),
            units: String::from("m"),
            abbrev: String::from("CTOPH"),
        },
        3 => TableCategory {
            parameter: String::from("Cloud Top Height Quality Indicator"),
            units: String::from("Code table 4.219"),
            abbrev: String::from("CTOPHQI"),
        },
        4 => TableCategory {
            parameter: String::from("Estimated u-Component of Wind"),
            units: String::from("m s-1"),
            abbrev: String::from("ESTUGRD"),
        },
        5 => TableCategory {
            parameter: String::from("Estimated v-Component of Wind"),
            units: String::from("m s-1"),
            abbrev: String::from("ESTVGRD"),
        },
        6 => TableCategory {
            parameter: String::from("Number Of Pixels Used"),
            units: String::from("Numeric"),
            abbrev: String::from("NPIXU"),
        },
        7 => TableCategory {
            parameter: String::from("Solar Zenith Angle"),
            units: String::from("°"),
            abbrev: String::from("SOLZA"),
        },
        8 => TableCategory {
            parameter: String::from("Relative Azimuth Angle"),
            units: String::from("°"),
            abbrev: String::from("RAZA"),
        },
        9 => TableCategory {
            parameter: String::from("Reflectance in 0.6 Micron Channel"),
            units: String::from("%"),
            abbrev: String::from("RFL06"),
        },
        10 => TableCategory {
            parameter: String::from("Reflectance in 0.8 Micron Channel"),
            units: String::from("%"),
            abbrev: String::from("RFL08"),
        },
        11 => TableCategory {
            parameter: String::from("Reflectance in 1.6 Micron Channel"),
            units: String::from("%"),
            abbrev: String::from("RFL16"),
        },
        12 => TableCategory {
            parameter: String::from("Reflectance in 3.9 Micron Channel"),
            units: String::from("%"),
            abbrev: String::from("RFL39"),
        },
        13 => TableCategory {
            parameter: String::from("Atmospheric Divergence"),
            units: String::from("s-1"),
            abbrev: String::from("ATMDIV"),
        },
        14 => TableCategory {
            parameter: String::from("Cloudy Brightness Temperature"),
            units: String::from("K"),
            abbrev: String::from("CBTMP"),
        },
        15 => TableCategory {
            parameter: String::from("Clear Sky Brightness Temperature"),
            units: String::from("K"),
            abbrev: String::from("CSBTMP"),
        },
        16 => TableCategory {
            parameter: String::from("Cloudy Radiance (with respect to wave number)"),
            units: String::from("W m-1 sr-1"),
            abbrev: String::from("CLDRAD"),
        },
        17 => TableCategory {
            parameter: String::from("Clear Sky Radiance (with respect to wave number)"),
            units: String::from("W m-1 sr-1"),
            abbrev: String::from("CSKYRAD"),
        },
        19 => TableCategory {
            parameter: String::from("Wind Speed"),
            units: String::from("m s-1"),
            abbrev: String::from("WINDS"),
        },
        20 => TableCategory {
            parameter: String::from("Aerosol Optical Thickness at 0.635 µm"),
            units: String::from(""),
            abbrev: String::from("AOT06"),
        },
        21 => TableCategory {
            parameter: String::from("Aerosol Optical Thickness at 0.810 µm"),
            units: String::from(""),
            abbrev: String::from("AOT08"),
        },
        22 => TableCategory {
            parameter: String::from("Aerosol Optical Thickness at 1.640 µm"),
            units: String::from(""),
            abbrev: String::from("AOT16"),
        },
        23 => TableCategory {
            parameter: String::from("Angstrom Coefficient"),
            units: String::from(""),
            abbrev: String::from("ANGCOE"),
        },
        27 => TableCategory {
            parameter: String::from("Bidirectional Reflectance Factor"),
            units: String::from("Numeric"),
            abbrev: String::from("BRFLF"),
        },
        28 => TableCategory {
            parameter: String::from("Brightness Temperature"),
            units: String::from("K"),
            abbrev: String::from("SPBRT"),
        },
        29 => TableCategory {
            parameter: String::from("Scaled Radiance"),
            units: String::from("Numeric"),
            abbrev: String::from("SCRAD"),
        },
        30 => TableCategory {
            parameter: String::from("Reflectance in 0.4 Micron Channel"),
            units: String::from("%"),
            abbrev: String::from("RFL04"),
        },
        31 => TableCategory {
            parameter: String::from("Cloudy Reflectance"),
            units: String::from("%"),
            abbrev: String::from("CLDREF"),
        },
        32 => TableCategory {
            parameter: String::from("Clear Reflectance"),
            units: String::from("%"),
            abbrev: String::from("CLRREF"),
        },
        98 => TableCategory {
            parameter: String::from(
                "Correlation Coefficient Between MPE Rain Rates for Co-located IR Data and \
                 Microwave Data Rain Rates",
            ),
            units: String::from("Numeric"),
            abbrev: String::from("CCMPEMRR"),
        },
        99 => TableCategory {
            parameter: String::from(
                "Standard Deviation Between MPE Rain Rates for Co-located IR Data and Microwave \
                 Data Rain Rates",
            ),
            units: String::from("Numeric"),
            abbrev: String::from("SDMPEMRR"),
        },
        192 => TableCategory {
            parameter: String::from("Scatterometer Estimated U Wind Component"),
            units: String::from("m s-1"),
            abbrev: String::from("USCT"),
        },
        193 => TableCategory {
            parameter: String::from("Scatterometer Estimated V Wind Component"),
            units: String::from("m s-1"),
            abbrev: String::from("VSCT"),
        },
        194 => TableCategory {
            parameter: String::from("Scatterometer Wind Quality"),
            units: String::from(""),
            abbrev: String::from("SWQI"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        18 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        24..=26 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        33..=97 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        100..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-3-2
/// PARAMETERS FOR DISCIPLINE 3, CATEGORY 2
/// **(Space products, Cloud Properties category)**
///
/// **Details**:
/// - **Discipline**: 3 (Space products)
/// - **Category**: 2 (Cloud Properties)
/// - **Section**: 4
/// - **Octet 10**: 2
/// - **Revised**: 07/15/2024
///
/// **Reserved Ranges**:
/// - `12-29`: Reserved
/// - `41-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// 1. Numbers 31 to 40 are deprecated.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 3, Category 2.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 3, Category 2 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_32(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Clear Sky Probability"),
            units: String::from("%"),
            abbrev: String::from("CSKPROB"),
        },
        1 => TableCategory {
            parameter: String::from("Cloud Top Temperature"),
            units: String::from("K"),
            abbrev: String::from("CTOPTMP"),
        },
        2 => TableCategory {
            parameter: String::from("Cloud Top Pressure"),
            units: String::from("Pa"),
            abbrev: String::from("CTOPRES"),
        },
        3 => TableCategory {
            parameter: String::from("Cloud Type"),
            units: String::from("See Table 4.218"),
            abbrev: String::from("CLDTYPE"),
        },
        4 => TableCategory {
            parameter: String::from("Cloud Phase"),
            units: String::from("See Table 4.218"),
            abbrev: String::from("CLDPHAS"),
        },
        5 => TableCategory {
            parameter: String::from("Cloud Optical Depth"),
            units: String::from("Numeric"),
            abbrev: String::from("CLDODEP"),
        },
        6 => TableCategory {
            parameter: String::from("Cloud Particle Effective Radius"),
            units: String::from("m"),
            abbrev: String::from("CLDPER"),
        },
        7 => TableCategory {
            parameter: String::from("Cloud Liquid Water Path"),
            units: String::from("kg m-2"),
            abbrev: String::from("CLDLWP"),
        },
        8 => TableCategory {
            parameter: String::from("Cloud Ice Water Path"),
            units: String::from("kg m-2"),
            abbrev: String::from("CLDIWP"),
        },
        9 => TableCategory {
            parameter: String::from("Cloud Albedo"),
            units: String::from("Numeric"),
            abbrev: String::from("CLDALB"),
        },
        10 => TableCategory {
            parameter: String::from("Cloud Emissivity"),
            units: String::from("Numeric"),
            abbrev: String::from("CLDEMISS"),
        },
        11 => TableCategory {
            parameter: String::from("Effective Absorption Optical Depth Ratio"),
            units: String::from("Numeric"),
            abbrev: String::from("EAODR"),
        },
        30 => TableCategory {
            parameter: String::from("Measurement Cost"),
            units: String::from("Numeric"),
            abbrev: String::from("MEACST"),
        },
        31 => TableCategory {
            parameter: String::from("Upper Layer Cloud Optical Depth (Deprecated)"),
            units: String::from("Numeric"),
            abbrev: String::from(""),
        },
        32 => TableCategory {
            parameter: String::from("Upper Layer Cloud Top Pressure (Deprecated)"),
            units: String::from("Pa"),
            abbrev: String::from(""),
        },
        33 => TableCategory {
            parameter: String::from("Upper Layer Cloud Effective Radius (Deprecated)"),
            units: String::from("m"),
            abbrev: String::from(""),
        },
        34 => TableCategory {
            parameter: String::from("Error in Upper Layer Cloud Optical Depth (Deprecated)"),
            units: String::from("Numeric"),
            abbrev: String::from(""),
        },
        35 => TableCategory {
            parameter: String::from("Error in Upper Layer Cloud Top Pressure (Deprecated)"),
            units: String::from("Pa"),
            abbrev: String::from(""),
        },
        36 => TableCategory {
            parameter: String::from("Error in Upper Layer Cloud Effective Radius (Deprecated)"),
            units: String::from("m"),
            abbrev: String::from(""),
        },
        37 => TableCategory {
            parameter: String::from("Lower Layer Cloud Optical Depth (Deprecated)"),
            units: String::from("Numeric"),
            abbrev: String::from(""),
        },
        38 => TableCategory {
            parameter: String::from("Lower Layer Cloud Top Pressure (Deprecated)"),
            units: String::from("Pa"),
            abbrev: String::from(""),
        },
        39 => TableCategory {
            parameter: String::from("Error in Lower Layer Cloud Optical Depth (Deprecated)"),
            units: String::from("Numeric"),
            abbrev: String::from(""),
        },
        40 => TableCategory {
            parameter: String::from("Error in Lower Layer Cloud Top Pressure (Deprecated)"),
            units: String::from("Pa"),
            abbrev: String::from(""),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        12..=29 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        41..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-3-3
/// PARAMETERS FOR DISCIPLINE 3, CATEGORY 3
/// **(Space products, Flight Rules Conditions category)**
///
/// **Details**:
/// - **Discipline**: 3 (Space products)
/// - **Category**: 3 (Flight Rules Conditions)
/// - **Section**: 4
/// - **Octet 10**: 3
/// - **Created**: 07/26/2016
///
/// **Reserved Ranges**:
/// - `3-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 3, Category 3.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 3, Category 3 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_33(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from(
                "Probability of Encountering Marginal Visual Flight Rules Conditions",
            ),
            units: String::from("%"),
            abbrev: String::from("PBMVFRC"),
        },
        1 => TableCategory {
            parameter: String::from(
                "Probability of Encountering Low Instrument Flight Rules Conditions",
            ),
            units: String::from("%"),
            abbrev: String::from("PBLIFRC"),
        },
        2 => TableCategory {
            parameter: String::from(
                "Probability of Encountering Instrument Flight Rules Conditions",
            ),
            units: String::from("%"),
            abbrev: String::from("PBINFRC"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
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

/// # GRIB2 - TABLE 4.2-3-4
/// PARAMETERS FOR DISCIPLINE 3, CATEGORY 4
/// **(Space products, Volcanic Ash category)**
///
/// **Details**:
/// - **Discipline**: 3 (Space products)
/// - **Category**: 4 (Volcanic Ash)
/// - **Section**: 4
/// - **Octet 10**: 4
/// - **Created**: 07/26/2016
///
/// **Reserved Ranges**:
/// - `9-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 3, Category 4.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 3, Category 4 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_34(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Volcanic Ash Probability"),
            units: String::from("%"),
            abbrev: String::from("VOLAPROB"),
        },
        1 => TableCategory {
            parameter: String::from("Volcanic Ash Cloud Top Temperature"),
            units: String::from("K"),
            abbrev: String::from("VOLACDTT"),
        },
        2 => TableCategory {
            parameter: String::from("Volcanic Ash Cloud Top Pressure"),
            units: String::from("Pa"),
            abbrev: String::from("VOLACDTP"),
        },
        3 => TableCategory {
            parameter: String::from("Volcanic Ash Cloud Top Height"),
            units: String::from("m"),
            abbrev: String::from("VOLACDTH"),
        },
        4 => TableCategory {
            parameter: String::from("Volcanic Ash Cloud Emissivity"),
            units: String::from("Numeric"),
            abbrev: String::from("VOLACDEM"),
        },
        5 => TableCategory {
            parameter: String::from("Volcanic Ash Effective Absorption Depth Ratio"),
            units: String::from("Numeric"),
            abbrev: String::from("VOLAEADR"),
        },
        6 => TableCategory {
            parameter: String::from("Volcanic Ash Cloud Optical Depth"),
            units: String::from("Numeric"),
            abbrev: String::from("VOLACDOD"),
        },
        7 => TableCategory {
            parameter: String::from("Volcanic Ash Column Density"),
            units: String::from("kg m-2"),
            abbrev: String::from("VOLACDEN"),
        },
        8 => TableCategory {
            parameter: String::from("Volcanic Ash Particle Effective Radius"),
            units: String::from("m"),
            abbrev: String::from("VOLAPER"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        9..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-3-5
/// PARAMETERS FOR DISCIPLINE 3, CATEGORY 5
/// **(Space products, Sea-Surface Temperature category)**
///
/// **Details**:
/// - **Discipline**: 3 (Space products)
/// - **Category**: 5 (Sea-Surface Temperature)
/// - **Section**: 4
/// - **Octet 10**: 5
/// - **Created**: 07/26/2016
///
/// **Reserved Ranges**:
/// - `6-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// 1. Interface Sea-Surface Temperature: Theoretical temperature at the precise air-sea interface.
/// 2. Skin Sea-Surface Temperature: Temperature across a very small depth (~20 micrometers).
/// 3. Sub-Skin Sea-Surface Temperature: Temperature at the base of the thermal skin layer.
/// 4. Foundation Sea-Surface Temperature: Temperature in the water column free of diurnal variability.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 3, Category 5.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 3, Category 5 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_35(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Interface Sea-Surface Temperature"),
            units: String::from("K"),
            abbrev: String::from("ISSTMP"),
        },
        1 => TableCategory {
            parameter: String::from("Skin Sea-Surface Temperature"),
            units: String::from("K"),
            abbrev: String::from("SKSSTMP"),
        },
        2 => TableCategory {
            parameter: String::from("Sub-Skin Sea-Surface Temperature"),
            units: String::from("K"),
            abbrev: String::from("SSKSSTMP"),
        },
        3 => TableCategory {
            parameter: String::from("Foundation Sea-Surface Temperature"),
            units: String::from("K"),
            abbrev: String::from("FDNSSTMP"),
        },
        4 => TableCategory {
            parameter: String::from("Estimated Bias Between Sea-Surface Temperature and Standard"),
            units: String::from("K"),
            abbrev: String::from("EBSSTSTD"),
        },
        5 => TableCategory {
            parameter: String::from(
                "Estimated Bias Standard Deviation Between Sea-Surface Temperature and Standard",
            ),
            units: String::from("K"),
            abbrev: String::from("EBSDSSTS"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
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

/// # GRIB2 - TABLE 4.2-3-6
/// PARAMETERS FOR DISCIPLINE 3, CATEGORY 6
/// **(Space products, Solar Radiation category)**
///
/// **Details**:
/// - **Discipline**: 3 (Space products)
/// - **Category**: 6 (Solar Radiation)
/// - **Section**: 4
/// - **Octet 10**: 6
/// - **Created**: 07/26/2016
///
/// **Reserved Ranges**:
/// - `6-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes
/// 1. Global Solar Irradiance: The solar flux per unit area received from a solid angle of 2π sr on a horizontal surface.
/// 2. Global Solar Exposure: The integral of global solar irradiance.
/// 3. Direct Solar Irradiance: Solar flux per unit area received from the Sun's disc on a surface normal to the Sun's direction.
/// 4. Direct Solar Exposure: Time integral of direct solar irradiance.
/// 5. Diffuse Solar Irradiance: Solar flux per unit area received from 2π sr, excluding the Sun's disc, on a horizontal surface.
/// 6. Diffuse Solar Exposure: Time integral of diffuse solar irradiance.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 3, Category 6.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 3, Category 6 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_36(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Global Solar Irradiance"),
            units: String::from("W m-2"),
            abbrev: String::from("GSOLIRR"),
        },
        1 => TableCategory {
            parameter: String::from("Global Solar Exposure"),
            units: String::from("J m-2"),
            abbrev: String::from("GSOLEXP"),
        },
        2 => TableCategory {
            parameter: String::from("Direct Solar Irradiance"),
            units: String::from("W m-2"),
            abbrev: String::from("DIRSOLIR"),
        },
        3 => TableCategory {
            parameter: String::from("Direct Solar Exposure"),
            units: String::from("J m-2"),
            abbrev: String::from("DIRSOLEX"),
        },
        4 => TableCategory {
            parameter: String::from("Diffuse Solar Irradiance"),
            units: String::from("W m-2"),
            abbrev: String::from("DIFSOLIR"),
        },
        5 => TableCategory {
            parameter: String::from("Diffuse Solar Exposure"),
            units: String::from("J m-2"),
            abbrev: String::from("DIFSOLEX"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
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

/// # GRIB2 - TABLE 4.2-3-192
/// PARAMETERS FOR DISCIPLINE 3, CATEGORY 192
/// **(Space products, Forecast Satellite Imagery category)**
///
/// **Details**:
/// - **Discipline**: 3 (Space products)
/// - **Category**: 192 (Forecast Satellite Imagery)
/// - **Section**: 4
/// - **Octet 10**: 192
/// - **Revised**: 03/28/2022
///
/// **Reserved Ranges**:
/// - `86-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 3, Category 192.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 3, Category 192 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_3192(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for GOES 12, Channel 2"),
            units: String::from("K"),
            abbrev: String::from("SBT122"),
        },
        1 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for GOES 12, Channel 3"),
            units: String::from("K"),
            abbrev: String::from("SBT123"),
        },
        2 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for GOES 12, Channel 4"),
            units: String::from("K"),
            abbrev: String::from("SBT124"),
        },
        3 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for GOES 12, Channel 6"),
            units: String::from("K"),
            abbrev: String::from("SBT126"),
        },
        4 => TableCategory {
            parameter: String::from("Simulated Brightness Counts for GOES 12, Channel 3"),
            units: String::from("Byte"),
            abbrev: String::from("SBC123"),
        },
        5 => TableCategory {
            parameter: String::from("Simulated Brightness Counts for GOES 12, Channel 4"),
            units: String::from("Byte"),
            abbrev: String::from("SBC124"),
        },
        6 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for GOES 11, Channel 2"),
            units: String::from("K"),
            abbrev: String::from("SBT112"),
        },
        7 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for GOES 11, Channel 3"),
            units: String::from("K"),
            abbrev: String::from("SBT113"),
        },
        8 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for GOES 11, Channel 4"),
            units: String::from("K"),
            abbrev: String::from("SBT114"),
        },
        9 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for GOES 11, Channel 5"),
            units: String::from("K"),
            abbrev: String::from("SBT115"),
        },
        10 => TableCategory {
            parameter: String::from(
                "Simulated Brightness Temperature for AMSRE on Aqua, Channel 9",
            ),
            units: String::from("K"),
            abbrev: String::from("AMSRE9"),
        },
        11 => TableCategory {
            parameter: String::from(
                "Simulated Brightness Temperature for AMSRE on Aqua, Channel 10",
            ),
            units: String::from("K"),
            abbrev: String::from("AMSRE10"),
        },
        12 => TableCategory {
            parameter: String::from(
                "Simulated Brightness Temperature for AMSRE on Aqua, Channel 11",
            ),
            units: String::from("K"),
            abbrev: String::from("AMSRE11"),
        },
        13 => TableCategory {
            parameter: String::from(
                "Simulated Brightness Temperature for AMSRE on Aqua, Channel 12",
            ),
            units: String::from("K"),
            abbrev: String::from("AMSRE12"),
        },
        14 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for ABI GOES-16, Band-1"),
            units: String::from(""),
            abbrev: String::from("SRFA161"),
        },
        15 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for ABI GOES-16, Band-2"),
            units: String::from(""),
            abbrev: String::from("SRFA162"),
        },
        16 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for ABI GOES-16, Band-3"),
            units: String::from(""),
            abbrev: String::from("SRFA163"),
        },
        17 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for ABI GOES-16, Band-4"),
            units: String::from(""),
            abbrev: String::from("SRFA164"),
        },
        18 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for ABI GOES-16, Band-5"),
            units: String::from(""),
            abbrev: String::from("SRFA165"),
        },
        19 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for ABI GOES-16, Band-6"),
            units: String::from(""),
            abbrev: String::from("SRFA166"),
        },
        20 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-16, Band-7"),
            units: String::from("K"),
            abbrev: String::from("SBTA167"),
        },
        21 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-16, Band-8"),
            units: String::from("K"),
            abbrev: String::from("SBTA168"),
        },
        22 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-16, Band-9"),
            units: String::from("K"),
            abbrev: String::from("SBTA169"),
        },
        23 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-16, Band-10"),
            units: String::from("K"),
            abbrev: String::from("SBTA1610"),
        },
        24 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-16, Band-11"),
            units: String::from("K"),
            abbrev: String::from("SBTA1611"),
        },
        25 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-16, Band-12"),
            units: String::from("K"),
            abbrev: String::from("SBTA1612"),
        },
        26 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-16, Band-13"),
            units: String::from("K"),
            abbrev: String::from("SBTA1613"),
        },
        27 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-16, Band-14"),
            units: String::from("K"),
            abbrev: String::from("SBTA1614"),
        },
        28 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-16, Band-15"),
            units: String::from("K"),
            abbrev: String::from("SBTA1615"),
        },
        29 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-16, Band-16"),
            units: String::from("K"),
            abbrev: String::from("SBTA1616"),
        },
        30 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for ABI GOES-17, Band-1"),
            units: String::from(""),
            abbrev: String::from("SRFA171"),
        },
        31 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for ABI GOES-17, Band-2"),
            units: String::from(""),
            abbrev: String::from("SRFA172"),
        },
        32 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for ABI GOES-17, Band-3"),
            units: String::from(""),
            abbrev: String::from("SRFA173"),
        },
        33 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for ABI GOES-17, Band-4"),
            units: String::from(""),
            abbrev: String::from("SRFA174"),
        },
        34 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for ABI GOES-17, Band-5"),
            units: String::from(""),
            abbrev: String::from("SRFA175"),
        },
        35 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for ABI GOES-17, Band-6"),
            units: String::from(""),
            abbrev: String::from("SRFA176"),
        },
        36 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-17, Band-7"),
            units: String::from("K"),
            abbrev: String::from("SBTA177"),
        },
        37 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-17, Band-8"),
            units: String::from("K"),
            abbrev: String::from("SBTA178"),
        },
        38 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-17, Band-9"),
            units: String::from("K"),
            abbrev: String::from("SBTA179"),
        },
        39 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-17, Band-10"),
            units: String::from("K"),
            abbrev: String::from("SBTA1710"),
        },
        40 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-17, Band-11"),
            units: String::from("K"),
            abbrev: String::from("SBTA1711"),
        },
        41 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-17, Band-12"),
            units: String::from("K"),
            abbrev: String::from("SBTA1712"),
        },
        42 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-17, Band-13"),
            units: String::from("K"),
            abbrev: String::from("SBTA1713"),
        },
        43 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-17, Band-14"),
            units: String::from("K"),
            abbrev: String::from("SBTA1714"),
        },
        44 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-17, Band-15"),
            units: String::from("K"),
            abbrev: String::from("SBTA1715"),
        },
        45 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-17, Band-16"),
            units: String::from("K"),
            abbrev: String::from("SBTA1716"),
        },
        46 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for nadir ABI GOES-R, Band-1"),
            units: String::from(""),
            abbrev: String::from("SRFAGR1"),
        },
        47 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for nadir ABI GOES-R, Band-2"),
            units: String::from(""),
            abbrev: String::from("SRFAGR2"),
        },
        48 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for nadir ABI GOES-R, Band-3"),
            units: String::from(""),
            abbrev: String::from("SRFAGR3"),
        },
        49 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for nadir ABI GOES-R, Band-4"),
            units: String::from(""),
            abbrev: String::from("SRFAGR4"),
        },
        50 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for nadir ABI GOES-R, Band-5"),
            units: String::from(""),
            abbrev: String::from("SRFAGR5"),
        },
        51 => TableCategory {
            parameter: String::from("Simulated Reflectance Factor for nadir ABI GOES-R, Band-6"),
            units: String::from(""),
            abbrev: String::from("SRFAGR6"),
        },
        52 => TableCategory {
            parameter: String::from(
                "Simulated Brightness Temperature for nadir ABI GOES-R, Band-7",
            ),
            units: String::from("K"),
            abbrev: String::from("SBTAGR7"),
        },
        53 => TableCategory {
            parameter: String::from(
                "Simulated Brightness Temperature for nadir ABI GOES-R, Band-8",
            ),
            units: String::from("K"),
            abbrev: String::from("SBTAGR8"),
        },
        54 => TableCategory {
            parameter: String::from(
                "Simulated Brightness Temperature for nadir ABI GOES-R, Band-9",
            ),
            units: String::from("K"),
            abbrev: String::from("SBTAGR9"),
        },
        55 => TableCategory {
            parameter: String::from(
                "Simulated Brightness Temperature for nadir ABI GOES-R, Band-10",
            ),
            units: String::from("K"),
            abbrev: String::from("SBTAGR10"),
        },
        56 => TableCategory {
            parameter: String::from(
                "Simulated Brightness Temperature for nadir ABI GOES-R, Band-11",
            ),
            units: String::from("K"),
            abbrev: String::from("SBTAGR11"),
        },
        57 => TableCategory {
            parameter: String::from(
                "Simulated Brightness Temperature for nadir ABI GOES-R, Band-12",
            ),
            units: String::from("K"),
            abbrev: String::from("SBTAGR12"),
        },
        58 => TableCategory {
            parameter: String::from(
                "Simulated Brightness Temperature for nadir ABI GOES-R, Band-13",
            ),
            units: String::from("K"),
            abbrev: String::from("SBTAGR13"),
        },
        59 => TableCategory {
            parameter: String::from(
                "Simulated Brightness Temperature for nadir ABI GOES-R, Band-14",
            ),
            units: String::from("K"),
            abbrev: String::from("SBTAGR14"),
        },
        60 => TableCategory {
            parameter: String::from(
                "Simulated Brightness Temperature for nadir ABI GOES-R, Band-15",
            ),
            units: String::from("K"),
            abbrev: String::from("SBTAGR15"),
        },
        61 => TableCategory {
            parameter: String::from(
                "Simulated Brightness Temperature for nadir ABI GOES-R, Band-16",
            ),
            units: String::from("K"),
            abbrev: String::from("SBTAGR16"),
        },
        62 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for SSMIS-F17, Channel 15"),
            units: String::from("K"),
            abbrev: String::from("SSMS1715"),
        },
        63 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for SSMIS-F17, Channel 16"),
            units: String::from("K"),
            abbrev: String::from("SSMS1716"),
        },
        64 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for SSMIS-F17, Channel 17"),
            units: String::from("K"),
            abbrev: String::from("SSMS1717"),
        },
        65 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for SSMIS-F17, Channel 18"),
            units: String::from("K"),
            abbrev: String::from("SSMS1718"),
        },
        66 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for Himawari-8, Band-7"),
            units: String::from("K"),
            abbrev: String::from("SBTAHI7"),
        },
        67 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for Himawari-8, Band-8"),
            units: String::from("K"),
            abbrev: String::from("SBTAHI8"),
        },
        68 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for Himawari-8, Band-9"),
            units: String::from("K"),
            abbrev: String::from("SBTAHI9"),
        },
        69 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for Himawari-8, Band-10"),
            units: String::from("K"),
            abbrev: String::from("SBTAHI10"),
        },
        70 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for Himawari-8, Band-11"),
            units: String::from("K"),
            abbrev: String::from("SBTAHI11"),
        },
        71 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for Himawari-8, Band-12"),
            units: String::from("K"),
            abbrev: String::from("SBTAHI12"),
        },
        72 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for Himawari-8, Band-13"),
            units: String::from("K"),
            abbrev: String::from("SBTAHI13"),
        },
        73 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for Himawari-8, Band-14"),
            units: String::from("K"),
            abbrev: String::from("SBTAHI14"),
        },
        74 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for Himawari-8, Band-15"),
            units: String::from("K"),
            abbrev: String::from("SBTAHI15"),
        },
        75 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for Himawari-8, Band-16"),
            units: String::from("K"),
            abbrev: String::from("SBTAHI16"),
        },
        76 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-18, Band-7"),
            units: String::from("K"),
            abbrev: String::from("SBTA187"),
        },
        77 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-18, Band-8"),
            units: String::from("K"),
            abbrev: String::from("SBTA188"),
        },
        78 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-18, Band-9"),
            units: String::from("K"),
            abbrev: String::from("SBTA189"),
        },
        79 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-18, Band-10"),
            units: String::from("K"),
            abbrev: String::from("SBTA1810"),
        },
        80 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-18, Band-11"),
            units: String::from("K"),
            abbrev: String::from("SBTA1811"),
        },
        81 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-18, Band-12"),
            units: String::from("K"),
            abbrev: String::from("SBTA1812"),
        },
        82 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-18, Band-13"),
            units: String::from("K"),
            abbrev: String::from("SBTA1813"),
        },
        83 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-18, Band-14"),
            units: String::from("K"),
            abbrev: String::from("SBTA1814"),
        },
        84 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-18, Band-15"),
            units: String::from("K"),
            abbrev: String::from("SBTA1815"),
        },
        85 => TableCategory {
            parameter: String::from("Simulated Brightness Temperature for ABI GOES-18, Band-16"),
            units: String::from("K"),
            abbrev: String::from("SBTA1816"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        86..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-4-0
/// PARAMETERS FOR DISCIPLINE 4, CATEGORY 0
/// **(Space Weather Products, Temperature category)**
///
/// **Details**:
/// - **Discipline**: 4 (Space Weather Products)
/// - **Category**: 0 (Temperature)
/// - **Section**: 4
/// - **Octet 10**: 0
/// - **Created**: 02/27/2012
///
/// **Reserved Ranges**:
/// - `6-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 4, Category 0.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 4, Category 0 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_40(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Temperature"),
            units: String::from("K"),
            abbrev: String::from("TMPSWP"),
        },
        1 => TableCategory {
            parameter: String::from("Electron Temperature"),
            units: String::from("K"),
            abbrev: String::from("ELECTMP"),
        },
        2 => TableCategory {
            parameter: String::from("Proton Temperature"),
            units: String::from("K"),
            abbrev: String::from("PROTTMP"),
        },
        3 => TableCategory {
            parameter: String::from("Ion Temperature"),
            units: String::from("K"),
            abbrev: String::from("IONTMP"),
        },
        4 => TableCategory {
            parameter: String::from("Parallel Temperature"),
            units: String::from("K"),
            abbrev: String::from("PRATMP"),
        },
        5 => TableCategory {
            parameter: String::from("Perpendicular Temperature"),
            units: String::from("K"),
            abbrev: String::from("PRPTMP"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
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

/// # GRIB2 - TABLE 4.2-4-1
/// PARAMETERS FOR DISCIPLINE 4, CATEGORY 1
/// **(Space Weather Products, Momentum category)**
///
/// **Details**:
/// - **Discipline**: 4 (Space Weather Products)
/// - **Category**: 1 (Momentum)
/// - **Section**: 4
/// - **Octet 10**: 1
/// - **Created**: 12/15/2011
///
/// **Reserved Ranges**:
/// - `4-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 4, Category 1.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 4, Category 1 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_41(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Velocity Magnitude (Speed)"),
            units: String::from("m s-1"),
            abbrev: String::from("SPEED"),
        },
        1 => TableCategory {
            parameter: String::from(
                "1st Vector Component of Velocity (Coordinate system dependent)",
            ),
            units: String::from("m s-1"),
            abbrev: String::from("VEL1"),
        },
        2 => TableCategory {
            parameter: String::from(
                "2nd Vector Component of Velocity (Coordinate system dependent)",
            ),
            units: String::from("m s-1"),
            abbrev: String::from("VEL2"),
        },
        3 => TableCategory {
            parameter: String::from(
                "3rd Vector Component of Velocity (Coordinate system dependent)",
            ),
            units: String::from("m s-1"),
            abbrev: String::from("VEL3"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        4..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-4-2
/// PARAMETERS FOR DISCIPLINE 4, CATEGORY 2
/// **(Space Weather Products, Charged Particle Mass and Number category)**
///
/// **Details**:
/// - **Discipline**: 4 (Space Weather Products)
/// - **Category**: 2 (Charged Particle Mass and Number)
/// - **Section**: 4
/// - **Octet 10**: 2
/// - **Revised**: 10/30/2023
///
/// **Reserved Ranges**:
/// - `14-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 4, Category 2.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 4, Category 2 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_42(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Particle Number Density"),
            units: String::from("m-3"),
            abbrev: String::from("PLSMDEN"),
        },
        1 => TableCategory {
            parameter: String::from("Electron Density"),
            units: String::from("m-3"),
            abbrev: String::from("ELCDEN"),
        },
        2 => TableCategory {
            parameter: String::from("Proton Density"),
            units: String::from("m-3"),
            abbrev: String::from("PROTDEN"),
        },
        3 => TableCategory {
            parameter: String::from("Ion Density"),
            units: String::from("m-3"),
            abbrev: String::from("IONDEN"),
        },
        4 => TableCategory {
            parameter: String::from("Vertical Total Electron Content"),
            units: String::from("TECU"),
            abbrev: String::from("VTEC"),
        },
        5 => TableCategory {
            parameter: String::from("HF Absorption Frequency"),
            units: String::from("Hz"),
            abbrev: String::from("ABSFRQ"),
        },
        6 => TableCategory {
            parameter: String::from("HF Absorption"),
            units: String::from("dB"),
            abbrev: String::from("ABSRB"),
        },
        7 => TableCategory {
            parameter: String::from("Spread F"),
            units: String::from("m"),
            abbrev: String::from("SPRDF"),
        },
        8 => TableCategory {
            parameter: String::from("h'F"),
            units: String::from("m"),
            abbrev: String::from("HPRIMF"),
        },
        9 => TableCategory {
            parameter: String::from("Critical Frequency"),
            units: String::from("Hz"),
            abbrev: String::from("CRTFRQ"),
        },
        10 => TableCategory {
            parameter: String::from("Maximal Usable Frequency (MUF)"),
            units: String::from("Hz"),
            abbrev: String::from("MAXUFZ"),
        },
        11 => TableCategory {
            parameter: String::from("Peak Height (hm)"),
            units: String::from("m"),
            abbrev: String::from("PEAKH"),
        },
        12 => TableCategory {
            parameter: String::from("Peak Density"),
            units: String::from("m-3"),
            abbrev: String::from("PEAKDEN"),
        },
        13 => TableCategory {
            parameter: String::from("Equivalent Slab Thickness (tau)"),
            units: String::from("km"),
            abbrev: String::from("EQSLABT"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
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

/// # GRIB2 - TABLE 4.2-4-3
/// PARAMETERS FOR DISCIPLINE 4, CATEGORY 3
/// **(Space Weather Products, Electric and Magnetic Fields category)**
///
/// **Details**:
/// - **Discipline**: 4 (Space Weather Products)
/// - **Category**: 3 (Electric and Magnetic Fields)
/// - **Section**: 4
/// - **Octet 10**: 3
/// - **Created**: 12/15/2011
///
/// **Reserved Ranges**:
/// - `8-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 4, Category 3.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 4, Category 3 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_43(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Magnetic Field Magnitude"),
            units: String::from("T"),
            abbrev: String::from("BTOT"),
        },
        1 => TableCategory {
            parameter: String::from("1st Vector Component of Magnetic Field"),
            units: String::from("T"),
            abbrev: String::from("BVEC1"),
        },
        2 => TableCategory {
            parameter: String::from("2nd Vector Component of Magnetic Field"),
            units: String::from("T"),
            abbrev: String::from("BVEC2"),
        },
        3 => TableCategory {
            parameter: String::from("3rd Vector Component of Magnetic Field"),
            units: String::from("T"),
            abbrev: String::from("BVEC3"),
        },
        4 => TableCategory {
            parameter: String::from("Electric Field Magnitude"),
            units: String::from("V m-1"),
            abbrev: String::from("ETOT"),
        },
        5 => TableCategory {
            parameter: String::from("1st Vector Component of Electric Field"),
            units: String::from("V m-1"),
            abbrev: String::from("EVEC1"),
        },
        6 => TableCategory {
            parameter: String::from("2nd Vector Component of Electric Field"),
            units: String::from("V m-1"),
            abbrev: String::from("EVEC2"),
        },
        7 => TableCategory {
            parameter: String::from("3rd Vector Component of Electric Field"),
            units: String::from("V m-1"),
            abbrev: String::from("EVEC3"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        8..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-4-4
/// PARAMETERS FOR DISCIPLINE 4, CATEGORY 4
/// **(Space Weather Products, Energetic Particles category)**
///
/// **Details**:
/// - **Discipline**: 4 (Space Weather Products)
/// - **Category**: 4 (Energetic Particles)
/// - **Section**: 4
/// - **Octet 10**: 4
/// - **Created**: 12/15/2011
///
/// **Reserved Ranges**:
/// - `7-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 4, Category 4.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 4, Category 4 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_44(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Proton Flux (Differential)"),
            units: String::from("(m2 s sr eV)-1"),
            abbrev: String::from("DIFPFLUX"),
        },
        1 => TableCategory {
            parameter: String::from("Proton Flux (Integral)"),
            units: String::from("(m2 s sr)-1"),
            abbrev: String::from("INTPFLUX"),
        },
        2 => TableCategory {
            parameter: String::from("Electron Flux (Differential)"),
            units: String::from("(m2 s sr eV)-1"),
            abbrev: String::from("DIFEFLUX"),
        },
        3 => TableCategory {
            parameter: String::from("Electron Flux (Integral)"),
            units: String::from("(m2 s sr)-1"),
            abbrev: String::from("INTEFLUX"),
        },
        4 => TableCategory {
            parameter: String::from("Heavy Ion Flux (Differential)"),
            units: String::from("(m2 s sr eV / nuc)-1"),
            abbrev: String::from("DIFIFLUX"),
        },
        5 => TableCategory {
            parameter: String::from("Heavy Ion Flux (Integral)"),
            units: String::from("(m2 s sr)-1"),
            abbrev: String::from("INTIFLUX"),
        },
        6 => TableCategory {
            parameter: String::from("Cosmic Ray Neutron Flux"),
            units: String::from("h-1"),
            abbrev: String::from("NTRNFLUX"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        7..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-4-5
/// PARAMETERS FOR DISCIPLINE 4, CATEGORY 5
/// **(Space Weather Products, Waves category)**
///
/// **Details**:
/// - **Discipline**: 4 (Space Weather Products)
/// - **Category**: 5 (Waves)
/// - **Section**: 4
/// - **Octet 10**: 5
/// - **Revised**: 06/29/2022
///
/// **Reserved Ranges**:
/// - `4-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 4, Category 5.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 4, Category 5 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_45(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Amplitude"),
            units: String::from("rad"),
            abbrev: String::from("AMPL"),
        },
        1 => TableCategory {
            parameter: String::from("Phase"),
            units: String::from("rad"),
            abbrev: String::from("PHASE"),
        },
        2 => TableCategory {
            parameter: String::from("Frequency"),
            units: String::from("Hz"),
            abbrev: String::from("FREQ"),
        },
        3 => TableCategory {
            parameter: String::from("Wavelength"),
            units: String::from("m"),
            abbrev: String::from("WAVELGTH"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        4..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-4-6
/// PARAMETERS FOR DISCIPLINE 4, CATEGORY 6
/// **(Space Weather Products, Solar Electromagnetic Emissions category)**
///
/// **Details**:
/// - **Discipline**: 4 (Space Weather Products)
/// - **Category**: 6 (Solar Electromagnetic Emissions)
/// - **Section**: 4
/// - **Octet 10**: 6
/// - **Created**: 12/15/2011
///
/// **Reserved Ranges**:
/// - `7-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 4, Category 6.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 4, Category 6 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_46(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Integrated Solar Irradiance"),
            units: String::from("W m-2"),
            abbrev: String::from("TSI"),
        },
        1 => TableCategory {
            parameter: String::from("Solar X-ray Flux (XRS Long)"),
            units: String::from("W m-2"),
            abbrev: String::from("XLONG"),
        },
        2 => TableCategory {
            parameter: String::from("Solar X-ray Flux (XRS Short)"),
            units: String::from("W m-2"),
            abbrev: String::from("XSHRT"),
        },
        3 => TableCategory {
            parameter: String::from("Solar EUV Irradiance"),
            units: String::from("W m-2"),
            abbrev: String::from("EUVIRR"),
        },
        4 => TableCategory {
            parameter: String::from("Solar Spectral Irradiance"),
            units: String::from("W m-2 nm-1"),
            abbrev: String::from("SPECIRR"),
        },
        5 => TableCategory {
            parameter: String::from("F10.7"),
            units: String::from("W m-2 Hz-1"),
            abbrev: String::from("F107"),
        },
        6 => TableCategory {
            parameter: String::from("Solar Radio Emissions"),
            units: String::from("W m-2 Hz-1"),
            abbrev: String::from("SOLRF"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        7..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-4-7
/// PARAMETERS FOR DISCIPLINE 4, CATEGORY 7
/// **(Space Weather Products, Terrestrial Electromagnetic Emissions category)**
///
/// **Details**:
/// - **Discipline**: 4 (Space Weather Products)
/// - **Category**: 7 (Terrestrial Electromagnetic Emissions)
/// - **Section**: 4
/// - **Octet 10**: 7
/// - **Created**: 12/15/2011
///
/// **Reserved Ranges**:
/// - `4-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 4, Category 7.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 4, Category 7 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_47(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Limb Intensity"),
            units: String::from("J m-2 s-1"),
            abbrev: String::from("LMBINT"),
        },
        1 => TableCategory {
            parameter: String::from("Disk Intensity"),
            units: String::from("J m-2 s-1"),
            abbrev: String::from("DSKINT"),
        },
        2 => TableCategory {
            parameter: String::from("Disk Intensity Day"),
            units: String::from("J m-2 s-1"),
            abbrev: String::from("DSKDAY"),
        },
        3 => TableCategory {
            parameter: String::from("Disk Intensity Night"),
            units: String::from("J m-2 s-1"),
            abbrev: String::from("DSKNGT"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        4..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-4-8
/// PARAMETERS FOR DISCIPLINE 4, CATEGORY 8
/// **(Space Weather Products, Imagery category)**
///
/// **Details**:
/// - **Discipline**: 4 (Space Weather Products)
/// - **Category**: 8 (Imagery)
/// - **Section**: 4
/// - **Octet 10**: 8
/// - **Revised**: 10/30/2023
///
/// **Reserved Ranges**:
/// - `9-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 4, Category 8.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 4, Category 8 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_48(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("X-Ray Radiance"),
            units: String::from("W sr-1 m-2"),
            abbrev: String::from("XRAYRAD"),
        },
        1 => TableCategory {
            parameter: String::from("EUV Radiance"),
            units: String::from("W sr-1 m-2"),
            abbrev: String::from("EUVRAD"),
        },
        2 => TableCategory {
            parameter: String::from("H-Alpha Radiance"),
            units: String::from("W sr-1 m-2"),
            abbrev: String::from("HARAD"),
        },
        3 => TableCategory {
            parameter: String::from("White Light Radiance"),
            units: String::from("W sr-1 m-2"),
            abbrev: String::from("WHTRAD"),
        },
        4 => TableCategory {
            parameter: String::from("CaII-K Radiance"),
            units: String::from("W sr-1 m-2"),
            abbrev: String::from("CAIIRAD"),
        },
        5 => TableCategory {
            parameter: String::from("White Light Coronagraph Radiance"),
            units: String::from("W sr-1 m-2"),
            abbrev: String::from("WHTCOR"),
        },
        6 => TableCategory {
            parameter: String::from("Heliospheric Radiance"),
            units: String::from("W sr-1 m-2"),
            abbrev: String::from("HELCOR"),
        },
        7 => TableCategory {
            parameter: String::from("Thematic Mask"),
            units: String::from("Numeric"),
            abbrev: String::from("MASK"),
        },
        8 => TableCategory {
            parameter: String::from("Solar Induced Chlorophyll Fluorescence"),
            units: String::from("W sr-1 m-2"),
            abbrev: String::from("SICFL"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        9..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-4-9
/// PARAMETERS FOR DISCIPLINE 4, CATEGORY 9
/// **(Space Weather Products, Ion-Neutral Coupling category)**
///
/// **Details**:
/// - **Discipline**: 4 (Space Weather Products)
/// - **Category**: 9 (Ion-Neutral Coupling)
/// - **Section**: 4
/// - **Octet 10**: 9
/// - **Created**: 12/15/2011
///
/// **Reserved Ranges**:
/// - `3-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 4, Category 9.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 4, Category 9 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_49(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Pedersen Conductivity"),
            units: String::from("S m-1"),
            abbrev: String::from("SIGPED"),
        },
        1 => TableCategory {
            parameter: String::from("Hall Conductivity"),
            units: String::from("S m-1"),
            abbrev: String::from("SIGHAL"),
        },
        2 => TableCategory {
            parameter: String::from("Parallel Conductivity"),
            units: String::from("S m-1"),
            abbrev: String::from("SIGPAR"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
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

/// # GRIB2 - TABLE 4.2-4-10
/// PARAMETERS FOR DISCIPLINE 4, CATEGORY 10
/// **(Space Weather Products, Space Weather Indices Category)**
///
/// **Details**:
/// - **Discipline**: 4 (Space Weather Products)
/// - **Category**: 10 (Space Weather Indices)
/// - **Section**: 4
/// - **Octet 10**: 10
/// - **Created**: 06/29/2022
///
/// **Reserved Ranges**:
/// - `8-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 4, Category 10.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 4, Category 10 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_410(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Scintillation Index (sigma phi)"),
            units: String::from("rad"),
            abbrev: String::from("SCINIDX"),
        },
        1 => TableCategory {
            parameter: String::from("Scintillation Index S4"),
            units: String::from("Numeric"),
            abbrev: String::from("SCIDEXS4"),
        },
        2 => TableCategory {
            parameter: String::from("Rate of Change of TEC Index (ROTI)"),
            units: String::from("TECU/min"),
            abbrev: String::from("ROTIDX"),
        },
        3 => TableCategory {
            parameter: String::from("Disturbance Ionosphere Index Spatial Gradient (DIXSG)"),
            units: String::from("Numeric"),
            abbrev: String::from("DIDXSG"),
        },
        4 => TableCategory {
            parameter: String::from("Along Arc TEC Rate (AATR)"),
            units: String::from("TECU/min"),
            abbrev: String::from("AATRATE"),
        },
        5 => TableCategory {
            parameter: String::from("Kp"),
            units: String::from("Numeric"),
            abbrev: String::from("KP"),
        },
        6 => TableCategory {
            parameter: String::from("Equatorial Disturbance Storm Time Index (Dst)"),
            units: String::from("nT"),
            abbrev: String::from("EDISSTIX"),
        },
        7 => TableCategory {
            parameter: String::from("Auroral Electrojet (AE)"),
            units: String::from("nT"),
            abbrev: String::from("AURELEC"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        8..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-10-0
/// PARAMETERS FOR DISCIPLINE 10, CATEGORY 0
/// **(Oceanographic products, Waves category)**
///
/// **Details**:
/// - **Discipline**: 10 (Oceanographic Products)
/// - **Category**: 0 (Waves)
/// - **Section**: 4
/// - **Octet 10**: 0
/// - **Revised**: 12/07/2023
///
/// **Reserved Ranges**:
/// - `99-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes:
/// 1. Further information concerning the wave parameters can be found in the Guide to Wave Analysis and Forecasting (WMO-No. 702).
/// 2. The Charnock parameter accounts for increased aerodynamic roughness as wave heights grow due to increasing surface stress. It depends on the wind speed, wave age, and other aspects of the sea state.
/// 3. Parameters are normalized by being divided by the product of air density and the square of the friction velocity.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 10, Category 0.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 10, Category 0 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_100(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Wave Spectra (1)"),
            units: String::from("-"),
            abbrev: String::from("WVSP1"),
        },
        1 => TableCategory {
            parameter: String::from("Wave Spectra (2)"),
            units: String::from("-"),
            abbrev: String::from("WVSP2"),
        },
        2 => TableCategory {
            parameter: String::from("Wave Spectra (3)"),
            units: String::from("-"),
            abbrev: String::from("WVSP3"),
        },
        3 => TableCategory {
            parameter: String::from("Significant Height of Combined Wind Waves and Swell"),
            units: String::from("m"),
            abbrev: String::from("HTSGW"),
        },
        4 => TableCategory {
            parameter: String::from("Direction of Wind Waves"),
            units: String::from("degree true"),
            abbrev: String::from("WVDIR"),
        },
        5 => TableCategory {
            parameter: String::from("Significant Height of Wind Waves"),
            units: String::from("m"),
            abbrev: String::from("WVHGT"),
        },
        6 => TableCategory {
            parameter: String::from("Mean Period of Wind Waves"),
            units: String::from("s"),
            abbrev: String::from("WVPER"),
        },
        7 => TableCategory {
            parameter: String::from("Direction of Swell Waves"),
            units: String::from("degree true"),
            abbrev: String::from("SWDIR"),
        },
        8 => TableCategory {
            parameter: String::from("Significant Height of Swell Waves"),
            units: String::from("m"),
            abbrev: String::from("SWELL"),
        },
        9 => TableCategory {
            parameter: String::from("Mean Period of Swell Waves"),
            units: String::from("s"),
            abbrev: String::from("SWPER"),
        },
        10 => TableCategory {
            parameter: String::from("Primary Wave Direction"),
            units: String::from("degree true"),
            abbrev: String::from("DIRPW"),
        },
        11 => TableCategory {
            parameter: String::from("Primary Wave Mean Period"),
            units: String::from("s"),
            abbrev: String::from("PERPW"),
        },
        12 => TableCategory {
            parameter: String::from("Secondary Wave Direction"),
            units: String::from("degree true"),
            abbrev: String::from("DIRSW"),
        },
        13 => TableCategory {
            parameter: String::from("Secondary Wave Mean Period"),
            units: String::from("s"),
            abbrev: String::from("PERSW"),
        },
        14 => TableCategory {
            parameter: String::from("Direction of Combined Wind Waves and Swell"),
            units: String::from("degree true"),
            abbrev: String::from("WWSDIR"),
        },
        15 => TableCategory {
            parameter: String::from("Mean Period of Combined Wind Waves and Swell"),
            units: String::from("s"),
            abbrev: String::from("MWSPER"),
        },
        16 => TableCategory {
            parameter: String::from("Coefficient of Drag With Waves"),
            units: String::from("-"),
            abbrev: String::from("CDWW"),
        },
        17 => TableCategory {
            parameter: String::from("Friction Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("FRICVW"),
        },
        18 => TableCategory {
            parameter: String::from("Wave Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("WSTR"),
        },
        19 => TableCategory {
            parameter: String::from("Normalised Waves Stress"),
            units: String::from("-"),
            abbrev: String::from("NWSTR"),
        },
        20 => TableCategory {
            parameter: String::from("Mean Square Slope of Waves"),
            units: String::from("-"),
            abbrev: String::from("MSSW"),
        },
        21 => TableCategory {
            parameter: String::from("U-component Surface Stokes Drift"),
            units: String::from("m s-1"),
            abbrev: String::from("USSD"),
        },
        22 => TableCategory {
            parameter: String::from("V-component Surface Stokes Drift"),
            units: String::from("m s-1"),
            abbrev: String::from("VSSD"),
        },
        23 => TableCategory {
            parameter: String::from("Period of Maximum Individual Wave Height"),
            units: String::from("s"),
            abbrev: String::from("PMAXWH"),
        },
        24 => TableCategory {
            parameter: String::from("Maximum Individual Wave Height"),
            units: String::from("m"),
            abbrev: String::from("MAXWH"),
        },
        25 => TableCategory {
            parameter: String::from("Inverse Mean Wave Frequency"),
            units: String::from("s"),
            abbrev: String::from("IMWF"),
        },
        26 => TableCategory {
            parameter: String::from("Inverse Mean Frequency of The Wind Waves"),
            units: String::from("s"),
            abbrev: String::from("IMFWW"),
        },
        27 => TableCategory {
            parameter: String::from("Inverse Mean Frequency of The Total Swell"),
            units: String::from("s"),
            abbrev: String::from("IMFTSW"),
        },
        28 => TableCategory {
            parameter: String::from("Mean Zero-Crossing Wave Period"),
            units: String::from("s"),
            abbrev: String::from("MZWPER"),
        },
        29 => TableCategory {
            parameter: String::from("Mean Zero-Crossing Period of The Wind Waves"),
            units: String::from("s"),
            abbrev: String::from("MZPWW"),
        },
        30 => TableCategory {
            parameter: String::from("Mean Zero-Crossing Period of The Total Swell"),
            units: String::from("s"),
            abbrev: String::from("MZPTSW"),
        },
        31 => TableCategory {
            parameter: String::from("Wave Directional Width"),
            units: String::from("-"),
            abbrev: String::from("WDIRW"),
        },
        32 => TableCategory {
            parameter: String::from("Directional Width of The Wind Waves"),
            units: String::from("-"),
            abbrev: String::from("DIRWWW"),
        },
        33 => TableCategory {
            parameter: String::from("Directional Width of The Total Swell"),
            units: String::from("-"),
            abbrev: String::from("DIRWTS"),
        },
        34 => TableCategory {
            parameter: String::from("Peak Wave Period"),
            units: String::from("s"),
            abbrev: String::from("PWPER"),
        },
        35 => TableCategory {
            parameter: String::from("Peak Period of The Wind Waves"),
            units: String::from("s"),
            abbrev: String::from("PPERWW"),
        },
        36 => TableCategory {
            parameter: String::from("Peak Period of The Total Swell"),
            units: String::from("s"),
            abbrev: String::from("PPERTS"),
        },
        37 => TableCategory {
            parameter: String::from("Altimeter Wave Height"),
            units: String::from("m"),
            abbrev: String::from("ALTWH"),
        },
        38 => TableCategory {
            parameter: String::from("Altimeter Corrected Wave Height"),
            units: String::from("m"),
            abbrev: String::from("ALCWH"),
        },
        39 => TableCategory {
            parameter: String::from("Altimeter Range Relative Correction"),
            units: String::from("-"),
            abbrev: String::from("ALRRC"),
        },
        40 => TableCategory {
            parameter: String::from("10 Metre Neutral Wind Speed Over Waves"),
            units: String::from("m s-1"),
            abbrev: String::from("MNWSOW"),
        },
        41 => TableCategory {
            parameter: String::from("10 Metre Wind Direction Over Waves"),
            units: String::from("degree true"),
            abbrev: String::from("MWDIRW"),
        },
        42 => TableCategory {
            parameter: String::from("Wave Engery Spectrum"),
            units: String::from("m-2 s rad-1"),
            abbrev: String::from("WESP"),
        },
        43 => TableCategory {
            parameter: String::from("Kurtosis of The Sea Surface Elevation Due to Waves"),
            units: String::from("-"),
            abbrev: String::from("KSSEW"),
        },
        44 => TableCategory {
            parameter: String::from("Benjamin-Feir Index"),
            units: String::from("-"),
            abbrev: String::from("BENINX"),
        },
        45 => TableCategory {
            parameter: String::from("Spectral Peakedness Factor"),
            units: String::from("s-1"),
            abbrev: String::from("SPFTR"),
        },
        46 => TableCategory {
            parameter: String::from("Peak wave direction"),
            units: String::from("°"),
            abbrev: String::from("PWAVEDIR"),
        },
        47 => TableCategory {
            parameter: String::from("Significant wave height of first swell partition"),
            units: String::from("m"),
            abbrev: String::from("SWHFSWEL"),
        },
        48 => TableCategory {
            parameter: String::from("Significant wave height of second swell partition"),
            units: String::from("m"),
            abbrev: String::from("SWHSSWEL"),
        },
        49 => TableCategory {
            parameter: String::from("Significant wave height of third swell partition"),
            units: String::from("m"),
            abbrev: String::from("SWHTSWEL"),
        },
        50 => TableCategory {
            parameter: String::from("Mean wave period of first swell partition"),
            units: String::from("s"),
            abbrev: String::from("MWPFSWEL"),
        },
        51 => TableCategory {
            parameter: String::from("Mean wave period of second swell partition"),
            units: String::from("s"),
            abbrev: String::from("MWPSSWEL"),
        },
        52 => TableCategory {
            parameter: String::from("Mean wave period of third swell partition"),
            units: String::from("s"),
            abbrev: String::from("MWPTSWEL"),
        },
        53 => TableCategory {
            parameter: String::from("Mean wave direction of first swell partition"),
            units: String::from("°"),
            abbrev: String::from("MWDFSWEL"),
        },
        54 => TableCategory {
            parameter: String::from("Mean wave direction of second swell partition"),
            units: String::from("°"),
            abbrev: String::from("MWDSSWEL"),
        },
        55 => TableCategory {
            parameter: String::from("Mean wave direction of third swell partition"),
            units: String::from("°"),
            abbrev: String::from("MWDTSWEL"),
        },
        56 => TableCategory {
            parameter: String::from("Wave directional width of first swell partition"),
            units: String::from("-"),
            abbrev: String::from("WDWFSWEL"),
        },
        57 => TableCategory {
            parameter: String::from("Wave directional width of second swell partition"),
            units: String::from("-"),
            abbrev: String::from("WDWSSWEL"),
        },
        58 => TableCategory {
            parameter: String::from("Wave directional width of third swell partition"),
            units: String::from("-"),
            abbrev: String::from("WDWTSWEL"),
        },
        59 => TableCategory {
            parameter: String::from("Wave frequency width of first swell partition"),
            units: String::from("-"),
            abbrev: String::from("WFWFSWEL"),
        },
        60 => TableCategory {
            parameter: String::from("Wave frequency width of second swell partition"),
            units: String::from("-"),
            abbrev: String::from("WFWSSWEL"),
        },
        61 => TableCategory {
            parameter: String::from("Wave frequency width of third swell partition"),
            units: String::from("-"),
            abbrev: String::from("WFWTSWEL"),
        },
        62 => TableCategory {
            parameter: String::from("Wave frequency width"),
            units: String::from("-"),
            abbrev: String::from("WAVEFREW"),
        },
        63 => TableCategory {
            parameter: String::from("Frequency width of wind waves"),
            units: String::from("-"),
            abbrev: String::from("FREWWW"),
        },
        64 => TableCategory {
            parameter: String::from("Frequency width of total swell"),
            units: String::from("-"),
            abbrev: String::from("FREWTSW"),
        },
        65 => TableCategory {
            parameter: String::from("Peak Wave Period of First Swell Partition"),
            units: String::from("s"),
            abbrev: String::from("PWPFSPAR"),
        },
        66 => TableCategory {
            parameter: String::from("Peak Wave Period of Second Swell Partition"),
            units: String::from("s"),
            abbrev: String::from("PWPSSPAR"),
        },
        67 => TableCategory {
            parameter: String::from("Peak Wave Period of Third Swell Partition"),
            units: String::from("s"),
            abbrev: String::from("PWPTSPAR"),
        },
        68 => TableCategory {
            parameter: String::from("Peak Wave Direction of First Swell Partition"),
            units: String::from("degree true"),
            abbrev: String::from("PWDFSPAR"),
        },
        69 => TableCategory {
            parameter: String::from("Peak Wave Direction of Second Swell Partition"),
            units: String::from("degree true"),
            abbrev: String::from("PWDSSPAR"),
        },
        70 => TableCategory {
            parameter: String::from("Peak Wave Direction of Third Swell Partition"),
            units: String::from("degree true"),
            abbrev: String::from("PWDTSPAR"),
        },
        71 => TableCategory {
            parameter: String::from("Peak Direction of Wind Waves"),
            units: String::from("degree true"),
            abbrev: String::from("PDWWAVE"),
        },
        72 => TableCategory {
            parameter: String::from("Peak Direction of Total Swell"),
            units: String::from("degree true"),
            abbrev: String::from("PDTSWELL"),
        },
        73 => TableCategory {
            parameter: String::from("Whitecap Fraction"),
            units: String::from("fraction"),
            abbrev: String::from("WCAPFRAC"),
        },
        74 => TableCategory {
            parameter: String::from("Mean Direction of Total Swell"),
            units: String::from("degree"),
            abbrev: String::from("MDTSWEL"),
        },
        75 => TableCategory {
            parameter: String::from("Mean Direction of Wind Waves"),
            units: String::from("degree"),
            abbrev: String::from("MDWWAVE"),
        },
        76 => TableCategory {
            parameter: String::from("Charnock (see Note 2)"),
            units: String::from("Numeric"),
            abbrev: String::from("CHNCK"),
        },
        77 => TableCategory {
            parameter: String::from("Wave Spectral Skewness"),
            units: String::from("Numeric"),
            abbrev: String::from("WAVESPSK"),
        },
        78 => TableCategory {
            parameter: String::from("Wave Energy Flux Magnitude"),
            units: String::from("W m-1"),
            abbrev: String::from("WAVEFMAG"),
        },
        79 => TableCategory {
            parameter: String::from("Wave Energy Flux Mean Direction"),
            units: String::from("degree true"),
            abbrev: String::from("WAVEFDIR"),
        },
        80 => TableCategory {
            parameter: String::from("Ratio of Wave Angular and Frequency Width"),
            units: String::from("Numeric"),
            abbrev: String::from("RWAVEAFW"),
        },
        81 => TableCategory {
            parameter: String::from("Free Convective Velocity over the Oceans"),
            units: String::from("m s-1"),
            abbrev: String::from("FCVOCEAN"),
        },
        82 => TableCategory {
            parameter: String::from("Air Density over the Oceans"),
            units: String::from("kg m-3"),
            abbrev: String::from("AIRDENOC"),
        },
        83 => TableCategory {
            parameter: String::from("Normalized Energy Flux into Waves (see Note 3)"),
            units: String::from("Numeric"),
            abbrev: String::from("NEFW"),
        },
        84 => TableCategory {
            parameter: String::from("Normalized Stress into Ocean (see Note 3)"),
            units: String::from("Numeric"),
            abbrev: String::from("NSOCEAN"),
        },
        85 => TableCategory {
            parameter: String::from("Normalized Energy Flux into Ocean (see Note 3)"),
            units: String::from("Numeric"),
            abbrev: String::from("NEFOCEAN"),
        },
        86 => TableCategory {
            parameter: String::from(
                "Surface Elevation Variance due to Waves (over all frequencies and directions)",
            ),
            units: String::from("m2 s rad-1"),
            abbrev: String::from("SEVWAVE"),
        },
        87 => TableCategory {
            parameter: String::from("Wave Induced Mean Sea Level Correction"),
            units: String::from("m"),
            abbrev: String::from("WAVEMSLC"),
        },
        88 => TableCategory {
            parameter: String::from("Spectral Width Index"),
            units: String::from("Numeric"),
            abbrev: String::from("SPECWI"),
        },
        89 => TableCategory {
            parameter: String::from("Number of Events in Freak Wave Statistics"),
            units: String::from("Numeric"),
            abbrev: String::from("EFWS"),
        },
        90 => TableCategory {
            parameter: String::from("U-Component of Surface Momentum Flux into Ocean"),
            units: String::from("N m-2"),
            abbrev: String::from("USMFO"),
        },
        91 => TableCategory {
            parameter: String::from("V-Component of Surface Momentum Flux into Ocean"),
            units: String::from("N m-2"),
            abbrev: String::from("VSMFO"),
        },
        92 => TableCategory {
            parameter: String::from("Wave Turbulent Energy Flux into Ocean"),
            units: String::from("W m-2"),
            abbrev: String::from("WAVETEFO"),
        },
        93 => TableCategory {
            parameter: String::from("Envelope Maximum Individual Wave Height"),
            units: String::from("m"),
            abbrev: String::from("EMIWAVE"),
        },
        94 => TableCategory {
            parameter: String::from("Time Domain Maximum Individual Crest Height"),
            units: String::from("m"),
            abbrev: String::from("TDMCREST"),
        },
        95 => TableCategory {
            parameter: String::from("Time Domain Maximum Individual Wave Height"),
            units: String::from("m"),
            abbrev: String::from("TDMWAVE"),
        },
        96 => TableCategory {
            parameter: String::from("Space Time Maximum Individual Crest Height"),
            units: String::from("m"),
            abbrev: String::from("STMCREST"),
        },
        97 => TableCategory {
            parameter: String::from("Space Time Maximum Individual Wave Height"),
            units: String::from("m"),
            abbrev: String::from("STMWAVE"),
        },
        98 => TableCategory {
            parameter: String::from("Goda Peakedness Factor"),
            units: String::from("Numeric"),
            abbrev: String::from("GODAPEAK"),
        },
        192 => TableCategory {
            parameter: String::from("Wave Steepness"),
            units: String::from("proportion"),
            abbrev: String::from("WSTP"),
        },
        193 => TableCategory {
            parameter: String::from("Wave Length"),
            units: String::from("-"),
            abbrev: String::from("WLENG"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        99..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-10-1
/// PARAMETERS FOR DISCIPLINE 10, CATEGORY 1
/// **(Oceanographic products, Currents category)**
///
/// **Details**:
/// - **Discipline**: 10 (Oceanographic Products)
/// - **Category**: 1 (Currents)
/// - **Section**: 4
/// - **Octet 10**: 1
/// - **Revised**: 10/23/2023
///
/// **Reserved Ranges**:
/// - `7-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 10, Category 1.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 10, Category 1 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_101(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Current Direction"),
            units: String::from("degree True"),
            abbrev: String::from("DIRC"),
        },
        1 => TableCategory {
            parameter: String::from("Current Speed"),
            units: String::from("m s-1"),
            abbrev: String::from("SPC"),
        },
        2 => TableCategory {
            parameter: String::from("U-Component of Current"),
            units: String::from("m s-1"),
            abbrev: String::from("UOGRD"),
        },
        3 => TableCategory {
            parameter: String::from("V-Component of Current"),
            units: String::from("m s-1"),
            abbrev: String::from("VOGRD"),
        },
        4 => TableCategory {
            parameter: String::from("Rip Current Occurrence Probability"),
            units: String::from("%"),
            abbrev: String::from("RIPCOP"),
        },
        5 => TableCategory {
            parameter: String::from("Eastward Current"),
            units: String::from("m s-1"),
            abbrev: String::from("EASTCUR"),
        },
        6 => TableCategory {
            parameter: String::from("Northward Current"),
            units: String::from("m s-1"),
            abbrev: String::from("NRTHCUR"),
        },
        192 => TableCategory {
            parameter: String::from("Ocean Mixed Layer U Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("OMLU"),
        },
        193 => TableCategory {
            parameter: String::from("Ocean Mixed Layer V Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("OMLV"),
        },
        194 => TableCategory {
            parameter: String::from("Barotropic U velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("UBARO"),
        },
        195 => TableCategory {
            parameter: String::from("Barotropic V velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("VBARO"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        7..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-10-2
/// PARAMETERS FOR DISCIPLINE 10, CATEGORY 2
/// **(Oceanographic products, Ice category)**
///
/// **Details**:
/// - **Discipline**: 10 (Oceanographic Products)
/// - **Category**: 2 (Ice)
/// - **Section**: 4
/// - **Octet 10**: 2
/// - **Revised**: 12/07/2023
///
/// **Reserved Ranges**:
/// - `26`: Reserved
/// - `30-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes:
/// 1. Ice internal pressure or stress (Pa m) is the integrated pressure across the vertical thickness of a layer of ice. It is produced when concentrated ice reacts to external forces such as wind and ocean currents.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 10, Category 2.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 10, Category 2 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_102(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Ice Cover"),
            units: String::from("Proportion"),
            abbrev: String::from("ICEC"),
        },
        1 => TableCategory {
            parameter: String::from("Ice Thickness"),
            units: String::from("m"),
            abbrev: String::from("ICETK"),
        },
        2 => TableCategory {
            parameter: String::from("Direction of Ice Drift"),
            units: String::from("degree True"),
            abbrev: String::from("DICED"),
        },
        3 => TableCategory {
            parameter: String::from("Speed of Ice Drift"),
            units: String::from("m s-1"),
            abbrev: String::from("SICED"),
        },
        4 => TableCategory {
            parameter: String::from("U-Component of Ice Drift"),
            units: String::from("m s-1"),
            abbrev: String::from("UICE"),
        },
        5 => TableCategory {
            parameter: String::from("V-Component of Ice Drift"),
            units: String::from("m s-1"),
            abbrev: String::from("VICE"),
        },
        6 => TableCategory {
            parameter: String::from("Ice Growth Rate"),
            units: String::from("m s-1"),
            abbrev: String::from("ICEG"),
        },
        7 => TableCategory {
            parameter: String::from("Ice Divergence"),
            units: String::from("s-1"),
            abbrev: String::from("ICED"),
        },
        8 => TableCategory {
            parameter: String::from("Ice Temperature"),
            units: String::from("K"),
            abbrev: String::from("ICETMP"),
        },
        9 => TableCategory {
            parameter: String::from("Module of Ice Internal Pressure"),
            units: String::from("Pa m"),
            abbrev: String::from("ICEPRS"),
        },
        10 => TableCategory {
            parameter: String::from(
                "Zonal Vector Component of Vertically Integrated Ice Internal Pressure",
            ),
            units: String::from("Pa m"),
            abbrev: String::from("ZVCICEP"),
        },
        11 => TableCategory {
            parameter: String::from(
                "Meridional Vector Component of Vertically Integrated Ice Internal Pressure",
            ),
            units: String::from("Pa m"),
            abbrev: String::from("MVCICEP"),
        },
        12 => TableCategory {
            parameter: String::from("Compressive Ice Strength"),
            units: String::from("N m-1"),
            abbrev: String::from("CICES"),
        },
        13 => TableCategory {
            parameter: String::from("Snow Temperature (over sea ice)"),
            units: String::from("K"),
            abbrev: String::from("SNOWTSI"),
        },
        14 => TableCategory {
            parameter: String::from("Albedo"),
            units: String::from("Numeric"),
            abbrev: String::from("ALBDOICE"),
        },
        15 => TableCategory {
            parameter: String::from("Sea Ice Volume per Unit Area"),
            units: String::from("m3m-2"),
            abbrev: String::from("SICEVOL"),
        },
        16 => TableCategory {
            parameter: String::from("Snow Volume Over Sea Ice per Unit Area"),
            units: String::from("m3m-2"),
            abbrev: String::from("SNVOLSI"),
        },
        17 => TableCategory {
            parameter: String::from("Sea Ice Heat Content"),
            units: String::from("J m-2"),
            abbrev: String::from("SICEHC"),
        },
        18 => TableCategory {
            parameter: String::from("Snow over Sea Ice Heat Content"),
            units: String::from("J m-2"),
            abbrev: String::from("SNCEHC"),
        },
        19 => TableCategory {
            parameter: String::from("Ice Freeboard Thickness"),
            units: String::from("m"),
            abbrev: String::from("ICEFTHCK"),
        },
        20 => TableCategory {
            parameter: String::from("Ice Melt Pond Fraction"),
            units: String::from("fraction"),
            abbrev: String::from("ICEMPF"),
        },
        21 => TableCategory {
            parameter: String::from("Ice Melt Pond Depth"),
            units: String::from("m"),
            abbrev: String::from("ICEMPD"),
        },
        22 => TableCategory {
            parameter: String::from("Ice Melt Pond Volume per Unit Area"),
            units: String::from("m3m-2"),
            abbrev: String::from("ICEMPV"),
        },
        23 => TableCategory {
            parameter: String::from("Sea Ice Fraction Tendency due to Parameterization"),
            units: String::from("s-1"),
            abbrev: String::from("SIFTP"),
        },
        24 => TableCategory {
            parameter: String::from("x-component of ice drift"),
            units: String::from("m s-1"),
            abbrev: String::from("XICE"),
        },
        25 => TableCategory {
            parameter: String::from("y-component of ice drift"),
            units: String::from("m s-1"),
            abbrev: String::from("YICE"),
        },
        27 => TableCategory {
            parameter: String::from("Freezing/melting potential (Tentatively accepted)"),
            units: String::from("W m-2"),
            abbrev: String::from("FRZMLTPOT"),
        },
        28 => TableCategory {
            parameter: String::from("Melt onset date (Tentatively accepted)"),
            units: String::from("Numeric"),
            abbrev: String::from("MLTDATE"),
        },
        29 => TableCategory {
            parameter: String::from("Freeze onset date (Tentatively accepted)"),
            units: String::from("Numeric"),
            abbrev: String::from("FRZDATE"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        26 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        30..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-10-3
/// PARAMETERS FOR DISCIPLINE 10, CATEGORY 3
/// **(Oceanographic products, Surface Properties category)**
///
/// **Details**:
/// - **Discipline**: 10 (Oceanographic Products)
/// - **Category**: 3 (Surface Properties)
/// - **Section**: 4
/// - **Octet 10**: 3
/// - **Revised**: 10/30/2023
///
/// **Reserved Ranges**:
/// - `22-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes:
/// 1. The x- and y- components of surface stress are not necessarily equivalent to the u- and v- components (eastward/northward).
///    The x- and y- components strictly follow the defined coordinate system which may or may not follow the eastward and northward directions.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 10, Category 3.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 10, Category 3 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_103(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Water Temperature"),
            units: String::from("K"),
            abbrev: String::from("WTMP"),
        },
        1 => TableCategory {
            parameter: String::from("Deviation of Sea Level from Mean"),
            units: String::from("m"),
            abbrev: String::from("DSLM"),
        },
        2 => TableCategory {
            parameter: String::from("Heat Exchange Coefficient"),
            units: String::from(""),
            abbrev: String::from("CH"),
        },
        3 => TableCategory {
            parameter: String::from("Practical Salinity"),
            units: String::from("Numeric"),
            abbrev: String::from("PRACTSAL"),
        },
        4 => TableCategory {
            parameter: String::from("Downward Heat Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("DWHFLUX"),
        },
        5 => TableCategory {
            parameter: String::from("Eastward Surface Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("EASTWSS"),
        },
        6 => TableCategory {
            parameter: String::from("Northward Surface Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("NORTHWSS"),
        },
        7 => TableCategory {
            parameter: String::from("x-component Surface Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("XCOMPSS"),
        },
        8 => TableCategory {
            parameter: String::from("y-component Surface Stress"),
            units: String::from("N m-2"),
            abbrev: String::from("YCOMPSS"),
        },
        9 => TableCategory {
            parameter: String::from("Thermosteric Change in Sea Surface Height"),
            units: String::from("m"),
            abbrev: String::from("THERCSSH"),
        },
        10 => TableCategory {
            parameter: String::from("Halosteric Change in Sea Surface Height"),
            units: String::from("m"),
            abbrev: String::from("HALOCSSH"),
        },
        11 => TableCategory {
            parameter: String::from("Steric Change in Sea Surface Height"),
            units: String::from("m"),
            abbrev: String::from("STERCSSH"),
        },
        12 => TableCategory {
            parameter: String::from("Sea Salt Flux"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("SEASFLUX"),
        },
        13 => TableCategory {
            parameter: String::from("Net Upward Water Flux"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("NETUPWFLUX"),
        },
        14 => TableCategory {
            parameter: String::from("Eastward Surface Water Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("ESURFWVEL"),
        },
        15 => TableCategory {
            parameter: String::from("Northward Surface Water Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("NSURFWVEL"),
        },
        16 => TableCategory {
            parameter: String::from("x-component of Surface Water Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("XSURFWVEL"),
        },
        17 => TableCategory {
            parameter: String::from("y-component of Surface Water Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("YSURFWVEL"),
        },
        18 => TableCategory {
            parameter: String::from("Heat Flux Correction"),
            units: String::from("W m-2"),
            abbrev: String::from("HFLUXCOR"),
        },
        19 => TableCategory {
            parameter: String::from("Sea Surface Height Tendency due to Parameterization"),
            units: String::from("m s-1"),
            abbrev: String::from("SSHGTPARM"),
        },
        20 => TableCategory {
            parameter: String::from(
                "Deviation of Sea Level from Mean with Inverse Barometer Correction",
            ),
            units: String::from("m"),
            abbrev: String::from("DSLIBARCOR"),
        },
        21 => TableCategory {
            parameter: String::from("Salinity"),
            units: String::from("kg kg-1"),
            abbrev: String::from("SALINITY"),
        },
        192 => TableCategory {
            parameter: String::from("Hurricane Storm Surge"),
            units: String::from("m"),
            abbrev: String::from("SURGE"),
        },
        193 => TableCategory {
            parameter: String::from("Extra Tropical Storm Surge"),
            units: String::from("m"),
            abbrev: String::from("ETSRG"),
        },
        194 => TableCategory {
            parameter: String::from("Ocean Surface Elevation Relative to Geoid"),
            units: String::from("m"),
            abbrev: String::from("ELEV"),
        },
        195 => TableCategory {
            parameter: String::from("Sea Surface Height Relative to Geoid"),
            units: String::from("m"),
            abbrev: String::from("SSHG"),
        },
        196 => TableCategory {
            parameter: String::from("Ocean Mixed Layer Potential Density (Reference 2000m)"),
            units: String::from("kg m-3"),
            abbrev: String::from("P2OMLT"),
        },
        197 => TableCategory {
            parameter: String::from("Net Air-Ocean Heat Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("AOHFLX"),
        },
        198 => TableCategory {
            parameter: String::from("Assimilative Heat Flux"),
            units: String::from("W m-2"),
            abbrev: String::from("ASHFL"),
        },
        199 => TableCategory {
            parameter: String::from("Surface Temperature Trend"),
            units: String::from("degree per day"),
            abbrev: String::from("SSTT"),
        },
        200 => TableCategory {
            parameter: String::from("Surface Salinity Trend"),
            units: String::from("psu per day"),
            abbrev: String::from("SSST"),
        },
        201 => TableCategory {
            parameter: String::from("Kinetic Energy"),
            units: String::from("J kg-1"),
            abbrev: String::from("KENG"),
        },
        202 => TableCategory {
            parameter: String::from("Salt Flux"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("SLTFL"),
        },
        203 => TableCategory {
            parameter: String::from("Heat Exchange Coefficient"),
            units: String::from(""),
            abbrev: String::from("LCH"),
        },
        204 => TableCategory {
            parameter: String::from("Freezing Spray"),
            units: String::from(""),
            abbrev: String::from("FRZSPR"),
        },
        205 => TableCategory {
            parameter: String::from("Total Water Level Accounting for Tide, Wind and Waves"),
            units: String::from("m"),
            abbrev: String::from("TWLWAV"),
        },
        206 => TableCategory {
            parameter: String::from("Total Water Level Increase due to Waves"),
            units: String::from("m"),
            abbrev: String::from("RUNUP"),
        },
        207 => TableCategory {
            parameter: String::from("Mean Increase in Water Level due to Waves"),
            units: String::from("m"),
            abbrev: String::from("SETUP"),
        },
        208 => TableCategory {
            parameter: String::from("Time-varying Increase in Water Level due to Waves"),
            units: String::from("m"),
            abbrev: String::from("SWASH"),
        },
        209 => TableCategory {
            parameter: String::from("Total Water Level Above Dune Toe"),
            units: String::from("m"),
            abbrev: String::from("TWLDT"),
        },
        210 => TableCategory {
            parameter: String::from("Total Water Level Above Dune Crest"),
            units: String::from("m"),
            abbrev: String::from("TWLDC"),
        },
        242 => TableCategory {
            parameter: String::from("20% Tropical Cyclone Storm Surge Exceedance"),
            units: String::from("m"),
            abbrev: String::from("TCSRG20"),
        },
        243 => TableCategory {
            parameter: String::from("30% Tropical Cyclone Storm Surge Exceedance"),
            units: String::from("m"),
            abbrev: String::from("TCSRG30"),
        },
        244 => TableCategory {
            parameter: String::from("40% Tropical Cyclone Storm Surge Exceedance"),
            units: String::from("m"),
            abbrev: String::from("TCSRG40"),
        },
        245 => TableCategory {
            parameter: String::from("50% Tropical Cyclone Storm Surge Exceedance"),
            units: String::from("m"),
            abbrev: String::from("TCSRG50"),
        },
        246 => TableCategory {
            parameter: String::from("60% Tropical Cyclone Storm Surge Exceedance"),
            units: String::from("m"),
            abbrev: String::from("TCSRG60"),
        },
        247 => TableCategory {
            parameter: String::from("70% Tropical Cyclone Storm Surge Exceedance"),
            units: String::from("m"),
            abbrev: String::from("TCSRG70"),
        },
        248 => TableCategory {
            parameter: String::from("80% Tropical Cyclone Storm Surge Exceedance"),
            units: String::from("m"),
            abbrev: String::from("TCSRG80"),
        },
        249 => TableCategory {
            parameter: String::from("90% Tropical Cyclone Storm Surge Exceedance"),
            units: String::from("m"),
            abbrev: String::from("TCSRG90"),
        },
        250 => TableCategory {
            parameter: String::from("Extra Tropical Storm Surge Combined Surge and Tide"),
            units: String::from("m"),
            abbrev: String::from("ETCWL"),
        },
        251 => TableCategory {
            parameter: String::from("Tide"),
            units: String::from("m"),
            abbrev: String::from("TIDE"),
        },
        252 => TableCategory {
            parameter: String::from("Erosion Occurrence Probability"),
            units: String::from("%"),
            abbrev: String::from("EROSNP"),
        },
        253 => TableCategory {
            parameter: String::from("Overwash Occurrence Probability"),
            units: String::from("%"),
            abbrev: String::from("OWASHP"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        22..=191 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        211..=241 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        254 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
    }
}

/// # GRIB2 - TABLE 4.2-10-4
/// PARAMETERS FOR DISCIPLINE 10, CATEGORY 4
/// **(Oceanographic products, Sub-Surface Properties category)**
///
/// **Details**:
/// - **Discipline**: 10 (Oceanographic Products)
/// - **Category**: 4 (Sub-Surface Properties)
/// - **Section**: 4
/// - **Octet 10**: 4
/// - **Revised**: 12/07/2023
///
/// **Reserved Ranges**:
/// - `8-10`: Reserved
/// - `52-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes:
/// 1. Numbers 17 and 20 are deviations from the reference value of 1000 kg m–3.
/// 2. The x- and y- components of water velocity are not necessarily equivalent to the u- and v- components (eastward/northward).
///    The x- and y- components strictly follow the defined coordinate system which may or may not follow the eastward and northward directions.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 10, Category 4.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 10, Category 4 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_104(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Main Thermocline Depth"),
            units: String::from("m"),
            abbrev: String::from("MTHD"),
        },
        1 => TableCategory {
            parameter: String::from("Main Thermocline Anomaly"),
            units: String::from("m"),
            abbrev: String::from("MTHA"),
        },
        2 => TableCategory {
            parameter: String::from("Transient Thermocline Depth"),
            units: String::from("m"),
            abbrev: String::from("TTHDP"),
        },
        3 => TableCategory {
            parameter: String::from("Salinity"),
            units: String::from("kg kg-1"),
            abbrev: String::from("SALTY"),
        },
        4 => TableCategory {
            parameter: String::from("Ocean Vertical Heat Diffusivity"),
            units: String::from("m2 s-1"),
            abbrev: String::from("OVHD"),
        },
        5 => TableCategory {
            parameter: String::from("Ocean Vertical Salt Diffusivity"),
            units: String::from("m2 s-1"),
            abbrev: String::from("OVSD"),
        },
        6 => TableCategory {
            parameter: String::from("Ocean Vertical Momentum Diffusivity"),
            units: String::from("m2 s-1"),
            abbrev: String::from("OVMD"),
        },
        7 => TableCategory {
            parameter: String::from("Bathymetry"),
            units: String::from("m"),
            abbrev: String::from("BATHY"),
        },
        11 => TableCategory {
            parameter: String::from("Shape Factor With Respect To Salinity Profile"),
            units: String::from(""),
            abbrev: String::from("SFSALP"),
        },
        12 => TableCategory {
            parameter: String::from(
                "Shape Factor With Respect To Temperature Profile In Thermocline",
            ),
            units: String::from(""),
            abbrev: String::from("SFTMPP"),
        },
        13 => TableCategory {
            parameter: String::from(
                "Attenuation Coefficient Of Water With Respect to Solar Radiation",
            ),
            units: String::from("m-1"),
            abbrev: String::from("ACWSRD"),
        },
        14 => TableCategory {
            parameter: String::from("Water Depth"),
            units: String::from("m"),
            abbrev: String::from("WDEPTH"),
        },
        15 => TableCategory {
            parameter: String::from("Water Temperature"),
            units: String::from("K"),
            abbrev: String::from("WTMPSS"),
        },
        16 => TableCategory {
            parameter: String::from("Water Density (rho)"),
            units: String::from("kg m-3"),
            abbrev: String::from("WATERDEN"),
        },
        17 => TableCategory {
            parameter: String::from("Water Density Anomaly (sigma)"),
            units: String::from("kg m-3"),
            abbrev: String::from("WATDENA"),
        },
        18 => TableCategory {
            parameter: String::from("Water Potential Temperature (theta)"),
            units: String::from("K"),
            abbrev: String::from("WATPTEMP"),
        },
        19 => TableCategory {
            parameter: String::from("Water Potential Density (rho theta)"),
            units: String::from("kg m-3"),
            abbrev: String::from("WATPDEN"),
        },
        20 => TableCategory {
            parameter: String::from("Water Potential Density Anomaly (sigma theta)"),
            units: String::from("kg m-3"),
            abbrev: String::from("WATPDENA"),
        },
        21 => TableCategory {
            parameter: String::from("Practical Salinity"),
            units: String::from("psu (numeric)"),
            abbrev: String::from("PRTSAL"),
        },
        22 => TableCategory {
            parameter: String::from("Water Column-integrated Heat Content"),
            units: String::from("J m-2"),
            abbrev: String::from("WCHEATC"),
        },
        23 => TableCategory {
            parameter: String::from("Eastward Water Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("EASTWVEL"),
        },
        24 => TableCategory {
            parameter: String::from("Northward Water Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("NRTHWVEL"),
        },
        25 => TableCategory {
            parameter: String::from("X-Component Water Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("XCOMPWV"),
        },
        26 => TableCategory {
            parameter: String::from("Y-Component Water Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("YCOMPWV"),
        },
        27 => TableCategory {
            parameter: String::from("Upward Water Velocity"),
            units: String::from("m s-1"),
            abbrev: String::from("UPWWVEL"),
        },
        28 => TableCategory {
            parameter: String::from("Vertical Eddy Diffusivity"),
            units: String::from("m2 s-1"),
            abbrev: String::from("VEDDYDIF"),
        },
        29 => TableCategory {
            parameter: String::from("Bottom Pressure Equivalent Height"),
            units: String::from("m"),
            abbrev: String::from("BPEH"),
        },
        30 => TableCategory {
            parameter: String::from("Fresh Water Flux into Sea Water from Rivers"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("FWFSW"),
        },
        31 => TableCategory {
            parameter: String::from("Fresh Water Flux Correction"),
            units: String::from("kg m-2s-1"),
            abbrev: String::from("FWFC"),
        },
        32 => TableCategory {
            parameter: String::from("Virtual Salt Flux into Sea Water"),
            units: String::from("g kg-1 m-2s-1"),
            abbrev: String::from("VSFSW"),
        },
        33 => TableCategory {
            parameter: String::from("Virtual Salt Flux Correction"),
            units: String::from("g kg-1 m-2s-1"),
            abbrev: String::from("VSFC"),
        },
        34 => TableCategory {
            parameter: String::from("Sea Water Temperature Tendency due to Newtonian Relaxation"),
            units: String::from("K s-1"),
            abbrev: String::from("SWTTNR"),
        },
        35 => TableCategory {
            parameter: String::from("Sea Water Salinity Tendency due to Newtonian Relaxation"),
            units: String::from("g kg-1s-1"),
            abbrev: String::from("SWSTNR"),
        },
        36 => TableCategory {
            parameter: String::from("Sea Water Temperature Tendency due to Parameterization"),
            units: String::from("K s-1"),
            abbrev: String::from("SWTTP"),
        },
        37 => TableCategory {
            parameter: String::from("Sea Water Salinity Tendency due to Parameterization"),
            units: String::from("g kg-1s-1"),
            abbrev: String::from("SWSTP"),
        },
        38 => TableCategory {
            parameter: String::from("Eastward Sea Water Velocity Tendency Due To Parameterization"),
            units: String::from("m s-2"),
            abbrev: String::from("ESWVP"),
        },
        39 => TableCategory {
            parameter: String::from(
                "Northward Sea Water Velocity Tendency Due To Parameterization",
            ),
            units: String::from("m s-2"),
            abbrev: String::from("NSWVP"),
        },
        40 => TableCategory {
            parameter: String::from("Sea Water Temperature Tendency Due to Direct Bias Correction"),
            units: String::from("K s-1"),
            abbrev: String::from("SWTTBC"),
        },
        41 => TableCategory {
            parameter: String::from("Sea Water Salinity Tendency due to Direct Bias Correction"),
            units: String::from("g kg-1s-1"),
            abbrev: String::from("SWSTBC"),
        },
        42 => TableCategory {
            parameter: String::from("Sea Water Meridional Volume Transport"),
            units: String::from("m3 m-2 s-1"),
            abbrev: String::from("SEAMVT"),
        },
        43 => TableCategory {
            parameter: String::from("Sea Water Zonal Volume Transport"),
            units: String::from("m3 m-2 s-1"),
            abbrev: String::from("SEAZVT"),
        },
        44 => TableCategory {
            parameter: String::from("Sea Water Column Integrated Meridional Volume Transport"),
            units: String::from("m3 m-2 s-1"),
            abbrev: String::from("SEACMVT"),
        },
        45 => TableCategory {
            parameter: String::from("Sea Water Column Integrated Zonal Volume Transport"),
            units: String::from("m3 m-2 s-1"),
            abbrev: String::from("SEACZVT"),
        },
        46 => TableCategory {
            parameter: String::from("Sea Water Meridional Mass Transport"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("SEAMMT"),
        },
        47 => TableCategory {
            parameter: String::from("Sea Water Zonal Mass Transport"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("SEAZMT"),
        },
        48 => TableCategory {
            parameter: String::from("Sea Water Column Integrated Meridional Mass Transport"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("SEACMMT"),
        },
        49 => TableCategory {
            parameter: String::from("Sea Water Column Integrated Zonal Mass Transport"),
            units: String::from("kg m-2 s-1"),
            abbrev: String::from("SEACZMT"),
        },
        50 => TableCategory {
            parameter: String::from("Sea Water Column Integrated Practical Salinity"),
            units: String::from("g kg-1 m"),
            abbrev: String::from("SEACPSALT"),
        },
        51 => TableCategory {
            parameter: String::from("Sea Water Column Integrated Salinity"),
            units: String::from("kg kg-1 m"),
            abbrev: String::from("SEACSALT"),
        },
        192 => TableCategory {
            parameter: String::from("3-D Temperature"),
            units: String::from("°C"),
            abbrev: String::from("WTMPC"),
        },
        193 => TableCategory {
            parameter: String::from("3-D Salinity"),
            units: String::from("psu"),
            abbrev: String::from("SALIN"),
        },
        194 => TableCategory {
            parameter: String::from("Barotropic Kinetic Energy"),
            units: String::from("J kg-1"),
            abbrev: String::from("BKENG"),
        },
        195 => TableCategory {
            parameter: String::from("Geometric Depth Below Sea Surface"),
            units: String::from("m"),
            abbrev: String::from("DBSS"),
        },
        196 => TableCategory {
            parameter: String::from("Interface Depths"),
            units: String::from("m"),
            abbrev: String::from("INTFD"),
        },
        197 => TableCategory {
            parameter: String::from("Ocean Heat Content"),
            units: String::from("J m-2"),
            abbrev: String::from("OHC"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        8..=10 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-10-191
/// PARAMETERS FOR DISCIPLINE 10, CATEGORY 191
/// **(Oceanographic products, Miscellaneous category)**
///
/// **Details**:
/// - **Discipline**: 10 (Oceanographic Products)
/// - **Category**: 191 (Miscellaneous)
/// - **Section**: 4
/// - **Octet 10**: 191
/// - **Revised**: 06/30/2022
///
/// **Reserved Ranges**:
/// - `2`: Reserved
/// - `5-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 10, Category 191.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 10, Category 191 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_10191(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from(
                "Seconds Prior To Initial Reference Time (Defined In Section 1)",
            ),
            units: String::from("s"),
            abbrev: String::from("IRTSEC"),
        },
        1 => TableCategory {
            parameter: String::from("Meridional Overturning Stream Function"),
            units: String::from("m3 s-1"),
            abbrev: String::from("MOSF"),
        },
        3 => TableCategory {
            parameter: String::from("Days Since Last Observation"),
            units: String::from("d"),
            abbrev: String::from("DSLOBSO"),
        },
        4 => TableCategory {
            parameter: String::from("Barotropic Stream Function"),
            units: String::from("m3 s-1"),
            abbrev: String::from("BARDSF"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        2 => TableCategory {
            parameter: String::from("Reserved"),
            units: String::from(""),
            abbrev: String::from("Reserved"),
        },
        5..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-20-0
/// PARAMETERS FOR DISCIPLINE 20, CATEGORY 0
/// **(Health and Socioeconomic Impacts, Health Indicators category)**
///
/// **Details**:
/// - **Discipline**: 20 (Health and Socioeconomic Impacts)
/// - **Category**: 0 (Health Indicators)
/// - **Section**: 4
/// - **Octet 10**: 0
/// - **Created**: 06/30/2022
///
/// **Reserved Ranges**:
/// - `9-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// ## Notes:
/// - Wet-bulb Globe Temperature (parameter 2) and Globe Temperature (parameter 3) may require additional environmental conditions to calculate.
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 20, Category 0.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 20, Category 0 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_2000(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Universal Thermal Climate Index"),
            units: String::from("K"),
            abbrev: String::from("UTHCIDX"),
        },
        1 => TableCategory {
            parameter: String::from("Mean Radiant Temperature"),
            units: String::from("K"),
            abbrev: String::from("MEANRTMP"),
        },
        2 => TableCategory {
            parameter: String::from("Wet-bulb Globe Temperature"),
            units: String::from("K"),
            abbrev: String::from("WETBGTMP"),
        },
        3 => TableCategory {
            parameter: String::from("Globe Temperature"),
            units: String::from("K"),
            abbrev: String::from("GLOBETMP"),
        },
        4 => TableCategory {
            parameter: String::from("Humidex"),
            units: String::from("K"),
            abbrev: String::from("HUMIDX"),
        },
        5 => TableCategory {
            parameter: String::from("Effective Temperature"),
            units: String::from("K"),
            abbrev: String::from("EFFTEMP"),
        },
        6 => TableCategory {
            parameter: String::from("Normal Effective Temperature"),
            units: String::from("K"),
            abbrev: String::from("NOREFTMP"),
        },
        7 => TableCategory {
            parameter: String::from("Standard Effective Temperature"),
            units: String::from("K"),
            abbrev: String::from("STDEFTMP"),
        },
        8 => TableCategory {
            parameter: String::from("Physiological Equivalent Temperature"),
            units: String::from("K"),
            abbrev: String::from("PEQUTMP"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        9..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-20-1
/// PARAMETERS FOR DISCIPLINE 20, CATEGORY 1
/// **(Health and Socioeconomic Impacts, Epidemiology category)**
///
/// **Details**:
/// - **Discipline**: 20 (Health and Socioeconomic Impacts)
/// - **Category**: 1 (Epidemiology)
/// - **Section**: 4
/// - **Octet 10**: 1
/// - **Created**: 06/30/2022
///
/// **Reserved Ranges**:
/// - `10-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 20, Category 1.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 20, Category 1 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_2001(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Malaria Cases"),
            units: String::from("Fraction"),
            abbrev: String::from("MALACASE"),
        },
        1 => TableCategory {
            parameter: String::from("Malaria Circumsporozoite Protein Rate"),
            units: String::from("Fraction"),
            abbrev: String::from("MACPRATE"),
        },
        2 => TableCategory {
            parameter: String::from("Plasmodium Falciparum Entomological Inoculation Rate"),
            units: String::from("Bites per day per person"),
            abbrev: String::from("PFEIRATE"),
        },
        3 => TableCategory {
            parameter: String::from("Human Bite Rate by Anopheles Vectors"),
            units: String::from("Bites per day per person"),
            abbrev: String::from("HBRATEAV"),
        },
        4 => TableCategory {
            parameter: String::from("Malaria Immunity"),
            units: String::from("Fraction"),
            abbrev: String::from("MALAIMM"),
        },
        5 => TableCategory {
            parameter: String::from("Falciparum Parasite Rates"),
            units: String::from("Fraction"),
            abbrev: String::from("FALPRATE"),
        },
        6 => TableCategory {
            parameter: String::from("Detectable Falciparum Parasite Ratio (after day 10)"),
            units: String::from("Fraction"),
            abbrev: String::from("DFPRATIO"),
        },
        7 => TableCategory {
            parameter: String::from("Anopheles Vector to Host Ratio"),
            units: String::from("Fraction"),
            abbrev: String::from("AVHRATIO"),
        },
        8 => TableCategory {
            parameter: String::from("Anopheles Vector Number"),
            units: String::from("Number m-2"),
            abbrev: String::from("AVECTNUM"),
        },
        9 => TableCategory {
            parameter: String::from("Fraction of Malarial Vector Reproductive Habitat"),
            units: String::from("Fraction"),
            abbrev: String::from("FMALVRH"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        10..=191 => TableCategory {
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

/// # GRIB2 - TABLE 4.2-20-2
/// PARAMETERS FOR DISCIPLINE 20, CATEGORY 2
/// **(Health and Socioeconomic Impacts, Socioeconomic indicators category)**
///
/// **Details**:
/// - **Discipline**: 20 (Health and Socioeconomic Impacts)
/// - **Category**: 2 (Socioeconomic Indicators)
/// - **Section**: 4
/// - **Octet 10**: 2
/// - **Created**: 06/30/2022
///
/// **Reserved Ranges**:
/// - `1-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 20, Category 2.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 20, Category 2 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_2002(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Population Density"),
            units: String::from("Person m-2"),
            abbrev: String::from("POPDEN"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
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

/// # GRIB2 - TABLE 4.2-20-3
/// PARAMETERS FOR DISCIPLINE 20, CATEGORY 3
/// **(Health and Socioeconomic Impacts, Renewable Energy Sector category)**
///
/// **Details**:
/// - **Discipline**: 20 (Health and Socioeconomic Impacts)
/// - **Category**: 3 (Renewable Energy Sector)
/// - **Section**: 4
/// - **Octet 10**: 3
/// - **Created**: 12/07/2023
///
/// **Reserved Ranges**:
/// - `10-191`: Reserved
/// - `192-254`: Reserved for Local Use
///
/// **Special Value**:
/// - `255`: Missing
///
/// This function provides a lookup for GRIB2 parameter categories for Discipline 20, Category 3.
///
/// # Arguments
/// * `category` - The parameter category within Discipline 20, Category 3 (u8).
///
/// # Returns
/// A `TableCategory` struct containing the description of the parameter category.
/// Returns a "Missing" TableCategory if the category is not found or is a special value.
pub fn grib2_lookup_table42_2003(category: u8) -> TableCategory {
    match category {
        0 => TableCategory {
            parameter: String::from("Renewable power capacity"),
            units: String::from("W"),
            abbrev: String::from("RENPCAP"),
        },
        1 => TableCategory {
            parameter: String::from("Renewable power production rate"),
            units: String::from("W"),
            abbrev: String::from("RENPPROD"),
        },
        2 => TableCategory {
            parameter: String::from("Wind power capacity"),
            units: String::from("W"),
            abbrev: String::from("WINDPCAP"),
        },
        3 => TableCategory {
            parameter: String::from("Wind power production rate"),
            units: String::from("W"),
            abbrev: String::from("WINDPPROD"),
        },
        4 => TableCategory {
            parameter: String::from("Solar photovoltaic (PV) power capacity"),
            units: String::from("W"),
            abbrev: String::from("SPVPCAP"),
        },
        5 => TableCategory {
            parameter: String::from("Solar photovoltaic (PV) power production rate"),
            units: String::from("W"),
            abbrev: String::from("SPVPPROD"),
        },
        6 => TableCategory {
            parameter: String::from("Solar non-photovoltaic (PV) power capacity"),
            units: String::from("W"),
            abbrev: String::from("SNPVPCAP"),
        },
        7 => TableCategory {
            parameter: String::from("Solar non-photovoltaic (PV) power production rate"),
            units: String::from("W"),
            abbrev: String::from("SNPVPPROD"),
        },
        8 => TableCategory {
            parameter: String::from("Concentrated solar power (CSP) power capacity"),
            units: String::from("W"),
            abbrev: String::from("CSPPCAP"),
        },
        9 => TableCategory {
            parameter: String::from("Concentrated solar power (CSP) power production rate"),
            units: String::from("W"),
            abbrev: String::from("CSPPROD"),
        },
        255 => TableCategory {
            parameter: String::from("Missing"),
            units: String::from(""),
            abbrev: String::from(""),
        },
        10..=191 => TableCategory {
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
