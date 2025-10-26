use crate::space::{
    EARTH_RADIUS_KM, Method, Satellite,
    propagation::{
        DpperOptions, DsInitOptions, DscomOptions, DscomOutput, InitlOptions, InitlOutput, dpper,
        dscom, dsinit, initl,
    },
    util::constants::{J2, J3_J2, J4, X2_3},
};
use core::f64::consts::PI;
use libm::{cos, fabs, pow, sin};

/// procedure sgp4init
///
/// this procedure initializes variables for sgp4.
///
/// author        david vallado                  719-573-2600   28 jun 2005
///
/// references
/// hoots, roehrich, norad spacetrack report #3 1980
/// hoots, norad spacetrack report #6 1986
/// hoots, schumacher and glover 2004
/// vallado, crawford, hujsak, kelso  2006
///
/// ## Parameters
/// - `sat`: Satellite object
pub fn sgp4init(sat: &mut Satellite) {
    let epoch = sat.jdsatepoch - 2433281.5;

    let cc1sq: f64;
    let cc2: f64;
    let mut cc3: f64;
    let coef: f64;
    let coef1: f64;
    let cosio4: f64;
    let eeta: f64;
    let etasq: f64;
    let argpm: f64;
    let nodem: f64;
    let inclm: f64;
    let mm: f64;
    let perige: f64;
    let pinvsq: f64;
    let psisq: f64;
    let mut qzms24: f64;
    let mut sfour: f64;
    let tc: f64;
    let temp: f64;
    let temp1: f64;
    let temp2: f64;
    let temp3: f64;
    let tsi: f64;
    let xpidot: f64;
    let xhdot1: f64;

    /* ------------------------ initialization --------------------- */
    // sgp4fix divisor for divide by zero check on inclination
    // the old check used 1.0 + cos(pi-1.0e-9), but then compared it to
    // 1.5 e-12, so the threshold was changed to 1.5e-12 for consistency
    let temp4 = 1.5e-12;

    // sgp4fix - note the following variables are also passed directly via sat.
    // it is possible to streamline the sgp4init call by deleting the "x"
    // variables, but the user would need to set the sat.* values first. we
    // include the additional assignments in case twoline2rv is not used.

    // ------------------------ earth constants -----------------------
    // sgp4fix identify constants and allow alternate values

    let ss = 78.0 / EARTH_RADIUS_KM + 1.0;
    // sgp4fix use multiply for speed instead of pow
    let qzms2ttemp = (120.0 - 78.0) / EARTH_RADIUS_KM;
    let qzms2t = qzms2ttemp * qzms2ttemp * qzms2ttemp * qzms2ttemp;

    sat.init = true;

    let initl_options = InitlOptions {
        // satn,
        ecco: sat.eccentricity,

        epoch,
        inclo: sat.inclination,
        no: sat.motion,

        opsmode: sat.opsmode,
    };

    let initl_result = initl(initl_options);
    let InitlOutput { ao, con42, cosio, cosio2, eccsq, omeosq, posq, rp, rteosq, sinio, .. } =
        initl_result;
    sat.motion = initl_result.no;
    sat.con41 = initl_result.con41;
    sat.gsto = initl_result.gsto;
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

    if omeosq >= 0.0 || sat.motion >= 0.0 {
        sat.isimp = 0.;
        if rp < 220.0 / EARTH_RADIUS_KM + 1.0 {
            sat.isimp = 1.;
        }
        sfour = ss;
        qzms24 = qzms2t;
        perige = (rp - 1.0) * EARTH_RADIUS_KM;

        // - for perigees below 156 km, s and qoms2t are altered -
        if perige < 156.0 {
            sfour = perige - 78.0;
            if perige < 98.0 {
                sfour = 20.0;
            }

            // sgp4fix use multiply for speed instead of pow
            let qzms24temp = (120.0 - sfour) / EARTH_RADIUS_KM;
            qzms24 = qzms24temp * qzms24temp * qzms24temp * qzms24temp;
            sfour = sfour / EARTH_RADIUS_KM + 1.0;
        }
        pinvsq = 1.0 / posq;

        tsi = 1.0 / (ao - sfour);
        sat.eta = ao * sat.eccentricity * tsi;
        etasq = sat.eta * sat.eta;
        eeta = sat.eccentricity * sat.eta;
        psisq = fabs(1.0 - etasq);
        coef = pow(qzms24 * tsi, 4.0);
        coef1 = coef / pow(psisq, 1.5);
        cc2 = coef1
            * sat.motion
            * (ao * (1.0 + 1.5 * etasq + eeta * (4.0 + etasq))
                + ((0.375 * J2 * tsi) / psisq) * sat.con41 * (8.0 + 3.0 * etasq * (8.0 + etasq)));
        sat.cc1 = sat.drag * cc2;
        cc3 = 0.0;
        if sat.eccentricity > 1.0e-4 {
            cc3 = (-2.0 * coef * tsi * J3_J2 * sat.motion * sinio) / sat.eccentricity;
        }
        sat.x1mth2 = 1.0 - cosio2;
        sat.cc4 = 2.0
            * sat.motion
            * coef1
            * ao
            * omeosq
            * (sat.eta * (2.0 + 0.5 * etasq) + sat.eccentricity * (0.5 + 2.0 * etasq)
                - ((J2 * tsi) / (ao * psisq))
                    * (-3.0 * sat.con41 * (1.0 - 2.0 * eeta + etasq * (1.5 - 0.5 * eeta))
                        + 0.75
                            * sat.x1mth2
                            * (2.0 * etasq - eeta * (1.0 + etasq))
                            * cos(2.0 * sat.perigee)));
        sat.cc5 = 2.0 * coef1 * ao * omeosq * (1.0 + 2.75 * (etasq + eeta) + eeta * etasq);
        cosio4 = cosio2 * cosio2;
        temp1 = 1.5 * J2 * pinvsq * sat.motion;
        temp2 = 0.5 * temp1 * J2 * pinvsq;
        temp3 = -0.46875 * J4 * pinvsq * pinvsq * sat.motion;
        sat.mdot = sat.motion
            + 0.5 * temp1 * rteosq * sat.con41
            + 0.0625 * temp2 * rteosq * (13.0 - 78.0 * cosio2 + 137.0 * cosio4);
        sat.argpdot = -0.5 * temp1 * con42
            + 0.0625 * temp2 * (7.0 - 114.0 * cosio2 + 395.0 * cosio4)
            + temp3 * (3.0 - 36.0 * cosio2 + 49.0 * cosio4);
        xhdot1 = -temp1 * cosio;
        sat.nodedot = xhdot1
            + (0.5 * temp2 * (4.0 - 19.0 * cosio2) + 2.0 * temp3 * (3.0 - 7.0 * cosio2)) * cosio;
        xpidot = sat.argpdot + sat.nodedot;
        sat.omgcof = sat.drag * cc3 * cos(sat.perigee);
        sat.xmcof = 0.0;
        if sat.eccentricity > 1.0e-4 {
            sat.xmcof = (-X2_3 * coef * sat.drag) / eeta;
        }
        sat.nodecf = 3.5 * omeosq * xhdot1 * sat.cc1;
        sat.t2cof = 1.5 * sat.cc1;

        // sgp4fix for divide by zero with xinco = 180 deg
        if fabs(cosio + 1.0) > 1.5e-12 {
            sat.xlcof = (-0.25 * J3_J2 * sinio * (3.0 + 5.0 * cosio)) / (1.0 + cosio);
        } else {
            sat.xlcof = (-0.25 * J3_J2 * sinio * (3.0 + 5.0 * cosio)) / temp4;
        }
        sat.aycof = -0.5 * J3_J2 * sinio;

        // sgp4fix use multiply for speed instead of pow
        let delmotemp = 1.0 + sat.eta * cos(sat.anomaly);
        sat.delmo = delmotemp * delmotemp * delmotemp;
        sat.sinmao = sin(sat.anomaly);
        sat.x7thm1 = 7.0 * cosio2 - 1.0;

        // --------------- deep space initialization -------------
        if (2. * PI) / sat.motion >= 225.0 {
            sat.method = Method::D;
            sat.isimp = 1.;
            tc = 0.0;
            inclm = sat.inclination;

            let dscom_options = DscomOptions {
                epoch,
                ep: sat.eccentricity,
                argpp: sat.perigee,
                tc,
                inclp: sat.inclination,
                nodep: sat.ascension,
                np: sat.motion,
            };

            let dscom_result = dscom(dscom_options);

            sat.e3 = dscom_result.e3;
            sat.ee2 = dscom_result.ee2;

            sat.peo = dscom_result.peo;
            sat.pgho = dscom_result.pgho;
            sat.pho = dscom_result.pho;

            sat.pinco = dscom_result.pinco;
            sat.plo = dscom_result.plo;
            sat.se2 = dscom_result.se2;
            sat.se3 = dscom_result.se3;

            sat.sgh2 = dscom_result.sgh2;
            sat.sgh3 = dscom_result.sgh3;
            sat.sgh4 = dscom_result.sgh4;
            sat.sh2 = dscom_result.sh2;
            sat.sh3 = dscom_result.sh3;

            sat.si2 = dscom_result.si2;
            sat.si3 = dscom_result.si3;
            sat.sl2 = dscom_result.sl2;
            sat.sl3 = dscom_result.sl3;
            sat.sl4 = dscom_result.sl4;

            let DscomOutput {
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
                nm,
                z1,
                z3,
                z11,
                z13,
                z21,
                z23,
                z31,
                z33,
                ..
            } = dscom_result;

            sat.xgh2 = dscom_result.xgh2;
            sat.xgh3 = dscom_result.xgh3;
            sat.xgh4 = dscom_result.xgh4;
            sat.xh2 = dscom_result.xh2;
            sat.xh3 = dscom_result.xh3;
            sat.xi2 = dscom_result.xi2;
            sat.xi3 = dscom_result.xi3;
            sat.xl2 = dscom_result.xl2;
            sat.xl3 = dscom_result.xl3;
            sat.xl4 = dscom_result.xl4;
            sat.zmol = dscom_result.zmol;
            sat.zmos = dscom_result.zmos;

            let dpper_options = DpperOptions {
                // inclo: inclm,
                init: sat.init,
                ep: sat.eccentricity,
                inclp: sat.inclination,
                nodep: sat.ascension,
                argpp: sat.perigee,
                mp: sat.anomaly,
                // opsmode: sat.opsmode,
            };

            let dpper_result = dpper(sat, dpper_options, 0.);

            sat.eccentricity = dpper_result.ep;
            sat.inclination = dpper_result.inclp;
            sat.ascension = dpper_result.nodep;
            sat.perigee = dpper_result.argpp;
            sat.anomaly = dpper_result.mp;

            argpm = 0.0;
            nodem = 0.0;
            mm = 0.0;

            let dsinit_options = DsInitOptions {
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

            let dsinit_result = dsinit(dsinit_options, 0.);

            sat.irez = dsinit_result.irez;
            sat.atime = dsinit_result.atime;
            sat.d2201 = dsinit_result.d2201;
            sat.d2211 = dsinit_result.d2211;

            sat.d3210 = dsinit_result.d3210;
            sat.d3222 = dsinit_result.d3222;
            sat.d4410 = dsinit_result.d4410;
            sat.d4422 = dsinit_result.d4422;
            sat.d5220 = dsinit_result.d5220;

            sat.d5232 = dsinit_result.d5232;
            sat.d5421 = dsinit_result.d5421;
            sat.d5433 = dsinit_result.d5433;
            sat.dedt = dsinit_result.dedt;
            sat.didt = dsinit_result.didt;

            sat.dmdt = dsinit_result.dmdt;
            sat.dnodt = dsinit_result.dnodt;
            sat.domdt = dsinit_result.domdt;
            sat.del1 = dsinit_result.del1;

            sat.del2 = dsinit_result.del2;
            sat.del3 = dsinit_result.del3;
            sat.xfact = dsinit_result.xfact;
            sat.xlamo = dsinit_result.xlamo;
            sat.xli = dsinit_result.xli;

            sat.xni = dsinit_result.xni;
        }

        // ----------- set variables if not deep space -----------
        if sat.isimp != 1. {
            cc1sq = sat.cc1 * sat.cc1;
            sat.d2 = 4.0 * ao * tsi * cc1sq;
            temp = (sat.d2 * tsi * sat.cc1) / 3.0;
            sat.d3 = (17.0 * ao + sfour) * temp;
            sat.d4 = 0.5 * temp * ao * tsi * (221.0 * ao + 31.0 * sfour) * sat.cc1;
            sat.t3cof = sat.d2 + 2.0 * cc1sq;
            sat.t4cof = 0.25 * (3.0 * sat.d3 + sat.cc1 * (12.0 * sat.d2 + 10.0 * cc1sq));
            sat.t5cof = 0.2
                * (3.0 * sat.d4
                    + 12.0 * sat.cc1 * sat.d3
                    + 6.0 * sat.d2 * sat.d2
                    + 15.0 * cc1sq * (2.0 * sat.d2 + cc1sq));
        }
    }
}
