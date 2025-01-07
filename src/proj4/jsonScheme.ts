/**
 * Schema for PROJJSON (v0.7)
 * @see https://proj.org/schemas/v0.7/projjson.schema.json
 */
export interface PROJJSON {
  $schema?: string;
  type?: string;
  name?: string;
  source_crs?: CRS;
  target_crs?: CRS;
  method?: Method;
  parameters?: ParameterValue[];
  id?: Id;
  ids?: Id[];
  accuracy?: string;
  scope?: string;
  area?: string;
  bbox?: BBox;
  vertical_extent?: VerticalExtent;
  temporal_extent?: TemporalExtent;
  remarks?: string;
  usages?: Usage[];
}

/** Coordinate Reference System */
export type CRS =
  | BoundCRS
  | CompoundCRS
  | EngineeringCRS
  | GeodeticCRS
  | ProjectedCRS
  | TemporalCRS
  | VerticalCRS;

/** Bounding Box */
export interface BBox {
  east_longitude: number;
  west_longitude: number;
  south_latitude: number;
  north_latitude: number;
}

/** Vertical Extent */
export interface VerticalExtent {
  minimum: number;
  maximum: number;
  unit?: Unit;
}

/** Temporal Extent */
export interface TemporalExtent {
  start: string;
  end: string;
}

/** ID Object */
export interface Id {
  authority: string;
  code: string | number;
  version?: string | number;
  authority_citation?: string;
  uri?: string;
}

/** Usage Object */
export interface Usage {
  scope?: string;
  area?: string;
  bbox?: BBox;
  vertical_extent?: VerticalExtent;
  temporal_extent?: TemporalExtent;
}

/** Parameter Value */
export interface ParameterValue {
  $schema?: string;
  type: 'ParameterValue';
  name: string;
  value: string | number;
  unit?: Unit;
  id?: Id;
  ids?: Id[];
}

/** Method Object */
export interface Method {
  $schema?: string;
  type: 'OperationMethod';
  name: string;
  id?: Id;
  ids?: Id[];
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
      ids?: Id[];
    };

/** CRS Variants */
export interface BoundCRS {
  type: 'BoundCRS';
  name: string;
  source_crs: CRS;
  target_crs: CRS;
  transformation: AbridgedTransformation;
  scope?: string;
  area?: string;
  bbox?: BBox;
  vertical_extent?: VerticalExtent;
  temporal_extent?: TemporalExtent;
  usages?: Usage[];
  remarks?: string;
  id?: Id;
  ids?: Id[];
}

/** Abridged Transformation */
export interface AbridgedTransformation {
  type: 'AbridgedTransformation';
  name: string;
  source_crs?: CRS;
  method: Method;
  parameters: ParameterValue[];
  id?: Id;
  ids?: Id[];
}

/**
 * CompoundCRS Interface
 *
 * Represents a compound coordinate reference system, which combines two or more coordinate reference systems.
 */
export interface CompoundCRS {
  /** Indicates the type of CRS. Always "CompoundCRS" for this interface. */
  type: 'CompoundCRS';
  /** The name of the compound CRS. */
  name: string;
  /**
   * Array of component CRS objects.
   * Each component is a CRS, such as GeodeticCRS, VerticalCRS, etc.
   */
  components: CRS[];
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
  ids?: Id[];
}

/**
 * EngineeringCRS Interface
 *
 * Represents an engineering coordinate reference system.
 */
export interface EngineeringCRS {
  /** Indicates the type of CRS. Always "EngineeringCRS" for this interface. */
  type: 'EngineeringCRS';
  /** The name of the engineering CRS. */
  name: string;
  /** The engineering datum associated with this CRS. */
  datum: EngineeringDatum;
  /** The coordinate system used in this CRS. */
  coordinate_system?: CoordinateSystem;
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
  ids?: Id[];
}

/**
 * EngineeringDatum Interface
 *
 * Represents the datum associated with an engineering CRS.
 */
export interface EngineeringDatum {
  /** Indicates the type of datum. Always "EngineeringDatum" for this interface. */
  type: 'EngineeringDatum';
  /** The name of the datum. */
  name: string;
  /** Anchor point of the datum. */
  anchor?: string;
  /** The schema URL or identifier. */
  $schema?: string;
  /** The scope of the datum. */
  scope?: string;
  /** The area of use for the datum. */
  area?: string;
  /** The bounding box of the datum. */
  bbox?: BBox;
  /** The vertical extent of the datum. */
  vertical_extent?: VerticalExtent;
  /** The temporal extent of the datum. */
  temporal_extent?: TemporalExtent;
  /** An array of usages for the datum. */
  usages?: Usage[];
  /** Remarks or additional information about the datum. */
  remarks?: string;
  /** An identifier for the datum. */
  id?: Id;
  /** An array of identifiers for the datum. */
  ids?: Id[];
}

/**
 * CoordinateSystem Interface
 *
 * Represents a coordinate system, which defines the axes and their properties.
 */
export interface CoordinateSystem {
  /** Indicates the type of coordinate system. Always "CoordinateSystem" for this interface. */
  type: 'CoordinateSystem';
  /** The name of the coordinate system. */
  name?: string;
  /**
   * The subtype of the coordinate system.
   * Examples include Cartesian, spherical, ellipsoidal, vertical, etc.
   */
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
  /** An array of axis definitions that describe the coordinate system. */
  axis: Axis[];
  /** The schema URL or identifier. */
  $schema?: string;
  /** An identifier for the coordinate system. */
  id?: Id;
  /** An array of identifiers for the coordinate system. */
  ids?: Id[];
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
  ids?: Id[];
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
  longitude: number | ValueAndUnit;
  /** The schema URL or identifier. */
  $schema?: string;
  /** An identifier for the meridian. */
  id?: Id;
  /** An array of identifiers for the meridian. */
  ids?: Id[];
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

/**
 * GeodeticCRS Interface
 *
 * Represents a geodetic or geographic coordinate reference system.
 */
export interface GeodeticCRS {
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
  ids?: Id[];
}

/**
 * GeodeticReferenceFrame Interface
 *
 * Represents the geodetic reference frame associated with a geodetic CRS.
 */
export interface GeodeticReferenceFrame {
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
  /** The schema URL or identifier. */
  $schema?: string;
  /** The scope of the reference frame. */
  scope?: string;
  /** The area of use for the reference frame. */
  area?: string;
  /** The bounding box of the reference frame. */
  bbox?: BBox;
  /** The vertical extent of the reference frame. */
  vertical_extent?: VerticalExtent;
  /** The temporal extent of the reference frame. */
  temporal_extent?: TemporalExtent;
  /** An array of usages for the reference frame. */
  usages?: Usage[];
  /** Remarks or additional information about the reference frame. */
  remarks?: string;
  /** An identifier for the reference frame. */
  id?: Id;
  /** An array of identifiers for the reference frame. */
  ids?: Id[];
}

/**
 * DynamicGeodeticReferenceFrame Interface
 *
 * Represents a dynamic geodetic reference frame.
 */
export interface DynamicGeodeticReferenceFrame {
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
  /** The schema URL or identifier. */
  $schema?: string;
  /** The scope of the reference frame. */
  scope?: string;
  /** The area of use for the reference frame. */
  area?: string;
  /** The bounding box of the reference frame. */
  bbox?: BBox;
  /** The vertical extent of the reference frame. */
  vertical_extent?: VerticalExtent;
  /** The temporal extent of the reference frame. */
  temporal_extent?: TemporalExtent;
  /** An array of usages for the reference frame. */
  usages?: Usage[];
  /** Remarks or additional information about the reference frame. */
  remarks?: string;
  /** An identifier for the reference frame. */
  id?: Id;
  /** An array of identifiers for the reference frame. */
  ids?: Id[];
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
    ids?: Id[];
  }>;
  /** The ellipsoid associated with the datum ensemble. */
  ellipsoid?: Ellipsoid;
  /** The accuracy of the datum ensemble. */
  accuracy: string;
  /** An identifier for the datum ensemble. */
  id?: Id;
  /** An array of identifiers for the datum ensemble. */
  ids?: Id[];
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
  semi_major_axis?: number | ValueAndUnit;
  /**
   * The semi-minor axis of the ellipsoid.
   * Represented as a number or a value with a unit.
   * Required when `inverse_flattening` is not provided.
   */
  semi_minor_axis?: number | ValueAndUnit;
  /**
   * The inverse flattening of the ellipsoid.
   * Required when `semi_minor_axis` is not provided.
   */
  inverse_flattening?: number;
  /**
   * The radius of the ellipsoid, used for spherical representations.
   * Required when neither `semi_minor_axis` nor `inverse_flattening` are provided.
   */
  radius?: number | ValueAndUnit;
  /** The schema URL or identifier. */
  $schema?: string;
  /** An identifier for the ellipsoid. */
  id?: Id;
  /** An array of identifiers for the ellipsoid. */
  ids?: Id[];
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
  longitude: number | ValueAndUnit;
  /** The schema URL or identifier. */
  $schema?: string;
  /** An identifier for the prime meridian. */
  id?: Id;
  /** An array of identifiers for the prime meridian. */
  ids?: Id[];
}

/**
 * ProjectedCRS Interface
 *
 * Represents a projected coordinate reference system, which transforms geodetic or geographic coordinates
 * into a flat, two-dimensional plane using a map projection.
 */
export interface ProjectedCRS {
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
  ids?: Id[];
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
  ids?: Id[];
}

/**
 * TemporalCRS Interface
 *
 * Represents a temporal coordinate reference system, which defines time-based coordinates.
 */
export interface TemporalCRS {
  /** Indicates the type of CRS. Always "TemporalCRS" for this interface. */
  type: 'TemporalCRS';
  /** The name of the temporal CRS. */
  name: string;
  /** The temporal datum associated with the CRS. */
  datum: TemporalDatum;
  /** The coordinate system used in the temporal CRS. */
  coordinate_system?: CoordinateSystem;
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
  ids?: Id[];
}

/**
 * TemporalDatum Interface
 *
 * Represents the temporal datum associated with a temporal CRS.
 */
export interface TemporalDatum {
  /** Indicates the type of datum. Always "TemporalDatum" for this interface. */
  type: 'TemporalDatum';
  /** The name of the temporal datum. */
  name: string;
  /** The calendar system used for the datum. */
  calendar: string;
  /** The time origin of the temporal datum, typically an ISO 8601 date/time string. */
  time_origin: string;
  /** The schema URL or identifier. */
  $schema?: string;
  /** The scope of the datum. */
  scope?: string;
  /** The area of use for the datum. */
  area?: string;
  /** The bounding box of the datum. */
  bbox?: BBox;
  /** The vertical extent of the datum. */
  vertical_extent?: VerticalExtent;
  /** The temporal extent of the datum. */
  temporal_extent?: TemporalExtent;
  /** An array of usages for the datum. */
  usages?: Usage[];
  /** Remarks or additional information about the datum. */
  remarks?: string;
  /** An identifier for the datum. */
  id?: Id;
  /** An array of identifiers for the datum. */
  ids?: Id[];
}

/**
 * VerticalCRS Interface
 *
 * Represents a vertical coordinate reference system, which is used for height or depth measurements.
 */
export interface VerticalCRS {
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
  ids?: Id[];
}

/**
 * VerticalReferenceFrame Interface
 *
 * Represents the vertical reference frame associated with a vertical CRS.
 */
export interface VerticalReferenceFrame {
  /** Indicates the type of reference frame. Always "VerticalReferenceFrame" for this interface. */
  type: 'VerticalReferenceFrame';
  /** The name of the vertical reference frame. */
  name: string;
  /** The anchor point of the reference frame. */
  anchor?: string;
  /** The epoch of the anchor point. */
  anchor_epoch?: number;
  /** The schema URL or identifier. */
  $schema?: string;
  /** The scope of the reference frame. */
  scope?: string;
  /** The area of use for the reference frame. */
  area?: string;
  /** The bounding box of the reference frame. */
  bbox?: BBox;
  /** The vertical extent of the reference frame. */
  vertical_extent?: VerticalExtent;
  /** The temporal extent of the reference frame. */
  temporal_extent?: TemporalExtent;
  /** An array of usages for the reference frame. */
  usages?: Usage[];
  /** Remarks or additional information about the reference frame. */
  remarks?: string;
  /** An identifier for the reference frame. */
  id?: Id;
  /** An array of identifiers for the reference frame. */
  ids?: Id[];
}

/**
 * DynamicVerticalReferenceFrame Interface
 *
 * Represents a dynamic vertical reference frame.
 */
export interface DynamicVerticalReferenceFrame {
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
  /** The schema URL or identifier. */
  $schema?: string;
  /** The scope of the reference frame. */
  scope?: string;
  /** The area of use for the reference frame. */
  area?: string;
  /** The bounding box of the reference frame. */
  bbox?: BBox;
  /** The vertical extent of the reference frame. */
  vertical_extent?: VerticalExtent;
  /** The temporal extent of the reference frame. */
  temporal_extent?: TemporalExtent;
  /** An array of usages for the reference frame. */
  usages?: Usage[];
  /** Remarks or additional information about the reference frame. */
  remarks?: string;
  /** An identifier for the reference frame. */
  id?: Id;
  /** An array of identifiers for the reference frame. */
  ids?: Id[];
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
