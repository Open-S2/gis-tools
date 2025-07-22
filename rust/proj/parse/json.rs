// https://docs.ogc.org/is/18-010r7/18-010r7.html
use crate::proj::{
    AxisSwapConverter, CoordinateStep, Proj, ProjectionTransform, Step, derive_eccentricity,
    derive_sphere, name_to_unit, to_camel_case,
};
use alloc::{
    boxed::Box,
    format,
    rc::Rc,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::cell::RefCell;
use libm::fabs;
use serde::{Deserialize, Serialize};

// NOTE: Because the 0.7 spec is not always being followed correctly by generators, serde(default) is applied to everything

/// Helper functions that default to do nothing
pub trait ToProjJSON {
    /// Set Usage
    fn set_usage(&mut self, _usage: ObjectUsage) {}
    /// Set an Anchor
    fn set_anchor(&mut self, _anchor: String) {}
    /// Set a Unit
    fn set_unit(&mut self, _unit: Unit) {}
    /// Set an Id
    fn set_id(&mut self, _id: Id) {}
    /// Set an Axis
    fn set_axis(&mut self, _axis: Axis) {}
    /// Set a CoordinateSystem
    fn set_coordinate_system(&mut self, _cs: CoordinateSystem) {}
    /// Set Temporal Extent
    fn set_temporal_extent(&mut self, _extent: TemporalExtent) {}
    /// Set Vertical Extent
    fn set_vertical_extent(&mut self, _extent: VerticalExtent) {}
    /// Set Bounding Box
    fn set_bbox(&mut self, _bbox: ProjBBox) {}
    /// Set Area
    fn set_area(&mut self, _area: Option<String>) {}
    /// Set a Method
    fn set_method(&mut self, _method: Method) {}
    /// Set a DatumEnsemble
    fn set_ensemble(&mut self, _ensemble: DatumEnsemble) {}
    /// Set a Member
    fn set_member(&mut self, _member: DatumEnsembleMember) {}
    /// Set an Ellipsoid
    fn set_ellipsoid(&mut self, _ellipsoid: Ellipsoid) {}
    /// Set Accuracy
    fn set_accuracy(&mut self, _accuracy: String) {}
    /// Set Epoch
    fn set_epoch(&mut self, _epoch: f64) {}
    /// Set a frame epoch
    fn set_frame_epoch(&mut self, _epoch: f64) {}
    /// Set a datum
    fn set_datum(&mut self, _datum: Datum) {}
    /// Set a Parameter
    fn set_parameter(&mut self, _parameter: ParameterValue) {}
    /// Set a Meridian
    fn set_meridian(&mut self, _meridian: Meridian) {}
    /// Set a PrimeMeridian
    fn set_prime_meridian(&mut self, _prime_meridian: PrimeMeridian) {}
    /// Set a Conversion
    fn set_conversion(&mut self, _conversion: Conversion) {}
    /// Set a GeodeticCRS
    fn set_geodetic_crs(&mut self, _geodetic_crs: GeodeticCRS) {}
    /// Set a ProjectedCRS
    fn set_projected_crs(&mut self, _projected_crs: ProjectedCRS) {}
    /// Set the name
    fn set_projection(&mut self, _name: String) {}
    /// Set the order
    fn set_order(&mut self, _order: usize) {}
}

/// # Schema for PROJJSON (v0.7)
/// @see https://proj.org/schemas/v0.7/projjson.schema.json
/// @see https://docs.ogc.org/is/18-010r7/18-010r7.html#1
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
impl ToProjJSON for ProjJSON {
    fn set_id(&mut self, id: Id) {
        match self {
            ProjJSON::CRS(crs) => crs.set_id(id),
            ProjJSON::Datum(datum) => datum.set_id(id),
            ProjJSON::DatumEnsemble(ensemble) => ensemble.set_id(id),
            ProjJSON::PrimeMeridian(prime_meridian) => prime_meridian.set_id(id),
            ProjJSON::Ellipsoid(ellipsoid) => ellipsoid.set_id(id),
            ProjJSON::SingleOperation(operation) => operation.set_id(id),
            ProjJSON::ConcatenatedOperation(operation) => operation.set_id(id),
            ProjJSON::CoordinateMetadata(metadata) => metadata.set_id(id),
        }
    }
    fn set_datum(&mut self, datum: Datum) {
        *self = ProjJSON::Datum(Box::new(datum));
    }
    fn set_ensemble(&mut self, ensemble: DatumEnsemble) {
        *self = ProjJSON::DatumEnsemble(Box::new(ensemble));
    }
    fn set_prime_meridian(&mut self, prime_meridian: PrimeMeridian) {
        *self = ProjJSON::PrimeMeridian(Box::new(prime_meridian));
    }
    fn set_geodetic_crs(&mut self, geodetic_crs: GeodeticCRS) {
        *self = ProjJSON::CRS(Box::new(CRS::GeodeticCRS(Box::new(geodetic_crs))));
    }
    fn set_projected_crs(&mut self, projected_crs: ProjectedCRS) {
        *self = ProjJSON::CRS(Box::new(CRS::ProjectedCRS(Box::new(projected_crs))));
    }
}
impl ProjJSON {
    /// Convert a Proj JSON to a ProjectionTransform (Proj constants with steps)
    pub fn to_projection_transform(&self) -> ProjectionTransform {
        let mut proj_transform = ProjectionTransform::default();
        match self {
            ProjJSON::CRS(crs) => crs.to_projection_transform(&mut proj_transform),
            _ => panic!("Unsupported operation"),
        }

        proj_transform
    }
}

/// Coordinate Reference System
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
impl ToProjJSON for CRS {
    fn set_id(&mut self, id: Id) {
        match self {
            CRS::BoundCRS(crs) => crs.set_id(id),
            CRS::CompoundCRS(crs) => crs.set_id(id),
            CRS::DerivedEngineeringCRS(crs) => crs.set_id(id),
            CRS::DerivedGeodeticCRS(crs) => crs.set_id(id),
            CRS::DerivedParametricCRS(crs) => crs.set_id(id),
            CRS::DerivedProjectedCRS(crs) => crs.set_id(id),
            CRS::DerivedTemporalCRS(crs) => crs.set_id(id),
            CRS::DerivedVerticalCRS(crs) => crs.set_id(id),
            CRS::EngineeringCRS(crs) => crs.set_id(id),
            CRS::GeodeticCRS(crs) => crs.set_id(id),
            CRS::ParametricCRS(crs) => crs.set_id(id),
            CRS::ProjectedCRS(crs) => crs.set_id(id),
            CRS::TemporalCRS(crs) => crs.set_id(id),
            CRS::VerticalCRS(crs) => crs.set_id(id),
        }
    }
    fn set_geodetic_crs(&mut self, geodetic_crs: GeodeticCRS) {
        *self = CRS::GeodeticCRS(geodetic_crs.into());
    }
    fn set_projected_crs(&mut self, projected_crs: ProjectedCRS) {
        *self = CRS::ProjectedCRS(projected_crs.into());
    }
}
impl CRS {
    /// Convert a Proj JSON to a ProjectionTransform (Proj constants with steps)
    pub fn to_projection_transform(&self, proj_transform: &mut ProjectionTransform) {
        match self {
            CRS::GeodeticCRS(crs) => crs.to_projection_transform(proj_transform),
            CRS::ProjectedCRS(crs) => crs.to_projection_transform(proj_transform),
            _ => panic!("Unsupported operation"),
        };
    }
}

/// # Datum Interface
///
/// Represents a datum which can be one of several types of reference frames or datums.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
impl Datum {
    /// Convert a GeodeticCRS to a ProjectionTransform
    pub fn to_projection_transform(&self, proj_transform: &mut ProjectionTransform) {
        // TODO: Implement for all
        match self {
            Datum::GeodeticReferenceFrame(d) => d.to_projection_transform(proj_transform),
            _ => todo!(),
            // Datum::VerticalReferenceFrame(d) => d.to_projection_transform(proj_transform),
            // Datum::DynamicGeodeticReferenceFrame(d) => d.to_projection_transform(proj_transform),
            // Datum::DynamicVerticalReferenceFrame(d) => d.to_projection_transform(proj_transform),
            // Datum::TemporalDatum(d) => d.to_projection_transform(proj_transform),
            // Datum::ParametricDatum(d) => d.to_projection_transform(proj_transform),
            // Datum::EngineeringDatum(d) => d.to_projection_transform(proj_transform),
        };
    }
}
impl Default for Datum {
    fn default() -> Self {
        Datum::GeodeticReferenceFrame(GeodeticReferenceFrame::default())
    }
}
impl ToProjJSON for Datum {
    fn set_id(&mut self, id: Id) {
        match self {
            Datum::GeodeticReferenceFrame(d) => d.set_id(id),
            Datum::VerticalReferenceFrame(d) => d.set_id(id),
            Datum::DynamicGeodeticReferenceFrame(d) => d.set_id(id),
            Datum::DynamicVerticalReferenceFrame(d) => d.set_id(id),
            Datum::TemporalDatum(d) => d.set_id(id),
            Datum::ParametricDatum(d) => d.set_id(id),
            Datum::EngineeringDatum(d) => d.set_id(id),
        }
    }
    fn set_prime_meridian(&mut self, pm: PrimeMeridian) {
        match self {
            Datum::GeodeticReferenceFrame(d) => {
                d.set_prime_meridian(pm);
            }
            Datum::DynamicGeodeticReferenceFrame(d) => {
                d.set_prime_meridian(pm);
            }
            _ => {}
        }
    }
}

/// # Geographic Bounding Box
///
/// ## Description
/// The geographic bounding box is an optional attribute which describes a "north up" area.
/// Upper right latitude will be greater than the lower left latitude. Generally the upper right
/// longitude will be greater than the lower left longitude. However when the area crosses the
/// 180° meridian, the value of the lower left longitude will be greater than the value of the
/// upper right longitude.
/// The geographic bounding box is an approximate description of location. For most purposes a
/// coordinate precision of two decimal places of a degree is sufficient. At this resolution the
/// identification of the geodetic CRS to which the bounding box coordinates are referenced is not
/// required.
///
/// ## Requirement
/// Bounding box latitude coordinates shall be given in decimal degrees in the range -90 to +90,
/// longitude coordinates shall be given in decimal degrees in the range -180 to +180 relative to
/// the international reference meridian.
///
/// ## Examples
/// - `BBOX[51.43,2.54,55.77,6.40]`
/// - `BBOX[-55.95,160.60,-25.88,-171.20]`
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ProjBBox {
    /// The southernmost latitude of the bounding box.
    pub south_latitude: f64,
    /// The westernmost longitude of the bounding box.
    pub west_longitude: f64,
    /// The northernmost latitude of the bounding box.
    pub north_latitude: f64,
    /// The easternmost longitude of the bounding box.
    pub east_longitude: f64,
}

/// # VerticalExtent
///
/// ## Description
/// Vertical extent is an optional attribute which describes a height range over which a CRS or
/// coordinate operation is applicable. Depths have negative height values. Vertical extent is an
/// approximate description of location; heights are relative to an unspecified mean sea level.
///
/// ## Requirement
/// If vertical extent units are not stated they shall be assumed to be metres.
///
/// ## Examples
/// - `VERTICALEXTENT[-1000,0,LENGTHUNIT["metre",1.0]]`
/// - `VERTICALEXTENT[-1000,0]` (where the heights are implicitly in metres).
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct VerticalExtent {
    /// Minimum height
    pub minimum: f64,
    /// Maximum height
    pub maximum: f64,
    /// Unit of measurement
    pub unit: Unit,
}

/// # Temporal Extent
///
/// ## Description
/// Temporal extent is an optional attribute which describes a date or time range over which a CRS
/// or coordinate operation is applicable. The format for date and time values is defined in
/// ISO/IEC 9075-2. Start time is earlier than end time.
///
/// ## Requirement
/// `<temporal extent end>` should have the same data type (dateTime or quoted Latin text) as
/// `<temporal extent start>`.
///
/// ## Examples
/// - `TIMEEXTENT[2013-01-01,2013-12-31]`
/// - `TIMEEXTENT["Jurassic","Quaternary"]`
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
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
pub enum ProjValue {
    /// Boolean case
    Bool(bool),
    /// Float case
    F64(f64),
    /// Integer case
    I64(i64),
    /// String case
    String(String),
}
impl Default for ProjValue {
    fn default() -> Self {
        ProjValue::String("".into())
    }
}
impl PartialEq for ProjValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ProjValue::Bool(a), ProjValue::Bool(b)) => a == b,
            (ProjValue::F64(a), ProjValue::F64(b)) => fabs(*a - *b) < f64::EPSILON,
            (ProjValue::I64(a), ProjValue::I64(b)) => a == b,
            (ProjValue::String(a), ProjValue::String(b)) => a == b,
            _ => false,
        }
    }
}
impl ProjValue {
    /// Get the boolean representation
    pub fn bool(&self) -> bool {
        match self {
            ProjValue::Bool(b) => *b,
            ProjValue::F64(f) => *f != 0.0,
            ProjValue::I64(i) => *i != 0,
            ProjValue::String(s) => s.to_lowercase() == "true" || self.i64() != 0,
        }
    }
    /// Get the float representation
    pub fn f64(&self) -> f64 {
        match self {
            ProjValue::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            ProjValue::F64(f) => *f,
            ProjValue::I64(i) => *i as f64,
            ProjValue::String(s) => s.parse().unwrap_or(0.0),
        }
    }
    /// Get the integer representation
    pub fn i64(&self) -> i64 {
        match self {
            ProjValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            ProjValue::F64(f) => *f as i64,
            ProjValue::I64(i) => *i,
            ProjValue::String(s) => s.parse().unwrap_or(0),
        }
    }
    /// Get the string representation
    pub fn string(&self) -> String {
        match self {
            ProjValue::Bool(b) => b.to_string(),
            ProjValue::F64(f) => f.to_string(),
            ProjValue::I64(i) => i.to_string(),
            ProjValue::String(s) => s.clone(),
        }
    }
}
impl From<&str> for ProjValue {
    fn from(v: &str) -> ProjValue {
        ProjValue::String(v.into())
    }
}
impl From<String> for ProjValue {
    fn from(v: String) -> ProjValue {
        ProjValue::String(v)
    }
}
impl From<ProjValue> for String {
    fn from(v: ProjValue) -> String {
        match v {
            ProjValue::String(s) => s,
            _ => "".into(),
        }
    }
}
impl From<f64> for ProjValue {
    fn from(v: f64) -> ProjValue {
        ProjValue::F64(v)
    }
}

/// # Identifier
///
/// ## Description
/// Identifier is an optional attribute which references an external description of the object and
/// which may be applied to a coordinate reference system, a coordinate operation or a bound CRS.
/// Multiple identifiers may be given for any object.
///
/// ## Examples
/// - `ID["Authority name","Abcd_Ef",7.1]`
/// - `ID["EPSG",4326]`
/// - `ID["EPSG",4326,URI["urn:ogc:def:crs:EPSG::4326"]]`
/// - `ID["EuroGeographics","ES_ED50 (BAL99) to ETRS89","2001-04-20"]`
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Id {
    /// Authority issuing the identifier
    pub authority: String,
    /// Code associated with the identifier
    pub code: ProjValue,
    /// Version of the identifier
    /// NOTE: This is not supposed to be optional, but it rarely shows up
    pub version: Option<ProjValue>,
    /// Citation of the authority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_citation: Option<String>,
    /// URI reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// Identifiers list
pub type Ids = Vec<Id>;

/// # Parameter
///
/// ## Description
/// Parameter name is for human readability. For interoperability it is the method formula and its parameters that are critical in determining the equivalence of methods. See Annex F. Identifiers for commonly encountered map projection methods are given in F.2; their parameters are listed in F.3.
///
/// The map projection parameters required are specific to the map projection method and will be listed sequentially. The order within the sequence is not significant but should be logical.
///
/// <map projection parameter unit> is an optional attribute, for reasons of backward compatibility. Best practice is that it is included explicitly in WKT strings.
///
/// ## Requirements
/// If <map projection parameter unit> is omitted from <map projection parameter> then:
/// - Map parameter values that are lengths shall be given in metres.
/// - Map projection parameter values that are angles shall be given in decimal degrees.
/// - Map projection parameters that are unitless (for example scale factor) shall be given as a number which is close to or is unity (1.0).
///
/// ## Examples
/// - `PARAMETER["semi_major",6378137.0]` - Defines a parameter named "semi_major" with a numeric value.
/// - `PARAMETER["towgs84","8,-183,-105,0,0,0,0"]` - Defines a parameter named "towgs84" with a string value.
/// - `PARAMETER["central_meridian",0.0,UNIT["degree",0.0174532925199433]]` - Defines a parameter with a name, numeric value, and a unit.
/// - `PARAMETER["standard_parallel_1",30.0,ID["EPSG",8831]]` - Defines a parameter with a name, numeric value, and an identifier.
/// - `PARAMETER["latitude_of_origin",0.0,UNIT["degree",0.0174532925199433],ID["EPSG",8821]]` - Defines a parameter with a name, numeric value, unit, and identifier.
/// - `PARAMETER["is_sphere",TRUE]` - Defines a boolean parameter.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ParameterValue {
    /// Schema reference
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Type identifier - always 'ParameterValue'
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'ParameterValue';
    /// Name of the parameter
    pub name: String,
    /// Parameter value, which can be a string or number
    pub value: ProjValue,
    /// Optional unit of measurement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// Alternative identifiers
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Ids,
    /// NOT PART OF SPEC
    pub is_file: bool,
}
impl ParameterValue {
    /// Get the radians of the value
    pub fn rad(&self) -> f64 {
        // If no unit, build from name_to_unit, if angle, it's degree, otherwise metre
        let unit = self.unit.clone().unwrap_or_else(|| name_to_unit(&self.name));
        self.value.f64() * unit.rad()
    }
}
impl From<&ParameterValue> for ProjValue {
    fn from(p_value: &ParameterValue) -> Self {
        ProjValue::F64(p_value.rad())
    }
}
impl ToProjJSON for ParameterValue {
    fn set_id(&mut self, id: Id) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if !self.ids.is_empty() {
            self.ids.push(id);
        } else if let Some(i) = self.id.clone() {
            self.ids.extend(vec![i, id]);
            self.id = None;
        } else {
            self.id = Some(id);
        }
    }
    fn set_unit(&mut self, unit: Unit) {
        self.unit = Some(unit);
    }
}

/// # Parametric CRS
///
/// Represents a parametric coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ParametricCRS {
    /// Type identifier - always 'ParametricCRS'
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'ParametricCRS';
    /// Name of the CRS
    pub name: String,
    /// Parametric datum
    pub datum: ParametricDatum,
    /// Coordinate system
    pub coordinate_system: Option<CoordinateSystem>,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for ParametricCRS {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_coordinate_system(&mut self, cs: CoordinateSystem) {
        self.coordinate_system = Some(cs);
    }
    // Pass down to the coordinate system
    fn set_axis(&mut self, axis: Axis) {
        if let Some(ref mut cs) = self.coordinate_system {
            cs.axis.push(axis);
        }
    }
    fn set_unit(&mut self, unit: Unit) {
        if let Some(ref mut cs) = self.coordinate_system {
            for axis in cs.axis.iter_mut() {
                axis.unit = Some(unit.clone());
            }
        }
    }
}

/// # Parametric Datum
///
/// Represents the parametric datum associated with a parametric CRS.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ParametricDatum {
    /// Type identifier - always 'ParametricDatum'
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'ParametricDatum';
    /// Name of the datum
    pub name: String,
    /// Anchor point
    pub anchor: String,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for ParametricDatum {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_anchor(&mut self, anchor: String) {
        self.anchor = anchor;
    }
}

/// # Point Motion Operation
///
/// Represents a point motion operation
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct PointMotionOperation {
    /// Type identifier
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'PointMotionOperation';
    /// Name of the operation
    pub name: String,
    /// Source coordinate reference system
    pub source_crs: CRS,
    /// Method used for point motion
    pub method: Method,
    /// Parameters used in the operation
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterValue>,
    /// Accuracy of the operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<String>,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for PointMotionOperation {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_accuracy(&mut self, accuracy: String) {
        self.accuracy = Some(accuracy);
    }
    fn set_projection(&mut self, name: String) {
        self.method = Method { name, ..Default::default() };
    }
    fn set_method(&mut self, method: Method) {
        self.method = method;
    }
    fn set_parameter(&mut self, parameter: ParameterValue) {
        self.parameters.push(parameter);
    }
}

/// # Method Object
///
/// Defines an operation method with a name and identifier
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Method {
    /// Schema reference
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Type identifier - always 'OperationMethod'
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'OperationMethod';
    /// Name of the method
    pub name: String,
    /// Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// Alternative identifiers
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Ids,
}
impl Method {
    /// Convert a Method to a ProjectionTransform
    pub fn to_projection_transform(&self, proj_transform: &mut ProjectionTransform) {
        // add projection from method
        proj_transform.method = Step::from_method(self, proj_transform.proj.clone()).unwrap();
    }
}
impl ToProjJSON for Method {
    fn set_id(&mut self, id: Id) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if !self.ids.is_empty() {
            self.ids.push(id);
        } else if let Some(i) = self.id.clone() {
            self.ids.extend(vec![i, id]);
            self.id = None;
        } else {
            self.id = Some(id);
        }
    }
}

/// Base Unit - common units as string input
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
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
impl From<&str> for UnitType {
    fn from(s: &str) -> Self {
        match s {
            "LENGTHUNIT" => UnitType::LinearUnit,
            "ANGLEUNIT" => UnitType::AngularUnit,
            "SCALEUNIT" => UnitType::ScaleUnit,
            "TIMEUNIT" => UnitType::TimeUnit,
            "PARAMETRICUNIT" => UnitType::ParametricUnit,
            _ => UnitType::Unit, // UNIT
        }
    }
}

/// Unit Object
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Ids,
}
impl UnitObject {
    /// Set the unit type
    pub fn set_unit_type(&mut self, unit_type: UnitType) {
        self.r#type = unit_type;
    }
    /// Convert to Radian
    pub fn rad(&self) -> f64 {
        match self.r#type {
            UnitType::AngularUnit => self.conversion_factor.unwrap_or(1.0),
            _ => 1.0,
        }
    }
    /// Convert to Meters
    pub fn meters(&self) -> f64 {
        match self.r#type {
            UnitType::LinearUnit => self.conversion_factor.unwrap_or(1.0),
            _ => 1.0,
        }
    }
}
impl ToProjJSON for UnitObject {
    fn set_id(&mut self, id: Id) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if !self.ids.is_empty() {
            self.ids.push(id);
        } else if let Some(i) = self.id.clone() {
            self.ids.extend(vec![i, id]);
            self.id = None;
        } else {
            self.id = Some(id);
        }
    }
}

/// # Unit
///
/// ## Description
/// Defines the unit of measure for lengths or distances, typically used within other spatial
/// extent definitions like `VerticalExtent`. It consists of a unit name (e.g., "metre", "foot")
/// and a corresponding numeric value, often representing the conversion factor to a base unit.
///
/// ## Common naming convention
/// - LENGTHUNIT
/// - ANGLEUNIT
/// - SCALEUNIT
/// - TIMEUNIT
/// - PARAMETRICUNIT
///
/// ## Requirement
/// The WKT representation of a `<length unit>` shall be:
/// ```bnf
/// <length unit> ::= <length unit keyword> <left delimiter> <quoted text> <wkt separator> <number> <right delimiter>
/// <length unit keyword> ::= LENGTHUNIT
/// ```
///
/// Unprovided defaults to BaseUnit->Metre
///
/// ## Examples
/// - `LENGTHUNIT["metre",1.0]`
/// - `LENGTHUNIT["foot",0.3048]`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
impl Unit {
    /// Create a degree unit
    pub fn new_deg() -> Self {
        Unit::BaseUnit(BaseUnit::Degree)
    }
    /// Set the unit type assuming the unit is a UnitObject
    pub fn set_unit_type(&mut self, unit_type: UnitType) {
        match self {
            Unit::BaseUnit(_) => {}
            Unit::UnitObject(unit) => unit.set_unit_type(unit_type),
        };
    }
    /// Get the multiplier for the unit to convert to radians
    pub fn rad(&self) -> f64 {
        match self {
            Unit::BaseUnit(unit) => match unit {
                BaseUnit::Metre => 1.0,
                BaseUnit::Degree => std::f64::consts::PI / 180.0,
                BaseUnit::Unity => 1.0,
            },
            Unit::UnitObject(unit) => unit.rad(),
        }
    }
    /// Get the multiplier for the unit to convert to meters
    pub fn meters(&self) -> f64 {
        match self {
            Unit::BaseUnit(unit) => match unit {
                BaseUnit::Metre => 1.0,
                BaseUnit::Degree => std::f64::consts::PI / 180.0,
                BaseUnit::Unity => 1.0,
            },
            Unit::UnitObject(unit) => unit.meters(),
        }
    }
}
impl ToProjJSON for Unit {
    fn set_unit(&mut self, unit: Unit) {
        *self = unit
    }
}

/// # BoundCRS Interface
///
/// Represents a coordinate reference system that is bounded by a source and target CRS with a transformation.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct BoundCRS {
    /// Indicates the type of object. Always "BoundCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The name of the bound CRS.
    pub name: Option<String>,
    /// The source coordinate reference system.
    pub source_crs: Box<CRS>,
    /// The target coordinate reference system.
    pub target_crs: Box<CRS>,
    /// The transformation applied to convert between the source and target CRS.
    pub transformation: AbridgedTransformation,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for BoundCRS {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
}

/// # ConcatenatedOperation Interface
///
/// Represents an operation that is composed of multiple steps, transforming one CRS to another.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ConcatenatedOperation {
    /// Indicates the type of object. Always "ConcatenatedOperation" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'ConcatenatedOperation';
    /// The name of the concatenated operation.
    pub name: String,
    /// The source coordinate reference system.
    pub source_crs: CRS,
    /// The target coordinate reference system.
    pub target_crs: CRS,
    /// An array of individual steps in the concatenated operation.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<SingleOperation>,
    /// The accuracy of the concatenated operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<String>,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for ConcatenatedOperation {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_accuracy(&mut self, accuracy: String) {
        self.accuracy = Some(accuracy);
    }
}

/// # AbridgedTransformation Interface
///
/// Represents an abridged transformation used for converting between different coordinate reference systems.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct AbridgedTransformation {
    /// The schema URL or identifier.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Indicates the type of object. Always "AbridgedTransformation" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'AbridgedTransformation';
    /// The name of the transformation.
    pub name: String,
    /// The source coordinate reference system, only present if it differs from the source CRS of the bound CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_crs: Option<Box<CRS>>,
    /// The method used for the transformation.
    pub method: Method,
    /// The parameters used in the transformation.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterValue>,
    /// An identifier for the axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the axis.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Ids,
}
impl ToProjJSON for AbridgedTransformation {
    fn set_projection(&mut self, name: String) {
        self.method = Method { name, ..Default::default() };
    }
    fn set_method(&mut self, method: Method) {
        self.method = method;
    }
    fn set_parameter(&mut self, parameter: ParameterValue) {
        self.parameters.push(parameter);
    }
    fn set_id(&mut self, id: Id) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if !self.ids.is_empty() {
            self.ids.push(id);
        } else if let Some(i) = self.id.clone() {
            self.ids.extend(vec![i, id]);
            self.id = None;
        } else {
            self.id = Some(id);
        }
    }
}

/// # CompoundCRS Interface
///
/// Represents a compound coordinate reference system, consisting of multiple components.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct CompoundCRS {
    /// Indicates the type of object. Always "CompoundCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'CompoundCRS';
    /// The name of the compound CRS.
    pub name: String,
    /// An array of coordinate reference systems that make up the compound CRS.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CRS>,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for CompoundCRS {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
}

/// # EngineeringCRS Interface
///
/// Represents an engineering coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct EngineeringCRS {
    /// Indicates the type of CRS. Always "EngineeringCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'EngineeringCRS';
    /// The name of the engineering CRS.
    pub name: String,
    /// The engineering datum associated with this CRS.
    pub datum: EngineeringDatum,
    /// The coordinate system used in this CRS.
    pub coordinate_system: Option<CoordinateSystem>,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for EngineeringCRS {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_coordinate_system(&mut self, cs: CoordinateSystem) {
        self.coordinate_system = Some(cs);
    }
    // Pass down to the coordinate system
    fn set_axis(&mut self, axis: Axis) {
        if let Some(ref mut cs) = self.coordinate_system {
            cs.axis.push(axis);
        }
    }
    fn set_unit(&mut self, unit: Unit) {
        if let Some(ref mut cs) = self.coordinate_system {
            for axis in cs.axis.iter_mut() {
                axis.unit = Some(unit.clone());
            }
        }
    }
}

/// # EngineeringDatum Interface
///
/// Represents the datum associated with an engineering CRS.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct EngineeringDatum {
    /// Indicates the type of datum. Always "EngineeringDatum" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'EngineeringDatum';
    /// The name of the datum.
    pub name: String,
    /// Anchor point of the datum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for EngineeringDatum {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_anchor(&mut self, anchor: String) {
        self.anchor = Some(anchor);
    }
}

/// Axis Direction defines an axis direction
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
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
impl From<String> for AxisDirection {
    fn from(s: String) -> Self {
        serde_json::from_str(&format!("\"{}\"", &s))
            .or_else(|_| serde_json::from_str(&format!("\"{}\"", to_camel_case(&s))))
            .unwrap_or_else(|_| {
                // try a few cases otherwise default
                match s.to_lowercase().as_str() {
                    "n" => AxisDirection::North,
                    "ne" | "northeast" => AxisDirection::NorthEast,
                    "e" => AxisDirection::East,
                    "se" | "southeast" => AxisDirection::SouthEast,
                    "s" => AxisDirection::South,
                    "sw" | "southwest" => AxisDirection::SouthWest,
                    "w" => AxisDirection::West,
                    "nw" | "northwest" => AxisDirection::NorthWest,
                    _ => AxisDirection::Unspecified,
                }
            })
    }
}

/// Axis Range Meaning
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Axis {
    /// Indicates the type of axis. Always "Axis" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'Axis';
    /// The name of the axis.
    pub name: String,
    /// Abbreviation for the axis name.
    pub abbreviation: String,
    /// The direction of the axis.
    /// Examples include north, east, up, down, geocentricX, geocentricY, geocentricZ, etc.
    pub direction: AxisDirection,
    /// The order of the axis.
    /// Not part of the specification, added for convenience.
    pub order: usize,
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Ids,
}
impl Axis {
    /// Adjust the order if needed
    pub fn adjust_if_needed(&mut self) {
        if self.order != 0 {
            return;
        }
        let name = self.name.to_lowercase();
        self.order = if name.contains("longitude")
            || name.contains("northing")
            || name.contains("(lon)")
            || name.contains("(Y)")
            || name.contains("(N)")
        {
            2
        } else if name.contains("z") {
            3
        } else {
            0
        };
    }
}
impl ToProjJSON for Axis {
    fn set_id(&mut self, id: Id) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if !self.ids.is_empty() {
            self.ids.push(id);
        } else if let Some(i) = self.id.clone() {
            self.ids.extend(vec![i, id]);
            self.id = None;
        } else {
            self.id = Some(id);
        }
    }
    fn set_unit(&mut self, unit: Unit) {
        self.unit = Some(unit);
    }
    fn set_meridian(&mut self, meridian: Meridian) {
        self.meridian = Some(meridian);
    }
    fn set_order(&mut self, order: usize) {
        self.order = order;
    }
}

/// # Meridian Interface
///
/// Represents a meridian, which defines the longitude for an axis.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Meridian {
    /// The schema URL or identifier.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Indicates the type of meridian. Always "Meridian" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'Meridian';
    /// The longitude of the meridian.
    pub longitude: ValueInDegreeOrValueAndUnit,
    /// An identifier for the meridian.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the meridian.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Ids,
}
impl ToProjJSON for Meridian {
    fn set_id(&mut self, id: Id) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if !self.ids.is_empty() {
            self.ids.push(id);
        } else if let Some(i) = self.id.clone() {
            self.ids.extend(vec![i, id]);
            self.id = None;
        } else {
            self.id = Some(id);
        }
    }
}

/// # ValueAndUnit Interface
///
/// Represents a value paired with a unit of measurement.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ValueAndUnit {
    /// The numeric degree value.
    pub value: f64,
    /// The unit of measurement.
    pub unit: Unit,
}
impl ValueAndUnit {
    /// Get the radians of the value
    pub fn rad(&self) -> f64 {
        self.value * self.unit.rad()
    }
}

/// Value in Degrees or Value and Unit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ValueInDegreeOrValueAndUnit {
    /// Float value (degrees)
    F64(f64),
    /// Value and Unit Object
    ValueAndUnit(ValueAndUnit),
}
impl ValueInDegreeOrValueAndUnit {
    /// Create a new `ValueInDegreeOrValueAndUnit` from a unit and value
    pub fn from_unit(unit: Unit, value: f64) -> Self {
        ValueInDegreeOrValueAndUnit::ValueAndUnit(ValueAndUnit { value, unit })
    }
    /// Get the radians of the value
    pub fn rad(&self) -> f64 {
        match self {
            ValueInDegreeOrValueAndUnit::F64(value) => (*value).to_radians(),
            ValueInDegreeOrValueAndUnit::ValueAndUnit(value) => value.rad(),
        }
    }
}
impl Default for ValueInDegreeOrValueAndUnit {
    fn default() -> Self {
        ValueInDegreeOrValueAndUnit::F64(0.0)
    }
}
impl ToProjJSON for ValueInDegreeOrValueAndUnit {
    fn set_unit(&mut self, unit: Unit) {
        match self {
            ValueInDegreeOrValueAndUnit::F64(val) => {
                *self =
                    ValueInDegreeOrValueAndUnit::ValueAndUnit(ValueAndUnit { value: *val, unit });
            }
            ValueInDegreeOrValueAndUnit::ValueAndUnit(value) => value.unit = unit,
        }
    }
}

/// Value in Metres or Value and Unit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ValueInMetreOrValueAndUnit {
    /// Float
    F64(f64),
    /// Value and Unit
    ValueAndUnit(ValueAndUnit),
}
impl ValueInMetreOrValueAndUnit {
    /// Create a new `ValueInMetreOrValueAndUnit` from a unit and value
    pub fn from_unit(unit: Unit, value: f64) -> Self {
        ValueInMetreOrValueAndUnit::ValueAndUnit(ValueAndUnit { value, unit })
    }
    /// Get the meters of the value
    pub fn meters(&self) -> f64 {
        match self {
            ValueInMetreOrValueAndUnit::F64(value) => *value,
            ValueInMetreOrValueAndUnit::ValueAndUnit(value) => value.unit.meters() * value.value,
        }
    }
}
impl Default for ValueInMetreOrValueAndUnit {
    fn default() -> Self {
        ValueInMetreOrValueAndUnit::F64(0.0)
    }
}
impl From<f64> for ValueInMetreOrValueAndUnit {
    fn from(value: f64) -> Self {
        ValueInMetreOrValueAndUnit::F64(value)
    }
}

/// # Single Operation
///
/// Represents a single operation, which can be a conversion, transformation, or point motion operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SingleOperation {
    /// Conversion Operation
    Conversion(Box<Conversion>),
    /// Transformation Operation
    Transformation(Box<Transformation>),
    /// Point & Motion Operation
    PointMotionOperation(Box<PointMotionOperation>),
}
impl ToProjJSON for SingleOperation {
    fn set_id(&mut self, id: Id) {
        match self {
            SingleOperation::Conversion(c) => c.set_id(id),
            SingleOperation::Transformation(t) => t.set_id(id),
            SingleOperation::PointMotionOperation(p) => p.set_id(id),
        }
    }
    fn set_conversion(&mut self, conversion: Conversion) {
        *self = SingleOperation::Conversion(Box::new(conversion));
    }
}

/// # DeformationModel Interface
///
/// Represents a deformation model associated with a point motion operation.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DerivedEngineeringCRS {
    /// Indicates the type of coordinate reference system. Always "DerivedEngineeringCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'DerivedEngineeringCRS';
    /// The name of the derived engineering CRS.
    pub name: String,
    /// The base CRS from which this derived CRS is created.
    pub base_crs: EngineeringCRS,
    /// The conversion method applied to the base CRS.
    pub conversion: Conversion,
    /// The coordinate system used in the CRS.
    pub coordinate_system: CoordinateSystem,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for DerivedEngineeringCRS {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_coordinate_system(&mut self, cs: CoordinateSystem) {
        self.coordinate_system = cs;
    }
    fn set_conversion(&mut self, conversion: Conversion) {
        self.conversion = conversion;
    }
    // Pass down to the coordinate system
    fn set_axis(&mut self, axis: Axis) {
        self.coordinate_system.axis.push(axis);
    }
    fn set_unit(&mut self, unit: Unit) {
        for axis in self.coordinate_system.axis.iter_mut() {
            axis.unit = Some(unit.clone());
        }
    }
}

/// # DerivedGeodeticCRS Interface
///
/// Represents a derived geodetic or geographic coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DerivedGeodeticCRS {
    /// Indicates the type of coordinate reference system. Can be either "DerivedGeodeticCRS" or "DerivedGeographicCRS".
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'DerivedGeodeticCRS' | 'DerivedGeographicCRS';
    /// The name of the derived geodetic CRS.
    pub name: String,
    /// The base CRS from which this derived CRS is created.
    pub base_crs: GeodeticCRS,
    /// The conversion method applied to the base CRS.
    pub conversion: Conversion,
    /// The coordinate system used in the CRS.
    pub coordinate_system: CoordinateSystem,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for DerivedGeodeticCRS {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_coordinate_system(&mut self, cs: CoordinateSystem) {
        self.coordinate_system = cs;
    }
    fn set_conversion(&mut self, conversion: Conversion) {
        self.conversion = conversion;
    }
    fn set_geodetic_crs(&mut self, geodetic_crs: GeodeticCRS) {
        self.base_crs = geodetic_crs;
    }
    // Pass down to the coordinate system
    fn set_axis(&mut self, axis: Axis) {
        self.coordinate_system.axis.push(axis);
    }
    fn set_unit(&mut self, unit: Unit) {
        for axis in self.coordinate_system.axis.iter_mut() {
            axis.unit = Some(unit.clone());
        }
    }
}

/// # GeodeticCRS Interface
///
/// Represents a geodetic or geographic coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct GeodeticCRS {
    /// Indicates the type of CRS. Can be "GeodeticCRS" or "GeographicCRS".
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'GeodeticCRS' | 'GeographicCRS';
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
    /// DYNAMIC[FRAMEEPOCH[2010.0],MODEL["NAD83(CSRS)v6 velocity grid"]] (referenced by MODEL)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deformation_models: Option<Vec<DeformationModel>>,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for GeodeticCRS {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_coordinate_system(&mut self, cs: CoordinateSystem) {
        self.coordinate_system = Some(cs);
    }
    fn set_datum(&mut self, datum: Datum) {
        self.datum = Some(datum);
    }
    fn set_ensemble(&mut self, ensemble: DatumEnsemble) {
        self.datum_ensemble = Some(ensemble);
    }
    // Pass down to the datum
    fn set_prime_meridian(&mut self, prime_meridian: PrimeMeridian) {
        if let Some(ref mut datum) = self.datum {
            datum.set_prime_meridian(prime_meridian);
        }
    }
    // Pass down to the coordinate system
    fn set_axis(&mut self, axis: Axis) {
        if let Some(ref mut cs) = self.coordinate_system {
            cs.axis.push(axis);
        }
    }
    fn set_unit(&mut self, unit: Unit) {
        if let Some(ref mut cs) = self.coordinate_system {
            for axis in cs.axis.iter_mut() {
                axis.unit = Some(unit.clone());
            }
        }
    }
}
impl GeodeticCRS {
    /// Convert a GeodeticCRS to a ProjectionTransform
    pub fn to_projection_transform(&self, proj_transform: &mut ProjectionTransform) {
        proj_transform.proj.borrow_mut().name = self.name.clone();
        // TODO: DeformationModel
        // TODO: Datum -> build_datum
        if let Some(datum_ensemble) = &self.datum_ensemble {
            datum_ensemble.to_projection_transform(proj_transform);
        }
        if let Some(datum) = &self.datum {
            datum.to_projection_transform(proj_transform);
        }
        if let Some(coordinate_system) = &self.coordinate_system {
            coordinate_system.to_projection_transform(proj_transform);
        }
    }
}

/// # Geodetic reference frame (geodetic datum)
///
/// Represented with the `DATUM` keyword
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct GeodeticReferenceFrame {
    /// Indicates the type of reference frame. Always "GeodeticReferenceFrame" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'GeodeticReferenceFrame';
    /// The name of the reference frame.
    pub name: String,
    /// The ellipsoid used in the reference frame.
    pub ellipsoid: Ellipsoid,
    /// The anchor point of the reference frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// The epoch of the anchor point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_epoch: Option<f64>,
    /// The prime meridian associated with the reference frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prime_meridian: Option<PrimeMeridian>,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl GeodeticReferenceFrame {
    /// Convert a GeodeticReferenceFrame to a ProjectionTransform
    pub fn to_projection_transform(&self, proj_transform: &mut ProjectionTransform) {
        self.ellipsoid.to_projection_transform(proj_transform);
        if let Some(pm) = &self.prime_meridian {
            pm.to_projection_transform(proj_transform);
        }
    }
}
impl ToProjJSON for GeodeticReferenceFrame {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_anchor(&mut self, anchor: String) {
        self.anchor = Some(anchor);
    }
    fn set_ellipsoid(&mut self, ellipsoid: Ellipsoid) {
        self.ellipsoid = ellipsoid;
    }
    fn set_epoch(&mut self, epoch: f64) {
        self.anchor_epoch = Some(epoch);
    }
    fn set_prime_meridian(&mut self, prime_meridian: PrimeMeridian) {
        self.prime_meridian = Some(prime_meridian);
    }
}

/// # DerivedParametricCRS Interface
///
/// Represents a derived parametric coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DerivedParametricCRS {
    /// Indicates the type of coordinate reference system. Always "DerivedParametricCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'DerivedParametricCRS';
    /// The name of the derived parametric CRS.
    pub name: String,
    /// The base parametric CRS from which this CRS is derived.
    pub base_crs: ParametricCRS,
    /// The conversion method applied to the base CRS.
    pub conversion: Conversion,
    /// The coordinate system used in the CRS.
    pub coordinate_system: CoordinateSystem,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for DerivedParametricCRS {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_coordinate_system(&mut self, cs: CoordinateSystem) {
        self.coordinate_system = cs;
    }
    fn set_conversion(&mut self, conversion: Conversion) {
        self.conversion = conversion;
    }
    // Pass down to the coordinate system
    fn set_axis(&mut self, axis: Axis) {
        self.coordinate_system.axis.push(axis);
    }
    fn set_unit(&mut self, unit: Unit) {
        for axis in self.coordinate_system.axis.iter_mut() {
            axis.unit = Some(unit.clone());
        }
    }
}

/// # DerivedProjectedCRS Interface
///
/// Represents a derived projected coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DerivedProjectedCRS {
    /// Indicates the type of coordinate reference system. Always "DerivedProjectedCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'DerivedProjectedCRS';
    /// The name of the derived projected CRS.
    pub name: String,
    /// The base projected CRS from which this CRS is derived.
    pub base_crs: ProjectedCRS,
    /// The conversion method applied to the base CRS.
    pub conversion: Conversion,
    /// The coordinate system used in the CRS.
    pub coordinate_system: CoordinateSystem,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for DerivedProjectedCRS {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_coordinate_system(&mut self, cs: CoordinateSystem) {
        self.coordinate_system = cs;
    }
    fn set_conversion(&mut self, conversion: Conversion) {
        self.conversion = conversion;
    }
    fn set_projected_crs(&mut self, projected_crs: ProjectedCRS) {
        self.base_crs = projected_crs;
    }
    // Pass down to the coordinate system
    fn set_axis(&mut self, axis: Axis) {
        self.coordinate_system.axis.push(axis);
    }
    fn set_unit(&mut self, unit: Unit) {
        for axis in self.coordinate_system.axis.iter_mut() {
            axis.unit = Some(unit.clone());
        }
    }
}

/// # DerivedTemporalCRS Interface
///
/// Represents a derived temporal coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DerivedTemporalCRS {
    /// Indicates the type of coordinate reference system. Always "DerivedTemporalCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'DerivedTemporalCRS';
    /// The name of the derived temporal CRS.
    pub name: String,
    /// The base temporal CRS from which this CRS is derived.
    pub base_crs: TemporalCRS,
    /// The conversion method applied to the base CRS.
    pub conversion: Conversion,
    /// The coordinate system used in the CRS.
    pub coordinate_system: CoordinateSystem,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for DerivedTemporalCRS {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_coordinate_system(&mut self, cs: CoordinateSystem) {
        self.coordinate_system = cs;
    }
    fn set_conversion(&mut self, conversion: Conversion) {
        self.conversion = conversion;
    }
    // Pass down to the coordinate system
    fn set_axis(&mut self, axis: Axis) {
        self.coordinate_system.axis.push(axis);
    }
    fn set_unit(&mut self, unit: Unit) {
        for axis in self.coordinate_system.axis.iter_mut() {
            axis.unit = Some(unit.clone());
        }
    }
}

/// # DerivedVerticalCRS Interface
///
/// Represents a derived vertical coordinate reference system.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DerivedVerticalCRS {
    /// Indicates the type of coordinate reference system. Always "DerivedVerticalCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'DerivedVerticalCRS';
    /// The name of the derived vertical CRS.
    pub name: String,
    /// The base vertical CRS from which this CRS is derived.
    pub base_crs: VerticalCRS,
    /// The conversion method applied to the base CRS.
    pub conversion: Conversion,
    /// The coordinate system used in the CRS.
    pub coordinate_system: CoordinateSystem,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for DerivedVerticalCRS {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_coordinate_system(&mut self, cs: CoordinateSystem) {
        self.coordinate_system = cs;
    }
    fn set_conversion(&mut self, conversion: Conversion) {
        self.conversion = conversion;
    }
    // Pass down to the coordinate system
    fn set_axis(&mut self, axis: Axis) {
        self.coordinate_system.axis.push(axis);
    }
    fn set_unit(&mut self, unit: Unit) {
        for axis in self.coordinate_system.axis.iter_mut() {
            axis.unit = Some(unit.clone());
        }
    }
}

/// # DynamicGeodeticReferenceFrame Interface
///
/// Represents a dynamic geodetic reference frame.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DynamicGeodeticReferenceFrame {
    /// Indicates the type of reference frame. Always "DynamicGeodeticReferenceFrame" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'DynamicGeodeticReferenceFrame';
    /// The name of the reference frame.
    pub name: String,
    /// The ellipsoid used in the reference frame.
    pub ellipsoid: Ellipsoid,
    /// The frame reference epoch.
    pub frame_reference_epoch: f64,
    /// The anchor point of the reference frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// The epoch of the anchor point.
    pub anchor_epoch: Option<f64>,
    /// The prime meridian associated with the reference frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prime_meridian: Option<PrimeMeridian>,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for DynamicGeodeticReferenceFrame {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_anchor(&mut self, anchor: String) {
        self.anchor = Some(anchor);
    }
    fn set_epoch(&mut self, epoch: f64) {
        self.anchor_epoch = Some(epoch);
    }
    fn set_frame_epoch(&mut self, epoch: f64) {
        self.frame_reference_epoch = epoch;
    }
    fn set_ellipsoid(&mut self, ellipsoid: Ellipsoid) {
        self.ellipsoid = ellipsoid;
    }
    fn set_prime_meridian(&mut self, prime_meridian: PrimeMeridian) {
        self.prime_meridian = Some(prime_meridian);
    }
}

/// members in the datum ensemble
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DatumEnsembleMember {
    /// The name of the datum.
    pub name: String,
    /// An identifier for the datum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the datum.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Ids,
}
impl ToProjJSON for DatumEnsembleMember {
    fn set_id(&mut self, id: Id) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if !self.ids.is_empty() {
            self.ids.push(id);
        } else if let Some(i) = self.id.clone() {
            self.ids.extend(vec![i, id]);
            self.id = None;
        } else {
            self.id = Some(id);
        }
    }
}

/// # DatumEnsemble Interface
///
/// Represents a datum ensemble, which is a collection of datums.
/// Geodetic and vertical CRSs are associated with either a reference frame (datum) or a datum ensemble. The members of a datum ensemble are given as a list of reference frames. The list may contain reference frame name and/or identifier. All members of a datum ensemble are realizations of one shared terrestrial or vertical reference system.
/// For an ensemble of geodetic reference frames (datums), the WKT string includes the description of the ellipsoid used by the members. This information is available from any and all of the definitions of each member. It is included in the ensemble WKT to facilitate direct access to the information. The WKT string for a datum ensemble may also include the description of the prime meridian applying to all members of the ensemble.
/// For both geodetic and vertical datum ensembles, the ensemble description includes its 'accuracy', an indication of the difference in coordinate values of a point between different members of the datum ensemble. It may be regarded as a measure of the inaccuracy introduced through the assumption that ensemble members are approximately equivalent.
/// Use of the datum ensemble concept comes with a health warning. If data is associated with a CRS having a datum ensemble, it will not be possible to identify which of the datum ensemble members the data might more accurately be referenced to. In high accuracy applications, datum ensembles should not be used; individual reference frames should be identified.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DatumEnsemble {
    /// Indicates the type of datum ensemble. Always "DatumEnsemble" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'DatumEnsemble';
    /// The name of the datum ensemble.
    pub name: String,
    /// An array of members in the datum ensemble.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<DatumEnsembleMember>,
    /// The accuracy of the datum ensemble.
    pub accuracy: String,
    /// The ellipsoid associated with the datum ensemble.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsoid: Option<Ellipsoid>,
    /// An identifier for the datum ensemble.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the datum ensemble.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Ids,
}
impl ToProjJSON for DatumEnsemble {
    fn set_accuracy(&mut self, accuracy: String) {
        self.accuracy = accuracy;
    }
    fn set_ellipsoid(&mut self, ellipsoid: Ellipsoid) {
        self.ellipsoid = Some(ellipsoid);
    }
    fn set_member(&mut self, member: DatumEnsembleMember) {
        self.members.push(member);
    }
    fn set_id(&mut self, id: Id) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if !self.ids.is_empty() {
            self.ids.push(id);
        } else if let Some(i) = self.id.clone() {
            self.ids.extend(vec![i, id]);
            self.id = None;
        } else {
            self.id = Some(id);
        }
    }
}
impl DatumEnsemble {
    /// Convert a DatumEnsemble to a ProjectionTransform
    pub fn to_projection_transform(&self, proj_transform: &mut ProjectionTransform) {
        if let Some(ellipsoid) = &self.ellipsoid {
            ellipsoid.to_projection_transform(proj_transform);
        }
    }
}

/// # Ellipsoid
///
/// ## Description
/// The `<ellipsoid>` object is an attribute of `<geodetic reference frame>`. It is not used with other types of datum.
///
/// ISO 19111 allows an oblate ellipsoid to be defined through semi-major axis (a) and either
/// semi-minor axis (b) or inverse flattening (1/f). If semi-minor axis is used as the second
/// defining parameter the value for inverse flattening to be shown in the WKT string should be
/// calculated from 1/f  =  a / (a – b).
///
/// ISO 19111 also allows for the earth model to be a sphere, for which 1/f is infinite.
/// In this document if the earth model is a sphere `<inverse flattening>` shall be given an
/// artificial value of zero.
///
/// ## Requirements:
/// a) The WKT representation of a sphere shall have an `<inverse flattening>` value of 0.
/// b) `<length unit>` is an optional attribute, optional for reasons of backward compatibility,
/// but it is recommended that it is explicitly included in WKT strings. Its `<conversion factor>`
/// shall be to metres and is the number of metres per unit. `<length unit>` is described in 7.4.
/// If it is omitted then the value for the length of the semi-axis or -axes shall be given in metres.
/// Conversely, if it is omitted then the value for the semi-major axis shall be assumed to be in
/// metres.
///
/// ## Note:
/// - In the WKT for a geodetic, geographic or projected CRS, the length unit for the ellipsoid may
///   differ from the length unit for the coordinate system. The units in which coordinates are expressed
///   are given by the CS element.
/// - In this document the preferred keyword is ELLIPSOID. SPHEROID is permitted for backward compatibility.
///   Implementations should be prepared to read both forms.
///
/// ## Examples of WKT describing an ellipsoid:
/// - `ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1.0]]`
/// - `SPHEROID["GRS 1980",6378137.0,298.257222101]` (unit = metre is implied)
/// - `ELLIPSOID["Clark 1866",20925832.164,294.97869821, LENGTHUNIT["US survey foot",0.304800609601219]]`
/// - `ELLIPSOID["Sphere",6371000,0,LENGTHUNIT["metre",1.0]]`
///
/// The definition of WKT for a triaxial ellipsoid required for planetary mapping is given in Annex E.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Ellipsoid {
    /// Indicates the type of ellipsoid. Always "Ellipsoid" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'Ellipsoid';
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
    pub inverse_flattening: Option<ValueInMetreOrValueAndUnit>,
    /// The radius of the ellipsoid, used for spherical representations.
    /// Required when neither `semi_minor_axis` nor `inverse_flattening` are provided.
    pub radius: Option<ValueInMetreOrValueAndUnit>,
    /// An identifier for the datum ensemble.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the datum ensemble.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Ids,
}
impl ToProjJSON for Ellipsoid {
    fn set_id(&mut self, id: Id) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if !self.ids.is_empty() {
            self.ids.push(id);
        } else if let Some(i) = self.id.clone() {
            self.ids.extend(vec![i, id]);
            self.id = None;
        } else {
            self.id = Some(id);
        }
    }
}
impl Ellipsoid {
    /// Convert a Ellipsoid to a ProjectionTransform
    pub fn to_projection_transform(&self, proj_transform: &mut ProjectionTransform) {
        let proj = &mut proj_transform.proj.borrow_mut();
        proj.ellps = self.name.clone();
        if let Some(semi_major_axis) = &self.semi_major_axis {
            proj.a = semi_major_axis.meters();
        }
        if let Some(semi_minor_axis) = &self.semi_minor_axis {
            proj.b = semi_minor_axis.meters();
        }
        if let Some(inverse_flattening) = &self.inverse_flattening {
            proj.rf = inverse_flattening.meters();
        }
        if let Some(radius) = &self.radius {
            proj.a = radius.meters();
            proj.b = radius.meters();
            proj.rf = 0.0;
        }
        derive_sphere(proj);
        derive_eccentricity(proj);
    }
}

/// # Prime meridian
/// The WKT for prime meridian is defined in 8.2.2.
/// In this document the following definition from both ISO 19125-1:2004 and OGC 01-009 has been deprecated but is included here for the purposes of documenting backward compatibility:
/// - <irm longitude> is the longitude of the prime meridian measured from the international reference meridian, positive eastward.
/// - <angle unit> is an optional attribute, optional for reasons of backward compatibility, but best practice is that it is included in WKT strings. If it is omitted then the value for <irm longitude> shall be given in the CRS's <cs unit> where this is angular, else in decimal degrees. If the subtype of the geodetic CRS to which the prime meridian is an attribute is geographic, the prime meridian's <irm longitude> value shall be given in the same angular units as those for the horizontal axes of the geographic CRS; if the geodetic CRS subtype is geocentric the prime meridian's <irm longitude> value shall be given in degrees. Its <conversion factor> shall be to radians and is the number of radians per unit. <angle unit> is described in 7.4.
///
/// Examples of WKT describing a prime meridian:
/// - `PRIMEM["Paris",2.5969213,ANGLEUNIT["grad",0.015707963267949]]`
/// - `PRIMEM["Greenwich",0.0]`
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct PrimeMeridian {
    /// Indicates the type of prime meridian. Always "PrimeMeridian" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'PrimeMeridian';
    /// The name of the prime meridian.
    pub name: String,
    /// The longitude of the prime meridian.
    /// Represented as a number or a value with a unit.
    pub longitude: ValueInDegreeOrValueAndUnit,
    /// An identifier for the datum ensemble.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the datum ensemble.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Ids,
}
impl PrimeMeridian {
    /// Convert a PrimeMeridian to a ProjectionTransform
    pub fn to_projection_transform(&self, proj_transform: &mut ProjectionTransform) {
        proj_transform.proj.borrow_mut().from_greenwich = self.longitude.rad();
    }
}
impl ToProjJSON for PrimeMeridian {
    fn set_id(&mut self, id: Id) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if !self.ids.is_empty() {
            self.ids.push(id);
        } else if let Some(i) = self.id.clone() {
            self.ids.extend(vec![i, id]);
            self.id = None;
        } else {
            self.id = Some(id);
        }
    }
    fn set_unit(&mut self, unit: Unit) {
        self.longitude.set_unit(unit);
    }
}

/// # ProjectedCRS Interface
///
/// Represents a projected coordinate reference system, which transforms geodetic or geographic
/// coordinates into a flat, two-dimensional plane using a map projection.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ProjectedCRS {
    /// Indicates the type of CRS. Always "ProjectedCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'ProjectedCRS';
    /// The name of the projected CRS.
    pub name: String,
    /// The base CRS upon which the projection is defined.
    /// Typically a geodetic CRS.
    pub base_crs: GeodeticCRS,
    /// The conversion defining the map projection.
    pub conversion: Conversion,
    /// The coordinate system used in the projected CRS.
    pub coordinate_system: CoordinateSystem,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for ProjectedCRS {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_coordinate_system(&mut self, cs: CoordinateSystem) {
        self.coordinate_system = cs;
    }
    fn set_conversion(&mut self, conversion: Conversion) {
        self.conversion = conversion;
    }
    fn set_geodetic_crs(&mut self, geodetic_crs: GeodeticCRS) {
        self.base_crs = geodetic_crs;
    }
    // Pass down to the coordinate system
    fn set_axis(&mut self, axis: Axis) {
        self.coordinate_system.set_axis(axis);
    }
    fn set_unit(&mut self, unit: Unit) {
        self.coordinate_system.set_unit(unit);
    }
    // NOTE: Conversion also needs variables passed down
    fn set_projection(&mut self, name: String) {
        self.conversion.set_projection(name);
    }
    fn set_method(&mut self, method: Method) {
        self.conversion.set_method(method);
    }
    fn set_parameter(&mut self, parameter: ParameterValue) {
        self.conversion.set_parameter(parameter);
    }
}
impl ProjectedCRS {
    /// Convert a ProjectedCRS to a ProjectionTransform
    pub fn to_projection_transform(&self, proj_transform: &mut ProjectionTransform) {
        // First update the conversion type with "coordinate_system" (set axis step at least)
        self.coordinate_system.to_projection_transform(proj_transform);
        // set base_crs as first step
        self.base_crs.to_projection_transform(proj_transform);
        // set conversion
        self.conversion.to_projection_transform(proj_transform);
    }
}

/// # Conversion Interface
///
/// Represents the map projection or transformation used in a projected CRS.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Conversion {
    /// The schema URL or identifier.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Indicates the type of conversion. Always "Conversion" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'Conversion';
    /// The name of the conversion (map projection or transformation).
    pub name: String,
    /// The method used for the conversion.
    pub method: Method,
    /// An array of parameter values defining the conversion.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterValue>,
    /// An identifier for the datum ensemble.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the datum ensemble.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Ids,
}
impl ToProjJSON for Conversion {
    fn set_id(&mut self, id: Id) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if !self.ids.is_empty() {
            self.ids.push(id);
        } else if let Some(i) = self.id.clone() {
            self.ids.extend(vec![i, id]);
            self.id = None;
        } else {
            self.id = Some(id);
        }
    }
    fn set_projection(&mut self, name: String) {
        self.method = Method { name, ..Default::default() };
    }
    fn set_method(&mut self, method: Method) {
        self.method = method;
    }
    fn set_parameter(&mut self, parameter: ParameterValue) {
        self.parameters.push(parameter);
    }
}
impl Conversion {
    /// Convert a ProjectedCRS to a ProjectionTransform
    pub fn to_projection_transform(&self, proj_transform: &mut ProjectionTransform) {
        // add Params
        for param in self.parameters.iter() {
            proj_transform.proj.borrow_mut().add_param(param);
        }
        self.method.to_projection_transform(proj_transform);
    }
}

/// # CoordinateMetadata Interface
///
/// Represents metadata associated with a coordinate, including its reference system and epoch.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct CoordinateMetadata {
    /// The schema URL or identifier.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Indicates the type of object. Always "CoordinateMetadata" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'CoordinateMetadata';
    /// The coordinate reference system associated with the coordinate.
    pub crs: CRS,
    /// The epoch of the coordinate.
    #[serde(rename = "coordinateEpoch", skip_serializing_if = "Option::is_none")]
    pub coordinate_epoch: Option<f64>,
}
impl ToProjJSON for CoordinateMetadata {
    fn set_epoch(&mut self, epoch: f64) {
        self.coordinate_epoch = Some(epoch);
    }
}

/// # Coordinate system type
///
/// For various types of CRS the type of coordinate system that may be used is constrained, as is
/// the permissible number of axes. Additionally the data type for coordinates in an ordinal
/// coordinate system and in a temporal coordinate system is constrained.
///
/// ## Examples of WKT describing an CoordinateSystem with the specified subtype as the first element:
/// - `CS[ordinal,2],AXIS["inline (I)",southeast,ORDER[1]],AXIS["crossline (J)",northeast,ORDER[2]]`
/// - `CS[Cartesian,3],AXIS["(X)",geocentricX],AXIS["(Y)",geocentricY],AXIS["(Z)",geocentricZ],LENGTHUNIT["metre",1.0]`
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
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
impl CoordinateSystemSubtype {
    /// Convert a CoordinateSystem to a ProjectionTransform
    pub fn to_projection_transform(&self, _proj_transform: &mut ProjectionTransform) {
        // TODO: ALL of them. I don't know the best way to add cart, but it needs to be a step
        match self {
            CoordinateSystemSubtype::Cartesian => {
                // let cart = CartesianConverter::new(Rc::new(RefCell::new(Proj::default())));
                // proj_transform.cart = Some(Box::new(cart.into()));
            }
            _ => todo!(), // CoordinateSystemSubtype::Spherical => {}
                          // CoordinateSystemSubtype::Ellipsoidal => {}
                          // CoordinateSystemSubtype::Vertical => {}
                          // CoordinateSystemSubtype::Ordinal => {}
                          // CoordinateSystemSubtype::Parametric => {}
                          // CoordinateSystemSubtype::Affine => {}
                          // CoordinateSystemSubtype::TemporalDateTime => {}
                          // CoordinateSystemSubtype::TemporalCount => {}
                          // CoordinateSystemSubtype::TemporalMeasure => {}
        }
    }
}

/// # Coordinate System
///
/// Represents a coordinate system, including its subtype and axes.
///
/// Most coordinate system attributes are common to all subtypes of spatial and temporal coordinate systems. Exceptions are associated with the coordinate system axis unit attribute and its qualifier, the conversion factor to an SI base unit:
/// - When the coordinate system type is 'temporalCount' or 'temporalMeasure', the inclusion of the axis unit conversion factor in WKT is conditional, see 7.4.3.
/// - When the coordinate system type is 'ordinal' or 'temporalDateTime', the axis unit attribute and its conversion factor are not required in WKT, see 7.5.6 and 13.3.
///
/// ## Examples of WKT describing an CoordinateSystem:
/// - `CS[ordinal,2],AXIS["inline (I)",southeast,ORDER[1]],AXIS["crossline (J)",northeast,ORDER[2]]`
/// - `CS[Cartesian,3],AXIS["(X)",geocentricX],AXIS["(Y)",geocentricY],AXIS["(Z)",geocentricZ],LENGTHUNIT["metre",1.0]`
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct CoordinateSystem {
    /// The schema URL or identifier.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Indicates the type of object. Always "CoordinateSystem" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'CoordinateSystem';
    /// The name of the coordinate system.
    pub name: Option<String>,
    /// The subtype of the coordinate system.
    pub subtype: CoordinateSystemSubtype,
    /// The axes of the coordinate system.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub axis: Vec<Axis>,
    /// An identifier for the datum ensemble.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the datum ensemble.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Ids,
}
impl ToProjJSON for CoordinateSystem {
    fn set_id(&mut self, id: Id) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if !self.ids.is_empty() {
            self.ids.push(id);
        } else if let Some(i) = self.id.clone() {
            self.ids.extend(vec![i, id]);
            self.id = None;
        } else {
            self.id = Some(id);
        }
    }
    fn set_axis(&mut self, axis: Axis) {
        self.axis.push(axis);
    }
    fn set_unit(&mut self, unit: Unit) {
        for axis in self.axis.iter_mut() {
            axis.unit = Some(unit.clone());
        }
    }
}
impl CoordinateSystem {
    /// Convert a CoordinateSystem to a ProjectionTransform
    pub fn to_projection_transform(&self, proj_transform: &mut ProjectionTransform) {
        self.subtype.to_projection_transform(proj_transform);
        if !self.axis.is_empty() {
            let mut axiss = self.axis.clone();
            axiss.sort_by(|a, b| a.order.cmp(&b.order));
            let axis: Vec<AxisDirection> = axiss.iter().map(|a| a.direction).collect();
            let mut axis_converter = AxisSwapConverter::new(Rc::new(RefCell::new(Proj::default())));
            axis_converter.swap = axis.into();
            proj_transform.axisswap = Some(Box::new(axis_converter.into()));
        }
    }
}

/// # Transformation Interface
///
/// Represents a transformation between two coordinate reference systems.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Transformation {
    /// Type identifier
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'Transformation';
    /// Name of the transformation
    pub name: String,
    /// Source CRS
    pub source_crs: CRS,
    /// Target CRS
    pub target_crs: CRS,
    /// Transformation method
    pub method: Method,
    /// Transformation parameters
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterValue>,
    /// Interpolation CRS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation_crs: Option<CRS>,
    /// Transformation accuracy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<String>,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for Transformation {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_accuracy(&mut self, accuracy: String) {
        self.accuracy = Some(accuracy);
    }
    fn set_parameter(&mut self, parameter: ParameterValue) {
        self.parameters.push(parameter);
    }
    fn set_projection(&mut self, name: String) {
        self.method = Method { name, ..Default::default() };
    }
    fn set_method(&mut self, method: Method) {
        self.method = method;
    }
}

/// # TemporalCRS Interface
///
/// Represents a temporal coordinate reference system, which defines time-based coordinates.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct TemporalCRS {
    /// Indicates the type of CRS. Always "TemporalCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'TemporalCRS';
    /// The name of the temporal CRS.
    pub name: String,
    /// The temporal datum associated with the CRS.
    pub datum: TemporalDatum,
    /// The coordinate system used in the temporal CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate_system: Option<CoordinateSystem>,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for TemporalCRS {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_coordinate_system(&mut self, cs: CoordinateSystem) {
        self.coordinate_system = Some(cs);
    }
    // Pass down to the coordinate system
    fn set_axis(&mut self, axis: Axis) {
        if let Some(ref mut cs) = self.coordinate_system {
            cs.axis.push(axis);
        }
    }
    fn set_unit(&mut self, unit: Unit) {
        if let Some(ref mut cs) = self.coordinate_system {
            for axis in cs.axis.iter_mut() {
                axis.unit = Some(unit.clone());
            }
        }
    }
}

/// # TemporalDatum Interface
///
/// Represents the temporal datum associated with a temporal CRS.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct TemporalDatum {
    /// Indicates the type of datum. Always "TemporalDatum" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'TemporalDatum';
    /// The name of the temporal datum.
    pub name: String,
    /// The calendar system used for the datum.
    pub calendar: String,
    /// The time origin of the temporal datum, typically an ISO 8601 date/time string.
    pub time_origin: Option<String>,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for TemporalDatum {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
}

/// # VerticalCRS Interface
///
/// Represents a vertical coordinate reference system, which is used for height or depth measurements.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct VerticalCRS {
    /// Indicates the type of CRS. Always "VerticalCRS" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'VerticalCRS';
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
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for VerticalCRS {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_coordinate_system(&mut self, cs: CoordinateSystem) {
        self.coordinate_system = Some(cs);
    }
    fn set_datum(&mut self, datum: Datum) {
        self.datum = Some(datum);
    }
    fn set_ensemble(&mut self, ensemble: DatumEnsemble) {
        self.datum_ensemble = Some(ensemble);
    }
    // Pass down to the datum
    fn set_prime_meridian(&mut self, prime_meridian: PrimeMeridian) {
        if let Some(ref mut datum) = self.datum {
            datum.set_prime_meridian(prime_meridian);
        }
    }
    // Pass down to the coordinate system
    fn set_axis(&mut self, axis: Axis) {
        if let Some(ref mut cs) = self.coordinate_system {
            cs.axis.push(axis);
        }
    }
    fn set_unit(&mut self, unit: Unit) {
        if let Some(ref mut cs) = self.coordinate_system {
            for axis in cs.axis.iter_mut() {
                axis.unit = Some(unit.clone());
            }
        }
    }
}

/// # VerticalReferenceFrame Interface
///
/// Represents the vertical reference frame associated with a vertical CRS.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct VerticalReferenceFrame {
    /// Indicates the type of reference frame. Always "VerticalReferenceFrame" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'VerticalReferenceFrame';
    /// The name of the vertical reference frame.
    pub name: String,
    /// The anchor point of the reference frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// The epoch of the anchor point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_epoch: Option<f64>,
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for VerticalReferenceFrame {
    fn set_anchor(&mut self, anchor: String) {
        self.anchor = Some(anchor);
    }
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_epoch(&mut self, epoch: f64) {
        self.anchor_epoch = Some(epoch);
    }
}

/// # DynamicVerticalReferenceFrame Interface
///
/// Represents a dynamic vertical reference frame.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DynamicVerticalReferenceFrame {
    /// Indicates the type of reference frame. Always "DynamicVerticalReferenceFrame" for this interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>, // 'DynamicVerticalReferenceFrame';
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
    /// Usage Information
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ObjectUsage>,
    /// Usages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usages: Vec<ObjectUsage>,
}
impl ToProjJSON for DynamicVerticalReferenceFrame {
    fn set_id(&mut self, id: Id) {
        if self.usage.is_none() && self.usages.is_empty() {
            self.usage = Some(ObjectUsage::default());
        }
        if let Some(u) = self.usage.as_mut().or_else(|| self.usages.last_mut()) {
            u.set_id(id);
        }
    }
    fn set_usage(&mut self, usage: ObjectUsage) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if self.usages.is_empty() {
            if let Some(u) = self.usage.clone() {
                self.usages.extend(vec![u, usage]);
                self.usage = None;
            } else {
                self.usage = Some(usage);
            }
        } else {
            self.usages.push(usage);
        }
    }
    fn set_anchor(&mut self, anchor: String) {
        self.anchor = Some(anchor);
    }
    fn set_epoch(&mut self, epoch: f64) {
        self.anchor_epoch = Some(epoch);
    }
    fn set_frame_epoch(&mut self, epoch: f64) {
        self.frame_reference_epoch = epoch;
    }
}

/// # GeoidModel Interface
///
/// Represents a geoid model associated with a vertical CRS.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct GeoidModel {
    /// The name of the geoid model.
    pub name: String,
    /// The interpolation CRS for the geoid model.
    pub interpolation_crs: Option<Box<CRS>>,
    /// An identifier for the geoid model.
    pub id: Option<Id>,
}

/// # Usage
///
/// ## Description
/// Usage is an optional attribute which if included in a WKT string shall include both
/// <scope> and <extent>. Multiple pairs of scope/extent may be used to describe the usage for
/// different purposes over different extents. In this document the <scope> and <extent> elements
/// may not be given alone but only as a pairing. Within each pairing, extent may consist of one or
/// more of area textual description, area bounding box, vertical extent and/or temporal extent,
/// see 7.3.2.3.
///
/// ## Examples
/// - `USAGE[<scope>,<extent>]`
/// - `USAGE[scope1,extent1],USAGE[scope2,extent2]`
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ObjectUsage {
    /// The schema URL or identifier.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// The scope of the CRS.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub scope: String,
    /// The extent if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    /// bbox
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<ProjBBox>,
    /// vertical_extent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_extent: Option<VerticalExtent>,
    /// temporal_extent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_extent: Option<TemporalExtent>,
    /// Remarks or additional information about the CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    /// An identifier for the CRS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    /// An array of identifiers for the CRS.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ids: Ids,
}
impl ToProjJSON for ObjectUsage {
    fn set_id(&mut self, id: Id) {
        // If array is active, add to array; if id is already set, migrate to array; otherwise set id
        if !self.ids.is_empty() {
            self.ids.push(id);
        } else if let Some(i) = self.id.clone() {
            self.ids.extend(vec![i, id]);
            self.id = None;
        } else {
            self.id = Some(id);
        }
    }
    fn set_temporal_extent(&mut self, extent: TemporalExtent) {
        self.temporal_extent = Some(extent);
    }
    fn set_vertical_extent(&mut self, extent: VerticalExtent) {
        self.vertical_extent = Some(extent);
    }
    fn set_bbox(&mut self, bbox: ProjBBox) {
        self.bbox = Some(bbox);
    }
    fn set_area(&mut self, area: Option<String>) {
        self.area = area;
    }
}
