use core::f64::consts::TAU;
use libm::{cos, fabs, sin};

/// Options for Dspace
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DspaceOptions {
    /// irez
    pub irez: f64,
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
    /// del1
    pub del1: f64,
    /// del2
    pub del2: f64,
    /// del3
    pub del3: f64,
    /// didt
    pub didt: f64,
    /// dmdt
    pub dmdt: f64,
    /// dnodt
    pub dnodt: f64,
    /// domdt
    pub domdt: f64,
    /// argpo
    pub argpo: f64,
    /// argpdot
    pub argpdot: f64,
    /// tc
    pub tc: f64,
    /// gsto
    pub gsto: f64,
    /// xfact
    pub xfact: f64,
    /// xlamo
    pub xlamo: f64,
    /// no
    pub no: f64,
    /// a time
    pub atime: f64,
    /// eccentricity
    pub em: f64,
    /// argument of perigee
    pub argpm: f64,
    /// inclination
    pub inclm: f64,
    /// right ascension of ascending node
    pub xli: f64,
    /// mean anomaly
    pub mm: f64,
    /// mean motion
    pub xni: f64,
    /// right ascension of ascending node
    pub nodem: f64,
    /// rate of right ascension of ascending node
    pub nm: f64,
}

/// Output from Dspace computation
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DspaceOutput {
    /// a time
    pub atime: f64,
    /// eccentricity
    pub em: f64, // eccentricity
    /// argument of perigee
    pub argpm: f64, // argument of perigee
    /// inclination
    pub inclm: f64, // inclination
    /// right ascension of ascending node
    pub xli: f64,
    /// mean anomaly
    pub mm: f64, // mean anomaly
    /// mean motion
    pub xni: f64,
    /// right ascension of ascending node
    pub nodem: f64, // right ascension of ascending node
    /// rate of right ascension of ascending node
    pub dndt: f64,
    /// mean motion
    pub nm: f64, // mean motion
}

/// procedure dspace
///
/// this procedure provides deep space contributions to mean elements for
/// perturbing third body.  these effects have been averaged over one
/// revolution of the sun and moon.  for earth resonance effects, the
/// effects have been averaged over no revolutions of the satellite.
/// (mean motion)
///
/// author         david vallado                  719-573-2600   28 jun 2005
///
/// references
/// - hoots, roehrich, norad spacetrack report #3 1980
/// - hoots, norad spacetrack report #6 1986
/// - hoots, schumacher and glover 2004
/// - vallado, crawford, hujsak, kelso  2006
///
/// ## Parameters
/// - `options`: options explaining how to compute
/// - `tsince`: time since epoch
///
/// ## Returns
/// Computed values
pub fn dspace(options: DspaceOptions, tsince: f64) -> DspaceOutput {
    let DspaceOptions {
        irez,
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
        del1,
        del2,
        del3,
        didt,
        dmdt,
        dnodt,
        domdt,
        argpo,
        argpdot,
        tc,
        gsto,
        xfact,
        xlamo,
        no,
        mut atime,
        mut em,
        mut argpm,
        mut inclm,
        mut xli,
        mut mm,
        mut xni,
        mut nodem,
        mut nm,
    } = options;

    let fasx2 = 0.13130908;
    let fasx4 = 2.8843198;
    let fasx6 = 0.37448087;
    let g22 = 5.7686396;
    let g32 = 0.95240898;
    let g44 = 1.8014998;
    let g52 = 1.050833;
    let g54 = 4.4108898;

    // eslint-disable-next-line no-loss-of-precision
    let rptim = 4.375_269_088_011_3e-3; // equates to 7.29211514668855e-5 rad/sec
    let stepp = 720.0;
    let stepn = -720.0;
    let step2 = 259200.0;

    let delt: f64;
    let mut x2li: f64;
    let mut x2omi: f64;
    let xl: f64;
    let mut xldot = 0.0;
    let mut xnddt = 0.0;
    let mut xndt = 0.0;
    let mut xomi: f64;
    let mut dndt = 0.0;
    let mut ft = 0.0;

    //  ----------- calculate deep space resonance effects -----------
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
    //   inclm = -inclm;
    //   argpm = argpm - pi;
    //   nodem = nodem + pi;
    // }

    /* - update resonances : numerical (euler-maclaurin) integration - */
    /* ------------------------- epoch restart ----------------------  */
    //   sgp4fix for propagator problems
    //   the following integration works for negative time steps and periods
    //   the specific changes are unknown because the original code was so convoluted

    // sgp4fix take out atime = 0.0 and fix for faster operation

    if irez != 0. {
        //  sgp4fix streamline check
        if atime == 0.0 || tsince * atime <= 0.0 || fabs(tsince) < fabs(atime) {
            atime = 0.0;
            xni = no;
            xli = xlamo;
        }

        // sgp4fix move check outside loop
        if tsince > 0.0 {
            delt = stepp;
        } else {
            delt = stepn;
        }

        let mut iretn = 381; // added for do loop
        while iretn == 381 {
            //  ------------------- dot terms calculated -------------
            //  ----------- near - synchronous resonance terms -------
            if irez != 2. {
                xndt = del1 * sin(xli - fasx2)
                    + del2 * sin(2.0 * (xli - fasx4))
                    + del3 * sin(3.0 * (xli - fasx6));
                xldot = xni + xfact;
                xnddt = del1 * cos(xli - fasx2)
                    + 2.0 * del2 * cos(2.0 * (xli - fasx4))
                    + 3.0 * del3 * cos(3.0 * (xli - fasx6));
                xnddt *= xldot;
            } else {
                // --------- near - half-day resonance terms --------
                xomi = argpo + argpdot * atime;
                x2omi = xomi + xomi;
                x2li = xli + xli;
                xndt = d2201 * sin(x2omi + xli - g22)
                    + d2211 * sin(xli - g22)
                    + d3210 * sin(xomi + xli - g32)
                    + d3222 * sin(-xomi + xli - g32)
                    + d4410 * sin(x2omi + x2li - g44)
                    + d4422 * sin(x2li - g44)
                    + d5220 * sin(xomi + xli - g52)
                    + d5232 * sin(-xomi + xli - g52)
                    + d5421 * sin(xomi + x2li - g54)
                    + d5433 * sin(-xomi + x2li - g54);
                xldot = xni + xfact;
                xnddt = d2201 * cos(x2omi + xli - g22)
                    + d2211 * cos(xli - g22)
                    + d3210 * cos(xomi + xli - g32)
                    + d3222 * cos(-xomi + xli - g32)
                    + d5220 * cos(xomi + xli - g52)
                    + d5232 * cos(-xomi + xli - g52)
                    + 2.0
                        * (d4410 * cos(x2omi + x2li - g44)
                            + d4422 * cos(x2li - g44)
                            + d5421 * cos(xomi + x2li - g54)
                            + d5433 * cos(-xomi + x2li - g54));
                xnddt *= xldot;
            }

            //  ----------------------- integrator -------------------
            //  sgp4fix move end checks to end of routine
            if fabs(tsince - atime) >= stepp {
                iretn = 381;
            } else {
                ft = tsince - atime;
                iretn = 0;
            }

            if iretn == 381 {
                xli += xldot * delt + xndt * step2;
                xni += xndt * delt + xnddt * step2;
                atime += delt;
            }
        }

        nm = xni + xndt * ft + xnddt * ft * ft * 0.5;
        xl = xli + xldot * ft + xndt * ft * ft * 0.5;
        if irez != 1. {
            mm = xl - 2.0 * nodem + 2.0 * theta;
            dndt = nm - no;
        } else {
            mm = xl - nodem - argpm + theta;
            dndt = nm - no;
        }
        nm = no + dndt;
    }

    DspaceOutput { atime, em, argpm, inclm, xli, mm, xni, nodem, dndt, nm }
}
