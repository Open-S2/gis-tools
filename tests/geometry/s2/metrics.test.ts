import {
  K_AVG_ANGLE_SPAN,
  K_AVG_AREA,
  K_AVG_DIAG,
  K_AVG_EDGE,
  K_AVG_WIDTH,
  K_MAX_ANGLE_SPAN,
  K_MAX_AREA,
  K_MAX_DIAG,
  K_MAX_EDGE,
  K_MAX_WIDTH,
  K_MIN_ANGLE_SPAN,
  K_MIN_AREA,
  K_MIN_DIAG,
  K_MIN_EDGE,
  K_MIN_WIDTH,
} from '../../../src/geometry/s2/metrics';
import { describe, expect, it } from 'bun:test';

// ANGLE SPAN

describe('K_AVG_ANGLE_SPAN', () => {
  it('getValue', () => {
    expect(K_AVG_ANGLE_SPAN.getValue(0)).toEqual(1.5707963267948966);
    expect(K_AVG_ANGLE_SPAN.getValue(1)).toEqual(0.7853981633974483);
    expect(K_AVG_ANGLE_SPAN.getValue(2)).toEqual(0.39269908169872414);
    expect(K_AVG_ANGLE_SPAN.getValue(3)).toEqual(0.19634954084936207);
  });

  it('getClosestLevel', () => {
    expect(K_AVG_ANGLE_SPAN.getClosestLevel(0)).toEqual(30);
    expect(K_AVG_ANGLE_SPAN.getClosestLevel(1.5707963267948966)).toEqual(0);
    expect(K_AVG_ANGLE_SPAN.getClosestLevel(0.7853981633974483)).toEqual(1);
    expect(K_AVG_ANGLE_SPAN.getClosestLevel(0.77)).toEqual(1);
    expect(K_AVG_ANGLE_SPAN.getClosestLevel(0.44)).toEqual(2);
    expect(K_AVG_ANGLE_SPAN.getClosestLevel(0.39269908169872414)).toEqual(2);
    expect(K_AVG_ANGLE_SPAN.getClosestLevel(0.19634954084936207)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_AVG_ANGLE_SPAN.getLevelForMaxValue(0)).toEqual(30);
    expect(K_AVG_ANGLE_SPAN.getLevelForMaxValue(1.5707963267948966)).toEqual(0);
    expect(K_AVG_ANGLE_SPAN.getLevelForMaxValue(0.7853981633974483)).toEqual(1);
    expect(K_AVG_ANGLE_SPAN.getLevelForMaxValue(0.77)).toEqual(2);
    expect(K_AVG_ANGLE_SPAN.getLevelForMaxValue(0.44)).toEqual(2);
    expect(K_AVG_ANGLE_SPAN.getLevelForMaxValue(0.39269908169872414)).toEqual(2);
    expect(K_AVG_ANGLE_SPAN.getLevelForMaxValue(0.19634954084936207)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_AVG_ANGLE_SPAN.getLevelForMinValue(0)).toEqual(30);
    expect(K_AVG_ANGLE_SPAN.getLevelForMinValue(1.5707963267948966)).toEqual(0);
    expect(K_AVG_ANGLE_SPAN.getLevelForMinValue(0.7853981633974483)).toEqual(1);
    expect(K_AVG_ANGLE_SPAN.getLevelForMinValue(0.77)).toEqual(1);
    expect(K_AVG_ANGLE_SPAN.getLevelForMinValue(0.44)).toEqual(1);
    expect(K_AVG_ANGLE_SPAN.getLevelForMinValue(0.39269908169872414)).toEqual(2);
    expect(K_AVG_ANGLE_SPAN.getLevelForMinValue(0.19634954084936207)).toEqual(3);
  });
});

describe('K_MAX_ANGLE_SPAN', () => {
  it('getValue', () => {
    expect(K_MAX_ANGLE_SPAN.getValue(0)).toEqual(1.704897179199218);
    expect(K_MAX_ANGLE_SPAN.getValue(1)).toEqual(0.852448589599609);
    expect(K_MAX_ANGLE_SPAN.getValue(2)).toEqual(0.4262242947998045);
    expect(K_MAX_ANGLE_SPAN.getValue(3)).toEqual(0.21311214739990225);
  });

  it('getClosestLevel', () => {
    expect(K_MAX_ANGLE_SPAN.getClosestLevel(0)).toEqual(30);
    expect(K_MAX_ANGLE_SPAN.getClosestLevel(1.704897179199218)).toEqual(0);
    expect(K_MAX_ANGLE_SPAN.getClosestLevel(0.852448589599609)).toEqual(1);
    expect(K_MAX_ANGLE_SPAN.getClosestLevel(0.77)).toEqual(1);
    expect(K_MAX_ANGLE_SPAN.getClosestLevel(0.44)).toEqual(2);
    expect(K_MAX_ANGLE_SPAN.getClosestLevel(0.4262242947998045)).toEqual(2);
    expect(K_MAX_ANGLE_SPAN.getClosestLevel(0.21311214739990225)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_MAX_ANGLE_SPAN.getLevelForMaxValue(0)).toEqual(30);
    expect(K_MAX_ANGLE_SPAN.getLevelForMaxValue(1.704897179199218)).toEqual(0);
    expect(K_MAX_ANGLE_SPAN.getLevelForMaxValue(0.852448589599609)).toEqual(1);
    expect(K_MAX_ANGLE_SPAN.getLevelForMaxValue(0.77)).toEqual(2);
    expect(K_MAX_ANGLE_SPAN.getLevelForMaxValue(0.44)).toEqual(2);
    expect(K_MAX_ANGLE_SPAN.getLevelForMaxValue(0.4262242947998045)).toEqual(2);
    expect(K_MAX_ANGLE_SPAN.getLevelForMaxValue(0.21311214739990225)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_MAX_ANGLE_SPAN.getLevelForMinValue(0)).toEqual(30);
    expect(K_MAX_ANGLE_SPAN.getLevelForMinValue(1.704897179199218)).toEqual(0);
    expect(K_MAX_ANGLE_SPAN.getLevelForMinValue(0.852448589599609)).toEqual(1);
    expect(K_MAX_ANGLE_SPAN.getLevelForMinValue(0.77)).toEqual(1);
    expect(K_MAX_ANGLE_SPAN.getLevelForMinValue(0.44)).toEqual(1);
    expect(K_MAX_ANGLE_SPAN.getLevelForMinValue(0.4262242947998045)).toEqual(2);
    expect(K_MAX_ANGLE_SPAN.getLevelForMinValue(0.21311214739990225)).toEqual(3);
  });
});

describe('K_MIN_ANGLE_SPAN', () => {
  it('getValue', () => {
    expect(K_MIN_ANGLE_SPAN.getValue(0)).toEqual(1.3333333333333333);
    expect(K_MIN_ANGLE_SPAN.getValue(1)).toEqual(0.6666666666666666);
    expect(K_MIN_ANGLE_SPAN.getValue(2)).toEqual(0.3333333333333333);
    expect(K_MIN_ANGLE_SPAN.getValue(3)).toEqual(0.16666666666666666);
  });

  it('getClosestLevel', () => {
    expect(K_MIN_ANGLE_SPAN.getClosestLevel(0)).toEqual(30);
    expect(K_MIN_ANGLE_SPAN.getClosestLevel(1.3333333333333333)).toEqual(0);
    expect(K_MIN_ANGLE_SPAN.getClosestLevel(0.6666666666666666)).toEqual(1);
    expect(K_MIN_ANGLE_SPAN.getClosestLevel(0.6)).toEqual(1);
    expect(K_MIN_ANGLE_SPAN.getClosestLevel(0.35)).toEqual(2);
    expect(K_MIN_ANGLE_SPAN.getClosestLevel(0.3333333333333333)).toEqual(2);
    expect(K_MIN_ANGLE_SPAN.getClosestLevel(0.16666666666666666)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_MIN_ANGLE_SPAN.getLevelForMaxValue(0)).toEqual(30);
    expect(K_MIN_ANGLE_SPAN.getLevelForMaxValue(1.3333333333333333)).toEqual(0);
    expect(K_MIN_ANGLE_SPAN.getLevelForMaxValue(0.6666666666666666)).toEqual(1);
    expect(K_MIN_ANGLE_SPAN.getLevelForMaxValue(0.6)).toEqual(2);
    expect(K_MIN_ANGLE_SPAN.getLevelForMaxValue(0.35)).toEqual(2);
    expect(K_MIN_ANGLE_SPAN.getLevelForMaxValue(0.3333333333333333)).toEqual(2);
    expect(K_MIN_ANGLE_SPAN.getLevelForMaxValue(0.16666666666666666)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_MIN_ANGLE_SPAN.getLevelForMinValue(0)).toEqual(30);
    expect(K_MIN_ANGLE_SPAN.getLevelForMinValue(1.3333333333333333)).toEqual(0);
    expect(K_MIN_ANGLE_SPAN.getLevelForMinValue(0.6666666666666666)).toEqual(1);
    expect(K_MIN_ANGLE_SPAN.getLevelForMinValue(0.6)).toEqual(1);
    expect(K_MIN_ANGLE_SPAN.getLevelForMinValue(0.35)).toEqual(1);
    expect(K_MIN_ANGLE_SPAN.getLevelForMinValue(0.3333333333333333)).toEqual(2);
    expect(K_MIN_ANGLE_SPAN.getLevelForMinValue(0.16666666666666666)).toEqual(3);
  });
});

// AREA

describe('K_AVG_AREA', () => {
  it('getValue', () => {
    expect(K_AVG_AREA.getValue(0)).toEqual(2.0943951023931953);
    expect(K_AVG_AREA.getValue(1)).toEqual(0.5235987755982988);
    expect(K_AVG_AREA.getValue(2)).toEqual(0.1308996938995747);
    expect(K_AVG_AREA.getValue(3)).toEqual(0.032724923474893676);
  });

  it('getClosestLevel', () => {
    expect(K_AVG_AREA.getClosestLevel(0)).toEqual(30);
    expect(K_AVG_AREA.getClosestLevel(2.0943951023931953)).toEqual(0);
    expect(K_AVG_AREA.getClosestLevel(0.5235987755982988)).toEqual(1);
    expect(K_AVG_AREA.getClosestLevel(0.5)).toEqual(1);
    expect(K_AVG_AREA.getClosestLevel(0.15)).toEqual(2);
    expect(K_AVG_AREA.getClosestLevel(0.1308996938995747)).toEqual(2);
    expect(K_AVG_AREA.getClosestLevel(0.032724923474893676)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_AVG_AREA.getLevelForMaxValue(0)).toEqual(30);
    expect(K_AVG_AREA.getLevelForMaxValue(2.0943951023931953)).toEqual(0);
    expect(K_AVG_AREA.getLevelForMaxValue(0.5235987755982988)).toEqual(1);
    expect(K_AVG_AREA.getLevelForMaxValue(0.5)).toEqual(2);
    expect(K_AVG_AREA.getLevelForMaxValue(0.15)).toEqual(2);
    expect(K_AVG_AREA.getLevelForMaxValue(0.1308996938995747)).toEqual(2);
    expect(K_AVG_AREA.getLevelForMaxValue(0.032724923474893676)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_AVG_AREA.getLevelForMinValue(0)).toEqual(30);
    expect(K_AVG_AREA.getLevelForMinValue(2.0943951023931953)).toEqual(0);
    expect(K_AVG_AREA.getLevelForMinValue(0.5235987755982988)).toEqual(1);
    expect(K_AVG_AREA.getLevelForMinValue(0.5)).toEqual(1);
    expect(K_AVG_AREA.getLevelForMinValue(0.15)).toEqual(1);
    expect(K_AVG_AREA.getLevelForMinValue(0.1308996938995747)).toEqual(2);
    expect(K_AVG_AREA.getLevelForMinValue(0.032724923474893676)).toEqual(3);
  });
});

describe('K_MAX_AREA', () => {
  it('getValue', () => {
    expect(K_MAX_AREA.getValue(0)).toEqual(2.6357992569631614);
    expect(K_MAX_AREA.getValue(1)).toEqual(0.6589498142407904);
    expect(K_MAX_AREA.getValue(2)).toEqual(0.1647374535601976);
    expect(K_MAX_AREA.getValue(3)).toEqual(0.0411843633900494);
  });

  it('getClosestLevel', () => {
    expect(K_MAX_AREA.getClosestLevel(0)).toEqual(30);
    expect(K_MAX_AREA.getClosestLevel(2.6357992569631614)).toEqual(0);
    expect(K_MAX_AREA.getClosestLevel(0.6589498142407904)).toEqual(1);
    expect(K_MAX_AREA.getClosestLevel(0.6)).toEqual(1);
    expect(K_MAX_AREA.getClosestLevel(0.2)).toEqual(2);
    expect(K_MAX_AREA.getClosestLevel(0.1647374535601976)).toEqual(2);
    expect(K_MAX_AREA.getClosestLevel(0.0411843633900494)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_MAX_AREA.getLevelForMaxValue(0)).toEqual(30);
    expect(K_MAX_AREA.getLevelForMaxValue(2.6357992569631614)).toEqual(0);
    expect(K_MAX_AREA.getLevelForMaxValue(0.6589498142407904)).toEqual(1);
    expect(K_MAX_AREA.getLevelForMaxValue(0.6)).toEqual(2);
    expect(K_MAX_AREA.getLevelForMaxValue(0.2)).toEqual(2);
    expect(K_MAX_AREA.getLevelForMaxValue(0.1647374535601976)).toEqual(2);
    expect(K_MAX_AREA.getLevelForMaxValue(0.0411843633900494)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_MAX_AREA.getLevelForMinValue(0)).toEqual(30);
    expect(K_MAX_AREA.getLevelForMinValue(2.6357992569631614)).toEqual(0);
    expect(K_MAX_AREA.getLevelForMinValue(0.6589498142407904)).toEqual(1);
    expect(K_MAX_AREA.getLevelForMinValue(0.6)).toEqual(1);
    expect(K_MAX_AREA.getLevelForMinValue(0.2)).toEqual(1);
    expect(K_MAX_AREA.getLevelForMinValue(0.1647374535601976)).toEqual(2);
    expect(K_MAX_AREA.getLevelForMinValue(0.0411843633900494)).toEqual(3);
  });
});

describe('K_MIN_AREA', () => {
  it('getValue', () => {
    expect(K_MIN_AREA.getValue(0)).toEqual(1.257078722109418);
    expect(K_MIN_AREA.getValue(1)).toEqual(0.3142696805273545);
    expect(K_MIN_AREA.getValue(2)).toEqual(0.07856742013183862);
    expect(K_MIN_AREA.getValue(3)).toEqual(0.019641855032959656);
  });

  it('getClosestLevel', () => {
    expect(K_MIN_AREA.getClosestLevel(0)).toEqual(30);
    expect(K_MIN_AREA.getClosestLevel(1.257078722109418)).toEqual(0);
    expect(K_MIN_AREA.getClosestLevel(0.3142696805273545)).toEqual(1);
    expect(K_MIN_AREA.getClosestLevel(0.3)).toEqual(1);
    expect(K_MIN_AREA.getClosestLevel(0.09)).toEqual(2);
    expect(K_MIN_AREA.getClosestLevel(0.07856742013183862)).toEqual(2);
    expect(K_MIN_AREA.getClosestLevel(0.019641855032959656)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_MIN_AREA.getLevelForMaxValue(0)).toEqual(30);
    expect(K_MIN_AREA.getLevelForMaxValue(1.257078722109418)).toEqual(0);
    expect(K_MIN_AREA.getLevelForMaxValue(0.3142696805273545)).toEqual(1);
    expect(K_MIN_AREA.getLevelForMaxValue(0.3)).toEqual(2);
    expect(K_MIN_AREA.getLevelForMaxValue(0.09)).toEqual(2);
    expect(K_MIN_AREA.getLevelForMaxValue(0.07856742013183862)).toEqual(2);
    expect(K_MIN_AREA.getLevelForMaxValue(0.019641855032959656)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_MIN_AREA.getLevelForMinValue(0)).toEqual(30);
    expect(K_MIN_AREA.getLevelForMinValue(1.257078722109418)).toEqual(0);
    expect(K_MIN_AREA.getLevelForMinValue(0.3142696805273545)).toEqual(1);
    expect(K_MIN_AREA.getLevelForMinValue(0.3)).toEqual(1);
    expect(K_MIN_AREA.getLevelForMinValue(0.09)).toEqual(1);
    expect(K_MIN_AREA.getLevelForMinValue(0.07856742013183862)).toEqual(2);
    expect(K_MIN_AREA.getLevelForMinValue(0.019641855032959656)).toEqual(3);
  });
});

// DIAG

describe('K_AVG_DIAG', () => {
  it('getValue', () => {
    expect(K_AVG_DIAG.getValue(0)).toEqual(2.060422738998471);
    expect(K_AVG_DIAG.getValue(1)).toEqual(1.0302113694992354);
    expect(K_AVG_DIAG.getValue(2)).toEqual(0.5151056847496177);
    expect(K_AVG_DIAG.getValue(3)).toEqual(0.25755284237480885);
  });

  it('getClosestLevel', () => {
    expect(K_AVG_DIAG.getClosestLevel(0)).toEqual(30);
    expect(K_AVG_DIAG.getClosestLevel(2.060422738998471)).toEqual(0);
    expect(K_AVG_DIAG.getClosestLevel(1.0302113694992354)).toEqual(1);
    expect(K_AVG_DIAG.getClosestLevel(1.01)).toEqual(1);
    expect(K_AVG_DIAG.getClosestLevel(0.55)).toEqual(2);
    expect(K_AVG_DIAG.getClosestLevel(0.5151056847496177)).toEqual(2);
    expect(K_AVG_DIAG.getClosestLevel(0.25755284237480885)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_AVG_DIAG.getLevelForMaxValue(0)).toEqual(30);
    expect(K_AVG_DIAG.getLevelForMaxValue(2.060422738998471)).toEqual(0);
    expect(K_AVG_DIAG.getLevelForMaxValue(1.0302113694992354)).toEqual(1);
    expect(K_AVG_DIAG.getLevelForMaxValue(1.01)).toEqual(2);
    expect(K_AVG_DIAG.getLevelForMaxValue(0.55)).toEqual(2);
    expect(K_AVG_DIAG.getLevelForMaxValue(0.5151056847496177)).toEqual(2);
    expect(K_AVG_DIAG.getLevelForMaxValue(0.25755284237480885)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_AVG_DIAG.getLevelForMinValue(0)).toEqual(30);
    expect(K_AVG_DIAG.getLevelForMinValue(2.060422738998471)).toEqual(0);
    expect(K_AVG_DIAG.getLevelForMinValue(1.0302113694992354)).toEqual(1);
    expect(K_AVG_DIAG.getLevelForMinValue(1.01)).toEqual(1);
    expect(K_AVG_DIAG.getLevelForMinValue(0.55)).toEqual(1);
    expect(K_AVG_DIAG.getLevelForMinValue(0.5151056847496177)).toEqual(2);
    expect(K_AVG_DIAG.getLevelForMinValue(0.25755284237480885)).toEqual(3);
  });
});

describe('K_MAX_DIAG', () => {
  it('getValue', () => {
    expect(K_MAX_DIAG.getValue(0)).toEqual(2.438654594434021);
    expect(K_MAX_DIAG.getValue(1)).toEqual(1.2193272972170106);
    expect(K_MAX_DIAG.getValue(2)).toEqual(0.6096636486085053);
    expect(K_MAX_DIAG.getValue(3)).toEqual(0.30483182430425265);
  });

  it('getClosestLevel', () => {
    expect(K_MAX_DIAG.getClosestLevel(0)).toEqual(30);
    expect(K_MAX_DIAG.getClosestLevel(2.438654594434021)).toEqual(0);
    expect(K_MAX_DIAG.getClosestLevel(1.2193272972170106)).toEqual(1);
    expect(K_MAX_DIAG.getClosestLevel(1.01)).toEqual(1);
    expect(K_MAX_DIAG.getClosestLevel(0.61)).toEqual(2);
    expect(K_MAX_DIAG.getClosestLevel(0.6096636486085053)).toEqual(2);
    expect(K_MAX_DIAG.getClosestLevel(0.30483182430425265)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_MAX_DIAG.getLevelForMaxValue(0)).toEqual(30);
    expect(K_MAX_DIAG.getLevelForMaxValue(2.438654594434021)).toEqual(0);
    expect(K_MAX_DIAG.getLevelForMaxValue(1.2193272972170106)).toEqual(1);
    expect(K_MAX_DIAG.getLevelForMaxValue(1.01)).toEqual(2);
    expect(K_MAX_DIAG.getLevelForMaxValue(0.61)).toEqual(2);
    expect(K_MAX_DIAG.getLevelForMaxValue(0.6096636486085053)).toEqual(2);
    expect(K_MAX_DIAG.getLevelForMaxValue(0.30483182430425265)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_MAX_DIAG.getLevelForMinValue(0)).toEqual(30);
    expect(K_MAX_DIAG.getLevelForMinValue(2.438654594434021)).toEqual(0);
    expect(K_MAX_DIAG.getLevelForMinValue(1.2193272972170106)).toEqual(1);
    expect(K_MAX_DIAG.getLevelForMinValue(1.01)).toEqual(1);
    expect(K_MAX_DIAG.getLevelForMinValue(0.61)).toEqual(1);
    expect(K_MAX_DIAG.getLevelForMinValue(0.6096636486085053)).toEqual(2);
    expect(K_MAX_DIAG.getLevelForMinValue(0.30483182430425265)).toEqual(3);
  });
});

describe('K_MIN_DIAG', () => {
  it('getValue', () => {
    expect(K_MIN_DIAG.getValue(0)).toEqual(1.257078722109418);
    expect(K_MIN_DIAG.getValue(1)).toEqual(0.628539361054709);
    expect(K_MIN_DIAG.getValue(2)).toEqual(0.3142696805273545);
    expect(K_MIN_DIAG.getValue(3)).toEqual(0.15713484026367724);
  });

  it('getClosestLevel', () => {
    expect(K_MIN_DIAG.getClosestLevel(0)).toEqual(30);
    expect(K_MIN_DIAG.getClosestLevel(1.257078722109418)).toEqual(0);
    expect(K_MIN_DIAG.getClosestLevel(0.628539361054709)).toEqual(1);
    expect(K_MIN_DIAG.getClosestLevel(0.61)).toEqual(1);
    expect(K_MIN_DIAG.getClosestLevel(0.35)).toEqual(2);
    expect(K_MIN_DIAG.getClosestLevel(0.3142696805273545)).toEqual(2);
    expect(K_MIN_DIAG.getClosestLevel(0.15713484026367724)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_MIN_DIAG.getLevelForMaxValue(0)).toEqual(30);
    expect(K_MIN_DIAG.getLevelForMaxValue(1.257078722109418)).toEqual(0);
    expect(K_MIN_DIAG.getLevelForMaxValue(0.628539361054709)).toEqual(1);
    expect(K_MIN_DIAG.getLevelForMaxValue(0.61)).toEqual(2);
    expect(K_MIN_DIAG.getLevelForMaxValue(0.35)).toEqual(2);
    expect(K_MIN_DIAG.getLevelForMaxValue(0.3142696805273545)).toEqual(2);
    expect(K_MIN_DIAG.getLevelForMaxValue(0.15713484026367724)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_MIN_DIAG.getLevelForMinValue(0)).toEqual(30);
    expect(K_MIN_DIAG.getLevelForMinValue(1.257078722109418)).toEqual(0);
    expect(K_MIN_DIAG.getLevelForMinValue(0.628539361054709)).toEqual(1);
    expect(K_MIN_DIAG.getLevelForMinValue(0.61)).toEqual(1);
    expect(K_MIN_DIAG.getLevelForMinValue(0.35)).toEqual(1);
    expect(K_MIN_DIAG.getLevelForMinValue(0.3142696805273545)).toEqual(2);
    expect(K_MIN_DIAG.getLevelForMinValue(0.15713484026367724)).toEqual(3);
  });
});

// EDGE

describe('K_AVG_EDGE', () => {
  it('getValue', () => {
    expect(K_AVG_EDGE.getValue(0)).toEqual(1.459213746386106);
    expect(K_AVG_EDGE.getValue(1)).toEqual(0.729606873193053);
    expect(K_AVG_EDGE.getValue(2)).toEqual(0.3648034365965265);
    expect(K_AVG_EDGE.getValue(3)).toEqual(0.18240171829826324);
  });

  it('getClosestLevel', () => {
    expect(K_AVG_EDGE.getClosestLevel(0)).toEqual(30);
    expect(K_AVG_EDGE.getClosestLevel(1.459213746386106)).toEqual(0);
    expect(K_AVG_EDGE.getClosestLevel(0.729606873193053)).toEqual(1);
    expect(K_AVG_EDGE.getClosestLevel(0.71)).toEqual(1);
    expect(K_AVG_EDGE.getClosestLevel(0.38)).toEqual(2);
    expect(K_AVG_EDGE.getClosestLevel(0.3648034365965265)).toEqual(2);
    expect(K_AVG_EDGE.getClosestLevel(0.18240171829826324)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_AVG_EDGE.getLevelForMaxValue(0)).toEqual(30);
    expect(K_AVG_EDGE.getLevelForMaxValue(1.459213746386106)).toEqual(0);
    expect(K_AVG_EDGE.getLevelForMaxValue(0.729606873193053)).toEqual(1);
    expect(K_AVG_EDGE.getLevelForMaxValue(0.71)).toEqual(2);
    expect(K_AVG_EDGE.getLevelForMaxValue(0.38)).toEqual(2);
    expect(K_AVG_EDGE.getLevelForMaxValue(0.3648034365965265)).toEqual(2);
    expect(K_AVG_EDGE.getLevelForMaxValue(0.18240171829826324)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_AVG_EDGE.getLevelForMinValue(0)).toEqual(30);
    expect(K_AVG_EDGE.getLevelForMinValue(1.459213746386106)).toEqual(0);
    expect(K_AVG_EDGE.getLevelForMinValue(0.729606873193053)).toEqual(1);
    expect(K_AVG_EDGE.getLevelForMinValue(0.71)).toEqual(1);
    expect(K_AVG_EDGE.getLevelForMinValue(0.38)).toEqual(1);
    expect(K_AVG_EDGE.getLevelForMinValue(0.3648034365965265)).toEqual(2);
    expect(K_AVG_EDGE.getLevelForMinValue(0.18240171829826324)).toEqual(3);
  });
});

describe('K_MAX_EDGE', () => {
  it('getValue', () => {
    expect(K_MAX_EDGE.getValue(0)).toEqual(1.704897179199218);
    expect(K_MAX_EDGE.getValue(1)).toEqual(0.852448589599609);
    expect(K_MAX_EDGE.getValue(2)).toEqual(0.4262242947998045);
    expect(K_MAX_EDGE.getValue(3)).toEqual(0.21311214739990225);
  });

  it('getClosestLevel', () => {
    expect(K_MAX_EDGE.getClosestLevel(0)).toEqual(30);
    expect(K_MAX_EDGE.getClosestLevel(1.704897179199218)).toEqual(0);
    expect(K_MAX_EDGE.getClosestLevel(0.852448589599609)).toEqual(1);
    expect(K_MAX_EDGE.getClosestLevel(0.84)).toEqual(1);
    expect(K_MAX_EDGE.getClosestLevel(0.45)).toEqual(2);
    expect(K_MAX_EDGE.getClosestLevel(0.4262242947998045)).toEqual(2);
    expect(K_MAX_EDGE.getClosestLevel(0.21311214739990225)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_MAX_EDGE.getLevelForMaxValue(0)).toEqual(30);
    expect(K_MAX_EDGE.getLevelForMaxValue(1.704897179199218)).toEqual(0);
    expect(K_MAX_EDGE.getLevelForMaxValue(0.852448589599609)).toEqual(1);
    expect(K_MAX_EDGE.getLevelForMaxValue(0.84)).toEqual(2);
    expect(K_MAX_EDGE.getLevelForMaxValue(0.45)).toEqual(2);
    expect(K_MAX_EDGE.getLevelForMaxValue(0.4262242947998045)).toEqual(2);
    expect(K_MAX_EDGE.getLevelForMaxValue(0.21311214739990225)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_MAX_EDGE.getLevelForMinValue(0)).toEqual(30);
    expect(K_MAX_EDGE.getLevelForMinValue(1.704897179199218)).toEqual(0);
    expect(K_MAX_EDGE.getLevelForMinValue(0.852448589599609)).toEqual(1);
    expect(K_MAX_EDGE.getLevelForMinValue(0.84)).toEqual(1);
    expect(K_MAX_EDGE.getLevelForMinValue(0.45)).toEqual(1);
    expect(K_MAX_EDGE.getLevelForMinValue(0.4262242947998045)).toEqual(2);
    expect(K_MAX_EDGE.getLevelForMinValue(0.21311214739990225)).toEqual(3);
  });
});

describe('K_MIN_EDGE', () => {
  it('getValue', () => {
    expect(K_MIN_EDGE.getValue(0)).toEqual(0.9428090415820635);
    expect(K_MIN_EDGE.getValue(1)).toEqual(0.47140452079103173);
    expect(K_MIN_EDGE.getValue(2)).toEqual(0.23570226039551587);
    expect(K_MIN_EDGE.getValue(3)).toEqual(0.11785113019775793);
  });

  it('getClosestLevel', () => {
    expect(K_MIN_EDGE.getClosestLevel(0)).toEqual(30);
    expect(K_MIN_EDGE.getClosestLevel(0.9428090415820635)).toEqual(0);
    expect(K_MIN_EDGE.getClosestLevel(0.47140452079103173)).toEqual(1);
    expect(K_MIN_EDGE.getClosestLevel(0.45)).toEqual(1);
    expect(K_MIN_EDGE.getClosestLevel(0.25)).toEqual(2);
    expect(K_MIN_EDGE.getClosestLevel(0.23570226039551587)).toEqual(2);
    expect(K_MIN_EDGE.getClosestLevel(0.11785113019775793)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_MIN_EDGE.getLevelForMaxValue(0)).toEqual(30);
    expect(K_MIN_EDGE.getLevelForMaxValue(0.9428090415820635)).toEqual(0);
    expect(K_MIN_EDGE.getLevelForMaxValue(0.47140452079103173)).toEqual(1);
    expect(K_MIN_EDGE.getLevelForMaxValue(0.45)).toEqual(2);
    expect(K_MIN_EDGE.getLevelForMaxValue(0.25)).toEqual(2);
    expect(K_MIN_EDGE.getLevelForMaxValue(0.23570226039551587)).toEqual(2);
    expect(K_MIN_EDGE.getLevelForMaxValue(0.11785113019775793)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_MIN_EDGE.getLevelForMinValue(0)).toEqual(30);
    expect(K_MIN_EDGE.getLevelForMinValue(0.9428090415820635)).toEqual(0);
    expect(K_MIN_EDGE.getLevelForMinValue(0.47140452079103173)).toEqual(1);
    expect(K_MIN_EDGE.getLevelForMinValue(0.45)).toEqual(1);
    expect(K_MIN_EDGE.getLevelForMinValue(0.25)).toEqual(1);
    expect(K_MIN_EDGE.getLevelForMinValue(0.23570226039551587)).toEqual(2);
    expect(K_MIN_EDGE.getLevelForMinValue(0.11785113019775793)).toEqual(3);
  });
});

// WIDTH

describe('K_AVG_WIDTH', () => {
  it('getValue', () => {
    expect(K_AVG_WIDTH.getValue(0)).toEqual(1.4345236728860993);
    expect(K_AVG_WIDTH.getValue(1)).toEqual(0.7172618364430496);
    expect(K_AVG_WIDTH.getValue(2)).toEqual(0.3586309182215248);
    expect(K_AVG_WIDTH.getValue(3)).toEqual(0.1793154591107624);
  });

  it('getClosestLevel', () => {
    expect(K_AVG_WIDTH.getClosestLevel(0)).toEqual(30);
    expect(K_AVG_WIDTH.getClosestLevel(1.4345236728860993)).toEqual(0);
    expect(K_AVG_WIDTH.getClosestLevel(0.7172618364430496)).toEqual(1);
    expect(K_AVG_WIDTH.getClosestLevel(0.71)).toEqual(1);
    expect(K_AVG_WIDTH.getClosestLevel(0.38)).toEqual(2);
    expect(K_AVG_WIDTH.getClosestLevel(0.3586309182215248)).toEqual(2);
    expect(K_AVG_WIDTH.getClosestLevel(0.1793154591107624)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_AVG_WIDTH.getLevelForMaxValue(0)).toEqual(30);
    expect(K_AVG_WIDTH.getLevelForMaxValue(1.4345236728860993)).toEqual(0);
    expect(K_AVG_WIDTH.getLevelForMaxValue(0.7172618364430496)).toEqual(1);
    expect(K_AVG_WIDTH.getLevelForMaxValue(0.71)).toEqual(2);
    expect(K_AVG_WIDTH.getLevelForMaxValue(0.38)).toEqual(2);
    expect(K_AVG_WIDTH.getLevelForMaxValue(0.3586309182215248)).toEqual(2);
    expect(K_AVG_WIDTH.getLevelForMaxValue(0.1793154591107624)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_AVG_WIDTH.getLevelForMinValue(0)).toEqual(30);
    expect(K_AVG_WIDTH.getLevelForMinValue(1.4345236728860993)).toEqual(0);
    expect(K_AVG_WIDTH.getLevelForMinValue(0.7172618364430496)).toEqual(1);
    expect(K_AVG_WIDTH.getLevelForMinValue(0.71)).toEqual(1);
    expect(K_AVG_WIDTH.getLevelForMinValue(0.38)).toEqual(1);
    expect(K_AVG_WIDTH.getLevelForMinValue(0.3586309182215248)).toEqual(2);
    expect(K_AVG_WIDTH.getLevelForMinValue(0.1793154591107624)).toEqual(3);
  });
});

describe('K_MAX_WIDTH', () => {
  it('getValue', () => {
    expect(K_MAX_WIDTH.getValue(0)).toEqual(1.704897179199218);
    expect(K_MAX_WIDTH.getValue(1)).toEqual(0.852448589599609);
    expect(K_MAX_WIDTH.getValue(2)).toEqual(0.4262242947998045);
    expect(K_MAX_WIDTH.getValue(3)).toEqual(0.21311214739990225);
  });

  it('getClosestLevel', () => {
    expect(K_MAX_WIDTH.getClosestLevel(0)).toEqual(30);
    expect(K_MAX_WIDTH.getClosestLevel(1.704897179199218)).toEqual(0);
    expect(K_MAX_WIDTH.getClosestLevel(0.852448589599609)).toEqual(1);
    expect(K_MAX_WIDTH.getClosestLevel(0.84)).toEqual(1);
    expect(K_MAX_WIDTH.getClosestLevel(0.45)).toEqual(2);
    expect(K_MAX_WIDTH.getClosestLevel(0.4262242947998045)).toEqual(2);
    expect(K_MAX_WIDTH.getClosestLevel(0.21311214739990225)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_MAX_WIDTH.getLevelForMaxValue(0)).toEqual(30);
    expect(K_MAX_WIDTH.getLevelForMaxValue(1.704897179199218)).toEqual(0);
    expect(K_MAX_WIDTH.getLevelForMaxValue(0.852448589599609)).toEqual(1);
    expect(K_MAX_WIDTH.getLevelForMaxValue(0.84)).toEqual(2);
    expect(K_MAX_WIDTH.getLevelForMaxValue(0.45)).toEqual(2);
    expect(K_MAX_WIDTH.getLevelForMaxValue(0.4262242947998045)).toEqual(2);
    expect(K_MAX_WIDTH.getLevelForMaxValue(0.21311214739990225)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_MAX_WIDTH.getLevelForMinValue(0)).toEqual(30);
    expect(K_MAX_WIDTH.getLevelForMinValue(1.704897179199218)).toEqual(0);
    expect(K_MAX_WIDTH.getLevelForMinValue(0.852448589599609)).toEqual(1);
    expect(K_MAX_WIDTH.getLevelForMinValue(0.84)).toEqual(1);
    expect(K_MAX_WIDTH.getLevelForMinValue(0.45)).toEqual(1);
    expect(K_MAX_WIDTH.getLevelForMinValue(0.4262242947998045)).toEqual(2);
    expect(K_MAX_WIDTH.getLevelForMinValue(0.21311214739990225)).toEqual(3);
  });
});

describe('K_MIN_WIDTH', () => {
  it('getValue', () => {
    expect(K_MIN_WIDTH.getValue(0)).toEqual(0.9428090415820635);
    expect(K_MIN_WIDTH.getValue(1)).toEqual(0.47140452079103173);
    expect(K_MIN_WIDTH.getValue(2)).toEqual(0.23570226039551587);
    expect(K_MIN_WIDTH.getValue(3)).toEqual(0.11785113019775793);
  });

  it('getClosestLevel', () => {
    expect(K_MIN_WIDTH.getClosestLevel(0)).toEqual(30);
    expect(K_MIN_WIDTH.getClosestLevel(0.9428090415820635)).toEqual(0);
    expect(K_MIN_WIDTH.getClosestLevel(0.47140452079103173)).toEqual(1);
    expect(K_MIN_WIDTH.getClosestLevel(0.45)).toEqual(1);
    expect(K_MIN_WIDTH.getClosestLevel(0.25)).toEqual(2);
    expect(K_MIN_WIDTH.getClosestLevel(0.23570226039551587)).toEqual(2);
    expect(K_MIN_WIDTH.getClosestLevel(0.11785113019775793)).toEqual(3);
  });

  it('getLevelForMaxValue', () => {
    expect(K_MIN_WIDTH.getLevelForMaxValue(0)).toEqual(30);
    expect(K_MIN_WIDTH.getLevelForMaxValue(0.9428090415820635)).toEqual(0);
    expect(K_MIN_WIDTH.getLevelForMaxValue(0.47140452079103173)).toEqual(1);
    expect(K_MIN_WIDTH.getLevelForMaxValue(0.45)).toEqual(2);
    expect(K_MIN_WIDTH.getLevelForMaxValue(0.25)).toEqual(2);
    expect(K_MIN_WIDTH.getLevelForMaxValue(0.23570226039551587)).toEqual(2);
    expect(K_MIN_WIDTH.getLevelForMaxValue(0.11785113019775793)).toEqual(3);
  });

  it('getLevelForMinValue', () => {
    expect(K_MIN_WIDTH.getLevelForMinValue(0)).toEqual(30);
    expect(K_MIN_WIDTH.getLevelForMinValue(0.9428090415820635)).toEqual(0);
    expect(K_MIN_WIDTH.getLevelForMinValue(0.47140452079103173)).toEqual(1);
    expect(K_MIN_WIDTH.getLevelForMinValue(0.45)).toEqual(1);
    expect(K_MIN_WIDTH.getLevelForMinValue(0.25)).toEqual(1);
    expect(K_MIN_WIDTH.getLevelForMinValue(0.23570226039551587)).toEqual(2);
    expect(K_MIN_WIDTH.getLevelForMinValue(0.11785113019775793)).toEqual(3);
  });
});
