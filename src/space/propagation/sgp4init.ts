import { earthRadius, j2, j3oj2, j4, pi, x2o3 } from '../util/constants';

import { Satellite } from '../sat';
import dpper from './dpper';
import dscom from './dscom';
import dsinit from './dsinit';
import initl from './initl';

/* -----------------------------------------------------------------------------
 *
 *                             procedure sgp4init
 *
 *  this procedure initializes variables for sgp4.
 *
 *  author        : david vallado                  719-573-2600   28 jun 2005
 *  author        : david vallado                  719-573-2600   28 jun 2005
 *
 *  inputs        :
 *    opsmode     - mode of operation afspc or improved 'a', 'i'
 *    satn        - satellite number
 *    drag       - sgp4 type drag coefficient              kg/m2er
 *    ecco        - eccentricity
 *    epoch       - epoch time in days from jan 0, 1950. 0 hr
 *    argpo       - argument of perigee (output if ds)
 *    inclo       - inclination
 *    mo          - mean anomaly (output if ds)
 *    no          - mean motion
 *    nodeo       - right ascension of ascending node
 *
 *  outputs       :
 *    rec      - common values for subsequent calls
 *    return code - non-zero on error.
 *                   1 - mean elements, ecc >= 1.0 or ecc < -0.001 or a < 0.95 er
 *                   2 - mean motion less than 0.0
 *                   3 - pert elements, ecc < 0.0  or  ecc > 1.0
 *                   4 - semi-latus rectum < 0.0
 *                   5 - epoch elements are sub-orbital
 *                   6 - satellite has decayed
 *
 *  locals        :
 *    cnodm  , snodm  , cosim  , sinim  , cosomm , sinomm
 *    cc1sq  , cc2    , cc3
 *    coef   , coef1
 *    cosio4      -
 *    day         -
 *    dndt        -
 *    em          - eccentricity
 *    emsq        - eccentricity squared
 *    eeta        -
 *    etasq       -
 *    gam         -
 *    argpm       - argument of perigee
 *    nodem       -
 *    inclm       - inclination
 *    mm          - mean anomaly
 *    nm          - mean motion
 *    perige      - perigee
 *    pinvsq      -
 *    psisq       -
 *    qzms24      -
 *    rtemsq      -
 *    s1, s2, s3, s4, s5, s6, s7          -
 *    sfour       -
 *    ss1, ss2, ss3, ss4, ss5, ss6, ss7         -
 *    sz1, sz2, sz3
 *    sz11, sz12, sz13, sz21, sz22, sz23, sz31, sz32, sz33        -
 *    tc          -
 *    temp        -
 *    temp1, temp2, temp3       -
 *    tsi         -
 *    xpidot      -
 *    xhdot1      -
 *    z1, z2, z3          -
 *    z11, z12, z13, z21, z22, z23, z31, z32, z33         -
 *
 *  coupling      :
 *    getgravconst-
 *    initl       -
 *    dscom       -
 *    dpper       -
 *    dsinit      -
 *    sgp4        -
 *
 *  references    :
 *    hoots, roehrich, norad spacetrack report #3 1980
 *    hoots, norad spacetrack report #6 1986
 *    hoots, schumacher and glover 2004
 *    vallado, crawford, hujsak, kelso  2006
 ---------------------------------------------------------------------------- */
/**
 * @param sat
 */
export default function sgp4init(sat: Satellite): void {
  const epoch = sat.jdsatepoch - 2433281.5;

  let cosim: number;
  let sinim: number;
  let cc1sq: number;
  let cc2: number;
  let cc3: number;
  let coef: number;
  let coef1: number;
  let cosio4: number;
  let em: number;
  let emsq: number;
  let eeta: number;
  let etasq: number;
  let argpm: number;
  let nodem: number;
  let inclm: number;
  let mm: number;
  let nm: number;
  let perige: number;
  let pinvsq: number;
  let psisq: number;
  let qzms24: number;
  let s1: number;
  let s2: number;
  let s3: number;
  let s4: number;
  let s5: number;
  let sfour: number;
  let ss1: number;
  let ss2: number;
  let ss3: number;
  let ss4: number;
  let ss5: number;
  let sz1: number;
  let sz3: number;
  let sz11: number;
  let sz13: number;
  let sz21: number;
  let sz23: number;
  let sz31: number;
  let sz33: number;
  let tc: number;
  let temp: number;
  let temp1: number;
  let temp2: number;
  let temp3: number;
  let tsi: number;
  let xpidot: number;
  let xhdot1: number;
  let z1: number;
  let z3: number;
  let z11: number;
  let z13: number;
  let z21: number;
  let z23: number;
  let z31: number;
  let z33: number;

  /* ------------------------ initialization --------------------- */
  // sgp4fix divisor for divide by zero check on inclination
  // the old check used 1.0 + Math.cos(pi-1.0e-9), but then compared it to
  // 1.5 e-12, so the threshold was changed to 1.5e-12 for consistency
  const temp4 = 1.5e-12;

  // sgp4fix - note the following variables are also passed directly via sat.
  // it is possible to streamline the sgp4init call by deleting the "x"
  // variables, but the user would need to set the sat.* values first. we
  // include the additional assignments in case twoline2rv is not used.

  // ------------------------ earth constants -----------------------
  // sgp4fix identify constants and allow alternate values

  const ss = 78.0 / earthRadius + 1.0;
  // sgp4fix use multiply for speed instead of pow
  const qzms2ttemp = (120.0 - 78.0) / earthRadius;
  const qzms2t = qzms2ttemp * qzms2ttemp * qzms2ttemp * qzms2ttemp;

  sat.init = true;

  const initlOptions = {
    // satn,
    ecco: sat.eccentricity,

    epoch,
    inclo: sat.inclination,
    no: sat.motion,

    opsmode: sat.opsmode,
  };

  const initlResult = initl(initlOptions);

  const { ao, con42, cosio, cosio2, eccsq, omeosq, posq, rp, rteosq, sinio } = initlResult;

  sat.motion = initlResult.no;
  sat.con41 = initlResult.con41;
  sat.gsto = initlResult.gsto;
  // const a = (sat.motion * tumin) ** (-2.0 / 3.0);
  // const alta = a * (1.0 + sat.eccentricity) - 1.0;
  // const altp = a * (1.0 - sat.eccentricity) - 1.0;

  // sgp4fix remove this check as it is unnecessary
  // the mrt check in sgp4 handles decaying satellite cases even if the starting
  // condition is below the surface of te earth
  // if (rp < 1.0)
  // {
  //   printf("// *** satn%d epoch elts sub-orbital ***\n", satn);
  //   sat.error = 5;
  // }

  if (omeosq >= 0.0 || sat.motion >= 0.0) {
    sat.isimp = 0;
    if (rp < 220.0 / earthRadius + 1.0) {
      sat.isimp = 1;
    }
    sfour = ss;
    qzms24 = qzms2t;
    perige = (rp - 1.0) * earthRadius;

    // - for perigees below 156 km, s and qoms2t are altered -
    if (perige < 156.0) {
      sfour = perige - 78.0;
      if (perige < 98.0) {
        sfour = 20.0;
      }

      // sgp4fix use multiply for speed instead of pow
      const qzms24temp = (120.0 - sfour) / earthRadius;
      qzms24 = qzms24temp * qzms24temp * qzms24temp * qzms24temp;
      sfour = sfour / earthRadius + 1.0;
    }
    pinvsq = 1.0 / posq;

    tsi = 1.0 / (ao - sfour);
    sat.eta = ao * sat.eccentricity * tsi;
    etasq = sat.eta * sat.eta;
    eeta = sat.eccentricity * sat.eta;
    psisq = Math.abs(1.0 - etasq);
    coef = qzms24 * tsi ** 4.0;
    coef1 = coef / psisq ** 3.5;
    cc2 =
      coef1 *
      sat.motion *
      (ao * (1.0 + 1.5 * etasq + eeta * (4.0 + etasq)) +
        ((0.375 * j2 * tsi) / psisq) * sat.con41 * (8.0 + 3.0 * etasq * (8.0 + etasq)));
    sat.cc1 = sat.drag * cc2;
    cc3 = 0.0;
    if (sat.eccentricity > 1.0e-4) {
      cc3 = (-2.0 * coef * tsi * j3oj2 * sat.motion * sinio) / sat.eccentricity;
    }
    sat.x1mth2 = 1.0 - cosio2;
    sat.cc4 =
      2.0 *
      sat.motion *
      coef1 *
      ao *
      omeosq *
      (sat.eta * (2.0 + 0.5 * etasq) +
        sat.eccentricity * (0.5 + 2.0 * etasq) -
        ((j2 * tsi) / (ao * psisq)) *
          (-3.0 * sat.con41 * (1.0 - 2.0 * eeta + etasq * (1.5 - 0.5 * eeta)) +
            0.75 *
              sat.x1mth2 *
              (2.0 * etasq - eeta * (1.0 + etasq)) *
              Math.cos(2.0 * sat.perigee)));
    sat.cc5 = 2.0 * coef1 * ao * omeosq * (1.0 + 2.75 * (etasq + eeta) + eeta * etasq);
    cosio4 = cosio2 * cosio2;
    temp1 = 1.5 * j2 * pinvsq * sat.motion;
    temp2 = 0.5 * temp1 * j2 * pinvsq;
    temp3 = -0.46875 * j4 * pinvsq * pinvsq * sat.motion;
    sat.mdot =
      sat.motion +
      0.5 * temp1 * rteosq * sat.con41 +
      0.0625 * temp2 * rteosq * (13.0 - 78.0 * cosio2 + 137.0 * cosio4);
    sat.argpdot =
      -0.5 * temp1 * con42 +
      0.0625 * temp2 * (7.0 - 114.0 * cosio2 + 395.0 * cosio4) +
      temp3 * (3.0 - 36.0 * cosio2 + 49.0 * cosio4);
    xhdot1 = -temp1 * cosio;
    sat.nodedot =
      xhdot1 + (0.5 * temp2 * (4.0 - 19.0 * cosio2) + 2.0 * temp3 * (3.0 - 7.0 * cosio2)) * cosio;
    xpidot = sat.argpdot + sat.nodedot;
    sat.omgcof = sat.drag * cc3 * Math.cos(sat.perigee);
    sat.xmcof = 0.0;
    if (sat.eccentricity > 1.0e-4) {
      sat.xmcof = (-x2o3 * coef * sat.drag) / eeta;
    }
    sat.nodecf = 3.5 * omeosq * xhdot1 * sat.cc1;
    sat.t2cof = 1.5 * sat.cc1;

    // sgp4fix for divide by zero with xinco = 180 deg
    if (Math.abs(cosio + 1.0) > 1.5e-12) {
      sat.xlcof = (-0.25 * j3oj2 * sinio * (3.0 + 5.0 * cosio)) / (1.0 + cosio);
    } else {
      sat.xlcof = (-0.25 * j3oj2 * sinio * (3.0 + 5.0 * cosio)) / temp4;
    }
    sat.aycof = -0.5 * j3oj2 * sinio;

    // sgp4fix use multiply for speed instead of pow
    const delmotemp = 1.0 + sat.eta * Math.cos(sat.anomaly);
    sat.delmo = delmotemp * delmotemp * delmotemp;
    sat.sinmao = Math.sin(sat.anomaly);
    sat.x7thm1 = 7.0 * cosio2 - 1.0;

    // --------------- deep space initialization -------------
    if ((2 * pi) / sat.motion >= 225.0) {
      sat.method = 'd';
      sat.isimp = 1;
      tc = 0.0;
      inclm = sat.inclination;

      const dscomOptions = {
        epoch,
        ep: sat.eccentricity,
        argpp: sat.perigee,
        tc,
        inclp: sat.inclination,
        nodep: sat.ascension,

        np: sat.motion,

        e3: sat.e3,
        ee2: sat.ee2,

        peo: sat.peo,
        pgho: sat.pgho,
        pho: sat.pho,
        pinco: sat.pinco,

        plo: sat.plo,
        se2: sat.se2,
        se3: sat.se3,

        sgh2: sat.sgh2,
        sgh3: sat.sgh3,
        sgh4: sat.sgh4,

        sh2: sat.sh2,
        sh3: sat.sh3,
        si2: sat.si2,
        si3: sat.si3,

        sl2: sat.sl2,
        sl3: sat.sl3,
        sl4: sat.sl4,

        xgh2: sat.xgh2,
        xgh3: sat.xgh3,
        xgh4: sat.xgh4,
        xh2: sat.xh2,

        xh3: sat.xh3,
        xi2: sat.xi2,
        xi3: sat.xi3,
        xl2: sat.xl2,

        xl3: sat.xl3,
        xl4: sat.xl4,

        zmol: sat.zmol,
        zmos: sat.zmos,
      };

      const dscomResult = dscom(dscomOptions);

      sat.e3 = dscomResult.e3;
      sat.ee2 = dscomResult.ee2;

      sat.peo = dscomResult.peo;
      sat.pgho = dscomResult.pgho;
      sat.pho = dscomResult.pho;

      sat.pinco = dscomResult.pinco;
      sat.plo = dscomResult.plo;
      sat.se2 = dscomResult.se2;
      sat.se3 = dscomResult.se3;

      sat.sgh2 = dscomResult.sgh2;
      sat.sgh3 = dscomResult.sgh3;
      sat.sgh4 = dscomResult.sgh4;
      sat.sh2 = dscomResult.sh2;
      sat.sh3 = dscomResult.sh3;

      sat.si2 = dscomResult.si2;
      sat.si3 = dscomResult.si3;
      sat.sl2 = dscomResult.sl2;
      sat.sl3 = dscomResult.sl3;
      sat.sl4 = dscomResult.sl4;

      ({
        sinim,
        cosim,
        em,
        emsq,
        s1,
        s2,
        s3,
        s4,
        s5,
        ss1,
        ss2,
        ss3,
        ss4,
        ss5,
        sz1,
        sz3,
        sz11,
        sz13,
        sz21,
        sz23,
        sz31,
        sz33,
      } = dscomResult);

      sat.xgh2 = dscomResult.xgh2;
      sat.xgh3 = dscomResult.xgh3;
      sat.xgh4 = dscomResult.xgh4;
      sat.xh2 = dscomResult.xh2;
      sat.xh3 = dscomResult.xh3;
      sat.xi2 = dscomResult.xi2;
      sat.xi3 = dscomResult.xi3;
      sat.xl2 = dscomResult.xl2;
      sat.xl3 = dscomResult.xl3;
      sat.xl4 = dscomResult.xl4;
      sat.zmol = dscomResult.zmol;
      sat.zmos = dscomResult.zmos;

      ({ nm, z1, z3, z11, z13, z21, z23, z31, z33 } = dscomResult);

      const dpperOptions = {
        inclo: inclm,
        init: sat.init,
        ep: sat.eccentricity,
        inclp: sat.inclination,
        nodep: sat.ascension,
        argpp: sat.perigee,
        mp: sat.anomaly,
        opsmode: sat.opsmode,
      };

      const dpperResult = dpper(sat, dpperOptions, 0);

      sat.eccentricity = dpperResult.ep;
      sat.inclination = dpperResult.inclp;
      sat.ascension = dpperResult.nodep;
      sat.perigee = dpperResult.argpp;
      sat.anomaly = dpperResult.mp;

      argpm = 0.0;
      nodem = 0.0;
      mm = 0.0;

      const dsinitOptions = {
        cosim,
        emsq,
        argpo: sat.perigee,
        s1,
        s2,
        s3,
        s4,
        s5,
        sinim,
        ss1,
        ss2,
        ss3,
        ss4,
        ss5,
        sz1,
        sz3,
        sz11,
        sz13,
        sz21,
        sz23,
        sz31,
        sz33,
        tc,
        gsto: sat.gsto,
        mo: sat.anomaly,
        mdot: sat.mdot,
        no: sat.motion,
        nodeo: sat.ascension,
        nodedot: sat.nodedot,
        xpidot,
        z1,
        z3,
        z11,
        z13,
        z21,
        z23,
        z31,
        z33,
        ecco: sat.eccentricity,
        eccsq,
        em,
        argpm,
        inclm,
        mm,
        nm,
        nodem,
        irez: sat.irez,
        atime: sat.atime,
        d2201: sat.d2201,
        d2211: sat.d2211,
        d3210: sat.d3210,
        d3222: sat.d3222,
        d4410: sat.d4410,
        d4422: sat.d4422,
        d5220: sat.d5220,
        d5232: sat.d5232,
        d5421: sat.d5421,
        d5433: sat.d5433,
        dedt: sat.dedt,
        didt: sat.didt,
        dmdt: sat.dmdt,
        dnodt: sat.dnodt,
        domdt: sat.domdt,
        del1: sat.del1,
        del2: sat.del2,
        del3: sat.del3,
        xfact: sat.xfact,
        xlamo: sat.xlamo,
        xli: sat.xli,
        xni: sat.xni,
      };

      const dsinitResult = dsinit(dsinitOptions, 0);

      sat.irez = dsinitResult.irez;
      sat.atime = dsinitResult.atime;
      sat.d2201 = dsinitResult.d2201;
      sat.d2211 = dsinitResult.d2211;

      sat.d3210 = dsinitResult.d3210;
      sat.d3222 = dsinitResult.d3222;
      sat.d4410 = dsinitResult.d4410;
      sat.d4422 = dsinitResult.d4422;
      sat.d5220 = dsinitResult.d5220;

      sat.d5232 = dsinitResult.d5232;
      sat.d5421 = dsinitResult.d5421;
      sat.d5433 = dsinitResult.d5433;
      sat.dedt = dsinitResult.dedt;
      sat.didt = dsinitResult.didt;

      sat.dmdt = dsinitResult.dmdt;
      sat.dnodt = dsinitResult.dnodt;
      sat.domdt = dsinitResult.domdt;
      sat.del1 = dsinitResult.del1;

      sat.del2 = dsinitResult.del2;
      sat.del3 = dsinitResult.del3;
      sat.xfact = dsinitResult.xfact;
      sat.xlamo = dsinitResult.xlamo;
      sat.xli = dsinitResult.xli;

      sat.xni = dsinitResult.xni;
    }

    // ----------- set variables if not deep space -----------
    if (sat.isimp !== 1) {
      cc1sq = sat.cc1 * sat.cc1;
      sat.d2 = 4.0 * ao * tsi * cc1sq;
      temp = (sat.d2 * tsi * sat.cc1) / 3.0;
      sat.d3 = (17.0 * ao + sfour) * temp;
      sat.d4 = 0.5 * temp * ao * tsi * (221.0 * ao + 31.0 * sfour) * sat.cc1;
      sat.t3cof = sat.d2 + 2.0 * cc1sq;
      sat.t4cof = 0.25 * (3.0 * sat.d3 + sat.cc1 * (12.0 * sat.d2 + 10.0 * cc1sq));
      sat.t5cof =
        0.2 *
        (3.0 * sat.d4 +
          12.0 * sat.cc1 * sat.d3 +
          6.0 * sat.d2 * sat.d2 +
          15.0 * cc1sq * (2.0 * sat.d2 + cc1sq));
    }
  }
}
