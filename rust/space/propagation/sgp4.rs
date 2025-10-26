use crate::space::{
    EARTH_RADIUS_KM, Method, Satellite,
    propagation::{DpperOptions, DspaceOptions, dpper, dspace},
    util::constants::{J2, J3_J2, VKMPERSEC, X2_3, XKE},
};
use alloc::{format, string::String};
use core::f64::consts::{PI, TAU};
use libm::{atan2, cos, fabs, pow, sin, sqrt};
use s2json::VectorPoint;

/// An error output from an sgp4 computation
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SGP4ErrorOutput {
    /// The type of error
    pub r#type: u64,
    /// The error
    pub error: String,
}

/// A successful output from an sgp4 computation
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SGP4Output {
    /// The position of the satellite
    pub position: VectorPoint,
    /// The velocity of the satellite
    pub velocity: VectorPoint,
}

/// procedure sgp4
///
/// this procedure is the sgp4 prediction model from space command. this is an
/// updated and combined version of sgp4 and sdp4, which were originally
/// published separately in spacetrack report //3. this version follows the
/// methodology from the aiaa paper (2006) describing the history and
/// development of the code.
///
/// author         david vallado                  719-573-2600   28 jun 2005
///
/// references
/// - hoots, roehrich, norad spacetrack report //3 1980
/// - hoots, norad spacetrack report //6 1986
/// - hoots, schumacher and glover 2004
/// - vallado, crawford, hujsak, kelso  2006
///
/// ## Parameters
/// - `sat`: the satellite object to propagate
/// - `tsince`: the time since the epoch
///
/// ## Returns
/// The position and velocity of the satellite or an error report
pub fn sgp4(sat: &Satellite, tsince: f64) -> Result<SGP4Output, SGP4ErrorOutput> {
    let Satellite {
        anomaly,
        motion,
        eccentricity,
        inclination,
        method,
        drag,
        mdot,
        perigee,
        argpdot,
        ascension,
        nodedot,
        nodecf,
        cc1,
        cc4,
        cc5,
        t2cof,
        isimp,
        omgcof,
        eta,
        xmcof,
        delmo,
        d2,
        d3,
        d4,
        sinmao,
        t3cof,
        t4cof,
        t5cof,
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
        // opsmode,
        gsto,
        xfact,
        xlamo,
        atime,
        xli,
        xni,
        mut aycof,
        mut xlcof,
        mut con41,
        mut x1mth2,
        mut x7thm1,
        ..
    } = *sat;

    let mut coseo1 = 0.;
    let mut sineo1 = 0.;
    let mut cosip: f64;
    let mut sinip: f64;
    let cosisq: f64;
    let delm: f64;
    let delomg: f64;
    let mut eo1: f64;
    let mut argpm: f64;
    let mut argpp: f64;
    let mut su: f64;
    let t3: f64;
    let t4: f64;
    let tc: f64;
    let mut tem5: f64;
    let mut temp: f64;
    let mut tempa: f64;
    let mut tempe: f64;
    let mut templ: f64;
    let mut inclm: f64;
    let mut mm: f64;
    let mut nm: f64;
    let mut nodem: f64;
    let mut xincp: f64;
    let mut xlm: f64;
    let mut mp: f64;
    let mut nodep: f64;

    /* ------------------ set mathematical constants --------------- */
    // sgp4fix divisor for divide by zero check on inclination
    // the old check used 1.0 + cos(pi-1.0e-9), but then compared it to
    // 1.5 e-12, so the threshold was changed to 1.5e-12 for consistency

    let temp4 = 1.5e-12;

    //  ------- update for secular gravity and atmospheric drag -----
    let xmdf = anomaly + mdot * tsince;
    let argpdf = perigee + argpdot * tsince;
    let nodedf = ascension + nodedot * tsince;
    argpm = argpdf;
    mm = xmdf;
    let t2 = tsince * tsince;
    nodem = nodedf + nodecf * t2;
    tempa = 1.0 - cc1 * tsince;
    tempe = drag * cc4 * tsince;
    templ = t2cof * t2;

    if isimp != 1. {
        delomg = omgcof * tsince;
        //  sgp4fix use mutliply for speed instead of pow
        let delmtemp = 1.0 + eta * cos(xmdf);
        delm = xmcof * (delmtemp * delmtemp * delmtemp - delmo);
        temp = delomg + delm;
        mm = xmdf + temp;
        argpm = argpdf - temp;
        t3 = t2 * tsince;
        t4 = t3 * tsince;
        tempa = tempa - d2 * t2 - d3 * t3 - d4 * t4;
        tempe += drag * cc5 * (sin(mm) - sinmao);
        templ = templ + t3cof * t3 + t4 * (t4cof + tsince * t5cof);
    }
    nm = motion;
    let mut em = eccentricity;
    inclm = inclination;
    if method == Method::D {
        tc = tsince;

        let dspace_options = DspaceOptions {
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
            argpo: perigee,
            argpdot,
            tc,
            gsto,
            xfact,
            xlamo,
            no: motion,
            atime,
            em,
            argpm,
            inclm,
            xli,
            mm,
            xni,
            nodem,
            nm,
        };

        let dspace_result = dspace(dspace_options, tsince);
        em = dspace_result.em;
        argpm = dspace_result.argpm;
        inclm = dspace_result.inclm;
        mm = dspace_result.mm;
        nodem = dspace_result.nodem;
        nm = dspace_result.nm;
    }

    if nm <= 0.0 {
        // sgp4fix add return
        return Err(SGP4ErrorOutput { r#type: 2, error: format!("error nm {}", nm) });
    }

    // let am = (XKE / nm) ** X2_3 * tempa * tempa;
    let am = pow(XKE / nm, X2_3) * tempa * tempa;
    // nm = XKE / am ** 1.5;
    nm = XKE / pow(am, 1.5);
    em -= tempe;

    // fix tolerance for error recognition
    // sgp4fix am is fixed from the previous nm check
    if !(-0.001..1.0).contains(&em) {
        // || (am < 0.95)
        // sgp4fix to return if there is an error in eccentricity
        return Err(SGP4ErrorOutput { r#type: 1, error: format!("error em {}", em) });
    }

    //  sgp4fix fix tolerance to avoid a divide by zero
    if em < 1.0e-6 {
        em = 1.0e-6;
    }
    mm += motion * templ;
    xlm = mm + argpm + nodem;

    nodem %= TAU;
    argpm %= TAU;
    xlm %= TAU;
    mm = (xlm - argpm - nodem) % TAU;

    // ----------------- compute extra mean quantities -------------
    let sinim = sin(inclm);
    let cosim = cos(inclm);

    // -------------------- add lunar-solar periodics --------------
    let mut ep = em;
    xincp = inclm;
    argpp = argpm;
    nodep = nodem;
    mp = mm;
    sinip = sinim;
    cosip = cosim;
    if method == Method::D {
        let dpper_parameters = DpperOptions { init: false, ep, inclp: xincp, nodep, argpp, mp };
        let dpper_result = dpper(sat, dpper_parameters, tsince);
        ep = dpper_result.ep;
        nodep = dpper_result.nodep;
        argpp = dpper_result.argpp;
        mp = dpper_result.mp;

        xincp = dpper_result.inclp;

        if xincp < 0.0 {
            xincp = -xincp;
            nodep += PI;
            argpp -= PI;
        }
        if !(0.0..=1.0).contains(&ep) {
            //  sgp4fix add return
            return Err(SGP4ErrorOutput { r#type: 3, error: format!("error ep {}", ep) });
        }
    }

    //  -------------------- long period periodics ------------------
    if method == Method::D {
        sinip = sin(xincp);
        cosip = cos(xincp);
        aycof = -0.5 * J3_J2 * sinip;

        //  sgp4fix for divide by zero for xincp = 180 deg
        if fabs(cosip + 1.0) > 1.5e-12 {
            xlcof = (-0.25 * J3_J2 * sinip * (3.0 + 5.0 * cosip)) / (1.0 + cosip);
        } else {
            xlcof = (-0.25 * J3_J2 * sinip * (3.0 + 5.0 * cosip)) / temp4;
        }
    }

    let axnl = ep * cos(argpp);
    temp = 1.0 / (am * (1.0 - ep * ep));
    let aynl = ep * sin(argpp) + temp * aycof;
    let xl = mp + argpp + nodep + temp * xlcof * axnl;

    // --------------------- solve kepler's equation ---------------
    let u = (xl - nodep) % TAU;
    eo1 = u;
    tem5 = 9999.9;
    let mut ktr = 1;

    //    sgp4fix for kepler iteration
    //    the following iteration needs better limits on corrections
    while fabs(tem5) >= 1.0e-12 && ktr <= 10 {
        sineo1 = sin(eo1);
        coseo1 = cos(eo1);
        tem5 = 1.0 - coseo1 * axnl - sineo1 * aynl;
        tem5 = (u - aynl * coseo1 + axnl * sineo1 - eo1) / tem5;
        if fabs(tem5) >= 0.95 {
            if tem5 > 0.0 {
                tem5 = 0.95;
            } else {
                tem5 = -0.95;
            }
        }
        eo1 += tem5;
        ktr += 1;
    }

    //  ------------- short period preliminary quantities -----------
    let ecose = axnl * coseo1 + aynl * sineo1;
    let esine = axnl * sineo1 - aynl * coseo1;
    let el2 = axnl * axnl + aynl * aynl;
    let pl = am * (1.0 - el2);
    if pl < 0.0 {
        //  sgp4fix add return
        return Err(SGP4ErrorOutput { r#type: 4, error: format!("error pl {}", pl) });
    }

    let rl = am * (1.0 - ecose);
    let rdotl = (sqrt(am) * esine) / rl;
    let rvdotl = sqrt(pl) / rl;
    let betal = sqrt(1.0 - el2);
    temp = esine / (1.0 + betal);
    let sinu = (am / rl) * (sineo1 - aynl - axnl * temp);
    let cosu = (am / rl) * (coseo1 - axnl + aynl * temp);
    su = atan2(sinu, cosu);
    let sin2u = (cosu + cosu) * sinu;
    let cos2u = 1.0 - 2.0 * sinu * sinu;
    temp = 1.0 / pl;
    let temp1 = 0.5 * J2 * temp;
    let temp2 = temp1 * temp;

    // -------------- update for short period periodics ------------
    if method == Method::D {
        cosisq = cosip * cosip;
        con41 = 3.0 * cosisq - 1.0;
        x1mth2 = 1.0 - cosisq;
        x7thm1 = 7.0 * cosisq - 1.0;
    }

    let mrt = rl * (1.0 - 1.5 * temp2 * betal * con41) + 0.5 * temp1 * x1mth2 * cos2u;

    // sgp4fix for decaying satellites
    if mrt < 1.0 {
        return Err(SGP4ErrorOutput { r#type: 6, error: format!("decay condition {}", mrt) });
    }

    su -= 0.25 * temp2 * x7thm1 * sin2u;
    let xnode = nodep + 1.5 * temp2 * cosip * sin2u;
    let xinc = xincp + 1.5 * temp2 * cosip * sinip * cos2u;
    let mvt = rdotl - (nm * temp1 * x1mth2 * sin2u) / XKE;
    let rvdot = rvdotl + (nm * temp1 * (x1mth2 * cos2u + 1.5 * con41)) / XKE;

    // --------------------- orientation vectors -------------------
    let sinsu = sin(su);
    let cossu = cos(su);
    let snod = sin(xnode);
    let cnod = cos(xnode);
    let sini = sin(xinc);
    let cosi = cos(xinc);
    let xmx = -snod * cosi;
    let xmy = cnod * cosi;
    let ux = xmx * sinsu + cnod * cossu;
    let uy = xmy * sinsu + snod * cossu;
    let uz = sini * sinsu;
    let vx = xmx * cossu - cnod * sinsu;
    let vy = xmy * cossu - snod * sinsu;
    let vz = sini * cossu;

    // --------- position and velocity (in km and km/sec) ----------
    Ok(SGP4Output {
        position: VectorPoint::new_xyz(
            mrt * ux * EARTH_RADIUS_KM,
            mrt * uy * EARTH_RADIUS_KM,
            mrt * uz * EARTH_RADIUS_KM,
            None,
        ),
        velocity: VectorPoint::new_xyz(
            (mvt * ux + rvdot * vx) * VKMPERSEC,
            (mvt * uy + rvdot * vy) * VKMPERSEC,
            (mvt * uz + rvdot * vz) * VKMPERSEC,
            None,
        ),
    })
}
