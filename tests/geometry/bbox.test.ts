import { bboxOverlap, clipBBox, mergeBBoxes, pointOverlap } from '../../src/geometry/bbox';
import { describe, expect, it, test } from 'bun:test';

import type { BBox, BBox3D } from '../../src/geometry';

describe('pointOverlap', () => {
  it('check if point is within bbox', () => {
    expect(pointOverlap([0, 0, 1, 1], { x: 0.5, y: 0.5 })).toBeTrue();
  });
  it('check if point is not within bbox', () => {
    expect(pointOverlap([0, 0, 1, 1], { x: 2, y: 2 })).toBeFalse();
  });
});

describe('bboxOverlap', () => {
  it('no overlap returns undefined', () => {
    expect(bboxOverlap([0, 0, 1, 1], [2, 2, 3, 3])).toBeUndefined();
  });
  it('overlap returns bbox', () => {
    expect(bboxOverlap([0, 0, 1, 1], [0.5, 0.5, 1.5, 1.5])).toEqual([0.5, 0.5, 1, 1]);
  });
});

describe('mergeBBoxes', () => {
  it('first is 2D, second is 3D', () => {
    const bb1: BBox = [0, 0, 1, 1];
    const bb2: BBox3D = [0.4, 0.4, 1.2, 1.2, 0, 1];

    expect(mergeBBoxes(bb1, bb2)).toEqual([0, 0, 1.2, 1.2, 0, 1]);
  });
});

test('clipBBox', () => {
  const bbox: BBox = [0, 0, 10, 10];
  const res = clipBBox(bbox, 0, 2, 8);
  expect(res).toEqual([2, 0, 8, 10]);
  const res2 = clipBBox(res, 1, 2, 8);
  expect(res2).toEqual([2, 2, 8, 8]);
});
