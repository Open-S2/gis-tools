use crate::proj::{
    AZIMUTH_PROJECTION_CENTRE, CoordinateStep, LATITUDE_OF_FIRST_POINT, LATITUDE_OF_SECOND_POINT,
    LONGITUDE_OF_FIRST_POINT, LONGITUDE_OF_PROJECTION_CENTRE, LONGITUDE_OF_SECOND_POINT, Proj,
    ProjValue, ProjectCoordinates, TransformCoordinates,
};
use alloc::rc::Rc;
use core::{
    cell::RefCell,
    f64::consts::{FRAC_PI_2, PI},
};
use libm::{asin, atan, atan2, cos, sin, sqrt, tan};

/// Oblique Cylindrical Equal Area Variables
#[derive(Debug, Default, Clone, PartialEq)]
pub struct OceaData {
    rok: f64,
    rtk: f64,
    sinphi: f64,
    cosphi: f64,
}

/// Oblique Cylindrical Equal Area Projection
#[derive(Debug, Clone, PartialEq)]
pub struct ObliqueCylindricalEqualAreaProjection {
    proj: Rc<RefCell<Proj>>,
    store: RefCell<OceaData>,
}
impl ProjectCoordinates for ObliqueCylindricalEqualAreaProjection {
    fn code(&self) -> i64 {
        -1
    }
    fn name(&self) -> &'static str {
        "Oblique Cylindrical Equal Area"
    }
    fn names() -> &'static [&'static str] {
        &["Oblique Cylindrical Equal Area", "ocea"]
    }
}
impl CoordinateStep for ObliqueCylindricalEqualAreaProjection {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        let mut store = OceaData::default();
        {
            let proj = &mut proj.borrow_mut();

            store.rok = 1. / proj.k0;
            store.rtk = proj.k0;
            let mut lam_p;
            let phi_p;
            // If the keyword "alpha" is found in the sentence then use 1point+1azimuth
            if let Some(alpha) = proj.params.get(&AZIMUTH_PROJECTION_CENTRE) {
                // Define Pole of oblique transformation from 1 point & 1 azimuth
                // ERO: I've added PI so that the alpha is the angle from point 1 to
                // point 2 from the North in a clockwise direction (to be consistent
                // with omerc behavior)
                let alpha = PI + alpha.f64();
                let lonz = proj
                    .params
                    .get(&LONGITUDE_OF_PROJECTION_CENTRE)
                    .unwrap_or(&ProjValue::default())
                    .f64();
                // Equation 9-8 page 80 (http://pubs.usgs.gov/pp/1395/report.pdf)
                // Actually slightliy modified to use atan2(), as it is suggested by
                // Snyder for equation 9-1, but this is not mentioned here
                lam_p = atan2(-cos(alpha), -sin(proj.phi0) * sin(alpha)) + lonz;
                // Equation 9-7 page 80 (http://pubs.usgs.gov/pp/1395/report.pdf)
                phi_p = asin(cos(proj.phi0) * sin(alpha));
                // If the keyword "alpha" is NOT found in the sentence then use 2points
            } else {
                // Define Pole of oblique transformation from 2 points
                let phi_1 = proj
                    .params
                    .get(&LATITUDE_OF_FIRST_POINT)
                    .unwrap_or(&ProjValue::default())
                    .f64()
                    .to_radians();
                let phi_2 = proj
                    .params
                    .get(&LATITUDE_OF_SECOND_POINT)
                    .unwrap_or(&ProjValue::default())
                    .f64()
                    .to_radians();
                let lam_1 = proj
                    .params
                    .get(&LONGITUDE_OF_FIRST_POINT)
                    .unwrap_or(&ProjValue::default())
                    .f64()
                    .to_radians();
                let lam_2 = proj
                    .params
                    .get(&LONGITUDE_OF_SECOND_POINT)
                    .unwrap_or(&ProjValue::default())
                    .f64()
                    .to_radians();
                // Equation 9-1 page 80 (http://pubs.usgs.gov/pp/1395/report.pdf)
                lam_p = atan2(
                    cos(phi_1) * sin(phi_2) * cos(lam_1) - sin(phi_1) * cos(phi_2) * cos(lam_2),
                    sin(phi_1) * cos(phi_2) * sin(lam_2) - cos(phi_1) * sin(phi_2) * sin(lam_1),
                );

                //  take care of proj.lam0 wrap-around when +lam_1=-90
                if lam_1 == -FRAC_PI_2 {
                    lam_p = -lam_p;
                }

                // Equation 9-2 page 80 (http://pubs.usgs.gov/pp/1395/report.pdf)
                let cos_lamp_m_minus_lam_1 = cos(lam_p - lam_1);
                let tan_phi_1 = tan(phi_1);
                if tan_phi_1 == 0.0 {
                    // Not sure if we want to support this case, but at least this
                    // avoids a division by zero, and gives the same result as the below
                    // atan()
                    phi_p = if cos_lamp_m_minus_lam_1 >= 0.0 { -FRAC_PI_2 } else { FRAC_PI_2 };
                } else {
                    phi_p = atan(-cos_lamp_m_minus_lam_1 / tan_phi_1);
                }
            }
            proj.lam0 = lam_p + FRAC_PI_2;
            store.cosphi = cos(phi_p);
            store.sinphi = sin(phi_p);
            proj.es = 0.;
        }
        ObliqueCylindricalEqualAreaProjection { proj, store: store.into() }
    }
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        ocea_s_forward(&self.store.borrow(), p);
    }
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        ocea_s_inverse(&self.store.borrow(), p);
    }
}

/// Oblated Equal Area Spheroidal forward project
pub fn ocea_s_forward<P: TransformCoordinates>(ocae: &OceaData, p: &mut P) {
    let mut y = sin(p.lam());
    let t = cos(p.lam());
    let mut x = atan((tan(p.phi()) * ocae.cosphi + ocae.sinphi * y) / t);
    if t < 0. {
        x += PI;
    }
    x *= ocae.rtk;
    y = ocae.rok * (ocae.sinphi * sin(p.phi()) - ocae.cosphi * cos(p.phi()) * y);

    p.set_x(x);
    p.set_y(y);
}

/// Oblated Equal Area Spheroidal inverse project
pub fn ocea_s_inverse<P: TransformCoordinates>(ocae: &OceaData, p: &mut P) {
    let y = p.y() / ocae.rok;
    let x = p.x() / ocae.rtk;
    let t = sqrt(1. - y * y);
    let s = sin(x);
    p.set_phi(asin(y * ocae.sinphi + t * ocae.cosphi * s));
    p.set_lam(atan2(t * ocae.sinphi * s - y * ocae.cosphi, t * cos(x)));
}
