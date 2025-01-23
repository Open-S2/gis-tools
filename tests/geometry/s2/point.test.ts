import {
  add,
  addMut,
  addScalar,
  cross,
  distance,
  distanceEarth,
  div,
  divMutScalar,
  divScalar,
  fromIJ,
  fromLonLat,
  fromLonLatGL,
  fromS2CellID,
  fromST,
  fromSTGL,
  fromUV,
  fromUVGL,
  getFace,
  length,
  mul,
  mulScalar,
  normalize,
  sub,
  subScalar,
  toIJ,
  toLonLat,
  toS2CellID,
  toST,
  toUV,
} from '../../../src/geometry/s2/point';
import { describe, expect, it } from 'bun:test';

import type { VectorPoint } from '../../../src/geometry';

describe('addScalar', (): void => {
  it('should add 1 to each component of an XYZ point', (): void => {
    expect(addScalar({ x: 1, y: 2, z: 3 }, 1)).toEqual({ x: 2, y: 3, z: 4 });
  });
});

describe('add', (): void => {
  it('should add an XYZ point to another XYZ point', (): void => {
    expect(add({ x: 1, y: 2, z: 3 }, { x: 1, y: 2, z: 3 })).toEqual({ x: 2, y: 4, z: 6 });
  });
});

describe('addMut', (): void => {
  it('should add an XYZ point to another XYZ point in place', (): void => {
    const a: VectorPoint = { x: 1, y: 2, z: 3 };
    const b: VectorPoint = { x: 1, y: 2, z: 3 };
    addMut(a, b);
    expect(a).toEqual({ x: 2, y: 4, z: 6 });
    expect(b).toEqual({ x: 1, y: 2, z: 3 });
  });
});

describe('mul', (): void => {
  it('should multiply each component of an XYZ point by another XYZ point', (): void => {
    expect(mul({ x: 1, y: 2, z: 3 }, { x: 2, y: 3, z: 4 })).toEqual({ x: 2, y: 6, z: 12 });
  });
});

describe('mulScalar', (): void => {
  it('should multiply each component of an XYZ point by 2', (): void => {
    expect(mulScalar({ x: 1, y: 2, z: 3 }, 2)).toEqual({ x: 2, y: 4, z: 6 });
  });
});

describe('div', (): void => {
  it('should divide each component of an XYZ point by another XYZ point', (): void => {
    expect(div({ x: 1, y: 2, z: 3 }, { x: 2, y: 3, z: 4 })).toEqual({
      x: 0.5,
      y: 0.6666666666666666,
      z: 0.75,
    });
  });
});

describe('divScalar', (): void => {
  it('should divide each component of an XYZ point by 2', (): void => {
    expect(divScalar({ x: 1, y: 2, z: 3 }, 2)).toEqual({ x: 0.5, y: 1, z: 1.5 });
  });
});

describe('divMutScalar', (): void => {
  it('should divide each component of an XYZ point by 2 in place', (): void => {
    const a: VectorPoint = { x: 1, y: 2, z: 3 };
    divMutScalar(a, 2);
    expect(a).toEqual({ x: 0.5, y: 1, z: 1.5 });
  });
});

describe('cross', (): void => {
  const a: VectorPoint = { x: 1, y: 2, z: 3 };
  const b: VectorPoint = { x: 1, y: 2, z: 3 };
  const c: VectorPoint = { x: 5, y: 6, z: 7 };
  expect(cross(a, b)).toEqual({ x: 0, y: 0, z: 0 });
  expect(cross(a, c)).toEqual({ x: -4, y: 8, z: -4 });
});

describe('distance', (): void => {
  it('should calculate the distance between two XYZ points', (): void => {
    expect(distance({ x: 1, y: 2, z: 3 }, { x: 4, y: 5, z: 6 })).toBeCloseTo(5.196152422706632);
  });
});

describe('distanceEarth', (): void => {
  it('should calculate the distance between two of the same lon/lat points', (): void => {
    expect(distanceEarth({ x: 0, y: 0, z: 0 }, { x: 0, y: 0, z: 0 })).toBeCloseTo(0);
  });
  it('should calculate the distance between two different lon/lat points', (): void => {
    expect(distanceEarth({ x: 0, y: 0, z: 0 }, { x: 90, y: 0, z: 0 })).toBeCloseTo(574032330);
  });
});

describe('fromIJ', (): void => {
  it('should convert an Face-I-J of 0-0-0 to an XYZ point', (): void => {
    expect(fromIJ(0, 0, 0)).toEqual({
      x: 0.5773502691896258,
      y: -0.5773502691896258,
      z: -0.5773502691896258,
    });
  });
  it('should convert an Face-I-J of 1-0-0 to an XYZ point', (): void => {
    expect(fromIJ(1, 0, 0)).toEqual({
      x: 0.5773502691896258,
      y: 0.5773502691896258,
      z: -0.5773502691896258,
    });
  });
  it('should convert an Face-I-J of 2-20-100 to an XYZ point', (): void => {
    expect(fromIJ(2, 20, 100)).toEqual({
      x: 0.5773502978669214,
      y: 0.577350183157724,
      z: 0.577350326544222,
    });
  });
});

describe('fromLonLat', (): void => {
  it('should convert a lon/lat of 0-0 to an XYZ point', (): void => {
    expect(fromLonLat({ x: 0, y: 0 })).toEqual({ x: 1, y: 0, z: 0 });
  });
  it('should convert a lon/lat of 90-0 to an XYZ point', (): void => {
    expect(fromLonLat({ x: 90, y: 0 })).toEqual({
      x: 0.00000000000000006123233995736766,
      y: 1,
      z: 0,
    });
  });
  it('should convert a lon/lat of 0-90 to an XYZ point', (): void => {
    expect(fromLonLat({ x: 0, y: 90 })).toEqual({
      x: 0.00000000000000006123233995736766,
      y: 0,
      z: 1,
    });
  });
});

describe('fromLonLatGL', (): void => {
  it('should convert a lon/lat of 0-0 to an XYZ point', (): void => {
    expect(fromLonLatGL({ x: 0, y: 0 })).toEqual({ x: 0, y: 0, z: 1 });
  });
  it('should convert a lon/lat of 90-0 to an XYZ point', (): void => {
    expect(fromLonLatGL({ x: 90, y: 0 })).toEqual({
      x: 1,
      y: 0,
      z: 0.00000000000000006123233995736766,
    });
  });
  it('should convert a lon/lat of 0-90 to an XYZ point', (): void => {
    expect(fromLonLatGL({ x: 0, y: 90 })).toEqual({
      x: 0,
      y: 1,
      z: 0.00000000000000006123233995736766,
    });
  });
});

// fromS2CellID
describe('fromS2CellID', (): void => {
  it('should convert a S2CellID of 0n to an XYZ point', (): void => {
    expect(fromS2CellID(0n)).toEqual({
      x: 0.5773502691896258,
      y: -0.5773502691896258,
      z: -0.5773502691896258,
    });
  });
  it('should convert a S2CellID of 1n to an XYZ point', (): void => {
    expect(fromS2CellID(1n)).toEqual({
      x: 0.5773502691896258,
      y: -0.5773502691896258,
      z: -0.5773502691896258,
    });
  });
  it('should convert a S2CellID of 2n to an XYZ point', (): void => {
    expect(fromS2CellID(2n)).toEqual({
      x: 0.5773502696675807,
      y: -0.5773502682337158,
      z: -0.5773502696675807,
    });
  });
  it('should convert a S2CellID of 12345678n to an XYZ point', (): void => {
    expect(fromS2CellID(12345678n)).toEqual({
      x: 0.5773521480306643,
      y: -0.5773512102802507,
      z: -0.5773474492472517,
    });
  });
});

describe('fromST', (): void => {
  it('should convert a Face-S-T of 0-0-0 to an XYZ point', (): void => {
    expect(fromST(0, 0, 0)).toEqual({
      x: 0.5773502691896258,
      y: -0.5773502691896258,
      z: -0.5773502691896258,
    });
  });
  it('should convert a Face-S-T of 1-0-0 to an XYZ point', (): void => {
    expect(fromST(1, 0, 0)).toEqual({
      x: 0.5773502691896258,
      y: 0.5773502691896258,
      z: -0.5773502691896258,
    });
  });
  it('should convert a Face-S-T of 2-0-0 to an XYZ point', (): void => {
    expect(fromST(2, 0, 0)).toEqual({
      x: 0.5773502691896258,
      y: 0.5773502691896258,
      z: 0.5773502691896258,
    });
  });
  it('should convert a Face-S-T of 3-0-0 to an XYZ point', (): void => {
    expect(fromST(3, 0, 0)).toEqual({
      x: -0.5773502691896258,
      y: 0.5773502691896258,
      z: 0.5773502691896258,
    });
  });
  it('should convert a Face-S-T of 4-0-0 to an XYZ point', (): void => {
    expect(fromST(4, 0, 0)).toEqual({
      x: -0.5773502691896258,
      y: -0.5773502691896258,
      z: 0.5773502691896258,
    });
  });
  it('should convert a Face-S-T of 5-0-0 to an XYZ point', (): void => {
    expect(fromST(5, 0, 0)).toEqual({
      x: -0.5773502691896258,
      y: -0.5773502691896258,
      z: -0.5773502691896258,
    });
  });
});

describe('fromSTGL', (): void => {
  it('should convert a Face-S-T of 0-0-0 to an XYZ point', (): void => {
    expect(fromSTGL(0, 0, 0)).toEqual({
      x: -0.5773502691896258,
      y: -0.5773502691896258,
      z: 0.5773502691896258,
    });
  });
  it('should convert a Face-S-T of 1-0-0 to an XYZ point', (): void => {
    expect(fromSTGL(1, 0, 0)).toEqual({
      x: 0.5773502691896258,
      y: -0.5773502691896258,
      z: 0.5773502691896258,
    });
  });
  it('should convert a Face-S-T of 2-0-0 to an XYZ point', (): void => {
    expect(fromSTGL(2, 0, 0)).toEqual({
      x: 0.5773502691896258,
      y: 0.5773502691896258,
      z: 0.5773502691896258,
    });
  });
  it('should convert a Face-S-T of 3-0-0 to an XYZ point', (): void => {
    expect(fromSTGL(3, 0, 0)).toEqual({
      x: 0.5773502691896258,
      y: 0.5773502691896258,
      z: -0.5773502691896258,
    });
  });
  it('should convert a Face-S-T of 4-0-0 to an XYZ point', (): void => {
    expect(fromSTGL(4, 0, 0)).toEqual({
      x: -0.5773502691896258,
      y: 0.5773502691896258,
      z: -0.5773502691896258,
    });
  });
  it('should convert a Face-S-T of 5-0-0 to an XYZ point', (): void => {
    expect(fromSTGL(5, 0, 0)).toEqual({
      x: -0.5773502691896258,
      y: -0.5773502691896258,
      z: -0.5773502691896258,
    });
  });
});

describe('fromUV', (): void => {
  it('should convert a Face-U-V of 0-0-0 to an XYZ point', (): void => {
    expect(fromUV(0, 0, 0)).toEqual({ x: 1, y: 0, z: 0 });
  });
  it('should convert a Face-U-V of 1-0-0 to an XYZ point', (): void => {
    expect(fromUV(1, 0, 0)).toEqual({ x: -0, y: 1, z: 0 });
  });
  it('should convert a Face-U-V of 2-0-0 to an XYZ point', (): void => {
    expect(fromUV(2, 0, 0)).toEqual({ x: -0, y: -0, z: 1 });
  });
  it('should convert a Face-U-V of 3-0-0 to an XYZ point', (): void => {
    expect(fromUV(3, 0, 0)).toEqual({ x: -1, y: -0, z: -0 });
  });
  it('should convert a Face-U-V of 4-0-0 to an XYZ point', (): void => {
    expect(fromUV(4, 0, 0)).toEqual({ x: 0, y: -1, z: -0 });
  });
  it('should convert a Face-U-V of 5-0-0 to an XYZ point', (): void => {
    expect(fromUV(5, 0, 0)).toEqual({ x: 0, y: 0, z: -1 });
  });
});

describe('fromUVGL', (): void => {
  it('should convert a Face-U-V of 0-0-0 to an XYZ point', (): void => {
    expect(fromUVGL(0, 0, 0)).toEqual({ x: 0, y: 0, z: 1 });
  });
  it('should convert a Face-U-V of 1-0-0 to an XYZ point', (): void => {
    expect(fromUVGL(1, 0, 0)).toEqual({ x: 1, y: 0, z: -0 });
  });
  it('should convert a Face-U-V of 2-0-0 to an XYZ point', (): void => {
    expect(fromUVGL(2, 0, 0)).toEqual({ x: -0, y: 1, z: -0 });
  });
  it('should convert a Face-U-V of 3-0-0 to an XYZ point', (): void => {
    expect(fromUVGL(3, 0, 0)).toEqual({ x: -0, y: -0, z: -1 });
  });
  it('should convert a Face-U-V of 4-0-0 to an XYZ point', (): void => {
    expect(fromUVGL(4, 0, 0)).toEqual({ x: -1, y: -0, z: 0 });
  });
  it('should convert a Face-U-V of 5-0-0 to an XYZ point', (): void => {
    expect(fromUVGL(5, 0, 0)).toEqual({ x: 0, y: -1, z: 0 });
  });
});

describe('getFace', (): void => {
  it('should return the face of an XYZ point', (): void => {
    expect(getFace({ x: 1, y: 0, z: 0 })).toEqual(0);
    expect(getFace({ x: 0, y: 1, z: 0 })).toEqual(1);
    expect(getFace({ x: 0, y: 0, z: 1 })).toEqual(2);
    expect(getFace({ x: -1, y: 0, z: 0 })).toEqual(3);
    expect(getFace({ x: 0, y: -1, z: 0 })).toEqual(4);
    expect(getFace({ x: 0, y: 0, z: -1 })).toEqual(5);
    expect(getFace({ x: 0.5, y: 0.5, z: 0.5 })).toEqual(2);
  });
});

describe('length', (): void => {
  it('should calculate the length of an XYZ point', (): void => {
    expect(length({ x: 1, y: 2, z: 3 })).toBeCloseTo(3.7416573867739413);
  });
});

describe('normalize', (): void => {
  it('should normalize an XYZ point', (): void => {
    expect(normalize({ x: 1, y: 2, z: 3 })).toEqual({
      x: 0.2672612419124244,
      y: 0.5345224838248488,
      z: 0.8017837257372732,
    });
  });
});

describe('subScalar', (): void => {
  it('should subtract 1 from each component of an XYZ point', (): void => {
    expect(subScalar({ x: 1, y: 2, z: 3 }, 1)).toEqual({ x: 0, y: 1, z: 2 });
  });
});

describe('sub', (): void => {
  it('should subtract an XYZ point from another XYZ point', (): void => {
    expect(sub({ x: 1, y: 2, z: 3 }, { x: 1, y: 2, z: 3 })).toEqual({ x: 0, y: 0, z: 0 });
  });
});

describe('toIJ', (): void => {
  it('should convert an XYZ point to a Face-I-J', (): void => {
    expect(toIJ({ x: 0.5773502691896258, y: -0.5773502691896258, z: -0.5773502691896258 })).toEqual(
      [5, 0, 1073741823],
    );
  });
  it('should convert an XYZ point to a Face-I-J', (): void => {
    expect(toIJ({ x: 0.5773502691896258, y: 0.5773502691896258, z: -0.5773502691896258 })).toEqual([
      5, 1073741823, 1073741823,
    ]);
  });
  it('should convert an XYZ point to a Face-I-J', (): void => {
    expect(toIJ({ x: 0.9999999503294631, y: 0.9999997516473249, z: 1 })).toEqual([2, 20, 100]);
  });
  it('should convert an XYZ point to a Face-I-J including a level of 10', (): void => {
    expect(
      toIJ({ x: 0.5773502691896258, y: -0.5773502691896258, z: -0.5773502691896258 }, 10),
    ).toEqual([5, 0, 1023]);
  });
});

describe('toLonLat', (): void => {
  it('should convert an XYZ point to a lon/lat', (): void => {
    expect(toLonLat({ x: 1, y: 0, z: 0 })).toEqual({ x: 0, y: 0 });
  });
  it('should convert an XYZ point to a lon/lat', (): void => {
    expect(toLonLat({ x: 0.00000000000000006123233995736766, y: 1, z: 0 })).toEqual({
      x: 90,
      y: 0,
    });
  });
  it('should convert an XYZ point to a lon/lat', (): void => {
    expect(toLonLat({ x: 0.00000000000000006123233995736766, y: 0, z: 1 })).toEqual({
      x: 0,
      y: 90,
    });
  });
});

describe('toS2CellID', (): void => {
  it('should convert an XYZ point to a S2CellID', (): void => {
    expect(
      toS2CellID({ x: 0.5773502691896258, y: -0.5773502691896258, z: -0.5773502691896258 }),
    ).toEqual(13835058055282163711n);
  });
  it('should convert an XYZ point to a S2CellID', (): void => {
    expect(
      toS2CellID({ x: 0.5773502691896258, y: 0.5773502691896258, z: -0.5773502691896258 }),
    ).toEqual(13066443718877599061n);
  });
  it('should convert an XYZ point to a S2CellID', (): void => {
    expect(toS2CellID({ x: 0.9999999503294631, y: 0.9999997516473249, z: 1 })).toEqual(
      4611686018427419201n,
    );
  });
});

describe('toST', (): void => {
  it('should convert an XYZ point to a Face-S-T', (): void => {
    expect(toST({ x: 0.5773502691896258, y: -0.5773502691896258, z: -0.5773502691896258 })).toEqual(
      [5, 0, 1],
    );
  });
  it('should convert an XYZ point to a Face-S-T', (): void => {
    expect(toST({ x: 0.5773502691896258, y: 0.5773502691896258, z: -0.5773502691896258 })).toEqual([
      5, 1, 1,
    ]);
  });
  it('should convert an XYZ point to a Face-S-T', (): void => {
    expect(toST({ x: 0.5773502691896258, y: 0.5773502691896258, z: 0.5773502691896258 })).toEqual([
      2, 0, 0,
    ]);
  });
  it('should convert an XYZ point to a Face-S-T', (): void => {
    expect(toST({ x: -0.5773502691896258, y: 0.5773502691896258, z: 0.5773502691896258 })).toEqual([
      2, 1, 0,
    ]);
  });
  it('should convert an XYZ point to a Face-S-T', (): void => {
    expect(toST({ x: -0.5773502691896258, y: -0.5773502691896258, z: 0.5773502691896258 })).toEqual(
      [2, 1, 1],
    );
  });
  it('should convert an XYZ point to a Face-S-T', (): void => {
    expect(
      toST({ x: -0.5773502691896258, y: -0.5773502691896258, z: -0.5773502691896258 }),
    ).toEqual([5, 0, 0]);
  });
});

describe('toUV', (): void => {
  it('should convert an XYZ point to a Face-U-V', (): void => {
    expect(toUV({ x: 1, y: 0, z: 0 })).toEqual([0, 0, 0]);
  });
  it('should convert an XYZ point to a Face-U-V', (): void => {
    expect(toUV({ x: 0, y: 1, z: 0 })).toEqual([1, -0, 0]);
  });
  it('should convert an XYZ point to a Face-U-V', (): void => {
    expect(toUV({ x: 0, y: 0, z: 1 })).toEqual([2, -0, -0]);
  });
  it('should convert an XYZ point to a Face-U-V', (): void => {
    expect(toUV({ x: -1, y: 0, z: 0 })).toEqual([3, -0, -0]);
  });
  it('should convert an XYZ point to a Face-U-V', (): void => {
    expect(toUV({ x: 0, y: -1, z: 0 })).toEqual([4, -0, 0]);
  });
  it('should convert an XYZ point to a Face-U-V', (): void => {
    expect(toUV({ x: 0, y: 0, z: -1 })).toEqual([5, 0, 0]);
  });
});
