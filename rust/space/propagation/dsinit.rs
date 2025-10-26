use crate::space::util::constants::{X2_3, XKE};
use core::f64::consts::{PI, TAU};
use libm::pow;

/// Options for DsInit
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DsInitOptions {
    /// cosim
    pub cosim: f64,
    /// argpo
    pub argpo: f64,
    /// s1
    pub s1: f64,
    /// s2
    pub s2: f64,
    /// s3
    pub s3: f64,
    /// s4
    pub s4: f64,
    /// s5
    pub s5: f64,
    /// sinim
    pub sinim: f64,
    /// ss1
    pub ss1: f64,
    /// ss2
    pub ss2: f64,
    /// ss3
    pub ss3: f64,
    /// ss4
    pub ss4: f64,
    /// ss5
    pub ss5: f64,
    /// sz1
    pub sz1: f64,
    /// sz3
    pub sz3: f64,
    /// sz11
    pub sz11: f64,
    /// sz13
    pub sz13: f64,
    /// sz21
    pub sz21: f64,
    /// sz23
    pub sz23: f64,
    /// sz31
    pub sz31: f64,
    /// sz33
    pub sz33: f64,
    /// tc
    pub tc: f64,
    /// gsto
    pub gsto: f64,
    /// mo
    pub mo: f64,
    /// mdot
    pub mdot: f64,
    /// no
    pub no: f64,
    /// nodeo
    pub nodeo: f64,
    /// nodedot
    pub nodedot: f64,
    /// xpidot
    pub xpidot: f64,
    /// z1
    pub z1: f64,
    /// z3
    pub z3: f64,
    /// z11
    pub z11: f64,
    /// z13
    pub z13: f64,
    /// z21
    pub z21: f64,
    /// z23
    pub z23: f64,
    /// z31
    pub z31: f64,
    /// z33
    pub z33: f64,
    /// ecco
    pub ecco: f64,
    /// eccsq
    pub eccsq: f64,
    /// emsq
    pub emsq: f64,
    /// em
    pub em: f64,
    /// argpm
    pub argpm: f64,
    /// inclm
    pub inclm: f64,
    /// mm
    pub mm: f64,
    /// nm
    pub nm: f64,
    /// nodem
    pub nodem: f64,
    /// irez
    pub irez: f64,
    /// atime
    pub atime: f64,
    /// d2201
    pub d2201: f64,
    /// d2211
    pub d2211: f64,
    /// d3210
    pub d3210: f64,
    /// d3222
    pub d3222: f64,
    /// d4410
    pub d4410: f64,
    /// d4422
    pub d4422: f64,
    /// d5220
    pub d5220: f64,
    /// d5232
    pub d5232: f64,
    /// d5421
    pub d5421: f64,
    /// d5433
    pub d5433: f64,
    /// dedt
    pub dedt: f64,
    /// didt
    pub didt: f64,
    /// dmdt
    pub dmdt: f64,
    /// dnodt
    pub dnodt: f64,
    /// domdt
    pub domdt: f64,
    /// del1
    pub del1: f64,
    /// del2
    pub del2: f64,
    /// del3
    pub del3: f64,
    /// xfact
    pub xfact: f64,
    /// xlamo
    pub xlamo: f64,
    /// xli
    pub xli: f64,
    /// xni
    pub xni: f64,
}

/// Output from DsInit
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DsInitOutput {
    /// em
    pub em: f64,
    /// argpm
    pub argpm: f64,
    /// inclm
    pub inclm: f64,
    /// mm
    pub mm: f64,
    /// nm
    pub nm: f64,
    /// nodem
    pub nodem: f64,

    /// irez
    pub irez: f64,
    /// atime
    pub atime: f64,

    /// d2201
    pub d2201: f64,
    /// d2211
    pub d2211: f64,
    /// d3210
    pub d3210: f64,
    /// d3222
    pub d3222: f64,
    /// d4410
    pub d4410: f64,

    /// d4422
    pub d4422: f64,
    /// d5220
    pub d5220: f64,
    /// d5232
    pub d5232: f64,
    /// d5421
    pub d5421: f64,
    /// d5433
    pub d5433: f64,

    /// dedt
    pub dedt: f64,
    /// didt
    pub didt: f64,
    /// dmdt
    pub dmdt: f64,
    /// dndt
    pub dndt: f64,
    /// dnodt
    pub dnodt: f64,
    /// domdt
    pub domdt: f64,

    /// del1
    pub del1: f64,
    /// del2
    pub del2: f64,
    /// del3
    pub del3: f64,

    /// xfact
    pub xfact: f64,
    /// xlamo
    pub xlamo: f64,
    /// xli
    pub xli: f64,
    /// xni
    pub xni: f64,
}

/// procedure dsinit
///
///  this procedure provides deep space contributions to mean motion dot due
///    to geopotential resonance with half day and one day orbits.
///
///  author         david vallado                  719-573-2600   28 jun 2005
///
///  references
///    hoots, roehrich, norad spacetrack report #3 1980
///    hoots, norad spacetrack report #6 1986
///    hoots, schumacher and glover 2004
///    vallado, crawford, hujsak, kelso  2006
///
/// ## Parameters
/// - `options`: the options
/// - `tsince`: the time since epoch
///
/// ## Returns
/// The computed dpsace initial values
pub fn dsinit(options: DsInitOptions, tsince: f64) -> DsInitOutput {
    let DsInitOptions {
        cosim,
        argpo,
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
        gsto,
        mo,
        mdot,
        no,
        nodeo,
        nodedot,
        xpidot,
        z1,
        z3,
        z11,
        z13,
        z21,
        z23,
        z31,
        z33,
        ecco,
        eccsq,
        mut emsq,
        mut em,
        mut argpm,
        mut inclm,
        mut mm,
        mut nm,
        mut nodem,
        // irez,
        mut atime,
        mut d2201,
        mut d2211,
        mut d3210,
        mut d3222,
        mut d4410,
        mut d4422,
        mut d5220,
        mut d5232,
        mut d5421,
        mut d5433,
        // mut dedt,
        // mut didt,
        // mut dmdt,
        // mut dnodt,
        // mut domdt,
        mut del1,
        mut del2,
        mut del3,
        mut xfact,
        mut xlamo,
        mut xli,
        mut xni,
        ..
    } = options;

    let mut f220: f64;
    let f221: f64;
    let f311: f64;
    let f321: f64;
    let f322: f64;
    let mut f330: f64;
    let f441: f64;
    let f442: f64;
    let f522: f64;
    let f523: f64;
    let f542: f64;
    let f543: f64;
    let g200: f64;
    let g201: f64;
    let g211: f64;
    let g300: f64;
    let mut g310: f64;
    let g322: f64;
    let g410: f64;
    let g422: f64;
    let g520: f64;
    let g521: f64;
    let g532: f64;
    let g533: f64;
    let sini2: f64;
    let mut temp: f64;
    let mut temp1: f64;
    let xno2: f64;
    let ainv2: f64;
    let aonv: f64;
    let cosisq: f64;
    let eoc: f64;

    let q22 = 1.7891679e-6;
    let q31 = 2.1460748e-6;
    let q33 = 2.2123015e-7;
    let root22 = 1.7891679e-6;
    let root44 = 7.3636953e-9;
    let root54 = 2.1765803e-9;

    // eslint-disable-next-line no-loss-of-precision
    let rptim = 4.375_269_088_011_3e-3; // equates to 7.29211514668855e-5 rad/sec
    let root32 = 3.7393792e-7;
    let root52 = 1.1428639e-7;
    let znl = 1.5835218e-4;
    let zns = 1.19459e-5;

    // -------------------- deep space initialization ------------
    let mut irez = 0;
    if nm < 0.0052359877 && nm > 0.0034906585 {
        irez = 1;
    }
    if (8.26e-3..=9.24e-3).contains(&nm) {
        irez = 2;
    }

    // ------------------------ do solar terms -------------------
    let ses = ss1 * zns * ss5;
    let sis = ss2 * zns * (sz11 + sz13);
    let sls = -zns * ss3 * (sz1 + sz3 - 14.0 - 6.0 * emsq);
    let sghs = ss4 * zns * (sz31 + sz33 - 6.0);
    let mut shs = -zns * ss2 * (sz21 + sz23);

    // sgp4fix for 180 deg incl
    if !(5.2359877e-2..=PI - 5.2359877e-2).contains(&inclm) {
        shs = 0.0;
    }
    if sinim != 0.0 {
        shs /= sinim;
    }
    let sgs = sghs - cosim * shs;

    // ------------------------- do lunar terms ------------------
    let dedt = ses + s1 * znl * s5;
    let didt = sis + s2 * znl * (z11 + z13);
    let dmdt = sls - znl * s3 * (z1 + z3 - 14.0 - 6.0 * emsq);
    let sghl = s4 * znl * (z31 + z33 - 6.0);
    let mut shll = -znl * s2 * (z21 + z23);

    // sgp4fix for 180 deg incl
    if !(5.2359877e-2..=PI - 5.2359877e-2).contains(&inclm) {
        shll = 0.0;
    }
    let mut domdt = sgs + sghl;
    let mut dnodt = shs;
    if sinim != 0.0 {
        domdt -= (cosim / sinim) * shll;
        dnodt += shll / sinim;
    }

    // ----------- calculate deep space resonance effects --------
    let dndt = 0.0;
    let theta = (gsto + tc * rptim) % TAU;
    em += dedt * tsince;
    inclm += didt * tsince;
    argpm += domdt * tsince;
    nodem += dnodt * tsince;
    mm += dmdt * tsince;

    // sgp4fix for negative inclinations
    // the following if statement should be commented out
    // if (inclm < 0.0)
    // {
    //   inclm  = -inclm;
    //   argpm  = argpm - pi;
    //   nodem = nodem + pi;
    // }

    // -------------- initialize the resonance terms -------------
    if irez != 0 {
        // aonv = (nm / xke) ** x2o3;
        aonv = pow(nm / XKE, X2_3);

        // ---------- geopotential resonance for 12 hour orbits ------
        if irez == 2 {
            cosisq = cosim * cosim;
            let emo = em;
            em = ecco;
            let emsqo = emsq;
            emsq = eccsq;
            eoc = em * emsq;
            g201 = -0.306 - (em - 0.64) * 0.44;

            if em <= 0.65 {
                g211 = 3.616 - 13.247 * em + 16.29 * emsq;
                g310 = -19.302 + 117.39 * em - 228.419 * emsq + 156.591 * eoc;
                g322 = -18.9068 + 109.7927 * em - 214.6334 * emsq + 146.5816 * eoc;
                g410 = -41.122 + 242.694 * em - 471.094 * emsq + 313.953 * eoc;
                g422 = -146.407 + 841.88 * em - 1629.014 * emsq + 1083.435 * eoc;
                g520 = -532.114 + 3017.977 * em - 5740.032 * emsq + 3708.276 * eoc;
            } else {
                g211 = -72.099 + 331.819 * em - 508.738 * emsq + 266.724 * eoc;
                g310 = -346.844 + 1582.851 * em - 2415.925 * emsq + 1246.113 * eoc;
                g322 = -342.585 + 1554.908 * em - 2366.899 * emsq + 1215.972 * eoc;
                g410 = -1052.797 + 4758.686 * em - 7193.992 * emsq + 3651.957 * eoc;
                g422 = -3581.69 + 16178.11 * em - 24462.77 * emsq + 12422.52 * eoc;
                if em > 0.715 {
                    g520 = -5149.66 + 29936.92 * em - 54087.36 * emsq + 31324.56 * eoc;
                } else {
                    g520 = 1464.74 - 4664.75 * em + 3763.64 * emsq;
                }
            }
            if em < 0.7 {
                g533 = -919.2277 + 4988.61 * em - 9064.77 * emsq + 5542.21 * eoc;
                g521 = -822.71072 + 4568.6173 * em - 8491.4146 * emsq + 5337.524 * eoc;
                g532 = -853.666 + 4690.25 * em - 8624.77 * emsq + 5341.4 * eoc;
            } else {
                g533 = -37995.78 + 161616.52 * em - 229838.2 * emsq + 109377.94 * eoc;
                g521 = -51752.104 + 218913.95 * em - 309468.16 * emsq + 146349.42 * eoc;
                g532 = -40023.88 + 170470.89 * em - 242699.48 * emsq + 115605.82 * eoc;
            }
            sini2 = sinim * sinim;
            f220 = 0.75 * (1.0 + 2.0 * cosim + cosisq);
            f221 = 1.5 * sini2;
            f321 = 1.875 * sinim * (1.0 - 2.0 * cosim - 3.0 * cosisq);
            f322 = -1.875 * sinim * (1.0 + 2.0 * cosim - 3.0 * cosisq);
            f441 = 35.0 * sini2 * f220;
            f442 = 39.375 * sini2 * sini2;

            f522 = 9.84375
                * sinim
                * (sini2 * (1.0 - 2.0 * cosim - 5.0 * cosisq)
                    + 0.33333333 * (-2.0 + 4.0 * cosim + 6.0 * cosisq));
            f523 = sinim
                * (4.92187512 * sini2 * (-2.0 - 4.0 * cosim + 10.0 * cosisq)
                    + 6.56250012 * (1.0 + 2.0 * cosim - 3.0 * cosisq));
            f542 = 29.53125
                * sinim
                * (2.0 - 8.0 * cosim + cosisq * (-12.0 + 8.0 * cosim + 10.0 * cosisq));
            f543 = 29.53125
                * sinim
                * (-2.0 - 8.0 * cosim + cosisq * (12.0 + 8.0 * cosim - 10.0 * cosisq));

            xno2 = nm * nm;
            ainv2 = aonv * aonv;
            temp1 = 3.0 * xno2 * ainv2;
            temp = temp1 * root22;
            d2201 = temp * f220 * g201;
            d2211 = temp * f221 * g211;
            temp1 *= aonv;
            temp = temp1 * root32;
            d3210 = temp * f321 * g310;
            d3222 = temp * f322 * g322;
            temp1 *= aonv;
            temp = 2.0 * temp1 * root44;
            d4410 = temp * f441 * g410;
            d4422 = temp * f442 * g422;
            temp1 *= aonv;
            temp = temp1 * root52;
            d5220 = temp * f522 * g520;
            d5232 = temp * f523 * g532;
            temp = 2.0 * temp1 * root54;
            d5421 = temp * f542 * g521;
            d5433 = temp * f543 * g533;
            xlamo = (mo + nodeo + nodeo - (theta + theta)) % TAU;
            xfact = mdot + dmdt + 2.0 * (nodedot + dnodt - rptim) - no;
            em = emo;
            emsq = emsqo;
        }

        //  ---------------- synchronous resonance terms --------------
        if irez == 1 {
            g200 = 1.0 + emsq * (-2.5 + 0.8125 * emsq);
            g310 = 1.0 + 2.0 * emsq;
            g300 = 1.0 + emsq * (-6.0 + 6.60937 * emsq);
            f220 = 0.75 * (1.0 + cosim) * (1.0 + cosim);
            f311 = 0.9375 * sinim * sinim * (1.0 + 3.0 * cosim) - 0.75 * (1.0 + cosim);
            f330 = 1.0 + cosim;
            // f330 *= 1.875 * f330 * f330;
            f330 = f330 * 1.875 * f330 * f330;
            del1 = 3.0 * nm * nm * aonv * aonv;
            del2 = 2.0 * del1 * f220 * g200 * q22;
            del3 = 3.0 * del1 * f330 * g300 * q33 * aonv;
            del1 = del1 * f311 * g310 * q31 * aonv;
            xlamo = (mo + nodeo + argpo - theta) % TAU;
            xfact = mdot + xpidot + dmdt + domdt + dnodt - (no + rptim);
        }

        //  ------------ for sgp4, initialize the integrator ----------
        xli = xlamo;
        xni = no;
        atime = 0.0;
        nm = no + dndt;
    }

    DsInitOutput {
        em,
        argpm,
        inclm,
        mm,
        nm,
        nodem,

        irez: irez as f64,
        atime,

        d2201,
        d2211,
        d3210,
        d3222,
        d4410,

        d4422,
        d5220,
        d5232,
        d5421,
        d5433,

        dedt,
        didt,
        dmdt,
        dndt,
        dnodt,
        domdt,

        del1,
        del2,
        del3,

        xfact,
        xlamo,
        xli,
        xni,
    }
}
