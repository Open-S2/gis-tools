struct Satellite {
  anomaly: f32,
  motion: f32,
  eccentricity: f32,
  inclination: f32,
  method: f32, // 0 -> 'd', 1 -> 'n'
  opsmode: f32, // 0 -> 'a'; 1 -> 'i'
  drag: f32,
  mdot: f32,
  perigee: f32,
  argpdot: f32,
  ascension: f32,
  nodedot: f32,
  nodecf: f32,
  cc1: f32,
  cc4: f32,
  cc5: f32,
  t2cof: f32,
  isimp: f32,
  omgcof: f32,
  eta: f32,
  xmcof: f32,
  delmo: f32,
  d2: f32,
  d3: f32,
  d4: f32,
  sinmao: f32,
  t3cof: f32,
  t4cof: f32,
  t5cof: f32,
  irez: f32,
  d2201: f32,
  d2211: f32,
  d3210: f32,
  d3222: f32,
  d4410: f32,
  d4422: f32,
  d5220: f32,
  d5232: f32,
  d5421: f32,
  d5433: f32,
  dedt: f32,
  del1: f32,
  del2: f32,
  del3: f32,
  didt: f32,
  dmdt: f32,
  dnodt: f32,
  domdt: f32,
  gsto: f32,
  xfact: f32,
  xlamo: f32,
  atime: f32,
  xli: f32,
  xni: f32,
  aycof: f32,
  xlcof: f32,
  con41: f32,
  x1mth2: f32,
  x7thm1: f32,
  zmos: f32,
  zmol: f32,
  se2: f32,
  se3: f32,
  si2: f32,
  si3: f32,
  sl2: f32,
  sl3: f32,
  sl4: f32,
  sgh2: f32,
  sgh3: f32,
  sgh4: f32,
  sh2: f32,
  sh3: f32,
  ee2: f32,
  e3: f32,
  xi2: f32,
  xi3: f32,
  xl2: f32,
  xl3: f32,
  xl4: f32,
  xgh2: f32,
  xgh3: f32,
  xgh4: f32,
  xh2: f32,
  xh3: f32,
  peo: f32,
  pinco: f32,
  plo: f32,
  pgho: f32,
  pho: f32
};

struct SatOutput {
  error: f32,
  position: vec3<f32>,
  velocity: vec3<f32>,
};

struct Constants {
  pi: f32,
  twoPi: f32,
  earthRadius: f32,
  xke: f32,
  vkmpersec: f32,
  j2: f32,
  j3oj2: f32,
  x2o3: f32
};

struct DpperOptions {
  init: bool,
  ep: f32,
  inclp: f32,
  nodep: f32,
  argpp: f32,
  mp: f32
};

struct DpperOutput {
  ep: f32
  inclp: f32
  nodep: f32
  argpp: f32
  mp: f32
};

struct DspaceOptions {
  irez: f32
  d2201: f32
  d2211: f32
  d3210: f32
  d3222: f32
  d4410: f32
  d4422: f32
  d5220: f32
  d5232: f32
  d5421: f32
  d5433: f32
  dedt: f32
  del1: f32
  del2: f32
  del3: f32
  didt: f32
  dmdt: f32
  dnodt: f32
  domdt: f32
  argpo: f32
  argpdot: f32
  tc: f32
  gsto: f32
  xfact: f32
  xlamo: f32
  no: f32
  atime: f32
  em: f32
  argpm: f32
  inclm: f32
  xli: f32
  mm: f32
  xni: f32
  nodem: f32
  nm: f32
};

struct DspaceOutput {
  atime: f32
  em: f32 // eccentricity
  argpm: f32 // argument of perigee
  inclm: f32 // inclination
  xli: f32
  mm: f32 // mean anomaly
  xni: f32
  nodem: f32 // right ascension of ascending node
  dndt: f32
  nm: f32 // mean motion
}

@binding(0) @group(0) var<uniform> constants : Constants; // time since epoch (minutes)
@binding(0) @group(1) var<uniform> size : u32; // time since epoch (minutes)
@binding(0) @group(2) var<storage, read> sat : array<Satellite>;
@binding(0) @group(3) var<storage, read_write> out : array<SatOutput>;
@binding(1) @group(0) var<uniform> tsince : f32; // time since epoch (minutes)

@compute @workgroup_size(256)
fn sgp4(
  @builtin(global_invocation_id) global_id : vec3<u32>
) {
  // Guard against out-of-bounds work group sizes
  if (global_id.x >= size) { return; }

  let sat: ptr<Satellite> = sat[global_id.x];
  let out: ptr<SatOutput> = &out[global_id.x];

  var aycof: f32 = sat.aycof;
  var xlcof: f32 = sat.xlcof;
  var con41: f32 = sat.con41;
  var x1mth2: f32 = sat.x1mth2;
  var x7thm1: f32 = sat.x7thm1;

  var coseo1: f32 = 0.0;
  var sineo1: f32 = 0.0;
  var cosip: f32 = 0.0;
  var sinip: f32 = 0.0;
  var cosisq: f32 = 0.0;
  var delm: f32 = 0.0;
  var delomg: f32 = 0.0;
  var eo1: f32 = 0.0;
  var argpm: f32 = 0.0;
  var argpp: f32 = 0.0;
  var su: f32 = 0.0;
  var t3: f32 = 0.0;
  var t4: f32 = 0.0;
  var tc: f32 = 0.0;
  var tem5: f32 = 0.0;
  var temp: f32 = 0.0;
  var tempa: f32 = 0.0;
  var tempe: f32 = 0.0;
  var templ: f32 = 0.0;
  var inclm: f32 = 0.0;
  var mm: f32 = 0.0;
  var nm: f32 = 0.0;
  var nodem: f32 = 0.0;
  var xincp: f32 = 0.0;
  var xlm: f32 = 0.0;
  var mp: f32 = 0.0;
  var nodep: f32 = 0.0;

  // ------------------ set mathematical constants --------------- */
  // sgp4fix divisor for divide by zero check on inclination
  // the old check used 1.0 + cos(pi-1.0e-9), but then compared it to
  // 1.5 e-12, so the threshold was changed to 1.5e-12 for consistency

  let temp4: f32 = 1.5e-12;

  //  ------- update for secular gravity and atmospheric drag -----
  let xmdf: f32 = sat.anomaly + (sat.mdot * tsince);
  let argpdf: f32 = sat.perigee + (sat.argpdot * tsince);
  let nodedf: f32 = sat.ascension + (sat.nodedot * tsince);
  argpm = argpdf;
  mm = xmdf;
  let t2: f32 = tsince * tsince;
  nodem = nodedf + (sat.nodecf * t2);
  tempa = 1.0 - (sat.cc1 * tsince);
  tempe = sat.drag * sat.cc4 * tsince;
  templ = sat.t2cof * t2;

  if (sat.isimp != 1.0) {
    delomg = sat.omgcof * tsince;
    //  sgp4fix use mutliply for speed instead of pow
    let delmtemp: f32 = 1.0 + (sat.eta * cos(xmdf));
    delm = sat.xmcof * ((delmtemp * delmtemp * delmtemp) - sat.delmo);
    temp = delomg + delm;
    mm = xmdf + temp;
    argpm = argpdf - temp;
    t3 = t2 * tsince;
    t4 = t3 * tsince;
    tempa = tempa - (sat.d2 * t2) - (sat.d3 * t3) - (sat.d4 * t4);
    tempe += sat.drag * sat.cc5 * (sin(mm) - sat.sinmao);
    templ = templ + (sat.t3cof * t3) + (t4 * (sat.t4cof + (tsince * sat.t5cof)));
  }
  nm = sat.motion;
  var em: f32 = sat.eccentricity;
  inclm = sat.inclination;
  if (sat.method == 0.0) {
    tc = tsince;

    let dspaceOptions: DspaceOptions = DspaceOptions(
      sat.irez,
      sat.d2201,
      sat.d2211,
      sat.d3210,
      sat.d3222,
      sat.d4410,
      sat.d4422,
      sat.d5220,
      sat.d5232,
      sat.d5421,
      sat.d5433,
      sat.dedt,
      sat.del1,
      sat.del2,
      sat.del3,
      sat.didt,
      sat.dmdt,
      sat.dnodt,
      sat.domdt,
      sat.perigee, // argpo
      sat.argpdot,
      tc,
      sat.gsto,
      sat.xfact,
      sat.xlamo,
      sat.motion, // no
      sat.atime,
      em,
      argpm,
      inclm,
      sat.xli,
      mm,
      sat.xni,
      nodem,
      nm
    );

    let dspaceResult: DspaceOutput = dspace(dspaceOptions);
    em = dspaceResult.em;
    argpm = dspaceResult.argpm;
    inclm = dspaceResult.inclm;
    // xli = dspaceResult.xli;
    mm = dspaceResult.mm;
    // xni = dspaceResult.xni;
    nodem = dspaceResult.nodem;
    nm = dspaceResult.nm;
  }

  if (nm <= 0.0) {
    // sgp4fix add return
    (*out).error = 2.0;
    return;
  }

  let am: f32 = pow((constants.xke / nm), constants.x2o3) * tempa * tempa;
  nm = constants.xke / pow(am, 1.5);
  em -= tempe;

  // fix tolerance for error recognition
  // sgp4fix am is fixed from the previous nm check
  if (em >= 1.0 || em < -0.001) { // || (am < 0.95)
    // sgp4fix to return if there is an error in eccentricity
    (*out).error = 1.0;
    return;
  }

  //  sgp4fix fix tolerance to avoid a divide by zero
  if (em < 1.0e-6) { em = 1.0e-6; }
  mm += sat.motion * templ;
  xlm = mm + argpm + nodem;

  nodem %= constants.twoPi;
  argpm %= constants.twoPi;
  xlm %= constants.twoPi;
  mm = (xlm - argpm - nodem) % constants.twoPi;

  // ----------------- compute extra mean quantities -------------
  let sinim: f32 = sin(inclm);
  let cosim: f32 = cos(inclm);

  // -------------------- add lunar-solar periodics --------------
  var ep: f32 = em;
  xincp = inclm;
  argpp = argpm;
  nodep = nodem;
  mp = mm;
  sinip = sinim;
  cosip = cosim;
  if (sat.method == 0.0) {
    let dpperParameters: DpperOptions = DpperOptions(
      0.0, // init
      ep,
      xincp, // inclp
      nodep,
      argpp,
      mp,
      sat.opsmode
    );
    let dpperResult: DpperOutput = dpper(sat, dpperParameters);
    ep = dpperResult.ep;
    nodep = dpperResult.nodep;
    argpp = dpperResult.argpp;
    mp = dpperResult.mp;

    xincp = dpperResult.inclp;

    if (xincp < 0.0) {
      xincp = -xincp;
      nodep += constants.pi;
      argpp -= constants.pi;
    }
    if (ep < 0.0 || ep > 1.0) {
      (*out).error = 3.0;
    }
  }

  //  -------------------- long period periodics ------------------
  if (sat.method == 0.0) {
    sinip = sin(xincp);
    cosip = cos(xincp);
    aycof = -0.5 * constants.j3oj2 * sinip;

    //  sgp4fix for divide by zero for xincp = 180 deg
    if (abs(cosip + 1.0) > 1.5e-12) {
      xlcof = (-0.25 * constants.j3oj2 * sinip * (3.0 + (5.0 * cosip))) / (1.0 + cosip);
    } else {
      xlcof = (-0.25 * constants.j3oj2 * sinip * (3.0 + (5.0 * cosip))) / temp4;
    }
  }

  let axnl: f32 = ep * cos(argpp);
  temp = 1.0 / (am * (1.0 - (ep * ep)));
  let aynl: f32 = (ep * sin(argpp)) + (temp * aycof);
  let xl: f32 = mp + argpp + nodep + (temp * xlcof * axnl);

  // --------------------- solve kepler's equation ---------------
  let u: f32 = (xl - nodep) % constants.twoPi;
  eo1 = u;
  tem5 = 9999.9;
  var ktr: i32 = 1;

  //    sgp4fix for kepler iteration
  //    the following iteration needs better limits on corrections
  while (abs(tem5) >= 1.0e-12 && ktr <= 10) {
    sineo1 = sin(eo1);
    coseo1 = cos(eo1);
    tem5 = 1.0 - (coseo1 * axnl) - (sineo1 * aynl);
    tem5 = (((u - (aynl * coseo1)) + (axnl * sineo1)) - eo1) / tem5;
    if (abs(tem5) >= 0.95) {
      if (tem5 > 0.0) {
        tem5 = 0.95;
      } else {
        tem5 = -0.95;
      }
    }
    eo1 += tem5;
    ktr += 1;
  }

  //  ------------- short period preliminary quantities -----------
  let ecose: f32 = (axnl * coseo1) + (aynl * sineo1);
  let esine: f32 = (axnl * sineo1) - (aynl * coseo1);
  let el2: f32 = (axnl * axnl) + (aynl * aynl);
  let pl: f32 = am * (1.0 - el2);
  if (pl < 0.0) {
    (*out).error = 4.0;
    return;
  }

  let rl: f32 = am * (1.0 - ecose);
  let rdotl: f32 = (sqrt(am) * esine) / rl;
  let rvdotl: f32 = sqrt(pl) / rl;
  let betal: f32 = sqrt(1.0 - el2);
  temp = esine / (1.0 + betal);
  let sinu: f32 = (am / rl) * (sineo1 - aynl - (axnl * temp));
  let cosu: f32 = (am / rl) * ((coseo1 - axnl) + (aynl * temp));
  su = atan2(sinu, cosu);
  let sin2u: f32 = (cosu + cosu) * sinu;
  let cos2u: f32 = 1.0 - (2.0 * sinu * sinu);
  temp = 1.0 / pl;
  let temp1: f32 = 0.5 * constants.j2 * temp;
  let temp2: f32 = temp1 * temp;

  // -------------- update for short period periodics ------------
  if (sat.method == 0.0) {
    cosisq = cosip * cosip;
    con41 = (3.0 * cosisq) - 1.0;
    x1mth2 = 1.0 - cosisq;
    x7thm1 = (7.0 * cosisq) - 1.0;
  }

  let mrt: f32 = (rl * (1.0 - (1.5 * temp2 * betal * sat.con41))) +
    (0.5 * temp1 * sat.x1mth2 * cos2u);

  // sgp4fix for decaying satellites
  if (mrt < 1.0) {
    (*out).error = 6.0;
    return;
  }

  su -= 0.25 * temp2 * x7thm1 * sin2u;
  let xnode: f32 = nodep + (1.5 * temp2 * cosip * sin2u);
  let xinc: f32 = xincp + (1.5 * temp2 * cosip * sinip * cos2u);
  let mvt: f32 = rdotl - ((nm * temp1 * x1mth2 * sin2u) / constants.xke);
  let rvdot: f32 = rvdotl + ((nm * temp1 * ((x1mth2 * cos2u) + (1.5 * con41))) / constants.xke);

  // --------------------- orientation vectors -------------------
  let sinsu: f32 = sin(su);
  let cossu: f32 = cos(su);
  let snod: f32 = sin(xnode);
  let cnod: f32 = cos(xnode);
  let sini: f32 = sin(xinc);
  let cosi: f32 = cos(xinc);
  let xmx: f32 = -snod * cosi;
  let xmy: f32 = cnod * cosi;
  let ux: f32 = (xmx * sinsu) + (cnod * cossu);
  let uy: f32 = (xmy * sinsu) + (snod * cossu);
  let uz: f32 = sini * sinsu;
  let vx: f32 = (xmx * cossu) - (cnod * sinsu);
  let vy: f32 = (xmy * cossu) - (snod * sinsu);
  let vz: f32 = sini * cossu;

  // --------- position and velocity (in km and km/sec) ----------
  (*out).position = vec3<f32>(
    (mrt * ux) * constants.earthRadius,
    (mrt * uy) * constants.earthRadius,
    (mrt * uz) * constants.earthRadius
  );
  (*out).velocity = vec3<f32>(
    ((mvt * ux) + (rvdot * vx)) * constants.vkmpersec,
    ((mvt * uy) + (rvdot * vy)) * constants.vkmpersec,
    ((mvt * uz) + (rvdot * vz)) * constants.vkmpersec
  );
}

//
//
//                           procedure dpper
//
//  this procedure provides deep space long period periodic contributions
//    to the mean elements.  by design, these periodics are zero at epoch.
//    this used to be dscom which included initialization, but it's really a
//    recurring function.
//
//  author        : david vallado                  719-573-2600   28 jun 2005
//
//  inputs        :
//    e3          -
//    ee2         -
//    peo         -
//    pgho        -
//    pho         -
//    pinco       -
//    plo         -
//    se2 , se3 , sgh2, sgh3, sgh4, sh2, sh3, si2, si3, sl2, sl3, sl4 -
//    t           -
//    xh2, xh3, xi2, xi3, xl2, xl3, xl4 -
//    zmol        -
//    zmos        -
//    ep          - eccentricity                           0.0 - 1.0
//    inclo       - inclination - needed for lyddane modification
//    nodep       - right ascension of ascending node
//    argpp       - argument of perigee
//    mp          - mean anomaly
//
//  outputs       :
//    ep          - eccentricity                           0.0 - 1.0
//    inclp       - inclination
//    nodep        - right ascension of ascending node
//    argpp       - argument of perigee
//    mp          - mean anomaly
//
//  locals        :
//    alfdp       -
//    betdp       -
//    cosip  , sinip  , cosop  , sinop  ,
//    dalf        -
//    dbet        -
//    dls         -
//    f2, f3      -
//    pe          -
//    pgh         -
//    ph          -
//    pinc        -
//    pl          -
//    sel   , ses   , sghl  , sghs  , shl   , shs   , sil   , sinzf , sis   ,
//    sll   , sls
//    xls         -
//    xnoh        -
//    zf          -
//    zm          -
//
//  coupling      :
//    none.
//
//  references    :
//    hoots, roehrich, norad spacetrack report #3 1980
//    hoots, norad spacetrack report #6 1986
//    hoots, schumacher and glover 2004
//    vallado, crawford, hujsak, kelso  2006
//----------------------------------------------------------------------------
fn dpper(sat: Satellite, options: DpperOptions) -> DpperOutput {
  var ep: f32 = options.ep;
  var inclp: f32 = options.inclp;
  var nodep: f32 = options.nodep;
  var argpp: f32 = options.argpp;
  var mp: f32 = options.mp;

  // Copy satellite attributes into local variables for convenience
  // and symmetry in writing formulae.

  var alfdp: f32;
  var betdp: f32;
  var cosip: f32;
  var sinip: f32;
  var cosop: f32;
  var sinop: f32;
  var dalf: f32;
  var dbet: f32;
  var dls: f32;
  var f2: f32;
  var f3: f32;
  var pe: f32;
  var pgh: f32;
  var ph: f32;
  var pinc: f32;
  var pl: f32;
  var sinzf: f32;
  var xls: f32;
  var xnoh: f32;
  var zf: f32;
  var zm: f32;

  //  ---------------------- constants -----------------------------
  let zns: f32 = 1.19459e-5;
  let zes: f32 = 0.01675;
  let znl: f32 = 1.5835218e-4;
  let zel: f32 = 0.05490;

  //  --------------- calculate time varying periodics -----------
  zm = sat.zmos + (zns * tsince);

  // be sure that the initial call has time set to zero
  if (options.init) {
    zm = sat.zmos;
  }
  zf = zm + (2.0 * zes * sin(zm));
  sinzf = sin(zf);
  f2 = (0.5 * sinzf * sinzf) - 0.25;
  f3 = -0.5 * sinzf * cos(zf);

  let ses: f32 = (sat.se2 * f2) + (sat.se3 * f3);
  let sis: f32 = (sat.si2 * f2) + (sat.si3 * f3);
  let sls: f32 = (sat.sl2 * f2) + (sat.sl3 * f3) + (sat.sl4 * sinzf);
  let sghs: f32 = (sat.sgh2 * f2) + (sat.sgh3 * f3) + (sat.sgh4 * sinzf);
  let shs: f32 = (sat.sh2 * f2) + (sat.sh3 * f3);

  zm = sat.zmol + (znl * tsince);
  if (options.init) { zm = sat.zmol; }

  zf = zm + (2.0 * zel * sin(zm));
  sinzf = sin(zf);
  f2 = (0.5 * sinzf * sinzf) - 0.25;
  f3 = -0.5 * sinzf * cos(zf);

  let sel: f32 = (sat.ee2 * f2) + (sat.e3 * f3);
  let sil: f32 = (sat.xi2 * f2) + (sat.xi3 * f3);
  let sll: f32 = (sat.xl2 * f2) + (sat.xl3 * f3) + (sat.xl4 * sinzf);
  let sghl: f32 = (sat.xgh2 * f2) + (sat.xgh3 * f3) + (sat.xgh4 * sinzf);
  let shll: f32 = (sat.xh2 * f2) + (sat.xh3 * f3);

  pe = ses + sel;
  pinc = sis + sil;
  pl = sls + sll;
  pgh = sghs + sghl;
  ph = shs + shll;

  if (!options.init) {
    pe -= sat.peo;
    pinc -= sat.pinco;
    pl -= sat.plo;
    pgh -= sat.pgho;
    ph -= sat.pho;
    inclp += pinc;
    ep += pe;
    sinip = sin(inclp);
    cosip = cos(inclp);

    // * ----------------- apply periodics directly ------------ */
    // sgp4fix for lyddane choice
    // strn3 used original inclination - this is technically feasible
    // gsfc used perturbed inclination - also technically feasible
    // probably best to readjust the 0.2 limit value and limit discontinuity
    // 0.2 rad = 11.45916 deg
    // use next line for original strn3 approach and original inclination
    // if (inclo >= 0.2)
    // use next line for gsfc version and perturbed inclination
    if (inclp >= 0.2) {
      ph /= sinip;
      pgh -= cosip * ph;
      argpp += pgh;
      nodep += ph;
      mp += pl;
    } else {
      //  ---- apply periodics with lyddane modification ----
      sinop = sin(nodep);
      cosop = cos(nodep);
      alfdp = sinip * sinop;
      betdp = sinip * cosop;
      dalf = (ph * cosop) + (pinc * cosip * sinop);
      dbet = (-ph * sinop) + (pinc * cosip * cosop);
      alfdp += dalf;
      betdp += dbet;
      nodep %= constants.twoPi;

      //  sgp4fix for afspc written intrinsic functions
      //  nodep used without a trigonometric function ahead
      if (nodep < 0.0 && sat.opsmode == 0.0) {
        nodep += constants.twoPi;
      }
      xls = mp + argpp + (cosip * nodep);
      dls = (pl + pgh) - (pinc * nodep * sinip);
      xls += dls;
      xnoh = nodep;
      nodep = atan2(alfdp, betdp);

      //  sgp4fix for afspc written intrinsic functions
      //  nodep used without a trigonometric function ahead
      if (nodep < 0.0 && sat.opsmode == 0.0) {
        nodep += constants.twoPi;
      }
      if (abs(xnoh - nodep) > constants.pi) {
        if (nodep < xnoh) {
          nodep += constants.twoPi;
        } else {
          nodep -= constants.twoPi;
        }
      }
      mp += pl;
      argpp = xls - mp - (cosip * nodep);
    }
  }

  return DpperOutput(
    ep,
    inclp,
    nodep,
    argpp,
    mp
  );
}

// -----------------------------------------------------------------------------
//
//                           procedure dspace
//
//  this procedure provides deep space contributions to mean elements for
//    perturbing third body.  these effects have been averaged over one
//    revolution of the sun and moon.  for earth resonance effects, the
//    effects have been averaged over no revolutions of the satellite.
//    (mean motion)
//
//  author        : david vallado                  719-573-2600   28 jun 2005
//
//  inputs        :
//    d2201, d2211, d3210, d3222, d4410, d4422, d5220, d5232, d5421, d5433 -
//    dedt        -
//    del1, del2, del3  -
//    didt        -
//    dmdt        -
//    dnodt       -
//    domdt       -
//    irez        - flag for resonance           0-none, 1-one day, 2-half day
//    argpo       - argument of perigee
//    argpdot     - argument of perigee dot (rate)
//    t           - time
//    tc          -
//    gsto        - gst
//    xfact       -
//    xlamo       -
//    no          - mean motion
//    atime       -
//    em          - eccentricity
//    ft          -
//    argpm       - argument of perigee
//    inclm       - inclination
//    xli         -
//    mm          - mean anomaly
//    xni         - mean motion
//    nodem       - right ascension of ascending node
//
//  outputs       :
//    atime       -
//    em          - eccentricity
//    argpm       - argument of perigee
//    inclm       - inclination
//    xli         -
//    mm          - mean anomaly
//    xni         -
//    nodem       - right ascension of ascending node
//    dndt        -
//    nm          - mean motion
//
//  locals        :
//    delt        -
//    ft          -
//    theta       -
//    x2li        -
//    x2omi       -
//    xl          -
//    xldot       -
//    xnddt       -
//    xndt        -
//    xomi        -
//
//  coupling      :
//    none        -
//
//  references    :
//    hoots, roehrich, norad spacetrack report #3 1980
//    hoots, norad spacetrack report #6 1986
//    hoots, schumacher and glover 2004
//    vallado, crawford, hujsak, kelso  2006
//----------------------------------------------------------------------------
fn dspace(options: DspaceOptions) -> DspaceOutput {
  var atime: f32 = options.atime;
  var em: f32 = options.em;
  var argpm: f32 = options.argpm;
  var inclm: f32 = options.inclm;
  var xli: f32 = options.xli;
  var mm: f32 = options.mm;
  var xni: f32 = options.xni;
  var nodem: f32 = options.nodem;
  var nm: f32 = options.nm;

  let fasx2: f32 = 0.13130908;
  let fasx4: f32 = 2.8843198;
  let fasx6: f32 = 0.37448087;
  let g22: f32 = 5.7686396;
  let g32: f32 = 0.95240898;
  let g44: f32 = 1.8014998;
  let g52: f32 = 1.0508330;
  let g54: f32 = 4.4108898;
  let rptim: f32 = 4.37526908801129966e-3; // equates to 7.29211514668855e-5 rad/sec
  let stepp: f32 = 720.0;
  let stepn: f32 = -720.0;
  let step2: f32 = 259200.0;

  var delt: f32;
  var x2li: f32;
  var x2omi: f32;
  var xl: f32;
  var xldot: f32 = 0.0;
  var xnddt: f32 = 0.0;
  var xndt: f32 = 0.0;
  var xomi: f32;
  var dndt: f32 = 0.0;
  var ft: f32 = 0.0;

  //  ----------- calculate deep space resonance effects -----------
  let theta: f32 = (options.gsto + (options.tc * rptim)) % constants.twoPi;
  em += options.dedt * tsince;

  inclm += options.didt * tsince;
  argpm += options.domdt * tsince;
  nodem += options.dnodt * tsince;
  mm += options.dmdt * tsince;

  // - update resonances : numerical (euler-maclaurin) integration - //
  // ------------------------- epoch restart ----------------------  //
  //   sgp4fix for propagator problems
  //   the following integration works for negative time steps and periods
  //   the specific changes are unknown because the original code was so convoluted

  // sgp4fix take out atime = 0.0 and fix for faster operation

  if (options.irez != 0.0) {
    //  sgp4fix streamline check
    if (
      atime == 0.0 ||
      tsince * atime <= 0.0 ||
      abs(tsince) < abs(atime)
    ) {
      atime = 0.0;
      xni = options.no;
      xli = options.xlamo;
    }

    // sgp4fix move check outside loop
    if (tsince > 0.0) {
      delt = stepp;
    } else {
      delt = stepn;
    }

    var iretn: f32 = 381.0; // added for do loop
    while (iretn == 381.0) {
      //  ------------------- dot terms calculated -------------
      //  ----------- near - synchronous resonance terms -------
      if (options.irez != 2.0) {
        xndt = (options.del1 * sin(xli - fasx2)) +
          (options.del2 * sin(2.0 * (xli - fasx4))) +
          (options.del3 * sin(3.0 * (xli - fasx6)));
        xldot = xni + options.xfact;
        xnddt = (options.del1 * cos(xli - fasx2)) +
          (2.0 * options.del2 * cos(2.0 * (xli - fasx4))) +
          (3.0 * options.del3 * cos(3.0 * (xli - fasx6)));
        xnddt *= xldot;
      } else {
        // --------- near - half-day resonance terms --------
        xomi = options.argpo + (options.argpdot * atime);
        x2omi = xomi + xomi;
        x2li = xli + xli;
        xndt = (options.d2201 * sin((x2omi + xli) - g22)) +
          (options.d2211 * sin(xli - g22)) +
          (options.d3210 * sin((xomi + xli) - g32)) +
          (options.d3222 * sin((-xomi + xli) - g32)) +
          (options.d4410 * sin((x2omi + x2li) - g44)) +
          (options.d4422 * sin(x2li - g44)) +
          (options.d5220 * sin((xomi + xli) - g52)) +
          (options.d5232 * sin((-xomi + xli) - g52)) +
          (options.d5421 * sin((xomi + x2li) - g54)) +
          (options.d5433 * sin((-xomi + x2li) - g54));
        xldot = xni + options.xfact;
        xnddt = (options.d2201 * cos((x2omi + xli) - g22)) +
          (options.d2211 * cos(xli - g22)) +
          (options.d3210 * cos((xomi + xli) - g32)) +
          (options.d3222 * cos((-xomi + xli) - g32)) +
          (options.d5220 * cos((xomi + xli) - g52)) +
          (options.d5232 * cos((-xomi + xli) - g52)) +
          2.0 * ((options.d4410 * cos((x2omi + x2li) - g44)) +
          (options.d4422 * cos(x2li - g44)) +
          (options.d5421 * cos((xomi + x2li) - g54)) +
          (options.d5433 * cos((-xomi + x2li) - g54)));
        xnddt *= xldot;
      }

      //  ----------------------- integrator -------------------
      //  sgp4fix move end checks to end of routine
      if (abs(tsince - atime) >= stepp) {
        iretn = 381.0;
      } else {
        ft = tsince - atime;
        iretn = 0.0;
      }

      if (iretn == 381.0) {
        xli += (xldot * delt) + (xndt * step2);
        xni += (xndt * delt) + (xnddt * step2);
        atime += delt;
      }
    }

    nm = xni + (xndt * ft) + (xnddt * ft * ft * 0.5);
    xl = xli + (xldot * ft) + (xndt * ft * ft * 0.5);
    if (options.irez != 1.0) {
      mm = (xl - (2.0 * nodem)) + (2.0 * theta);
      dndt = nm - options.no;
    } else {
      mm = (xl - nodem - argpm) + theta;
      dndt = nm - options.no;
    }
    nm = options.no + dndt;
  }

  return DspaceOutput(
    atime,
    em,
    argpm,
    inclm,
    xli,
    mm,
    xni,
    nodem,
    dndt,
    nm
  );
}
