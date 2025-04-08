use alloc::{boxed::Box, string::String, vec::Vec};
use serde::{Deserialize, Serialize};

// NOTE: r#type has been replaced with Option<String> instead of String because I find too
// many examples of it not being used.
// NOTE: A few variables have been switched to Option because the 0.7 spec is not being followed
// correctly by generators.

/// # Schema for PROJJSON (v0.7)
/// @see https://proj.org/schemas/v0.7/projjson.schema.json
/// @see https://docs.ogc.org/is/18-010r7/18-010r7.html#1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjJSON {
    /// Coordinate Reference System
    CRS(Box<CRS>),
    /// Represents a datum which can be one of several types of reference frames or datums.
    Datum(Box<Datum>),
    /// Represents a datum ensemble, which is a collection of datums.
    DatumEnsemble(Box<DatumEnsemble>),
    /// Represents an ellipsoid, a geometric figure used in geodetic reference frames.
    Ellipsoid(Box<Ellipsoid>),
    /// Represents a prime meridian, which defines the origin of longitude in a geographic coordinate system.
    PrimeMeridian(Box<PrimeMeridian>),
    /// Represents a single operation, which can be a conversion, transformation, or point motion operation.
    SingleOperation(Box<SingleOperation>),
    /// Represents an operation that is composed of multiple steps, transforming one CRS to another.
    ConcatenatedOperation(Box<ConcatenatedOperation>),
    /// Represents metadata associated with a coordinate, including its reference system and epoch.
    CoordinateMetadata(Box<CoordinateMetadata>),
}
impl Default for ProjJSON {
    fn default() -> Self {
        ProjJSON::CRS(Box::default())
    }
}

/// Coordinate Reference System
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CRS {
    /// Represents a coordinate reference system that is bounded by a source and target CRS with a transformation.
    BoundCRS(Box<BoundCRS>),
    /// Represents a compound coordinate reference system, consisting of multiple components.
    CompoundCRS(Box<CompoundCRS>),
    /// Represents a derived engineering coordinate reference system.
    DerivedEngineeringCRS(Box<DerivedEngineeringCRS>),
    /// Represents a derived geodetic or geographic coordinate reference system.
    DerivedGeodeticCRS(Box<DerivedGeodeticCRS>),
    /// Represents a derived parametric coordinate reference system.
    DerivedParametricCRS(Box<DerivedParametricCRS>),
    /// Represents a derived projected coordinate reference system.
    DerivedProjectedCRS(Box<DerivedProjectedCRS>),
    /// Represents a derived temporal coordinate reference system.
    DerivedTemporalCRS(Box<DerivedTemporalCRS>),
    /// Represents a derived vertical coordinate reference system.
    DerivedVerticalCRS(Box<DerivedVerticalCRS>),
    /// Represents an engineering coordinate reference system.
    EngineeringCRS(Box<EngineeringCRS>),
    /// Represents a geodetic or geographic coordinate reference system.
    GeodeticCRS(Box<GeodeticCRS>),
    /// Represents a parametric coordinate reference system.
    ParametricCRS(Box<ParametricCRS>),
    /// Represents a projected coordinate reference system, which transforms geodetic or geographic coordinates
    /// into a flat, two-dimensional plane using a map projection.
    ProjectedCRS(Box<ProjectedCRS>),
    /// Represents a temporal coordinate reference system, which defines time-based coordinates.
    TemporalCRS(Box<TemporalCRS>),
    /// Represents a vertical coordinate reference system, which is used for height or depth measurements.
    VerticalCRS(Box<VerticalCRS>),
}
impl Default for CRS {
    fn default() -> Self {
        CRS::GeodeticCRS(Box::default())
    }
}

/// # Datum Interface
///
/// Represents a datum which can be one of several types of reference frames or datums.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Datum {
    /// Represents the geodetic reference frame associated with a geodetic CRS.
    GeodeticReferenceFrame(GeodeticReferenceFrame),
    /// Represents the vertical reference frame associated with a vertical CRS.
    VerticalReferenceFrame(VerticalReferenceFrame),
    /// Represents a dynamic geodetic reference frame.
    DynamicGeodeticReferenceFrame(DynamicGeodeticReferenceFrame),
    /// Represents a dynamic vertical reference frame.
    DynamicVerticalReferenceFrame(DynamicVerticalReferenceFrame),
    /// Represents the temporal datum associated with a temporal CRS.
    TemporalDatum(TemporalDatum),
    /// Represents the parametric datum associated with a parametric CRS.
    ParametricDatum(ParametricDatum),
    /// Represents the datum associated with an engineering CRS.
    EngineeringDatum(EngineeringDatum),
}

/// # Bounding Box Interface
///
/// Represents a bounding box defined by its east, west, south, and north boundaries.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ProjBBox {
    /// The easternmost longitude of the bounding box.
    pub east_longitude: f64,
    /// The westernmost longitude of the bounding box.
    pub west_longitude: f64,
    /// The southernmost latitude of the bounding box.
    pub south_latitude: f64,
    /// The northernmost latitude of the bounding box.
    pub north_latitude: f64,
}

/// Vertical Extent
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct VerticalExtent {
    /// Minimum height
    pub minimum: f64,
    /// Maximum height
    pub maximum: f64,
    /// Unit of measurement
    pub unit: Unit,
}

/// Temporal Extent
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TemporalExtent {
    /// Start time (ISO 8601 format)
    pub start: String,
    /// End time (ISO 8601 format)
    pub end: String,
}

/// String or Number
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    /// String
    String(String),
    /// Unsigned
    Unsigned(u64),
    /// Signed
    Signed(i64),
    /// Float
    Float(f64),
}
impl Default for StringOrNumber {
    fn default() -> Self {
        StringOrNumber::String(String::default())
    }
}
impl StringOrNumber {
    /// Grab the unsigned integer if it exists. Otherwise default to 0
    pub fn to_u64(&self) -> u64 {
        match self {
            StringOrNumber::String(s) => s.parse().unwrap_or(0),
            StringOrNumber::Unsigned(u) => *u,
            _ => 0,
        }
    }
    /// Grab the signed integer if it exists. Otherwise default to 0
    pub fn to_i64(&self) -> i64 {
        match self {
            StringOrNumber::String(s) => s.parse().unwrap_or(0),
            StringOrNumber::Signed(i) => *i,
            _ => 0,
        }
    }
    /// Get the float. If a string, convert to float
    pub fn to_f64(&self) -> f64 {
        match self {
            StringOrNumber::String(s) => s.parse().unwrap_or(0.0),
            StringOrNumber::Unsigned(u) => *u as f64,
            StringOrNumber::Signed(i) => *i as f64,
            StringOrNumber::Float(f) => *f,
        }
    }
}
impl From<StringOrNumber> for String {
    fn from(v: StringOrNumber) -> String {
        match v {
            StringOrNumber::String(s) => s,
            _ => "".into(),
        }
    }
}

/// ID Object
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Id {
    /// Authority issuing the identifier
    pub authority: String,
    /// Code associated with the identifier
    pub code: StringOrNumber,
    /// Version of the identifier
    /// NOTE: This is not supposed to be optional, but it rarely shows up
    pub version: Option<StringOrNumber>,
    /// Citation of the authority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_citation: Option<String>,
    /// URI reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// Identifiers list
pub type Ids = Vec<Id>;

/// Usage Object
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Usage {
    /// Scope of the usage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Defined area
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    /// Bounding box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<ProjBBox>,
    /// Vertical extent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_extent: Option<VerticalExtent>,
    /// Temporal extent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_extent: Option<TemporalExtent>,
}

/// Parameter Value
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ParameterValue {
    /// Schema reference
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Type identifier - always 'ParameterValue'
    #[serde(rename = "type")]
    pub r#type: String, // 'ParameterValue';
    /// Name of the parameter
    pub name: String,
    /// Parameter value, which can be a string or number
    pub value: StringOrNumber,
    /// Optional unit of measurement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// Alternative identifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
}

/// # Parametric CRS
///
/// Represents a parametric coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ParametricCRS {
    /// Type identifier - always 'ParametricCRS'
    #[serde(rename = "type")]
    pub r#type: String, // 'ParametricCRS';
    /// Name of the CRS
    pub name: String,
    /// Parametric datum
    pub datum: ParametricDatum,
    /// Coordinate system
    pub coordinate_system: CoordinateSystem,
    /// Schema reference
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Scope of the CRS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Defined area
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    /// Bounding box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<ProjBBox>,
    /// Vertical extent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_extent: Option<VerticalExtent>,
    /// Temporal extent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_extent: Option<TemporalExtent>,
    /// Usages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usages: Option<Vec<Usage>>,
    /// Additional remarks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// Alternative identifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # Parametric Datum
///
/// Represents the parametric datum associated with a parametric CRS.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ParametricDatum {
    /// Type identifier - always 'ParametricDatum'
    #[serde(rename = "type")]
    pub r#type: String, // 'ParametricDatum';
    /// Name of the datum
    pub name: String,
    /// Anchor point
    pub anchor: String,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # Point Motion Operation
///
/// Represents a point motion operation
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PointMotionOperation {
    /// Type identifier
    #[serde(rename = "type")]
    pub r#type: String, // 'PointMotionOperation';
    /// Name of the operation
    pub name: String,
    /// Source coordinate reference system
    pub source_crs: CRS,
    /// Method used for point motion
    pub method: Method,
    /// Parameters used in the operation
    pub parameters: Vec<ParameterValue>,
    /// Accuracy of the operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<String>,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # Method Object
///
/// Defines an operation method with a name and identifier
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Method {
    /// Schema reference
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Type identifier - always 'OperationMethod'
    #[serde(rename = "type")]
    pub r#type: String, // 'OperationMethod';
    /// Name of the method
    pub name: String,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// Alternative identifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
}

/// Base Unit - common units as string input
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum BaseUnit {
    /// Metre
    #[serde(rename = "metre")]
    #[default]
    Metre,
    /// Degree
    #[serde(rename = "degree")]
    Degree,
    /// Unity
    #[serde(rename = "unity")]
    Unity,
}

/// Unit Type - String input
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UnitType {
    /// Linear
    LinearUnit,
    /// Angular
    AngularUnit,
    /// Scale
    ScaleUnit,
    /// Time
    TimeUnit,
    /// Parametric
    ParametricUnit,
    /// Unit
    #[default]
    Unit,
}

/// Unit Object
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UnitObject {
    /// Type of unit
    #[serde(rename = "type")]
    pub r#type: UnitType,
    /// Name of the unit
    pub name: String,
    /// Conversion factor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_factor: Option<f64>,
    /// Schema reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// Alternative identifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
}

/// Unit Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Unit {
    /// Base case
    BaseUnit(BaseUnit),
    /// Object with complex units
    UnitObject(UnitObject),
}
impl Default for Unit {
    fn default() -> Self {
        Unit::BaseUnit(BaseUnit::default())
    }
}

/// # BoundCRS Interface
///
/// Represents a coordinate reference system that is bounded by a source and target CRS with a transformation.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BoundCRS {
    /// Indicates the type of object. Always "BoundCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'BoundCRS';
    /// The name of the bound CRS.
    pub name: String,
    /// The source coordinate reference system.
    pub source_crs: Box<CRS>,
    /// The target coordinate reference system.
    pub target_crs: Box<CRS>,
    /// The transformation applied to convert between the source and target CRS.
    pub transformation: AbridgedTransformation,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # ConcatenatedOperation Interface
///
/// Represents an operation that is composed of multiple steps, transforming one CRS to another.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ConcatenatedOperation {
    /// Indicates the type of object. Always "ConcatenatedOperation" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'ConcatenatedOperation';
    /// The name of the concatenated operation.
    pub name: String,
    /// The source coordinate reference system.
    pub source_crs: CRS,
    /// The target coordinate reference system.
    pub target_crs: CRS,
    /// An array of individual steps in the concatenated operation.
    pub steps: Vec<SingleOperation>,
    /// The accuracy of the concatenated operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<String>,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # AbridgedTransformation Interface
///
/// Represents an abridged transformation used for converting between different coordinate reference systems.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AbridgedTransformation {
    /// The schema URL or identifier.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Indicates the type of object. Always "AbridgedTransformation" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'AbridgedTransformation';
    /// The name of the transformation.
    pub name: String,
    /// The source coordinate reference system, only present if it differs from the source CRS of the bound CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_crs: Option<Box<CRS>>,
    /// The method used for the transformation.
    pub method: Method,
    /// The parameters used in the transformation.
    pub parameters: Vec<ParameterValue>,
    /// An identifier for the transformation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the transformation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
}

/// # CompoundCRS Interface
///
/// Represents a compound coordinate reference system, consisting of multiple components.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CompoundCRS {
    /// Indicates the type of object. Always "CompoundCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'CompoundCRS';
    /// The name of the compound CRS.
    pub name: String,
    /// An array of coordinate reference systems that make up the compound CRS.
    pub components: Vec<CRS>,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # EngineeringCRS Interface
///
/// Represents an engineering coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EngineeringCRS {
    /// Indicates the type of CRS. Always "EngineeringCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'EngineeringCRS';
    /// The name of the engineering CRS.
    pub name: String,
    /// The engineering datum associated with this CRS.
    pub datum: EngineeringDatum,
    /// The coordinate system used in this CRS.
    pub coordinate_system: Option<CoordinateSystem>,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # EngineeringDatum Interface
///
/// Represents the datum associated with an engineering CRS.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EngineeringDatum {
    /// Indicates the type of datum. Always "EngineeringDatum" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'EngineeringDatum';
    /// The name of the datum.
    pub name: String,
    /// Anchor point of the datum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// Axis Direction defines an axis direction
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AxisDirection {
    /// North
    North,
    /// North by North East
    NorthNorthEast,
    /// North East
    NorthEast,
    /// East by North East
    EastNorthEast,
    /// East
    East,
    /// East by South East
    EastSouthEast,
    /// South East
    SouthEast,
    /// South by South East
    SouthSouthEast,
    /// South
    South,
    /// South by South West
    SouthSouthWest,
    /// South West
    SouthWest,
    /// West by South West
    WestSouthWest,
    /// West
    West,
    /// West by North West
    WestNorthWest,
    /// North West
    NorthWest,
    /// North by North West
    NorthNorthWest,
    /// Up
    Up,
    /// Down
    Down,
    /// Geocentric X
    GeocentricX,
    /// Geocentric Y
    GeocentricY,
    /// Geocentric Z
    GeocentricZ,
    /// Column Positive
    ColumnPositive,
    /// Column Negative
    ColumnNegative,
    /// Row Positive
    RowPositive,
    /// Row Negative
    RowNegative,
    /// Display Right
    DisplayRight,
    /// Display Left
    DisplayLeft,
    /// Display Up
    DisplayUp,
    /// Display Down
    DisplayDown,
    /// Forward
    Forward,
    /// Aft
    Aft,
    /// Port
    Port,
    /// Starboard
    Starboard,
    /// Clockwise
    Clockwise,
    /// Counter Clockwise
    CounterClockwise,
    /// Towards
    Towards,
    /// Away From
    AwayFrom,
    /// Future
    Future,
    /// Past
    Past,
    /// Unspecified
    #[default]
    Unspecified,
}

/// Axis Range Meaning
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AxisRangeMeaning {
    /// Exact
    #[default]
    Exact,
    /// Wraparound
    Wraparound,
}

/// # Axis Interface
///
/// Represents an individual axis in a coordinate system.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Axis {
    /// Indicates the type of axis. Always "Axis" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'Axis';
    /// The name of the axis.
    pub name: String,
    /// Abbreviation for the axis name.
    pub abbreviation: String,
    /// The direction of the axis.
    /// Examples include north, east, up, down, geocentricX, geocentricY, geocentricZ, etc.
    pub direction: AxisDirection,
    /// The meridian for the axis, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meridian: Option<Meridian>,
    /// The unit of measurement for the axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    /// The minimum value allowed for the axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_value: Option<f64>,
    /// The maximum value allowed for the axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_value: Option<f64>,
    /// The range meaning for the axis.
    /// Can be either "exact" or "wraparound".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_meaning: Option<AxisRangeMeaning>,
    /// An identifier for the axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
}

/// # Meridian Interface
///
/// Represents a meridian, which defines the longitude for an axis.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Meridian {
    /// Indicates the type of meridian. Always "Meridian" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'Meridian';
    /// The longitude of the meridian.
    longitude: ValueInDegreeOrValueAndUnit,
    /// The schema URL or identifier.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// An identifier for the meridian.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the meridian.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
}

/// # ValueAndUnit Interface
///
/// Represents a value paired with a unit of measurement.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ValueAndUnit {
    /// The numeric value.
    pub value: f64,
    /// The unit of measurement.
    pub unit: Unit,
}

/// Value in Degrees or Value and Unit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueInDegreeOrValueAndUnit {
    /// Float value
    F64(f64),
    /// Value and Unit Object
    ValueAndUnit(ValueAndUnit),
}
impl Default for ValueInDegreeOrValueAndUnit {
    fn default() -> Self {
        ValueInDegreeOrValueAndUnit::F64(0.0)
    }
}

/// Value in Metres or Value and Unit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueInMetreOrValueAndUnit {
    /// Float
    F64(f64),
    /// Value and Unit
    ValueAndUnit(ValueAndUnit),
}
impl Default for ValueInMetreOrValueAndUnit {
    fn default() -> Self {
        ValueInMetreOrValueAndUnit::F64(0.0)
    }
}

/// # Single Operation
///
/// Represents a single operation, which can be a conversion, transformation, or point motion operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SingleOperation {
    /// Conversion Operation
    Conversion(Box<Conversion>),
    /// Transformation Operation
    Transformation(Box<Transformation>),
    /// Point & Motion Operation
    PointMotionOperation(Box<PointMotionOperation>),
}

/// # DatumMember Interface
///
/// Represents a member of a datum ensemble.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DatumMember {
    /// The name of the datum member.
    pub name: String,
    /// An identifier for the datum member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the datum member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
}

/// # DeformationModel Interface
///
/// Represents a deformation model associated with a point motion operation.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DeformationModel {
    /// The name of the deformation model.
    pub name: String,
    /// An identifier for the deformation model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
}

/// # DerivedEngineeringCRS Interface
///
/// Represents a derived engineering coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DerivedEngineeringCRS {
    /// Indicates the type of coordinate reference system. Always "DerivedEngineeringCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'DerivedEngineeringCRS';
    /// The name of the derived engineering CRS.
    pub name: String,
    /// The base CRS from which this derived CRS is created.
    pub base_crs: EngineeringCRS,
    /// The conversion method applied to the base CRS.
    pub conversion: Conversion,
    /// The coordinate system used in the CRS.
    pub coordinate_system: CoordinateSystem,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # DerivedGeodeticCRS Interface
///
/// Represents a derived geodetic or geographic coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DerivedGeodeticCRS {
    /// Indicates the type of coordinate reference system. Can be either "DerivedGeodeticCRS" or "DerivedGeographicCRS".
    #[serde(rename = "type")]
    pub r#type: String, // 'DerivedGeodeticCRS' | 'DerivedGeographicCRS';
    /// The name of the derived geodetic CRS.
    pub name: String,
    /// The base CRS from which this derived CRS is created.
    pub base_crs: GeodeticCRS,
    /// The conversion method applied to the base CRS.
    pub conversion: Conversion,
    /// The coordinate system used in the CRS.
    pub coordinate_system: CoordinateSystem,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # GeodeticCRS Interface
///
/// Represents a geodetic or geographic coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeodeticCRS {
    /// Indicates the type of CRS. Can be "GeodeticCRS" or "GeographicCRS".
    #[serde(rename = "type")]
    pub r#type: String, // 'GeodeticCRS' | 'GeographicCRS';
    /// The name of the geodetic CRS.
    pub name: String,
    /// The datum associated with the geodetic CRS.
    /// One and only one of `datum` or `datum_ensemble` must be provided.
    /// Can only be `GeodeticReferenceFrame` or `DynamicGeodeticReferenceFrame`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datum: Option<Datum>,
    /// The datum ensemble associated with the geodetic CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datum_ensemble: Option<DatumEnsemble>,
    /// The coordinate system used in the geodetic CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,
    /// An array of deformation models associated with the geodetic CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deformation_models: Option<Vec<DeformationModel>>,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # GeodeticReferenceFrame Interface
///
/// Represents the geodetic reference frame associated with a geodetic CRS.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeodeticReferenceFrame {
    /// Indicates the type of reference frame. Always "GeodeticReferenceFrame" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'GeodeticReferenceFrame';
    /// The name of the reference frame.
    pub name: String,
    /// The anchor point of the reference frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// The epoch of the anchor point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_epoch: Option<f64>,
    /// The ellipsoid used in the reference frame.
    pub ellipsoid: Ellipsoid,
    /// The prime meridian associated with the reference frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prime_meridian: Option<PrimeMeridian>,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # DerivedParametricCRS Interface
///
/// Represents a derived parametric coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DerivedParametricCRS {
    /// Indicates the type of coordinate reference system. Always "DerivedParametricCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'DerivedParametricCRS';
    /// The name of the derived parametric CRS.
    pub name: String,
    /// The base parametric CRS from which this CRS is derived.
    pub base_crs: ParametricCRS,
    /// The conversion method applied to the base CRS.
    pub conversion: Conversion,
    /// The coordinate system used in the CRS.
    pub coordinate_system: CoordinateSystem,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # DerivedProjectedCRS Interface
///
/// Represents a derived projected coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DerivedProjectedCRS {
    /// Indicates the type of coordinate reference system. Always "DerivedProjectedCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'DerivedProjectedCRS';
    /// The name of the derived projected CRS.
    pub name: String,
    /// The base projected CRS from which this CRS is derived.
    pub base_crs: ProjectedCRS,
    /// The conversion method applied to the base CRS.
    pub conversion: Conversion,
    /// The coordinate system used in the CRS.
    pub coordinate_system: CoordinateSystem,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # DerivedTemporalCRS Interface
///
/// Represents a derived temporal coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DerivedTemporalCRS {
    /// Indicates the type of coordinate reference system. Always "DerivedTemporalCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'DerivedTemporalCRS';
    /// The name of the derived temporal CRS.
    pub name: String,
    /// The base temporal CRS from which this CRS is derived.
    pub base_crs: TemporalCRS,
    /// The conversion method applied to the base CRS.
    pub conversion: Conversion,
    /// The coordinate system used in the CRS.
    pub coordinate_system: CoordinateSystem,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # DerivedVerticalCRS Interface
///
/// Represents a derived vertical coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DerivedVerticalCRS {
    /// Indicates the type of coordinate reference system. Always "DerivedVerticalCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'DerivedVerticalCRS';
    /// The name of the derived vertical CRS.
    pub name: String,
    /// The base vertical CRS from which this CRS is derived.
    pub base_crs: VerticalCRS,
    /// The conversion method applied to the base CRS.
    pub conversion: Conversion,
    /// The coordinate system used in the CRS.
    pub coordinate_system: CoordinateSystem,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # DynamicGeodeticReferenceFrame Interface
///
/// Represents a dynamic geodetic reference frame.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DynamicGeodeticReferenceFrame {
    /// Indicates the type of reference frame. Always "DynamicGeodeticReferenceFrame" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'DynamicGeodeticReferenceFrame';
    /// The name of the reference frame.
    pub name: String,
    /// The anchor point of the reference frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// The epoch of the anchor point.
    pub anchor_epoch: Option<f64>,
    /// The ellipsoid used in the reference frame.
    pub ellipsoid: Ellipsoid,
    /// The prime meridian associated with the reference frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prime_meridian: Option<PrimeMeridian>,
    /// The frame reference epoch.
    pub frame_reference_epoch: f64,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// members in the datum ensemble
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DatumEnsembleMember {
    /// The name of the datum.
    pub name: String,
    /// An identifier for the datum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the datum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
}

/// # DatumEnsemble Interface
///
/// Represents a datum ensemble, which is a collection of datums.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DatumEnsemble {
    /// Indicates the type of datum ensemble. Always "DatumEnsemble" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'DatumEnsemble';
    /// The name of the datum ensemble.
    pub name: String,
    /// An array of members in the datum ensemble.
    pub members: Vec<DatumEnsembleMember>,
    /// The ellipsoid associated with the datum ensemble.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsoid: Option<Ellipsoid>,
    /// The accuracy of the datum ensemble.
    pub accuracy: String,
    /// An identifier for the datum ensemble.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the datum ensemble.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
}

/// # Ellipsoid Interface
///
/// Represents an ellipsoid, a geometric figure used in geodetic reference frames.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Ellipsoid {
    /// Indicates the type of ellipsoid. Always "Ellipsoid" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'Ellipsoid';
    /// The name of the ellipsoid.
    pub name: String,
    /// The semi-major axis of the ellipsoid.
    /// Represented as a number or a value with a unit.
    pub semi_major_axis: Option<ValueInMetreOrValueAndUnit>,
    /// The semi-minor axis of the ellipsoid.
    /// Represented as a number or a value with a unit.
    /// Required when `inverse_flattening` is not provided.
    pub semi_minor_axis: Option<ValueInMetreOrValueAndUnit>,
    /// The inverse flattening of the ellipsoid.
    /// Required when `semi_minor_axis` is not provided.
    pub inverse_flattening: Option<f64>,
    /// The radius of the ellipsoid, used for spherical representations.
    /// Required when neither `semi_minor_axis` nor `inverse_flattening` are provided.
    pub radius: Option<ValueInMetreOrValueAndUnit>,
    /// The schema URL or identifier.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// An identifier for the ellipsoid.
    pub id: Option<Id>,
    /// An array of identifiers for the ellipsoid.
    pub ids: Option<Ids>,
}

/// # PrimeMeridian Interface
///
/// Represents a prime meridian, which defines the origin of longitude in a geographic coordinate system.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PrimeMeridian {
    /// Indicates the type of prime meridian. Always "PrimeMeridian" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'PrimeMeridian';
    /// The name of the prime meridian.
    pub name: String,
    /// The longitude of the prime meridian.
    /// Represented as a number or a value with a unit.
    pub longitude: ValueInDegreeOrValueAndUnit,
    /// The schema URL or identifier.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// An identifier for the prime meridian.
    pub id: Option<Id>,
    /// An array of identifiers for the prime meridian.
    pub ids: Option<Ids>,
}

/// # ProjectedCRS Interface
///
/// Represents a projected coordinate reference system, which transforms geodetic or geographic coordinates
/// into a flat, two-dimensional plane using a map projection.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ProjectedCRS {
    /// Indicates the type of CRS. Always "ProjectedCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'ProjectedCRS';
    /// The name of the projected CRS.
    pub name: String,
    /// The base CRS upon which the projection is defined.
    /// Typically a geodetic CRS.
    pub base_crs: GeodeticCRS,
    /// The conversion defining the map projection.
    pub conversion: Conversion,
    /// The coordinate system used in the projected CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # Conversion Interface
///
/// Represents the map projection or transformation used in a projected CRS.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Conversion {
    /// Indicates the type of conversion. Always "Conversion" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'Conversion';
    /// The name of the conversion (map projection or transformation).
    pub name: String,
    /// The method used for the conversion.
    pub method: Method,
    /// An array of parameter values defining the conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterValue>>,
    /// The schema URL or identifier.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// An identifier for the conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the conversion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
}

/// # CoordinateMetadata Interface
///
/// Represents metadata associated with a coordinate, including its reference system and epoch.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CoordinateMetadata {
    /// The schema URL or identifier.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Indicates the type of object. Always "CoordinateMetadata" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'CoordinateMetadata';
    /// The coordinate reference system associated with the coordinate.
    pub crs: CRS,
    /// The epoch of the coordinate.
    #[serde(rename = "coordinateEpoch", skip_serializing_if = "Option::is_none")]
    pub coordinate_epoch: Option<f64>,
}

/// The subtype of the coordinate system.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum CoordinateSystemSubtype {
    /// Cartesian
    #[serde(rename = "Cartesian")]
    #[default]
    Cartesian,
    /// Spherical
    #[serde(rename = "spherical")]
    Spherical,
    /// Ellipsoidal
    #[serde(rename = "ellipsoidal")]
    Ellipsoidal,
    /// Vertical
    #[serde(rename = "vertical")]
    Vertical,
    /// Ordinal
    #[serde(rename = "ordinal")]
    Ordinal,
    /// Parametric
    #[serde(rename = "parametric")]
    Parametric,
    /// Affine
    #[serde(rename = "affine")]
    Affine,
    /// TemporalDateTime
    #[serde(rename = "TemporalDateTime")]
    TemporalDateTime,
    /// TemporalCount
    #[serde(rename = "TemporalCount")]
    TemporalCount,
    /// TemporalMeasure
    #[serde(rename = "TemporalMeasure")]
    TemporalMeasure,
}

/// # CoordinateSystem Interface
///
/// Represents a coordinate system, including its subtype and axes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CoordinateSystem {
    /// The schema URL or identifier.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Indicates the type of object. Always "CoordinateSystem" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'CoordinateSystem';
    /// The name of the coordinate system.
    /// NOTE: Should be String but its often missing
    pub name: String,
    /// The subtype of the coordinate system.
    pub subtype: CoordinateSystemSubtype,
    /// The axes of the coordinate system.
    pub axis: Vec<Axis>,
    /// An identifier for the coordinate system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the coordinate system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
}

/// # Transformation Interface
///
/// Represents a transformation between two coordinate reference systems.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Transformation {
    /// Type identifier
    #[serde(rename = "type")]
    pub r#type: String, // 'Transformation';
    /// Name of the transformation
    pub name: String,
    /// Source CRS
    pub source_crs: CRS,
    /// Target CRS
    pub target_crs: CRS,
    /// Interpolation CRS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation_crs: Option<CRS>,
    /// Transformation method
    pub method: Method,
    /// Transformation parameters
    pub parameters: Vec<ParameterValue>,
    /// Transformation accuracy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<String>,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # TemporalCRS Interface
///
/// Represents a temporal coordinate reference system, which defines time-based coordinates.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TemporalCRS {
    /// Indicates the type of CRS. Always "TemporalCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'TemporalCRS';
    /// The name of the temporal CRS.
    pub name: String,
    /// The temporal datum associated with the CRS.
    pub datum: TemporalDatum,
    /// The coordinate system used in the temporal CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # TemporalDatum Interface
///
/// Represents the temporal datum associated with a temporal CRS.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TemporalDatum {
    /// Indicates the type of datum. Always "TemporalDatum" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'TemporalDatum';
    /// The name of the temporal datum.
    pub name: String,
    /// The calendar system used for the datum.
    pub calendar: String,
    /// The time origin of the temporal datum, typically an ISO 8601 date/time string.
    pub time_origin: String,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # VerticalCRS Interface
///
/// Represents a vertical coordinate reference system, which is used for height or depth measurements.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct VerticalCRS {
    /// Indicates the type of CRS. Always "VerticalCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'VerticalCRS';
    /// The name of the vertical CRS.
    pub name: String,
    /// The vertical datum associated with the CRS.
    /// One and only one of `datum` or `datum_ensemble` must be provided.
    /// Can only be a `VerticalReferenceFrame` or a `DynamicVerticalReferenceFrame`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datum: Option<Datum>,
    /// The datum ensemble associated with the CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datum_ensemble: Option<DatumEnsemble>,
    /// The coordinate system used in the vertical CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,
    /// The geoid model associated with the vertical CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geoid_model: Option<GeoidModel>,
    /// An array of geoid models associated with the vertical CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geoid_models: Option<Vec<GeoidModel>>,
    /// An array of deformation models associated with the vertical CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deformation_models: Option<Vec<DeformationModel>>,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # VerticalReferenceFrame Interface
///
/// Represents the vertical reference frame associated with a vertical CRS.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct VerticalReferenceFrame {
    /// Indicates the type of reference frame. Always "VerticalReferenceFrame" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'VerticalReferenceFrame';
    /// The name of the vertical reference frame.
    pub name: String,
    /// The anchor point of the reference frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// The epoch of the anchor point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_epoch: Option<f64>,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # DynamicVerticalReferenceFrame Interface
///
/// Represents a dynamic vertical reference frame.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DynamicVerticalReferenceFrame {
    /// Indicates the type of reference frame. Always "DynamicVerticalReferenceFrame" for this interface.
    #[serde(rename = "type")]
    pub r#type: String, // 'DynamicVerticalReferenceFrame';
    /// The name of the reference frame.
    pub name: String,
    /// The anchor point of the reference frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// The epoch of the anchor point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_epoch: Option<f64>,
    /// The frame reference epoch for the dynamic reference frame.
    pub frame_reference_epoch: f64,
    /// Base Properties
    #[serde(flatten)]
    pub object_usage: ObjectUsage,
}

/// # GeoidModel Interface
///
/// Represents a geoid model associated with a vertical CRS.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeoidModel {
    /// The name of the geoid model.
    pub name: String,
    /// The interpolation CRS for the geoid model.
    pub interpolation_crs: Option<Box<CRS>>,
    /// An identifier for the geoid model.
    pub id: Option<Id>,
}

/// # Object Usage
///
/// Represents common variables across all coordinate reference systems.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ObjectUsage {
    /// The schema URL or identifier.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// The scope of the CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// The area of use for the CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    /// The bounding box of the CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<ProjBBox>,
    /// The vertical extent of the CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_extent: Option<VerticalExtent>,
    /// The temporal extent of the CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_extent: Option<TemporalExtent>,
    /// An array of usages for the CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usages: Option<Vec<Usage>>,
    /// Remarks or additional information about the CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    /// An identifier for the CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id() {
        let json = r#"{
            "authority": "EPSG",
            "code": 8251
        }"#;

        let id: Id = serde_json::from_str(json).unwrap();

        assert_eq!(id.authority, "EPSG");
        assert_eq!(id.code.to_u64(), 8251);
    }

    #[test]
    fn it_works() {
        let json = r#"{
                "type": "GeographicCRS",
                "name": "NAD83(CSRS)v6",
                "datum": {
                    "type": "GeodeticReferenceFrame",
                    "name": "North American Datum of 1983 (CSRS) version 6",
                    "ellipsoid": {
                    "name": "GRS 1980",
                    "semi_major_axis": 6378137,
                    "inverse_flattening": 298.257222101
                    }
                },
                "coordinate_system": {
                    "type": "CoordinateSystem",
                    "name": "Geodetic",
                    "subtype": "ellipsoidal",
                    "axis": [
                        {
                            "name": "Geodetic latitude",
                            "abbreviation": "Lat",
                            "direction": "north",
                            "unit": "degree"
                        },
                        {
                            "name": "Geodetic longitude",
                            "abbreviation": "Lon",
                            "direction": "east",
                            "unit": "degree"
                        },
                        {
                            "name": "Ellipsoidal height",
                            "abbreviation": "h",
                            "direction": "up",
                            "unit": "metre"
                        }
                    ]
                },
                "scope": "Geodesy.",
                "area": "Canada - onshore and offshore - Alberta; British Columbia; Manitoba; New Brunswick; Newfoundland and Labrador; Northwest Territories; Nova Scotia; Nunavut; Ontario; Prince Edward Island; Quebec; Saskatchewan; Yukon.",
                "bbox": {
                    "south_latitude": 38.21,
                    "west_longitude": -141.01,
                    "north_latitude": 86.46,
                    "east_longitude": -40.73
                },
                "id": {
                    "authority": "EPSG",
                    "code": 8251
                }
            }"#;

        let proj_json: GeodeticCRS = serde_json::from_str(json).unwrap();
        assert_eq!(proj_json.r#type, "GeographicCRS");
    }

    #[test]
    fn proj_crs() {
        let json = r#"{
            "$schema": "https://proj.org/schemas/v0.7/projjson.schema.json",
            "type": "ProjectedCRS",
            "name": "WGS 84 / Pseudo-Mercator",
            "base_crs": {
                "name": "WGS 84",
                "datum_ensemble": {
                    "name": "World Geodetic System 1984 ensemble",
                    "members": [
                        {
                            "name": "World Geodetic System 1984 (Transit)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1166
                            }
                        },
                        {
                            "name": "World Geodetic System 1984 (G730)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1152
                            }
                        },
                        {
                            "name": "World Geodetic System 1984 (G873)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1153
                            }
                        },
                        {
                            "name": "World Geodetic System 1984 (G1150)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1154
                            }
                        },
                        {
                            "name": "World Geodetic System 1984 (G1674)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1155
                            }
                        },
                        {
                            "name": "World Geodetic System 1984 (G1762)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1156
                            }
                        },
                        {
                            "name": "World Geodetic System 1984 (G2139)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1309
                            }
                        },
                        {
                            "name": "World Geodetic System 1984 (G2296)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1383
                            }
                        }
                    ],
                    "ellipsoid": {
                        "name": "WGS 84",
                        "semi_major_axis": 6378137,
                        "inverse_flattening": 298.257223563
                    },
                    "accuracy": "2.0",
                    "id": {
                        "authority": "EPSG",
                        "code": 6326
                    }
                },
                "coordinate_system": {
                    "subtype": "ellipsoidal",
                    "axis": [
                        {
                            "name": "Geodetic latitude",
                            "abbreviation": "Lat",
                            "direction": "north",
                            "unit": "degree"
                        },
                        {
                            "name": "Geodetic longitude",
                            "abbreviation": "Lon",
                            "direction": "east",
                            "unit": "degree"
                        }
                    ]
                },
                "id": {
                    "authority": "EPSG",
                    "code": 4326
                }
            },
            "conversion": {
                "name": "Popular Visualisation Pseudo-Mercator",
                "method": {
                    "name": "Popular Visualisation Pseudo Mercator",
                    "id": {
                        "authority": "EPSG",
                        "code": 1024
                    }
                },
                "parameters": [
                    {
                        "name": "Latitude of natural origin",
                        "value": 0,
                        "unit": "degree",
                        "id": {
                            "authority": "EPSG",
                            "code": 8801
                        }
                    },
                    {
                        "name": "Longitude of natural origin",
                        "value": 0,
                        "unit": "degree",
                        "id": {
                            "authority": "EPSG",
                            "code": 8802
                        }
                    },
                    {
                        "name": "False easting",
                        "value": 0,
                        "unit": "metre",
                        "id": {
                            "authority": "EPSG",
                            "code": 8806
                        }
                    },
                    {
                        "name": "False northing",
                        "value": 0,
                        "unit": "metre",
                        "id": {
                            "authority": "EPSG",
                            "code": 8807
                        }
                    }
                ]
            },
            "coordinate_system": {
                "subtype": "Cartesian",
                "axis": [
                    {
                        "name": "Easting",
                        "abbreviation": "X",
                        "direction": "east",
                        "unit": "metre"
                    },
                    {
                        "name": "Northing",
                        "abbreviation": "Y",
                        "direction": "north",
                        "unit": "metre"
                    }
                ]
            },
            "scope": "Web mapping and visualisation.",
            "area": "World between 85.06°S and 85.06°N.",
            "bbox": {
                "south_latitude": -85.06,
                "west_longitude": -180,
                "north_latitude": 85.06,
                "east_longitude": 180
            },
            "id": {
                "authority": "EPSG",
                "code": 3857
            }
        }"#;

        let proj: ProjectedCRS = serde_json::from_str(json).unwrap();
        assert_eq!(proj.object_usage.id.unwrap().code.to_u64(), 3857);

        let full: ProjJSON = serde_json::from_str(json).unwrap();
        if let ProjJSON::CRS(crs) = full {
            if let CRS::ProjectedCRS(proj) = *crs {
                assert_eq!(proj.object_usage.id.unwrap().code.to_u64(), 3857);
            }
        } else {
            panic!("Expected ProjectedCRS");
        }
    }
}
