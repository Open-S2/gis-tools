use crate::{
    space::{
        Method, OperationMode,
        util::{
            constants::{J2, X2_3, XKE},
            time::gstime,
        },
    },
    util::Date,
};
use core::f64::consts::TAU;
use libm::{cos, floor, pow, round, sin, sqrt};

/// Options for Initl
pub struct InitlOptions {
    /// eccentricity of orbit
    pub ecco: f64,
    /// epoch of orbit
    pub epoch: f64,
    /// inclination of orbit
    pub inclo: f64,
    /// mean motion of orbit
    pub no: f64,
    /// satellite number
    pub opsmode: OperationMode,
}

/// Output for Initl
pub struct InitlOutput {
    /// mean motion of orbit
    pub no: f64,
    /// method
    pub method: Method,
    /// orbit period
    pub ainv: f64,
    /// semimajor axis
    pub ao: f64,
    /// con41
    pub con41: f64,
    /// con42
    pub con42: f64,
    /// cosio
    pub cosio: f64,
    /// cosio2
    pub cosio2: f64,
    /// eccentricity squared
    pub eccsq: f64,
    /// omeosq
    pub omeosq: f64,
    /// posq
    pub posq: f64,
    /// rp
    pub rp: f64,
    /// rteosq
    pub rteosq: f64,
    /// sinio
    pub sinio: f64,
    /// gsto
    pub gsto: f64,
}

/// procedure initl
///
/// this procedure initializes the sgp4 propagator. all the initialization is
/// consolidated here instead of having multiple loops inside other routines.
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
/// - `options`: initl options
///
/// ## Returns
/// Initialization params for sgp4
pub fn initl(options: InitlOptions) -> InitlOutput {
    let InitlOptions { ecco, epoch, inclo, opsmode, mut no } = options;

    // sgp4fix use old way of finding gst
    // ----------------------- earth constants ---------------------
    // sgp4fix identify constants and allow alternate values

    // ------------- calculate auxillary epoch quantities ----------
    let eccsq = ecco * ecco;
    let omeosq = 1.0 - eccsq;
    let rteosq = sqrt(omeosq);
    let cosio = cos(inclo);
    let cosio2 = cosio * cosio;

    // ------------------ un-kozai the mean motion -----------------
    // let ak = (XKE / no) ** X2_3;
    let ak = pow(XKE / no, X2_3);
    let d1 = (0.75 * J2 * (3.0 * cosio2 - 1.0)) / (rteosq * omeosq);
    let mut del_prime = d1 / (ak * ak);
    let adel = ak
        * (1.0
            - del_prime * del_prime
            - del_prime * (1.0 / 3.0 + (134.0 * del_prime * del_prime) / 81.0));
    del_prime = d1 / (adel * adel);
    no /= 1.0 + del_prime;

    // let ao = (XKE / no) ** X2_3;
    let ao = pow(XKE / no, X2_3);
    let sinio = sin(inclo);
    let po = ao * omeosq;
    let con42 = 1.0 - 5.0 * cosio2;
    let con41 = -con42 - cosio2 - cosio2;
    let ainv = 1.0 / ao;
    let posq = po * po;
    let rp = ao * (1.0 - ecco);
    let method = Method::N;

    //  sgp4fix modern approach to finding sidereal time
    let mut gsto;
    if opsmode == OperationMode::A {
        //  sgp4fix use old way of finding gst
        //  count integer number of days from 0 jan 1970
        let ts70 = epoch - 7305.0;
        let ds70 = floor(ts70 + 1.0e-8);
        let tfrac = ts70 - ds70;

        //  find greenwich location at epoch
        let c1 = 1.720_279_169_407_036_2e-2;
        let thgr70 = 1.7321343856509374;
        let fk5r = 5.075_514_194_322_695e-15;
        let c1p2p = c1 + TAU;
        gsto = (thgr70 + c1 * ds70 + c1p2p * tfrac + ts70 * ts70 * fk5r) % TAU;
        if gsto < 0.0 {
            gsto += TAU;
        }
    } else {
        gsto = gstime(&Date::from_time(round(epoch + 2433281.5) as i64));
    }

    InitlOutput {
        no,
        method,
        ainv,
        ao,
        con41,
        con42,
        cosio,
        cosio2,
        eccsq,
        omeosq,
        posq,
        rp,
        rteosq,
        sinio,
        gsto,
    }
}
