mod tables;

use crate::{
    parsers::Reader,
    readers::{_0::Grib2Table0, other::Grib2TableC},
    util::Date,
};
pub use tables::*;
use tables::{
    Grib2Table1_0 as MasterTablesVersion, Grib2Table1_1 as LocalTablesVersion,
    Grib2Table1_2 as SignificanceOfRT, Grib2Table1_3 as ProductionStatus,
    Grib2Table1_4 as TypeOfProcessedData,
};

/// # Identification Section
///
/// ## Notes
/// - 1. Local tables define those parts of the master table which are reserved for local use except for the case described below.  In any case, the use of local tables in the messages are intended for non-local or international exchange is strongly discouraged.
/// - 2.  If octet 10 is set to 255 then only local tables are in use.  In this case, the local table version number (octet 11) must not be zero nor missing.  Local tables may include entries from the entire range of the tables.
/// - 3.  If octet 11 is zero, octet 10 must contain a valid master table version number and only those parts of the tables not reserved for local use may be used.
/// - 4.  If octets 8-9 is zero, Not a sub-center, the originating/generating center is the center defined by octets 6-7.
///
/// ## Links
/// - [Read more...](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_sect1.shtml)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grib2IdentificationSection {
    /// Number of GRIB section
    pub section_number: u8,
    /// Length of GRIB section
    pub length: u32,
    /// Identification of originating/generating center [Table 0](https://www.nco.ncep.noaa.gov/pmb/docs/on388/table0.html)
    pub center: Grib2Table0,
    /// Identification of originating/generating subcenter [Table C](https://www.nco.ncep.noaa.gov/pmb/docs/on388/tablec.html)
    pub subcenter: Grib2TableC,
    /// GRIB master tables version number [Table 1.0](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-0.shtml)
    pub grib_master_tables_version: MasterTablesVersion,
    /// Version number of GRIB local tables used to augment Master Tables [Table 1.1](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-1.shtml)
    pub grib_local_tables_version: LocalTablesVersion,
    /// Significance of reference time [Table 1.2](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-2.shtml)
    pub significance_of_rt: SignificanceOfRT,
    /// Reference Time
    pub ref_time: Date,
    /// Production Status of Processed data in the GRIB message [Table 1.3](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-3.shtml)
    pub production_status: ProductionStatus,
    /// Type of processed data in this GRIB message [Table 1.4](https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/grib2_table1-4.shtml)
    pub type_of_processed_data: TypeOfProcessedData,
}
impl Grib2IdentificationSection {
    /// Create a new Grib2IdentificationSection
    ///
    /// ## Parameters
    /// - `section`: The byte block to pull ideintification information
    ///
    /// ## Returns
    /// The parsed identification section
    pub fn new<T: Reader>(section: &T) -> Grib2IdentificationSection {
        let center = section.uint16_be(Some(5)) as u8;
        let subcenter = section.uint16_be(Some(7)) as u8;
        let grib_master_tables_version = section.uint8(Some(9)); // should be 2
        let grib_local_tables_version = section.uint8(Some(10));
        let significance_of_rt = section.uint8(Some(11));
        let year = section.uint16_be(Some(12));
        let month = section.uint8(Some(14));
        let day = section.uint8(Some(15));
        let hours = section.uint8(Some(16));
        let minutes = section.uint8(Some(17));
        let seconds = section.uint8(Some(18));
        let production_status = section.uint8(Some(19));
        let type_of_processed_data = section.uint8(Some(20));

        let ref_time = Date::new_full(year, month, day, hours, minutes, seconds);

        if grib_master_tables_version != 2 {
            panic!("Invalid grib_master_tables_version: {}", grib_master_tables_version);
        }

        Grib2IdentificationSection {
            section_number: section.uint8(Some(4)),
            length: section.uint32_be(Some(0)),
            center: center.into(),
            subcenter: subcenter.into(),
            grib_master_tables_version: grib_master_tables_version.into(),
            grib_local_tables_version: grib_local_tables_version.into(),
            significance_of_rt: significance_of_rt.into(),
            ref_time,
            production_status: production_status.into(),
            type_of_processed_data: type_of_processed_data.into(),
        }
    }
}
