import { K_MAX_EDGE } from './metrics';
import {
  K_MAX_LENGTH_2,
  chordAngleSin2,
  fromAngle,
  fromLength2,
  fromS2Points,
  negativeAngle,
  rightAngle,
  straightAngle,
  toAngle,
} from '../s1/chordAngle';
import {
  children,
  fromFace,
  getEdgesRaw,
  getVertices,
  containsS2Point as idContainsS2Point,
  level,
} from '../id';
import {
  invert,
  cross as s2PointCross,
  dot as s2PointDot,
  norm2 as s2PointNorm2,
} from '../s2/point';

import type { LengthMetric } from './metrics';
import type { S1Angle } from '../s1/angle';
import type { S1ChordAngle } from '../s1/chordAngle';
import type { Face, Point3D, S2CellId, Vertices } from '../';

let kMaxEdge: LengthMetric | undefined;

/**
 * S2Cap represents a disc-shaped region defined by a center and radius.
 * Technically this shape is called a "spherical cap" (rather than disc)
 * because it is not planar; the cap represents a portion of the sphere that
 * has been cut off by a plane.  The boundary of the cap is the circle defined
 * by the intersection of the sphere and the plane.  For containment purposes,
 * the cap is a closed set, i.e. it contains its boundary.
 *
 * For the most part, you can use a spherical cap wherever you would use a
 * disc in planar geometry.  The radius of the cap is measured along the
 * surface of the sphere (rather than the straight-line distance through the
 * interior).  Thus a cap of radius Pi/2 is a hemisphere, and a cap of radius
 * Pi covers the entire sphere.
 *
 * A cap can also be defined by its center point and height.  The height is
 * simply the distance from the center point to the cutoff plane.  There is
 * also support for empty and full caps, which contain no points and all
 * points respectively.
 *
 * This class is intended to be copied by value as desired.  It uses the
 * default copy constructor and assignment operator, however it is not a
 * "plain old datatype" (POD) because it has virtual functions.
 *
 * Here are some useful relationships between the cap height (h), the cap
 * radius (r), the maximum chord length from the cap's center (d), and the
 * radius of cap's base (a).
 *
 *     h = 1 - cos(r)
 *       = 2 * sin^2(r/2)
 *   d^2 = 2 * h
 *       = a^2 + h^2
 */
export interface S2Cap<T> {
  /** the center of the cap */
  center: Point3D;
  /** the radius of the cap */
  radius: S1ChordAngle;
  /** the data associated with the cap */
  data: T;
}

/**
 * Return an empty cap, i.e. a cap that contains no points.
 * @param data - the data
 * @returns - the empty cap
 */
export function emptyCap<T>(data: T): S2Cap<T> {
  return { center: [1, 0, 0], radius: negativeAngle(), data };
}

/**
 * Return a full cap, i.e. a cap that contains all points.
 * @param data - the data
 * @returns - the full cap
 */
export function fullCap<T>(data: T): S2Cap<T> {
  // return Init(S2Point.Init(1.0, 0.0, 0.0), S1ChordAngle.Straight(), data_);
  return { center: [1, 0, 0], radius: straightAngle(), data };
}

/**
 * Return the area of the cap.
 * @param cap - the cap
 * @returns - the area
 */
export function getArea<T>(cap: S2Cap<T>): number {
  return 2 * Math.PI * Math.max(0, height(cap));
}

/**
 * Return true if the cap is empty, i.e. it contains no points.
 * @param cap - the cap
 * @returns - true if the cap is empty
 */
export function isEmpty<T>(cap: S2Cap<T>): boolean {
  return cap.radius < 0;
}

/**
 * Return true if the cap is full, i.e. it contains all points.
 * @param cap - the cap
 * @returns - true if the cap is full
 */
export function isFull<T>(cap: S2Cap<T>): boolean {
  return cap.radius === 4;
}

/**
 * Returns the height of the cap, i.e. the distance from the center point to
 * the cutoff plane.
 * @param cap - the cap
 * @returns - the height
 */
export function height<T>(cap: S2Cap<T>): number {
  return 0.5 * cap.radius;
}

/**
 * Constructs a cap with the given center and radius.  A negative radius
 * yields an empty cap; a radius of 180 degrees or more yields a full cap
 * (containing the entire sphere).  "center" should be unit length.
 * @param center - the center point
 * @param radius - the radius
 * @param data - the data
 * @returns - the cap
 */
export function fromS1Angle<T>(center: Point3D, radius: S1Angle, data: T): S2Cap<T> {
  return { center, radius: fromAngle(radius), data };
}

/**
 * Constructs a cap where the angle is expressed as an S1ChordAngle.  This
 * constructor is more efficient than the one above.
 * @param center - the center
 * @param radius - the radius
 * @param data - the data
 * @returns - the cap
 */
export function fromS1ChordAngle<T>(center: Point3D, radius: S1ChordAngle, data: T): S2Cap<T> {
  return { center, radius, data };
}

/**
 * Convenience function that creates a cap containing a single point.  This
 * method is more efficient that the S2Cap(center, radius) constructor.
 * @param center - the center
 * @param data - the data
 * @returns - an empty cap
 */
export function fromS2Point<T>(center: Point3D, data: T): S2Cap<T> {
  return { center, radius: 0, data };
}

/**
 * Return the cap radius as an S1Angle.  (Note that the cap angle is stored
 * internally as an S1ChordAngle, so this method requires a trigonometric
 * operation and may yield a slightly different result than the value passed
 * to the (S2Point, S1Angle) constructor.)
 * @param cap - the cap
 * @returns - the radius as an S1Angle in radians
 */
export function radius<T>(cap: S2Cap<T>): S1Angle {
  return toAngle(cap.radius);
}

/**
 * Returns true if the cap contains the given point.
 * NOTE: The point "p" should be a unit-length vector.
 * @param cap - the cap
 * @param p - the point
 * @returns - true if the cap contains the point
 */
export function containsS2Point<T>(cap: S2Cap<T>, p: Point3D): boolean {
  return fromS2Points(cap.center, p) <= cap.radius;
}

/**
 * Return the complement of the interior of the cap.  A cap and its
 * complement have the same boundary but do not share any interior points.
 * The complement operator is not a bijection because the complement of a
 * singleton cap (containing a single point) is the same as the complement
 * of an empty cap.
 * @param cap - the cap
 * @returns - the complement
 */
export function complement<T>(cap: S2Cap<T>): S2Cap<T> {
  // The complement of a full cap is an empty cap, not a singleton.
  // Also make sure that the complement of an empty cap is full.
  if (isFull(cap)) return emptyCap(cap.data);
  if (isEmpty(cap)) return fullCap(cap.data);
  return {
    center: invert(cap.center),
    radius: fromLength2(K_MAX_LENGTH_2 - cap.radius),
    data: cap.data,
  };
}

/**
 * Return true if the cap contains the given cell.
 * @param cap - the cap
 * @param cell - the cell
 * @returns - true if the cap contains the cell
 */
export function containsS2CellVertexCount<T>(cap: S2Cap<T>, cell: S2CellId): number {
  // If the cap does not contain all cell vertices, return false.
  let count = 0;
  for (const vertex of getVertices(cell)) {
    if (containsS2Point(cap, vertex)) count++;
  }

  return count;
}

/**
 * Return true if the cap contains the given cell.
 * @param cap - the cap
 * @param cell - the cell
 * @returns - true if the cap contains the cell
 */
export function containsS2Cell<T>(cap: S2Cap<T>, cell: S2CellId): boolean {
  // If the cap does not contain all cell vertices, return false.
  // We check the vertices before taking the complement() because we can't
  // accurately represent the complement of a very small cap (a height
  // of 2-epsilon is rounded off to 2).
  const vertices = getVertices(cell);
  for (const vertex of vertices) {
    if (!containsS2Point(cap, vertex)) return false;
  }

  // Otherwise, return true if the complement of the cap does not intersect
  // the cell.  (This test is slightly conservative, because technically we
  // want complement().InteriorIntersects() here.)
  return !intersectsS2Cell(complement(cap), cell, vertices);
}

/**
 * Return true if the cap intersects "cell", given that the cap does intersect
 * any of the cell vertices or edges.
 * @param cap - the cap
 * @param cell - the cell
 * @returns - true if the cap intersects the cell
 */
export function intersectsS2CellFast<T>(cap: S2Cap<T>, cell: S2CellId): boolean {
  // If the cap contains any cell vertex, return true.
  const vertices = getVertices(cell);
  for (const vertex of vertices) {
    if (containsS2Point(cap, vertex)) return true;
  }

  return intersectsS2Cell(cap, cell, vertices);
}

/**
 * Return true if the cap intersects "cell", given that the cap does contain
 * any of the cell vertices (supplied in "vertices", an array of length 4).
 * Return true if this cap intersects any point of 'cell' excluding its
 * vertices (which are assumed to already have been checked).
 * @param cap - the cap
 * @param cell - the cell
 * @param vertices - the vertices of the cell
 * @returns - true if the cap intersects the cell
 */
export function intersectsS2Cell<T>(cap: S2Cap<T>, cell: S2CellId, vertices: Vertices): boolean {
  // If the cap is a hemisphere or larger, the cell and the complement of the
  // cap are both convex.  Therefore since no vertex of the cell is contained,
  // no other interior point of the cell is contained either.
  if (cap.radius >= rightAngle()) return false;
  // We need to check for empty caps due to the center check just below.
  if (isEmpty(cap)) return false;
  // Optimization: return true if the cell contains the cap center.  (This
  // allows half of the edge checks below to be skipped.)
  if (idContainsS2Point(cell, cap.center)) return true;

  // At this point we know that the cell does not contain the cap center,
  // and the cap does not contain any cell vertex.  The only way that they
  // can intersect is if the cap intersects the interior of some edge.
  const sin2Angle = chordAngleSin2(cap.radius);
  const edges = getEdgesRaw(cell);
  for (let k = 0; k < 4; k += 1) {
    const edge = edges[k];
    const dot = s2PointDot(cap.center, edge);
    if (dot > 0) {
      // The center is in the interior half-space defined by the edge.  We don't
      // need to consider these edges, since if the cap intersects this edge
      // then it also intersects the edge on the opposite side of the cell
      // (because we know the center is not contained with the cell).
      continue;
    }
    // The Norm2() factor is necessary because "edge" is not normalized.
    if (dot * dot > sin2Angle * s2PointNorm2(edge)) {
      return false; // Entire cap is on the exterior side of this edge.
    }
    // Otherwise, the great circle containing this edge intersects
    // the interior of the cap.  We just need to check whether the point
    // of closest approach occurs between the two edge endpoints.
    const dir = s2PointCross(edge, cap.center);
    if (s2PointDot(dir, vertices[k]) < 0 && s2PointDot(dir, vertices[(k + 1) & 3]) > 0) return true;
  }
  return false;
}

/**
 * @param cap - the cap
 * @returns - the cells that intersect the cap
 */
export function getIntersectingCells<T>(cap: S2Cap<T>): S2CellId[] {
  if (kMaxEdge === undefined) kMaxEdge = K_MAX_EDGE();
  const res: S2CellId[] = [];
  // Find appropriate max depth for radius
  // while loop:
  // - if cell corners are all in cap, store in res.
  // - if cell corners are all outside cap, move on.
  // - if even one cell corner is outside cap:
  // - - if reached max depth, store in res
  // - - if not reached max depth, store children in cache for another pass

  if (isEmpty(cap)) return res;
  const queue: S2CellId[] = ([0, 1, 2, 3, 4, 5] as Face[]).map(fromFace);
  if (isFull(cap)) return queue;
  const maxDepth = kMaxEdge.getClosestLevel(toAngle(cap.radius));
  while (true) {
    const cell = queue.pop();
    if (cell === undefined) break;
    const vertexCount = containsS2CellVertexCount(cap, cell);
    const maxLevel = level(cell) >= maxDepth;
    if (vertexCount === 4 || (vertexCount > 0 && maxLevel)) {
      res.push(cell);
    } else if (vertexCount === 0 && !maxLevel) {
      // if cap center is in the cell, then we check all children because the cell is larger than the cap
      if (idContainsS2Point(cell, cap.center)) {
        queue.push(...children(cell));
      } else continue;
    } else {
      if (maxLevel) continue;
      queue.push(...children(cell));
    }
  }

  return res;
}
