use super::TransformCoordinates;
use crate::proj::{
    ALBERS_EQUAL_AREA, AZIMUTHAL_EQUIDISTANT, AiryProjection, AlbersConicEqualAreaProjection,
    AxisSwapConverter, AzimuthalEquidistantProjection, BONNE, BaseProjection, BonneProjection,
    CASSINI, CartesianConverter, CassiniProjection, EqualAreaCylindricalProjection,
    EquidistantCylindricalProjection, GeocentricConverter, LambertEqualAreaConicProjection,
    MERCATOR, MercatorProjection, Method, Proj, ProjectCoordinates, VanDerGrintenIProjection,
    WEB_MERCATOR, WebMercatorProjection,
};
use core::cell::RefCell;

// TODO:
// EquidistantCylindricalProjection
// EquidistantConicProjection
// EqualEarthProjection
// GnomonicProjection
// GoodeHomolosineProjection
// GaussSchreiberTransverseMercatorProjection
// KrovakProjection
// KrovakNorthOrientedProjection
// KrovakModifiedProjection
// KrovakModifiedNorthOrientedProjection
// LambertAzimuthalEqualAreaProjection
// LambertAzimuthalEqualAreaSphericalProjection
// LambertConformalConic1SPProjection
// LambertConformalConic2SPProjection
// LambertConformalConicAlternativeProjection
// MillerCylindricalProjection
// MollweideProjection
// WagnerIVProjection
// WagnerVProjection
// NewZealandMapGridProjection
// ObliqueCylindricalEqualAreaProjection
// OblatedEqualAreaProjection
// HotineObliqueMercatorVariantAProjection
// HotineObliqueMercatorVariantBProjection
// OrthographicProjection
// PolyconicProjection
// RobinsonProjection
// SinusoidalProjection
// EckertVIProjection
// McBrydeThomasFlatPolarSinusoidalProjection
// GeneralSinusoidalSeriesProjection
// SwissOblMercatorProjection
// StereographicProjection
// PolarStereographicVariantAProjection
// PolarStereographicVariantBProjection
// PolarStereographicVariantCProjection
// ObliqueStereographicAlternativeProjection
// TransverseCentralCylindricalProjection
// TransverseCylindricalEqualArealProjection
// TransverseMercatorProjection
// TransverseMercatorSouthOrientedProjection
// ExtendedTransverseMercatorProjection
// UniversalTransverseMercatorProjection

macro_rules! dispatch_step {
    ($self:ident, $point:ident, $method:ident, [ $($variant:ident),* ]) => {
        match $self {
            $(
                Step::$variant(inner) => inner.$method($point),
            )*
        }
    };
}
macro_rules! match_projections {
    ($name:expr, $proj:expr, [
        $( ($variant:ident, $projection:ty) ),* $(,)?
    ]) => {{
        $(
            if <$projection>::names().contains(&$name) {
                return Some(Step::$variant(<$projection>::new($proj)));
            }
        )*
        None
    }};
}

/// Conversion/Transform/Projection step
#[derive(Debug, Clone, PartialEq)]
pub enum Step {
    // CONVERTERS
    /// Axis Swapping
    AxisSwap(AxisSwapConverter),
    /// Cartesian
    Cartesian(CartesianConverter),
    /// Geocentric
    Geocentric(GeocentricConverter),

    // PROJECTIONS
    /// Albers Conic Equal Area Projection
    Aea(AlbersConicEqualAreaProjection),
    /// Azimuthal Equidistant Projection
    Aeqd(AzimuthalEquidistantProjection),
    /// Airy Projection
    Airy(AiryProjection),
    /// Base Projection
    Base(BaseProjection),
    /// BonneProjection
    Bonne(BonneProjection),
    /// Cassini Projection
    Cass(CassiniProjection),
    /// Equal Area Cylindrical Projection
    Cea(EqualAreaCylindricalProjection),
    /// Equidistant Cylindrica Projection
    Eqc(EquidistantCylindricalProjection),
    /// Lambert Equal Area Conic Projection
    Leac(LambertEqualAreaConicProjection),
    /// MercatorProjection
    Merc(MercatorProjection),
    /// Van Der Grinten (I) Projection
    Vandg(VanDerGrintenIProjection),
    /// WebMercatorProjection
    WebMerc(WebMercatorProjection),
}
impl Step {
    /// forward conversion
    pub fn forward<P: TransformCoordinates>(&self, point: &mut P) {
        dispatch_step!(
            self,
            point,
            forward,
            [
                AxisSwap, Cartesian, Geocentric, Aea, Aeqd, Airy, Base, Bonne, Cass, Cea, Eqc,
                Leac, Merc, WebMerc, Vandg
            ]
        );
    }
    /// inverse conversion
    pub fn inverse<P: TransformCoordinates>(&self, point: &mut P) {
        dispatch_step!(
            self,
            point,
            inverse,
            [
                AxisSwap, Cartesian, Geocentric, Aea, Aeqd, Airy, Base, Bonne, Cass, Cea, Eqc,
                Leac, Merc, WebMerc, Vandg
            ]
        );
    }
    /// Create a Step from JSON Method
    pub fn from_method(method: &Method, proj: RefCell<Proj>) -> Option<Step> {
        // first try ID
        if let Some(id) = method.id.as_ref() {
            if let Some(step) = Step::from_id(id.code.i64(), proj.clone()) {
                return Some(step);
            }
        }
        // second try IDs
        for id in method.ids.iter() {
            if let Some(step) = Step::from_id(id.code.i64(), proj.clone()) {
                return Some(step);
            }
        }
        // last try name
        Step::from_name(&method.name, proj)
    }
    /// Create a Step from ID
    pub fn from_id(id: i64, proj: RefCell<Proj>) -> Option<Step> {
        match id {
            0 => Some(Step::Base(BaseProjection::new(proj))),
            ALBERS_EQUAL_AREA => Some(Step::Aea(AlbersConicEqualAreaProjection::new(proj))),
            AZIMUTHAL_EQUIDISTANT => Some(Step::Aeqd(AzimuthalEquidistantProjection::new(proj))),
            BONNE => Some(Step::Bonne(BonneProjection::new(proj))),
            CASSINI => Some(Step::Cass(CassiniProjection::new(proj))),
            MERCATOR => Some(Step::Merc(MercatorProjection::new(proj))),
            WEB_MERCATOR => Some(Step::WebMerc(WebMercatorProjection::new(proj))),
            _ => None,
        }
    }
    /// Create a Step from Projection name
    pub fn from_name(name: &str, proj: RefCell<Proj>) -> Option<Step> {
        match_projections!(
            name,
            proj,
            [
                (Aea, AlbersConicEqualAreaProjection),
                (Aeqd, AzimuthalEquidistantProjection),
                (Airy, AiryProjection),
                (Base, BaseProjection),
                (Bonne, BonneProjection),
                (Cass, CassiniProjection),
                (Cea, EqualAreaCylindricalProjection),
                (Eqc, EquidistantCylindricalProjection),
                (Leac, LambertEqualAreaConicProjection),
                (Merc, MercatorProjection),
                (Vandg, VanDerGrintenIProjection),
                (WebMerc, WebMercatorProjection),
            ]
        )
    }
}

/// Conversion trait for modifying a Point
pub trait CoordinateStep {
    /// Create a new Converter
    fn new(proj: RefCell<Proj>) -> Self;
    /// forward conversion
    fn forward<P: TransformCoordinates>(&self, point: &mut P);
    /// inverse conversion
    fn inverse<P: TransformCoordinates>(&self, point: &mut P);
}
