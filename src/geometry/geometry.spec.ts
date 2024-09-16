import type { Face } from '.';
import type {
  LineStringMValues,
  MValue,
  MValues,
  MultiLineStringMValues,
  MultiPolygonMValues,
  PolygonMValues,
} from 's2json-spec';

/**
 * A BBOX is defined in lon-lat space and helps with zooming motion to
 * see the entire line or polygon
 */
export type BBox = [left: number, bottom: number, right: number, top: number];
/**
 * A BBOX is defined in lon-lat space and helps with zooming motion to
 * see the entire 3D line or polygon
 */
export type BBox3D = [
  left: number,
  bottom: number,
  right: number,
  top: number,
  front: number,
  back: number,
];

/** Either a 2D or 3D bounding box */
export type BBOX = BBox | BBox3D;

/** A Point in S2 Space with a Face */
export interface STPoint {
  /** The face of the point */
  face: Face;
  /** The S coordinates of the point */
  s: number;
  /** The T coordinates of the point */
  t: number;
  /** The Z coordinates of the point */
  z?: number;
  /** The M coordinates of the point */
  m?: MValue;
}

/** Definition of a Point. May represent WebMercator Lon-Lat or S2Geometry S-T */
export type Point = [x: number, y: number];
/** Definition of a MultiPoint */
export type MultiPoint = Point[];
/** Definition of a LineString */
export type LineString = Point[];
/** Definition of a MultiLineString */
export type MultiLineString = LineString[];
/** Definition of a Polygon */
export type Polygon = LineString[];
/** Definition of a MultiPolygon */
export type MultiPolygon = Polygon[];
/** Definition of a 3D Point. May represent WebMercator Lon-Lat or S2Geometry S-T with a z-value */
export type Point3D = [x: number, y: number, z: number];
/** Definition of a 3D MultiPoint */
export type MultiPoint3D = Point3D[];
/** Definition of a 3D LineString */
export type LineString3D = Point3D[];
/** Definition of a 3D MultiLineString */
export type MultiLineString3D = LineString3D[];
/** Definition of a 3D Polygon */
export type Polygon3D = Point3D[][];
/** Definition of a 3D MultiPolygon */
export type MultiPolygon3D = Polygon3D[];
/** All possible geometry coordinates */
export type Coordinates =
  | Point
  | MultiPoint
  | LineString
  | MultiLineString
  | Polygon
  | MultiPolygon
  | Point3D
  | MultiPoint3D
  | LineString3D
  | MultiLineString3D
  | Polygon3D
  | MultiPolygon3D;

/** All possible geometry types */
export type GeometryType =
  | 'Point'
  | 'MultiPoint'
  | 'LineString'
  | 'MultiLineString'
  | 'Polygon'
  | 'MultiPolygon'
  | 'Point3D'
  | 'MultiPoint3D'
  | 'LineString3D'
  | 'MultiLineString3D'
  | 'Polygon3D'
  | 'MultiPolygon3D';
/** All possible geometry shapes */
export type Geometry<M extends MValue = MValue> =
  // 2D
  | PointGeometry<M>
  | MultiPointGeometry<M>
  | LineStringGeometry<M>
  | MultiLineStringGeometry<M>
  | PolygonGeometry<M>
  | MultiPolygonGeometry<M>
  // 3D
  | Point3DGeometry<M>
  | MultiPoint3DGeometry<M>
  | LineString3DGeometry<M>
  | MultiLineString3DGeometry<M>
  | Polygon3DGeometry<M>
  | MultiPolygon3DGeometry<M>;

/** BaseGeometry with MValues is the a generic geometry type that includes MValues */
export interface BaseGeometry<T = GeometryType, C = Coordinates, M = MValues, B = BBOX> {
  type: T;
  coordinates: C;
  mValues?: M;
  bbox?: B;
}

/** PointGeometry is a point */
export type PointGeometry<M extends MValue = MValue> = BaseGeometry<'Point', Point, M, BBox>;
/** MultiPointGeometry contains multiple points */
export type MultiPointGeometry<M extends MValue = MValue> = BaseGeometry<
  'MultiPoint',
  MultiPoint,
  LineStringMValues<M>,
  BBox
>;
/** LineStringGeometry is a line */
export type LineStringGeometry<M extends MValue = MValue> = BaseGeometry<
  'LineString',
  LineString,
  LineStringMValues<M>,
  BBox
>;
/** MultiLineStringGeometry contians multiple lines */
export type MultiLineStringGeometry<M extends MValue = MValue> = BaseGeometry<
  'MultiLineString',
  MultiLineString,
  MultiLineStringMValues<M>,
  BBox
>;
/** PolygonGeometry is a polygon with potential holes */
export type PolygonGeometry<M extends MValue = MValue> = BaseGeometry<
  'Polygon',
  Polygon,
  PolygonMValues<M>,
  BBox
>;
/** MultiPolygonGeometry is a polygon with multiple polygons with their own potential holes */
export type MultiPolygonGeometry<M extends MValue = MValue> = BaseGeometry<
  'MultiPolygon',
  MultiPolygon,
  MultiPolygonMValues<M>,
  BBox
>;
/** Point3DGeometry is a 3D point */
export type Point3DGeometry<M extends MValue = MValue> = BaseGeometry<
  'Point3D',
  Point3D,
  M,
  BBox3D
>;
/** MultiPoint3DGeometry contains multiple 3D points */
export type MultiPoint3DGeometry<M extends MValue = MValue> = BaseGeometry<
  'MultiPoint3D',
  MultiPoint3D,
  LineStringMValues<M>,
  BBox3D
>;
/** LineString3DGeometry is a 3D line */
export type LineString3DGeometry<M extends MValue = MValue> = BaseGeometry<
  'LineString3D',
  LineString3D,
  LineStringMValues<M>,
  BBox3D
>;
/** MultiLineString3DGeometry contians multiple 3D lines */
export type MultiLineString3DGeometry<M extends MValue = MValue> = BaseGeometry<
  'MultiLineString3D',
  MultiLineString3D,
  MultiLineStringMValues<M>,
  BBox3D
>;
/** Polygon3DGeometry is a 3D polygon with potential holes */
export type Polygon3DGeometry<M extends MValue = MValue> = BaseGeometry<
  'Polygon3D',
  Polygon3D,
  PolygonMValues<M>,
  BBox3D
>;
/** MultiPolygon3DGeometry is a 3D polygon with multiple polygons with their own potential holes */
export type MultiPolygon3DGeometry<M extends MValue = MValue> = BaseGeometry<
  'MultiPolygon3D',
  MultiPolygon3D,
  MultiPolygonMValues<M>,
  BBox3D
>;

/// Vector Types

/** Definition of a Vector Point */
export interface VectorPoint<M extends MValue = MValue> {
  x: number;
  y: number;
  z?: number;
  m?: M;
  // t for tolerance. A tmp value used for simplification
  t?: number;
}
/** Definition of a Vector MultiPoint */
export type VectorMultiPoint = VectorPoint[];
/** Definition of a Vector LineString */
export type VectorLineString = VectorPoint[];
/** Definition of a Vector MultiLineString */
export type VectorMultiLineString = VectorLineString[];
/** Definition of a Vector Polygon */
export type VectorPolygon = VectorLineString[];
/** Definition of a Vector MultiPolygon */
export type VectorMultiPolygon = VectorPolygon[];

/** All possible geometry coordinates */
export type VectorCoordinates =
  | VectorPoint
  | VectorMultiPoint
  | VectorLineString
  | VectorMultiLineString
  | VectorPolygon
  | VectorMultiPolygon;

/** All possible geometry types */
export type VectorGeometryType =
  | 'Point'
  | 'MultiPoint'
  | 'LineString'
  | 'MultiLineString'
  | 'Polygon'
  | 'MultiPolygon';
/** All possible geometry shapes */
export type VectorGeometry =
  | VectorPointGeometry
  | VectorMultiPointGeometry
  | VectorLineStringGeometry
  | VectorMultiLineStringGeometry
  | VectorPolygonGeometry
  | VectorMultiPolygonGeometry;

/** BaseGeometry with MValues is the a generic geometry type that includes MValues */
export interface VectorBaseGeometry<
  T = VectorGeometryType,
  C = VectorCoordinates,
  O = VectorOffsets,
  B = BBOX,
> {
  type: T;
  coordinates: C;
  is3D: boolean;
  offset?: O;
  // always a [lon-min, lat-min, lon-max, lat-max] regardless of projection.
  // Used for visualization tools
  bbox?: B;
  // tmp bbox to track 0->1 clipping
  vecBBox?: B;
}

/** All possible geometry offsets */
export type VectorOffsets =
  | VectorLineOffset
  | VectorMultiLineOffset
  | VectorPolygonOffset
  | VectorMultiPolygonOffset;

/** An offset defines how far the starting line is from the original starting point pre-slice */
export type VectorLineOffset = number;
/** A collection of offsets */
export type VectorMultiLineOffset = VectorLineOffset[];
/** A collection of offsets */
export type VectorPolygonOffset = VectorLineOffset[];
/** A collection of collections of offsets */
export type VectorMultiPolygonOffset = VectorPolygonOffset[];

/** PointGeometry is a point */
export type VectorPointGeometry = VectorBaseGeometry<'Point', VectorPoint, undefined, BBOX>;
/** MultiPointGeometry contains multiple points */
export type VectorMultiPointGeometry = VectorBaseGeometry<
  'MultiPoint',
  VectorMultiPoint,
  undefined,
  BBOX
>;
/** LineStringGeometry is a line */
export type VectorLineStringGeometry = VectorBaseGeometry<
  'LineString',
  VectorLineString,
  VectorLineOffset,
  BBOX
>;
/** MultiLineStringGeometry contians multiple lines */
export type VectorMultiLineStringGeometry = VectorBaseGeometry<
  'MultiLineString',
  VectorMultiLineString,
  VectorMultiLineOffset,
  BBOX
>;
/** PolygonGeometry is a polygon with potential holes */
export interface VectorPolygonGeometry
  extends VectorBaseGeometry<'Polygon', VectorPolygon, VectorPolygonOffset, BBOX> {
  indices?: number[];
  tesselation?: number[];
}
/** MultiPolygonGeometry is a polygon with multiple polygons with their own potential holes */
export interface VectorMultiPolygonGeometry
  extends VectorBaseGeometry<'MultiPolygon', VectorMultiPolygon, VectorMultiPolygonOffset, BBOX> {
  indices?: number[];
  tesselation?: number[];
}
