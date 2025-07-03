/// # GRIB2 - TABLE 6.0 - BIT MAP INDICATOR
///
/// **Details**:
/// - **Section**: 6
/// - **Octet**: 6
/// - **Revised**: 05/17/2005
///
/// **Value Ranges**:
/// - `1-253`: A bit map pre-determined by the originating/generating center applies to this product and is not specified in this section.
///
/// **Special Value**:
/// - `255`: A bit map does not apply to this product.
///
/// ## Description
/// This table defines the bit map indicators used in GRIB2 files,
/// specifying how bit maps apply to products based on various definitions.
///
/// ## Notes
/// - Revised 05/17/2005
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2Table6_0 {
    BitmapSpecifiedInThisSection = 0,
    BitmapPredeterminedByCenter(u8), // For values 1-253
    BitmapPreviouslyDefined = 254,
    BitmapDoesNotApply = 255,
}
impl From<u8> for Grib2Table6_0 {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::BitmapSpecifiedInThisSection,
            1..=253 => Self::BitmapPredeterminedByCenter(val),
            254 => Self::BitmapPreviouslyDefined,
            255 => Self::BitmapDoesNotApply,
        }
    }
}
impl core::fmt::Display for Grib2Table6_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let desc = match self {
            Self::BitmapSpecifiedInThisSection => {
                "A bit map applies to this product and is specified in this section."
            }
            Self::BitmapPredeterminedByCenter(v) => {
                return write!(
                    f,
                    "A bit map pre-determined by the originating/generating center applies to this product and is not specified in this section. (Value: {})",
                    v
                );
            }
            Self::BitmapPreviouslyDefined => {
                "A bit map previously defined in the same GRIB2 message applies to this product."
            }
            Self::BitmapDoesNotApply => "A bit map does not apply to this product.",
        };
        f.write_str(desc)
    }
}
