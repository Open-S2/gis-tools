use crate::space::{OperationMode, Satellite};
use core::f64::consts::{PI, TAU};
use libm::{atan2, cos, fabs, sin};

/// Options for Dpper
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DpperOptions {
    /// true to initialize
    pub init: bool,
    /// epoch
    pub ep: f64,
    /// mean anomaly
    pub inclp: f64,
    /// right ascension of ascending node
    pub nodep: f64,
    /// argument of perigee
    pub argpp: f64,
    /// mean anomaly
    pub mp: f64,
}

/// Output for Dpper
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DpperOutput {
    /// true anomaly
    pub ep: f64,
    /// inclination
    pub inclp: f64,
    /// right ascension of ascending node
    pub nodep: f64,
    /// argument of perigee
    pub argpp: f64,
    /// mean anomaly
    pub mp: f64,
}

/// procedure dpper
///
/// this procedure provides deep space long period periodic contributions
/// to the mean elements.  by design, these periodics are zero at epoch.
/// this used to be dscom which included initialization, but it's really a
/// recurring function.
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
/// - `sat`: Satellite object
/// - `options`: dpper options
/// - `tsince`: time in minutes since epoch
///
/// ## Returns
/// Deep space long period periodic contributions
pub fn dpper(
    sat: &Satellite,
    options: DpperOptions,
    tsince: f64, // defaults to 0 (sgp4init doesn't set a time)
) -> DpperOutput {
    let Satellite {
        e3,
        ee2,
        peo,
        pgho,
        pho,
        pinco,
        plo,
        se2,
        se3,
        sgh2,
        sgh3,
        sgh4,
        sh2,
        sh3,
        si2,
        si3,
        sl2,
        sl3,
        sl4,
        xgh2,
        xgh3,
        xgh4,
        xh2,
        xh3,
        xi2,
        xi3,
        xl2,
        xl3,
        xl4,
        zmol,
        zmos,
        opsmode,
        ..
    } = sat;

    let DpperOptions { init, mut ep, mut inclp, mut nodep, mut argpp, mut mp } = options;

    // Copy satellite attributes into local variables for convenience
    // and symmetry in writing formulae.

    let mut alfdp: f64;
    let mut betdp: f64;
    let cosip: f64;
    let sinip: f64;
    let cosop: f64;
    let sinop: f64;
    let dalf: f64;
    let dbet: f64;
    let dls: f64;
    let mut f2: f64;
    let mut f3: f64;
    let mut pe: f64;
    let mut pgh: f64;
    let mut ph: f64;
    let mut pinc: f64;
    let mut pl: f64;
    let mut sinzf: f64;
    let mut xls: f64;
    let xnoh: f64;
    let mut zf: f64;

    //  ---------------------- constants -----------------------------
    let zns = 1.19459e-5;
    let zes = 0.01675;
    let znl = 1.5835218e-4;
    let zel = 0.0549;

    //  --------------- calculate time varying periodics -----------
    let mut zm = zmos + zns * tsince;

    // be sure that the initial call has time set to zero
    if init {
        zm = *zmos;
    }
    zf = zm + 2.0 * zes * sin(zm);
    sinzf = sin(zf);
    f2 = 0.5 * sinzf * sinzf - 0.25;
    f3 = -0.5 * sinzf * cos(zf);

    let ses = se2 * f2 + se3 * f3;
    let sis = si2 * f2 + si3 * f3;
    let sls = sl2 * f2 + sl3 * f3 + sl4 * sinzf;
    let sghs = sgh2 * f2 + sgh3 * f3 + sgh4 * sinzf;
    let shs = sh2 * f2 + sh3 * f3;

    zm = zmol + znl * tsince;
    if init {
        zm = *zmol;
    }

    zf = zm + 2.0 * zel * sin(zm);
    sinzf = sin(zf);
    f2 = 0.5 * sinzf * sinzf - 0.25;
    f3 = -0.5 * sinzf * cos(zf);

    let sel = ee2 * f2 + e3 * f3;
    let sil = xi2 * f2 + xi3 * f3;
    let sll = xl2 * f2 + xl3 * f3 + xl4 * sinzf;
    let sghl = xgh2 * f2 + xgh3 * f3 + xgh4 * sinzf;
    let shll = xh2 * f2 + xh3 * f3;

    pe = ses + sel;
    pinc = sis + sil;
    pl = sls + sll;
    pgh = sghs + sghl;
    ph = shs + shll;

    if !init {
        pe -= peo;
        pinc -= pinco;
        pl -= plo;
        pgh -= pgho;
        ph -= pho;
        inclp += pinc;
        ep += pe;
        sinip = sin(inclp);
        cosip = cos(inclp);

        /* ----------------- apply periodics directly ------------ */
        // sgp4fix for lyddane choice
        // strn3 used original inclination - this is technically feasible
        // gsfc used perturbed inclination - also technically feasible
        // probably best to readjust the 0.2 limit value and limit discontinuity
        // 0.2 rad = 11.45916 deg
        // use next line for original strn3 approach and original inclination
        // if (inclo >= 0.2)
        // use next line for gsfc version and perturbed inclination
        if inclp >= 0.2 {
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
            dalf = ph * cosop + pinc * cosip * sinop;
            dbet = -ph * sinop + pinc * cosip * cosop;
            alfdp += dalf;
            betdp += dbet;
            nodep %= TAU;

            //  sgp4fix for afspc written intrinsic functions
            //  nodep used without a trigonometric function ahead
            if nodep < 0.0 && *opsmode == OperationMode::A {
                nodep += TAU;
            }
            xls = mp + argpp + cosip * nodep;
            dls = pl + pgh - pinc * nodep * sinip;
            xls += dls;
            xnoh = nodep;
            nodep = atan2(alfdp, betdp);

            //  sgp4fix for afspc written intrinsic functions
            //  nodep used without a trigonometric function ahead
            if nodep < 0.0 && *opsmode == OperationMode::A {
                nodep += TAU;
            }
            if fabs(xnoh - nodep) > PI {
                if nodep < xnoh {
                    nodep += TAU;
                } else {
                    nodep -= TAU;
                }
            }
            mp += pl;
            argpp = xls - mp - cosip * nodep;
        }
    }

    DpperOutput { ep, inclp, nodep, argpp, mp }
}
