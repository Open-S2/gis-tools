import {
  boundsST,
  centerST,
  child,
  childPosition,
  children,
  childrenIJ,
  contains,
  containsS2Point,
  distance,
  face,
  fromDistance,
  fromFace,
  fromFacePosLevel,
  fromIJ,
  fromIJSame,
  fromIJWrap,
  fromLonLat,
  fromS2Point,
  fromST,
  fromUV,
  getBoundUV,
  getEdges,
  getEdgesRaw,
  getVertices,
  getVerticesRaw,
  intersects,
  isFace,
  isLeaf,
  level,
  neighbors,
  neighborsIJ,
  next,
  parent,
  pos,
  prev,
  range,
  sizeST,
  toFaceIJ,
  toIJ,
  toLonLat,
  toS2Point,
  toST,
  toUV,
  vertexNeighbors,
} from '../../src/geometry/id';
import { describe, expect, it } from 'bun:test';

// Helpers
import { fromLonLat as pointFromLonLat } from '../../src/geometry/s2/point';

describe('boundsST', () => {
  it('should return the bounds for a given id and level', () => {
    expect(boundsST(fromFace(0), 0)).toEqual([0, 0, 1, 1]);
    expect(boundsST(fromFace(0), 1)).toEqual([0.25, 0.25, 0.75, 0.75]);
    expect(boundsST(fromFace(0), 2)).toEqual([0.375, 0.375, 0.625, 0.625]);
    expect(boundsST(fromFace(1), 0)).toEqual([0, 0, 1, 1]);
  });
});

describe('centerST', () => {
  it('should return the center for a given id and level', () => {
    expect(centerST(fromFace(0))).toEqual([0, 0.5, 0.5]);
    expect(centerST(fromFace(1))).toEqual([1, 0.5, 0.5]);
    expect(centerST(fromFace(2))).toEqual([2, 0.5, 0.5]);
    expect(centerST(fromFace(3))).toEqual([3, 0.5, 0.5]);
  });
});

describe('child', () => {
  it('should return the child cell', () => {
    expect(child(fromFace(0), 0n)).toEqual(288230376151711744n);
    expect(child(fromFace(0), 1n)).toEqual(864691128455135232n);
    expect(child(fromFace(0), 2n)).toEqual(1441151880758558720n);
    expect(child(fromFace(0), 3n)).toEqual(2017612633061982208n);
  });
});

describe('childPosition', () => {
  it('should return the child position', () => {
    expect(childPosition(fromFace(0), 0)).toEqual(0);
    expect(childPosition(fromFace(0), 1)).toEqual(2);
    expect(childPosition(fromFace(0), 2)).toEqual(0);
    expect(childPosition(fromFace(1), 0)).toEqual(1);
    expect(childPosition(child(fromFace(0), 1n), 1)).toEqual(1);
  });
});

describe('children', () => {
  it('should return the children cells', () => {
    expect(children(fromFace(0))).toEqual([
      288230376151711744n,
      864691128455135232n,
      1441151880758558720n,
      2017612633061982208n,
    ]);
  });
});

describe('childrenIJ', () => {
  it('should return the children cells', () => {
    expect(childrenIJ(0, 0, 0, 0)).toEqual([
      288230376151711744n,
      2017612633061982208n,
      864691128455135232n,
      1441151880758558720n,
    ]);
  });
});

describe('contains', () => {
  it('should return if the cell contains the given cell', () => {
    expect(contains(fromFace(0), fromFace(0))).toEqual(true);
    expect(contains(fromFace(0), fromFace(1))).toEqual(false);
    expect(contains(fromFace(0), child(fromFace(0), 1n))).toEqual(true);
  });
});

describe('distance', () => {
  it('should return the distance between two cells', () => {
    expect(distance(fromFace(0), 0)).toEqual(0n);
    expect(distance(fromFace(0), 1)).toEqual(2n);
    expect(distance(fromFace(0), 2)).toEqual(8n);
    expect(distance(fromFace(0), 3)).toEqual(32n);
  });
});

describe('face', () => {
  it('should return the face for a given cell', () => {
    expect(face(fromFace(0))).toEqual(0);
    expect(face(fromFace(1))).toEqual(1);
    expect(face(fromFace(2))).toEqual(2);
    expect(face(fromFace(3))).toEqual(3);
    expect(face(fromFace(4))).toEqual(4);
    expect(face(fromFace(5))).toEqual(5);
  });
});

describe('fromDistance', () => {
  it('should return the cell id for a given distance', () => {
    expect(fromDistance(0n)).toEqual(1n);
    expect(fromDistance(1n)).toEqual(3n);
    expect(fromDistance(2n)).toEqual(5n);
    expect(fromDistance(3n)).toEqual(7n);
    expect(fromDistance(4n)).toEqual(9n);
    expect(fromDistance(5n)).toEqual(11n);
  });
});

describe('fromFace', () => {
  it('should return the cell id for a given face', () => {
    expect(fromFace(0)).toEqual(1152921504606846976n);
    expect(fromFace(1)).toEqual(3458764513820540928n);
    expect(fromFace(2)).toEqual(5764607523034234880n);
    expect(fromFace(3)).toEqual(8070450532247928832n);
    expect(fromFace(4)).toEqual(10376293541461622784n);
    expect(fromFace(5)).toEqual(12682136550675316736n);
  });
});

describe('fromFacePosLevel', () => {
  it('should return the cell id for a given face, position, and level', () => {
    expect(fromFacePosLevel(0, 0n, 0)).toEqual(1152921504606846976n);
    expect(fromFacePosLevel(1, 0n, 0)).toEqual(3458764513820540928n);
    expect(fromFacePosLevel(2, 0n, 0)).toEqual(5764607523034234880n);
    expect(fromFacePosLevel(3, 0n, 0)).toEqual(8070450532247928832n);
    expect(fromFacePosLevel(4, 0n, 0)).toEqual(10376293541461622784n);
    expect(fromFacePosLevel(5, 0n, 0)).toEqual(12682136550675316736n);
  });

  it('zoom 1', () => {
    expect(fromFacePosLevel(0, 0n, 1)).toEqual(288230376151711744n);
    expect(fromFacePosLevel(0, 1n, 1)).toEqual(288230376151711744n);
    expect(fromFacePosLevel(0, 2n, 1)).toEqual(288230376151711744n);
    expect(fromFacePosLevel(0, 3n, 1)).toEqual(288230376151711744n);
  });
});

describe('fromIJ', () => {
  it('should return the cell id for a given i-j', () => {
    expect(fromIJ(0, 0, 0)).toEqual(1n);
    expect(fromIJ(0, 1, 0)).toEqual(3n);
    expect(fromIJ(0, 1, 1)).toEqual(5n);
    expect(fromIJ(0, 0, 1)).toEqual(7n);
  });
});

describe('fromIJSame', () => {
  it('should return the cell id for a given i-j', () => {
    expect(fromIJSame(0, 0, 0, true)).toEqual(1n);
    expect(fromIJSame(0, 1, 0, true)).toEqual(3n);
    expect(fromIJSame(0, 1, 1, true)).toEqual(5n);
    expect(fromIJSame(0, 0, 1, true)).toEqual(7n);
  });
});

describe('fromIJWrap', () => {
  it('should return the cell id for a given i-j', () => {
    expect(fromIJWrap(0, 0, 0)).toEqual(1n);
    expect(fromIJWrap(0, 1, 0)).toEqual(3n);
    expect(fromIJWrap(0, 1, 1)).toEqual(5n);
    expect(fromIJWrap(0, 0, 1)).toEqual(7n);
  });
});

describe('fromLonLat', () => {
  it('should return the cell id for a given lon-lat', () => {
    expect(fromLonLat({ x: 0, y: 0 })).toEqual(1152921504606846977n);
    expect(fromLonLat({ x: 90, y: 0 })).toEqual(3458764513820540929n);
    expect(fromLonLat({ x: 0, y: 90 })).toEqual(5764607523034234881n);
    expect(fromLonLat({ x: -90, y: 0 })).toEqual(10376293541461622785n);
    expect(fromLonLat({ x: 0, y: -90 })).toEqual(12682136550675316737n);
  });
});

describe('fromS2Point', () => {
  it('should return the cell id for a given s2 point', () => {
    expect(fromS2Point({ x: 1, y: 0, z: 0 })).toEqual(1152921504606846977n);
    expect(fromS2Point({ x: 0, y: 1, z: 0 })).toEqual(3458764513820540929n);
    expect(fromS2Point({ x: 0, y: 0, z: 1 })).toEqual(5764607523034234881n);
    expect(fromS2Point({ x: -1, y: 0, z: 0 })).toEqual(8070450532247928833n);
    expect(fromS2Point({ x: 0, y: -1, z: 0 })).toEqual(10376293541461622785n);
    expect(fromS2Point({ x: 0, y: 0, z: -1 })).toEqual(12682136550675316737n);
  });
});

describe('fromST', () => {
  it('should return the cell id for a given a Face-S-T', () => {
    expect(fromST(0, 0, 0)).toEqual(1n);
    expect(fromST(0, 1, 0)).toEqual(2305843009213693951n);
    expect(fromST(0, 0, 1)).toEqual(768614336404564651n);
    expect(fromST(0, 0.5, 0.5)).toEqual(1152921504606846977n);
    expect(fromST(0, 1, 1)).toEqual(1537228672809129301n);
  });
});

describe('fromUV', () => {
  it('should return the cell id for a given a Face-U-V', () => {
    expect(fromUV(0, 0, 0)).toEqual(1152921504606846977n);
    expect(fromUV(0, 1, 0)).toEqual(1729382256910270463n);
    expect(fromUV(0, 0, 1)).toEqual(1345075088707988139n);
    expect(fromUV(0, -1, 0)).toEqual(576460752303423489n);
    expect(fromUV(0, 0, -1)).toEqual(2113689425112552789n);
  });
});

describe('intersects', () => {
  it('should return if the cell intersects the given cell', () => {
    expect(intersects(fromFace(0), fromFace(0))).toEqual(true);
    expect(intersects(fromFace(0), fromFace(1))).toEqual(false);
    expect(intersects(fromFace(0), child(fromFace(0), 1n))).toEqual(true);
  });
});

describe('isFace', () => {
  it('should return if the cell is a face', () => {
    expect(isFace(1152921504606846976n)).toEqual(true);
    expect(isFace(1152921504606846977n)).toEqual(false);
  });
});

describe('isLeaf', () => {
  it('should return if the cell is a leaf', () => {
    expect(isLeaf(1152921504606846976n)).toEqual(false);
    expect(isLeaf(1152921504606846977n)).toEqual(true);
  });
});

describe('level', () => {
  it('should return the level for a given cell', () => {
    expect(level(1152921504606846976n)).toEqual(0);
    expect(level(1152921504606846977n)).toEqual(30);
  });
});

describe('neighbors', () => {
  it('should return the neighbors cells', () => {
    expect(neighbors(fromFace(0))).toEqual([
      12682136550675316736n,
      3458764513820540928n,
      5764607523034234880n,
      10376293541461622784n,
    ]);
  });
});

describe('neighborsIJ', () => {
  it('should return the neighbors cells', () => {
    expect(neighborsIJ(0, 0, 0, 0)).toEqual([
      12682136550675316736n,
      3458764513820540928n,
      5764607523034234880n,
      10376293541461622784n,
    ]);
  });
});

describe('next', () => {
  it('should return the next cell', () => {
    expect(next(fromFace(0))).toEqual(3458764513820540928n);
    expect(next(fromFace(1))).toEqual(5764607523034234880n);
    expect(next(fromFace(2))).toEqual(8070450532247928832n);
    expect(next(fromFace(3))).toEqual(10376293541461622784n);
  });
});

describe('parent', () => {
  it('should return the parent cell', () => {
    expect(parent(child(fromFace(0), 0n))).toEqual(fromFace(0));
  });
});

describe('pos', () => {
  it('should return the position for a given cell', () => {
    expect(pos(fromFace(0))).toEqual(1152921504606846976n);
    expect(pos(fromFace(1))).toEqual(1152921504606846976n);
    expect(pos(fromFace(2))).toEqual(1152921504606846976n);
    expect(pos(fromFace(3))).toEqual(1152921504606846976n);
  });
});

describe('prev', () => {
  it('should return the prev cell', () => {
    expect(prev(fromFace(1))).toEqual(1152921504606846976n);
    expect(prev(fromFace(2))).toEqual(3458764513820540928n);
    expect(prev(fromFace(3))).toEqual(5764607523034234880n);
    expect(prev(fromFace(4))).toEqual(8070450532247928832n);
    expect(prev(fromFace(5))).toEqual(10376293541461622784n);
  });
});

describe('range', () => {
  it('should return the range for a given level', () => {
    expect(range(0n)).toEqual([1n, -1n]);
    expect(range(1n)).toEqual([1n, 1n]);
    expect(range(fromFace(0))).toEqual([1n, 2305843009213693951n]);
  });
});

describe('sizeST', () => {
  it('should return the size for a given level', () => {
    expect(sizeST(0)).toEqual(1);
    expect(sizeST(1)).toEqual(0.5);
    expect(sizeST(2)).toEqual(0.25);
  });
});

describe('toIJ', () => {
  it('should return the i-j for a given cell', () => {
    expect(toIJ(fromFace(0))).toEqual([0, 536870912, 536870912, 0]);
    expect(toIJ(fromFace(1))).toEqual([1, 536870912, 536870912, 1]);
    expect(toIJ(fromFace(2))).toEqual([2, 536870912, 536870912, 0]);
    expect(toIJ(fromFace(3))).toEqual([3, 536870912, 536870912, 1]);
  });
  it('also given a level, should return the i-j for a given cell', () => {
    expect(toIJ(fromFace(0), 0)).toEqual([0, 0, 0, 0]);
    expect(toIJ(fromFace(0), 1)).toEqual([0, 1, 1, 0]);
  });
});

describe('toLonLat', () => {
  it('should return the lon-lat for a given cell', () => {
    expect(toLonLat(fromFace(0))).toEqual({ x: 0, y: 0 });
    expect(toLonLat(fromFace(1))).toEqual({ x: 90, y: 0 });
    expect(toLonLat(fromFace(2))).toEqual({ x: -180, y: 90 });
    expect(toLonLat(fromFace(3))).toEqual({ x: -180, y: -0 });
  });
});

describe('toS2Point', () => {
  it('should return the s2 point for a given cell', () => {
    expect(toS2Point(fromFace(0))).toEqual({ x: 1, y: 0, z: 0 });
    expect(toS2Point(fromFace(1))).toEqual({ x: -0, y: 1, z: 0 });
    expect(toS2Point(fromFace(2))).toEqual({ x: -0, y: -0, z: 1 });
    expect(toS2Point(fromFace(3))).toEqual({ x: -1, y: -0, z: -0 });
  });
});

describe('toST', () => {
  it('should return the s-t for a given cell', () => {
    expect(toST(fromFace(0))).toEqual([0, 0.5, 0.5]);
    expect(toST(fromFace(1))).toEqual([1, 0.5, 0.5]);
    expect(toST(fromFace(2))).toEqual([2, 0.5, 0.5]);
    expect(toST(fromFace(3))).toEqual([3, 0.5, 0.5]);
    expect(toST(child(fromFace(0), 0n))).toEqual([0, 0.25, 0.25]);
    expect(toST(child(fromFace(0), 1n))).toEqual([0, 0.25, 0.75]);
  });
});

describe('toUV', () => {
  it('should return the u-v for a given cell', () => {
    expect(toUV(fromFace(0))).toEqual([0, 0, 0]);
    expect(toUV(fromFace(1))).toEqual([1, 0, 0]);
    expect(toUV(fromFace(2))).toEqual([2, 0, 0]);
    expect(toUV(fromFace(3))).toEqual([3, 0, 0]);
    expect(toUV(child(fromFace(0), 0n))).toEqual([0, -0.41666666666666663, -0.41666666666666663]);
    expect(toUV(child(fromFace(0), 1n))).toEqual([0, -0.41666666666666663, 0.41666666666666663]);
  });
});

describe('vertexNeighbors', () => {
  it('should return the vertex neighbors for a given cell', () => {
    expect(vertexNeighbors(fromFace(0))).toEqual([
      1152921504606846976n,
      3458764513820540928n,
      5764607523034234880n,
    ]);
    expect(vertexNeighbors(123974589433424n)).toEqual([
      123974589433424n,
      123974589433584n,
      123974589433776n,
      123974589433616n,
    ]);
  });
});

describe('toFaceIJ', () => {
  const id = toFaceIJ(0n);
  expect(id).toEqual([0, 0, 0, 0]);
});

describe('containsS2Point', () => {
  const face0 = fromFace(0);
  const point = pointFromLonLat({ x: 0, y: 0 });
  const point2 = pointFromLonLat({ x: -160, y: 70 });
  expect(containsS2Point(face0, point)).toEqual(true);
  expect(containsS2Point(face0, point2)).toEqual(false);
});

describe('getBoundUV', () => {
  expect(getBoundUV(fromFace(0))).toEqual([-1, 1, -1, 1]);
  expect(getBoundUV(fromFace(1))).toEqual([-1, 1, -1, 1]);

  const [a, b, c, d] = children(fromFace(0));
  expect(getBoundUV(a)).toEqual([-1, 0, -1, 0]);
  expect(getBoundUV(b)).toEqual([-1, 0, 0, 1]);
  expect(getBoundUV(c)).toEqual([0, 1, 0, 1]);
  expect(getBoundUV(d)).toEqual([0, 1, -1, 0]);
});

describe('getEdgesRaw', () => {
  expect(getEdgesRaw(fromFace(0))).toEqual([
    { x: 1, y: 0, z: 1 },
    { x: 1, y: -1, z: 0 },
    { x: 1, y: -0, z: -1 },
    { x: 1, y: 1, z: -0 },
  ]);

  const level10 = fromIJ(0, 10, 20, 10);
  expect(getEdgesRaw(level10)).toEqual([
    { x: 0.94842529296875, y: 0, z: 1 },
    { x: -0.9715080261230469, y: -1, z: 0 },
    { x: -0.9458732604980469, y: -0, z: -1 },
    { x: 0.9740854899088541, y: 1, z: -0 },
  ]);
});

describe('getEdges', () => {
  expect(getEdges(fromFace(0))).toEqual([
    { x: 0.7071067811865475, y: 0, z: 0.7071067811865475 },
    { x: 0.7071067811865475, y: -0.7071067811865475, z: 0 },
    { x: 0.7071067811865475, y: -0, z: -0.7071067811865475 },
    { x: 0.7071067811865475, y: 0.7071067811865475, z: -0 },
  ]);

  const level10 = fromIJ(0, 10, 20, 10);
  expect(getEdges(level10)).toEqual([
    { x: 0.6881486685943737, y: 0, z: 0.7255697140260132 },
    { x: -0.696815003909965, y: -0.7172508977519342, z: 0 },
    { x: -0.6871719848800082, y: -0, z: -0.7264947785057162 },
    { x: 0.6977642240401561, y: 0.7163275002745871, z: -0 },
  ]);
});

describe('getVerticesRaw', () => {
  expect(getVerticesRaw(fromFace(0))).toEqual([
    { x: 1, y: -1, z: -1 },
    { x: 1, y: 1, z: -1 },
    { x: 1, y: 1, z: 1 },
    { x: 1, y: -1, z: 1 },
  ]);

  const level10 = fromIJ(0, 10, 20, 10);
  expect(getVerticesRaw(level10)).toEqual([
    { x: 1, y: -0.9740854899088541, z: -0.94842529296875 },
    { x: 1, y: -0.9715080261230469, z: -0.94842529296875 },
    { x: 1, y: -0.9715080261230469, z: -0.9458732604980469 },
    { x: 1, y: -0.9740854899088541, z: -0.9458732604980469 },
  ]);
});

describe('getVertices', () => {
  expect(getVertices(fromFace(0))).toEqual([
    { x: 0.5773502691896258, y: -0.5773502691896258, z: -0.5773502691896258 },
    { x: 0.5773502691896258, y: 0.5773502691896258, z: -0.5773502691896258 },
    { x: 0.5773502691896258, y: 0.5773502691896258, z: 0.5773502691896258 },
    { x: 0.5773502691896258, y: -0.5773502691896258, z: 0.5773502691896258 },
  ]);

  const level10 = fromIJ(0, 10, 20, 10);
  expect(getVertices(level10)).toEqual([
    { x: 0.5925201015153633, y: -0.5771652333654366, z: -0.5619610508695819 },
    { x: 0.5930423748666049, y: -0.5761454270139794, z: -0.5624563881257431 },
    { x: 0.593547171020095, y: -0.576635840528651, z: -0.5614203979121691 },
    { x: 0.5930235640377648, y: -0.5776556489032209, z: -0.5609251320685729 },
  ]);
});
