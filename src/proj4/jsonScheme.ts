/**
 * Schema for PROJJSON (v0.7)
 * @see https://proj.org/schemas/v0.7/projjson.schema.json
 */
export type PROJJSON =
  | CRS
  | Datum
  | DatumEnsemble
  | Ellipsoid
  | PrimeMeridian
  | SingleOperation
  | ConcatenatedOperation
  | CoordinateMetadata;

/** Coordinate Reference System */
export type CRS =
  | BoundCRS
  | CompoundCRS
  | DerivedEngineeringCRS
  | DerivedGeodeticCRS
  | DerivedParametricCRS
  | DerivedProjectedCRS
  | DerivedTemporalCRS
  | DerivedVerticalCRS
  | EngineeringCRS
  | GeodeticCRS
  | ParametricCRS
  | ProjectedCRS
  | TemporalCRS
  | VerticalCRS;

/**
 * Datum Interface
 *
 * Represents a datum which can be one of several types of reference frames or datums.
 */
export type Datum =
  | GeodeticReferenceFrame
  | VerticalReferenceFrame
  | DynamicGeodeticReferenceFrame
  | DynamicVerticalReferenceFrame
  | TemporalDatum
  | ParametricDatum
  | EngineeringDatum;

/**
 * Bounding Box Interface
 *
 * Represents a bounding box defined by its east, west, south, and north boundaries.
 */
export interface BBox {
  /** The easternmost longitude of the bounding box. */
  east_longitude: number;
  /** The westernmost longitude of the bounding box. */
  west_longitude: number;
  /** The southernmost latitude of the bounding box. */
  south_latitude: number;
  /** The northernmost latitude of the bounding box. */
  north_latitude: number;
}

/** Vertical Extent */
export interface VerticalExtent {
  /** Minimum height */
  minimum: number;
  /** Maximum height */
  maximum: number;
  /** Unit of measurement */
  unit: Unit;
}

/** Temporal Extent */
export interface TemporalExtent {
  /** Start time (ISO 8601 format) */
  start: string;
  /** End time (ISO 8601 format) */
  end: string;
}

/** ID Object */
export interface Id {
  /** Authority issuing the identifier */
  authority: string;
  /** Code associated with the identifier */
  code: string | number;
  /** Version of the identifier */
  version?: string | number;
  /** Citation of the authority */
  authority_citation?: string;
  /** URI reference */
  uri?: string;
}

/** Identifiers list */
export type Ids = Id[];

/** Usage Object */
export interface Usage {
  /** Scope of the usage */
  scope?: string;
  /** Defined area */
  area?: string;
  /** Bounding box */
  bbox?: BBox;
  /** Vertical extent */
  vertical_extent?: VerticalExtent;
  /** Temporal extent */
  temporal_extent?: TemporalExtent;
}

/** Parameter Value */
export interface ParameterValue {
  /** Schema reference */
  $schema?: string;
  /** Type identifier */
  type: 'ParameterValue';
  /** Name of the parameter */
  name: string;
  /** Parameter value, which can be a string or number */
  value: string | number;
  /** Optional unit of measurement */
  unit?: Unit;
  /** Identifier */
  id?: Id;
  /** Alternative identifiers */
  ids?: Ids;
}

/**
 * # Parametric CRS
 *
 * Represents a parametric coordinate reference system.
 */
export interface ParametricCRS extends ObjectUsage {
  /** Type identifier */
  type: 'ParametricCRS';
  /** Name of the CRS */
  name: string;
  /** Parametric datum */
  datum: ParametricDatum;
  /** Coordinate system */
  coordinate_system: CoordinateSystem;
  /** Schema reference */
  $schema?: string;
  /** Scope of the CRS */
  scope?: string;
  /** Defined area */
  area?: string;
  /** Bounding box */
  bbox?: BBox;
  /** Vertical extent */
  vertical_extent?: VerticalExtent;
  /** Temporal extent */
  temporal_extent?: TemporalExtent;
  /** Usages */
  usages?: Usage[];
  /** Additional remarks */
  remarks?: string;
  /** Identifier */
  id?: Id;
  /** Alternative identifiers */
  ids?: Ids;
}

/**
 * # Parametric Datum
 *
 * Represents the parametric datum associated with a parametric CRS.
 */
export interface ParametricDatum extends ObjectUsage {
  /** Type identifier */
  type: 'ParametricDatum';
  /** Name of the datum */
  name: string;
  /** Anchor point */
  anchor: string;
}

/**
 * # Point Motion Operation
 *
 * Represents a point motion operation
 */
export interface PointMotionOperation extends ObjectUsage {
  /** Type identifier */
  type: 'PointMotionOperation';
  /** Name of the operation */
  name: string;
  /** Source coordinate reference system */
  source_crs: CRS;
  /** Method used for point motion */
  method: Method;
  /** Parameters used in the operation */
  parameters: ParameterValue[];
  /** Accuracy of the operation */
  accuracy?: string;
}

/**
 * Method Object
 *
 * Defines an operation method with a name and identifier
 */
export interface Method {
  /** Schema reference */
  $schema?: string;
  /** Type identifier */
  type: 'OperationMethod';
  /** Name of the method */
  name: string;
  /** Identifier */
  id?: Id;
  /** Alternative identifiers */
  ids?: Ids;
}

/** Unit Definition */
export type Unit =
  | 'metre'
  | 'degree'
  | 'unity'
  | {
      type: 'LinearUnit' | 'AngularUnit' | 'ScaleUnit' | 'TimeUnit' | 'ParametricUnit' | 'Unit';
      name: string;
      conversion_factor?: number;
      id?: Id;
      ids?: Ids;
    };

/**
 * BoundCRS Interface
 *
 * Represents a coordinate reference system that is bounded by a source and target CRS with a transformation.
 */
export interface BoundCRS extends ObjectUsage {
  /** Indicates the type of object. Always "BoundCRS" for this interface. */
  type: 'BoundCRS';
  /** The name of the bound CRS. */
  name: string;
  /** The source coordinate reference system. */
  source_crs: CRS;
  /** The target coordinate reference system. */
  target_crs: CRS;
  /** The transformation applied to convert between the source and target CRS. */
  transformation: AbridgedTransformation;
}

/**
 * ConcatenatedOperation Interface
 *
 * Represents an operation that is composed of multiple steps, transforming one CRS to another.
 */
export interface ConcatenatedOperation extends ObjectUsage {
  /** Indicates the type of object. Always "ConcatenatedOperation" for this interface. */
  type: 'ConcatenatedOperation';
  /** The name of the concatenated operation. */
  name: string;
  /** The source coordinate reference system. */
  source_crs: CRS;
  /** The target coordinate reference system. */
  target_crs: CRS;
  /** An array of individual steps in the concatenated operation. */
  steps: SingleOperation[];
  /** The accuracy of the concatenated operation. */
  accuracy?: string;
}

/**
 * AbridgedTransformation Interface
 *
 * Represents an abridged transformation used for converting between different coordinate reference systems.
 */
export interface AbridgedTransformation {
  /** The schema URL or identifier. */
  $schema?: string;
  /** Indicates the type of object. Always "AbridgedTransformation" for this interface. */
  type: 'AbridgedTransformation';
  /** The name of the transformation. */
  name: string;
  /** The source coordinate reference system, only present if it differs from the source CRS of the bound CRS. */
  source_crs?: CRS;
  /** The method used for the transformation. */
  method: Method;
  /** The parameters used in the transformation. */
  parameters: ParameterValue[];
  /** An identifier for the transformation. */
  id?: Id;
  /** An array of identifiers for the transformation. */
  ids?: Ids;
}

/**
 * CompoundCRS Interface
 *
 * Represents a compound coordinate reference system, consisting of multiple components.
 */
export interface CompoundCRS extends ObjectUsage {
  /** Indicates the type of object. Always "CompoundCRS" for this interface. */
  type: 'CompoundCRS';
  /** The name of the compound CRS. */
  name: string;
  /** An array of coordinate reference systems that make up the compound CRS. */
  components: CRS[];
}

/**
 * EngineeringCRS Interface
 *
 * Represents an engineering coordinate reference system.
 */
export interface EngineeringCRS extends ObjectUsage {
  /** Indicates the type of CRS. Always "EngineeringCRS" for this interface. */
  type: 'EngineeringCRS';
  /** The name of the engineering CRS. */
  name: string;
  /** The engineering datum associated with this CRS. */
  datum: EngineeringDatum;
  /** The coordinate system used in this CRS. */
  coordinate_system?: CoordinateSystem;
}

/**
 * EngineeringDatum Interface
 *
 * Represents the datum associated with an engineering CRS.
 */
export interface EngineeringDatum extends ObjectUsage {
  /** Indicates the type of datum. Always "EngineeringDatum" for this interface. */
  type: 'EngineeringDatum';
  /** The name of the datum. */
  name: string;
  /** Anchor point of the datum. */
  anchor?: string;
}

/**
 * Axis Interface
 *
 * Represents an individual axis in a coordinate system.
 */
export interface Axis {
  /** Indicates the type of axis. Always "Axis" for this interface. */
  type: 'Axis';
  /** The name of the axis. */
  name: string;
  /** Abbreviation for the axis name. */
  abbreviation: string;
  /**
   * The direction of the axis.
   * Examples include north, east, up, down, geocentricX, geocentricY, geocentricZ, etc.
   */
  direction:
    | 'north'
    | 'northNorthEast'
    | 'northEast'
    | 'eastNorthEast'
    | 'east'
    | 'eastSouthEast'
    | 'southEast'
    | 'southSouthEast'
    | 'south'
    | 'southSouthWest'
    | 'southWest'
    | 'westSouthWest'
    | 'west'
    | 'westNorthWest'
    | 'northWest'
    | 'northNorthWest'
    | 'up'
    | 'down'
    | 'geocentricX'
    | 'geocentricY'
    | 'geocentricZ'
    | 'columnPositive'
    | 'columnNegative'
    | 'rowPositive'
    | 'rowNegative'
    | 'displayRight'
    | 'displayLeft'
    | 'displayUp'
    | 'displayDown'
    | 'forward'
    | 'aft'
    | 'port'
    | 'starboard'
    | 'clockwise'
    | 'counterClockwise'
    | 'towards'
    | 'awayFrom'
    | 'future'
    | 'past'
    | 'unspecified';
  /** The meridian for the axis, if applicable. */
  meridian?: Meridian;
  /** The unit of measurement for the axis. */
  unit?: Unit;
  /** The minimum value allowed for the axis. */
  minimum_value?: number;
  /** The maximum value allowed for the axis. */
  maximum_value?: number;
  /**
   * The range meaning for the axis.
   * Can be either "exact" or "wraparound".
   */
  range_meaning?: 'exact' | 'wraparound';
  /** An identifier for the axis. */
  id?: Id;
  /** An array of identifiers for the axis. */
  ids?: Ids;
}

/**
 * Meridian Interface
 *
 * Represents a meridian, which defines the longitude for an axis.
 */
export interface Meridian {
  /** Indicates the type of meridian. Always "Meridian" for this interface. */
  type: 'Meridian';
  /** The longitude of the meridian. */
  longitude: ValueInDegreeOrValueAndUnit;
  /** The schema URL or identifier. */
  $schema?: string;
  /** An identifier for the meridian. */
  id?: Id;
  /** An array of identifiers for the meridian. */
  ids?: Ids;
}

/**
 * ValueAndUnit Interface
 *
 * Represents a value paired with a unit of measurement.
 */
export interface ValueAndUnit {
  /** The numeric value. */
  value: number;
  /** The unit of measurement. */
  unit: Unit;
}

/** Value in Degrees or Value and Unit */
export type ValueInDegreeOrValueAndUnit = number | ValueAndUnit;

/** Value in Metres or Value and Unit */
export type ValueInMetreOrValueAndUnit = number | ValueAndUnit;

/**
 * # Single Operation
 *
 * Represents a single operation, which can be a conversion, transformation, or point motion operation.
 */
export type SingleOperation = Conversion | Transformation | PointMotionOperation;

/**
 * DatumMember Interface
 *
 * Represents a member of a datum ensemble.
 */
export interface DatumMember {
  /** The name of the datum member. */
  name: string;
  /** An identifier for the datum member. */
  id?: Id;
  /** An array of identifiers for the datum member. */
  ids?: Ids;
}

/**
 * DeformationModel Interface
 *
 * Represents a deformation model associated with a point motion operation.
 */
export interface DeformationModel {
  /** The name of the deformation model. */
  name: string;
  /** An identifier for the deformation model. */
  id?: Id;
}

/**
 * DerivedEngineeringCRS Interface
 *
 * Represents a derived engineering coordinate reference system.
 */
export interface DerivedEngineeringCRS extends ObjectUsage {
  /** Indicates the type of coordinate reference system. Always "DerivedEngineeringCRS" for this interface. */
  type: 'DerivedEngineeringCRS';
  /** The name of the derived engineering CRS. */
  name: string;
  /** The base CRS from which this derived CRS is created. */
  base_crs: EngineeringCRS;
  /** The conversion method applied to the base CRS. */
  conversion: Conversion;
  /** The coordinate system used in the CRS. */
  coordinate_system: CoordinateSystem;
}

/**
 * DerivedGeodeticCRS Interface
 *
 * Represents a derived geodetic or geographic coordinate reference system.
 */
export interface DerivedGeodeticCRS extends ObjectUsage {
  /** Indicates the type of coordinate reference system. Can be either "DerivedGeodeticCRS" or "DerivedGeographicCRS". */
  type: 'DerivedGeodeticCRS' | 'DerivedGeographicCRS';
  /** The name of the derived geodetic CRS. */
  name: string;
  /** The base CRS from which this derived CRS is created. */
  base_crs: GeodeticCRS;
  /** The conversion method applied to the base CRS. */
  conversion: Conversion;
  /** The coordinate system used in the CRS. */
  coordinate_system: CoordinateSystem;
}

/**
 * GeodeticCRS Interface
 *
 * Represents a geodetic or geographic coordinate reference system.
 */
export interface GeodeticCRS extends ObjectUsage {
  /** Indicates the type of CRS. Can be "GeodeticCRS" or "GeographicCRS". */
  type: 'GeodeticCRS' | 'GeographicCRS';
  /** The name of the geodetic CRS. */
  name: string;
  /**
   * The datum associated with the geodetic CRS.
   * One and only one of `datum` or `datum_ensemble` must be provided.
   */
  datum?: GeodeticReferenceFrame | DynamicGeodeticReferenceFrame;
  /** The datum ensemble associated with the geodetic CRS. */
  datum_ensemble?: DatumEnsemble;
  /** The coordinate system used in the geodetic CRS. */
  coordinate_system?: CoordinateSystem;
  /** An array of deformation models associated with the geodetic CRS. */
  deformation_models?: DeformationModel[];
}

/**
 * GeodeticReferenceFrame Interface
 *
 * Represents the geodetic reference frame associated with a geodetic CRS.
 */
export interface GeodeticReferenceFrame extends ObjectUsage {
  /** Indicates the type of reference frame. Always "GeodeticReferenceFrame" for this interface. */
  type: 'GeodeticReferenceFrame';
  /** The name of the reference frame. */
  name: string;
  /** The anchor point of the reference frame. */
  anchor?: string;
  /** The epoch of the anchor point. */
  anchor_epoch?: number;
  /** The ellipsoid used in the reference frame. */
  ellipsoid: Ellipsoid;
  /** The prime meridian associated with the reference frame. */
  prime_meridian?: PrimeMeridian;
}

/**
 * DerivedParametricCRS Interface
 *
 * Represents a derived parametric coordinate reference system.
 */
export interface DerivedParametricCRS extends ObjectUsage {
  /** Indicates the type of coordinate reference system. Always "DerivedParametricCRS" for this interface. */
  type: 'DerivedParametricCRS';
  /** The name of the derived parametric CRS. */
  name: string;
  /** The base parametric CRS from which this CRS is derived. */
  base_crs: ParametricCRS;
  /** The conversion method applied to the base CRS. */
  conversion: Conversion;
  /** The coordinate system used in the CRS. */
  coordinate_system: CoordinateSystem;
}

/**
 * DerivedProjectedCRS Interface
 *
 * Represents a derived projected coordinate reference system.
 */
export interface DerivedProjectedCRS extends ObjectUsage {
  /** Indicates the type of coordinate reference system. Always "DerivedProjectedCRS" for this interface. */
  type: 'DerivedProjectedCRS';
  /** The name of the derived projected CRS. */
  name: string;
  /** The base projected CRS from which this CRS is derived. */
  base_crs: ProjectedCRS;
  /** The conversion method applied to the base CRS. */
  conversion: Conversion;
  /** The coordinate system used in the CRS. */
  coordinate_system: CoordinateSystem;
}

/**
 * DerivedTemporalCRS Interface
 *
 * Represents a derived temporal coordinate reference system.
 */
export interface DerivedTemporalCRS extends ObjectUsage {
  /** Indicates the type of coordinate reference system. Always "DerivedTemporalCRS" for this interface. */
  type: 'DerivedTemporalCRS';
  /** The name of the derived temporal CRS. */
  name: string;
  /** The base temporal CRS from which this CRS is derived. */
  base_crs: TemporalCRS;
  /** The conversion method applied to the base CRS. */
  conversion: Conversion;
  /** The coordinate system used in the CRS. */
  coordinate_system: CoordinateSystem;
}

/**
 * DerivedVerticalCRS Interface
 *
 * Represents a derived vertical coordinate reference system.
 */
export interface DerivedVerticalCRS extends ObjectUsage {
  /** Indicates the type of coordinate reference system. Always "DerivedVerticalCRS" for this interface. */
  type: 'DerivedVerticalCRS';
  /** The name of the derived vertical CRS. */
  name: string;
  /** The base vertical CRS from which this CRS is derived. */
  base_crs: VerticalCRS;
  /** The conversion method applied to the base CRS. */
  conversion: Conversion;
  /** The coordinate system used in the CRS. */
  coordinate_system: CoordinateSystem;
}

/**
 * DynamicGeodeticReferenceFrame Interface
 *
 * Represents a dynamic geodetic reference frame.
 */
export interface DynamicGeodeticReferenceFrame extends ObjectUsage {
  /** Indicates the type of reference frame. Always "DynamicGeodeticReferenceFrame" for this interface. */
  type: 'DynamicGeodeticReferenceFrame';
  /** The name of the reference frame. */
  name: string;
  /** The anchor point of the reference frame. */
  anchor?: string;
  /** The epoch of the anchor point. */
  anchor_epoch?: number;
  /** The ellipsoid used in the reference frame. */
  ellipsoid: Ellipsoid;
  /** The prime meridian associated with the reference frame. */
  prime_meridian?: PrimeMeridian;
  /** The frame reference epoch. */
  frame_reference_epoch: number;
}

/**
 * DatumEnsemble Interface
 *
 * Represents a datum ensemble, which is a collection of datums.
 */
export interface DatumEnsemble {
  /** Indicates the type of datum ensemble. Always "DatumEnsemble" for this interface. */
  type: 'DatumEnsemble';
  /** The name of the datum ensemble. */
  name: string;
  /** An array of members in the datum ensemble. */
  members: Array<{
    name: string;
    id?: Id;
    ids?: Ids;
  }>;
  /** The ellipsoid associated with the datum ensemble. */
  ellipsoid?: Ellipsoid;
  /** The accuracy of the datum ensemble. */
  accuracy: string;
  /** An identifier for the datum ensemble. */
  id?: Id;
  /** An array of identifiers for the datum ensemble. */
  ids?: Ids;
}

/**
 * DeformationModel Interface
 *
 * Represents a deformation model associated with a geodetic CRS.
 */
export interface DeformationModel {
  /** The name of the deformation model. */
  name: string;
  /** An identifier for the deformation model. */
  id?: Id;
}

/**
 * Ellipsoid Interface
 *
 * Represents an ellipsoid, a geometric figure used in geodetic reference frames.
 */
export interface Ellipsoid {
  /** Indicates the type of ellipsoid. Always "Ellipsoid" for this interface. */
  type: 'Ellipsoid';
  /** The name of the ellipsoid. */
  name: string;
  /**
   * The semi-major axis of the ellipsoid.
   * Represented as a number or a value with a unit.
   */
  semi_major_axis?: ValueInMetreOrValueAndUnit;
  /**
   * The semi-minor axis of the ellipsoid.
   * Represented as a number or a value with a unit.
   * Required when `inverse_flattening` is not provided.
   */
  semi_minor_axis?: ValueInMetreOrValueAndUnit;
  /**
   * The inverse flattening of the ellipsoid.
   * Required when `semi_minor_axis` is not provided.
   */
  inverse_flattening?: number;
  /**
   * The radius of the ellipsoid, used for spherical representations.
   * Required when neither `semi_minor_axis` nor `inverse_flattening` are provided.
   */
  radius?: ValueInMetreOrValueAndUnit;
  /** The schema URL or identifier. */
  $schema?: string;
  /** An identifier for the ellipsoid. */
  id?: Id;
  /** An array of identifiers for the ellipsoid. */
  ids?: Ids;
}

/**
 * PrimeMeridian Interface
 *
 * Represents a prime meridian, which defines the origin of longitude in a geographic coordinate system.
 */
export interface PrimeMeridian {
  /** Indicates the type of prime meridian. Always "PrimeMeridian" for this interface. */
  type: 'PrimeMeridian';
  /** The name of the prime meridian. */
  name: string;
  /**
   * The longitude of the prime meridian.
   * Represented as a number or a value with a unit.
   */
  longitude: ValueInDegreeOrValueAndUnit;
  /** The schema URL or identifier. */
  $schema?: string;
  /** An identifier for the prime meridian. */
  id?: Id;
  /** An array of identifiers for the prime meridian. */
  ids?: Ids;
}

/**
 * ProjectedCRS Interface
 *
 * Represents a projected coordinate reference system, which transforms geodetic or geographic coordinates
 * into a flat, two-dimensional plane using a map projection.
 */
export interface ProjectedCRS extends ObjectUsage {
  /** Indicates the type of CRS. Always "ProjectedCRS" for this interface. */
  type: 'ProjectedCRS';
  /** The name of the projected CRS. */
  name: string;
  /**
   * The base CRS upon which the projection is defined.
   * Typically a geodetic CRS.
   */
  base_crs: GeodeticCRS;
  /** The conversion defining the map projection. */
  conversion: Conversion;
  /** The coordinate system used in the projected CRS. */
  coordinate_system?: CoordinateSystem;
}

/**
 * Conversion Interface
 *
 * Represents the map projection or transformation used in a projected CRS.
 */
export interface Conversion {
  /** Indicates the type of conversion. Always "Conversion" for this interface. */
  type: 'Conversion';
  /** The name of the conversion (map projection or transformation). */
  name: string;
  /** The method used for the conversion. */
  method: Method;
  /** An array of parameter values defining the conversion. */
  parameters?: ParameterValue[];
  /** The schema URL or identifier. */
  $schema?: string;
  /** An identifier for the conversion. */
  id?: Id;
  /** An array of identifiers for the conversion. */
  ids?: Ids;
}

/**
 * CoordinateMetadata Interface
 *
 * Represents metadata associated with a coordinate, including its reference system and epoch.
 */
export interface CoordinateMetadata {
  /** The schema URL or identifier. */
  $schema?: string;
  /** Indicates the type of object. Always "CoordinateMetadata" for this interface. */
  type: 'CoordinateMetadata';
  /** The coordinate reference system associated with the coordinate. */
  crs: CRS;
  /** The epoch of the coordinate. */
  coordinateEpoch?: number;
}

/**
 * CoordinateSystem Interface
 *
 * Represents a coordinate system, including its subtype and axes.
 */
export interface CoordinateSystem {
  /** The schema URL or identifier. */
  $schema?: string;
  /** Indicates the type of object. Always "CoordinateSystem" for this interface. */
  type: 'CoordinateSystem';
  /** The name of the coordinate system. */
  name: string;
  /** The subtype of the coordinate system. */
  subtype:
    | 'Cartesian'
    | 'spherical'
    | 'ellipsoidal'
    | 'vertical'
    | 'ordinal'
    | 'parametric'
    | 'affine'
    | 'TemporalDateTime'
    | 'TemporalCount'
    | 'TemporalMeasure';
  /** The axes of the coordinate system. */
  axis: Axis[];
  /** An identifier for the coordinate system. */
  id?: Id;
  /** An array of identifiers for the coordinate system. */
  ids?: Ids;
}

/**
 * # Transformation Interface
 *
 * Represents a transformation between two coordinate reference systems.
 */
export interface Transformation extends ObjectUsage {
  /** Type identifier */
  type: 'Transformation';
  /** Name of the transformation */
  name: string;
  /** Source CRS */
  source_crs: CRS;
  /** Target CRS */
  target_crs: CRS;
  /** Interpolation CRS */
  interpolation_crs?: CRS;
  /** Transformation method */
  method: Method;
  /** Transformation parameters */
  parameters: ParameterValue[];
  /** Transformation accuracy */
  accuracy?: string;
}

/**
 * TemporalCRS Interface
 *
 * Represents a temporal coordinate reference system, which defines time-based coordinates.
 */
export interface TemporalCRS extends ObjectUsage {
  /** Indicates the type of CRS. Always "TemporalCRS" for this interface. */
  type: 'TemporalCRS';
  /** The name of the temporal CRS. */
  name: string;
  /** The temporal datum associated with the CRS. */
  datum: TemporalDatum;
  /** The coordinate system used in the temporal CRS. */
  coordinate_system?: CoordinateSystem;
}

/**
 * TemporalDatum Interface
 *
 * Represents the temporal datum associated with a temporal CRS.
 */
export interface TemporalDatum extends ObjectUsage {
  /** Indicates the type of datum. Always "TemporalDatum" for this interface. */
  type: 'TemporalDatum';
  /** The name of the temporal datum. */
  name: string;
  /** The calendar system used for the datum. */
  calendar: string;
  /** The time origin of the temporal datum, typically an ISO 8601 date/time string. */
  time_origin: string;
}

/**
 * VerticalCRS Interface
 *
 * Represents a vertical coordinate reference system, which is used for height or depth measurements.
 */
export interface VerticalCRS extends ObjectUsage {
  /** Indicates the type of CRS. Always "VerticalCRS" for this interface. */
  type: 'VerticalCRS';
  /** The name of the vertical CRS. */
  name: string;
  /**
   * The vertical datum associated with the CRS.
   * One and only one of `datum` or `datum_ensemble` must be provided.
   */
  datum?: VerticalReferenceFrame | DynamicVerticalReferenceFrame;
  /** The datum ensemble associated with the CRS. */
  datum_ensemble?: DatumEnsemble;
  /** The coordinate system used in the vertical CRS. */
  coordinate_system?: CoordinateSystem;
  /** The geoid model associated with the vertical CRS. */
  geoid_model?: GeoidModel;
  /** An array of geoid models associated with the vertical CRS. */
  geoid_models?: GeoidModel[];
  /** An array of deformation models associated with the vertical CRS. */
  deformation_models?: DeformationModel[];
}

/**
 * VerticalReferenceFrame Interface
 *
 * Represents the vertical reference frame associated with a vertical CRS.
 */
export interface VerticalReferenceFrame extends ObjectUsage {
  /** Indicates the type of reference frame. Always "VerticalReferenceFrame" for this interface. */
  type: 'VerticalReferenceFrame';
  /** The name of the vertical reference frame. */
  name: string;
  /** The anchor point of the reference frame. */
  anchor?: string;
  /** The epoch of the anchor point. */
  anchor_epoch?: number;
}

/**
 * DynamicVerticalReferenceFrame Interface
 *
 * Represents a dynamic vertical reference frame.
 */
export interface DynamicVerticalReferenceFrame extends ObjectUsage {
  /** Indicates the type of reference frame. Always "DynamicVerticalReferenceFrame" for this interface. */
  type: 'DynamicVerticalReferenceFrame';
  /** The name of the reference frame. */
  name: string;
  /** The anchor point of the reference frame. */
  anchor?: string;
  /** The epoch of the anchor point. */
  anchor_epoch?: number;
  /** The frame reference epoch for the dynamic reference frame. */
  frame_reference_epoch: number;
}

/**
 * GeoidModel Interface
 *
 * Represents a geoid model associated with a vertical CRS.
 */
export interface GeoidModel {
  /** The name of the geoid model. */
  name: string;
  /** The interpolation CRS for the geoid model. */
  interpolation_crs?: CRS;
  /** An identifier for the geoid model. */
  id?: Id;
}

/**
 * # Object Usage
 *
 * Represents common variables across all coordinate reference systems.
 */
export interface ObjectUsage {
  /** The schema URL or identifier. */
  $schema?: string;
  /** The scope of the CRS. */
  scope?: string;
  /** The area of use for the CRS. */
  area?: string;
  /** The bounding box of the CRS. */
  bbox?: BBox;
  /** The vertical extent of the CRS. */
  vertical_extent?: VerticalExtent;
  /** The temporal extent of the CRS. */
  temporal_extent?: TemporalExtent;
  /** An array of usages for the CRS. */
  usages?: Usage[];
  /** Remarks or additional information about the CRS. */
  remarks?: string;
  /** An identifier for the CRS. */
  id?: Id;
  /** An array of identifiers for the CRS. */
  ids?: Ids;
}
