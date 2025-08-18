use super::TransformCoordinates;
use crate::proj::{
    ALBERS_EQUAL_AREA, AZIMUTHAL_EQUIDISTANT, AiryProjection, AlbersConicEqualAreaProjection,
    AxisSwapConverter, AzimuthalEquidistantProjection, BONNE, BaseProjection, BonneProjection,
    CASSINI, CartesianConverter, CassiniProjection, EQUAL_EARTH, EQUIDISTANT_CONIC,
    EQUIDISTANT_CYLINDRICAL, EckertVIProjection, EqualAreaCylindricalProjection,
    EqualEarthProjection, EquidistantConicProjection, EquidistantCylindricalProjection,
    ExtendedTransverseMercatorProjection, GaussSchreiberTransverseMercatorProjection,
    GeneralSinusoidalSeriesProjection, GeocentricConverter, GeocentricLatitudeConverter,
    GnomonicProjection, GoodeHomolosineProjection, HOTINE_OBLIQUE_MERCATOR_VARIANT_A,
    HOTINE_OBLIQUE_MERCATOR_VARIANT_B, HotineObliqueMercatorVariantAProjection,
    HotineObliqueMercatorVariantBProjection, KROVAK, KROVAK_MODIFIED,
    KROVAK_MODIFIED_NORTH_ORIENTED, KROVAK_NORTH_ORIENTED, KrovakModifiedNorthOrientedProjection,
    KrovakModifiedProjection, KrovakNorthOrientedProjection, KrovakProjection, LABORDE,
    LAMBERT_AZIMUTHAL_EQUAL_AREA, LAMBERT_AZIMUTHAL_EQUAL_AREA_SPHERICAL,
    LAMBERT_CONFORMAL_CONIC_1SP, LAMBERT_CONFORMAL_CONIC_2SP, LabordeProjection,
    LambertAzimuthalEqualAreaProjection, LambertAzimuthalEqualAreaSphericalProjection,
    LambertConformalConic1SPProjection, LambertConformalConic2SPProjection,
    LambertConformalConicAlternativeProjection, LambertEqualAreaConicProjection, MERCATOR,
    McBrydeThomasFlatPolarSinusoidalProjection, MercatorProjection, Method,
    MillerCylindricalProjection, MollweideProjection, NewZealandMapGridProjection,
    OBLIQUE_STEREOGRAPHIC, ORTHOGRAPHIC, OblatedEqualAreaProjection,
    ObliqueCylindricalEqualAreaProjection, ObliqueStereographicAlternativeProjection,
    OrthographicProjection, POLAR_STEREOGRAPHIC_VARIANT_A, POLAR_STEREOGRAPHIC_VARIANT_B,
    POLAR_STEREOGRAPHIC_VARIANT_C, POLYCONIC, PolarStereographicVariantAProjection,
    PolarStereographicVariantBProjection, PolarStereographicVariantCProjection,
    PolyconicProjection, Proj, ProjectCoordinates, RobinsonProjection, SOMERC,
    SinusoidalProjection, StereographicProjection, SwissOblMercatorProjection, TRANSVERSE_MERCATOR,
    TRANSVERSE_MERCATOR_SOUTH_ORIENTATED, TransverseCentralCylindricalProjection,
    TransverseCylindricalEqualArealProjection, TransverseMercatorProjection,
    TransverseMercatorSouthOrientedProjection, UniversalTransverseMercatorProjection,
    VanDerGrintenIProjection, WEB_MERCATOR, WagnerIVProjection, WagnerVProjection,
    WebMercatorProjection,
};
use alloc::{boxed::Box, rc::Rc};
use core::cell::RefCell;

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
                return Some(Step::$variant(Box::new(<$projection>::new($proj))));
            }
        )*
        None
    }};
}

macro_rules! match_ids {
    ($id:expr, $proj:expr, [
        $( ($const_id:ident, $variant:ident, $projection:ty) ),* $(,)?
    ]) => {{
        match $id {
            $(
                $const_id => Some(Step::$variant(Box::new(<$projection>::new($proj)))),
            )*
            _ => None,
        }
    }};
}

macro_rules! dispatch_name {
    ($self:ident, [ $($variant:ident),* ]) => {
        match $self {
            $(
                Step::$variant(inner) => inner.name(),
            )*
        }
    };
}

/// Conversion/Transform/Projection step
#[derive(Debug, Clone, PartialEq)]
pub enum Step {
    // CONVERTERS
    /// Axis Swapping
    AxisSwap(Box<AxisSwapConverter>),
    /// Cartesian
    Cartesian(Box<CartesianConverter>),
    /// Geocentric Latitude
    GeoLat(Box<GeocentricLatitudeConverter>),
    /// Geocentric
    Geocentric(Box<GeocentricConverter>),

    // PROJECTIONS
    /// Albers Conic Equal Area Projection
    Aea(Box<AlbersConicEqualAreaProjection>),
    /// Azimuthal Equidistant Projection
    Aeqd(Box<AzimuthalEquidistantProjection>),
    /// Airy Projection
    Airy(Box<AiryProjection>),
    /// Base Projection
    Base(Box<BaseProjection>),
    /// BonneProjection
    Bonne(Box<BonneProjection>),
    /// Cassini Projection
    Cass(Box<CassiniProjection>),
    /// Equal Area Cylindrical Projection
    Cea(Box<EqualAreaCylindricalProjection>),
    /// Eckert VI Projection
    Eck6(Box<EckertVIProjection>),
    /// Equidistant Cylindrica Projection
    Eqc(Box<EquidistantCylindricalProjection>),
    /// Equidistant Conic rojection
    Eqdc(Box<EquidistantConicProjection>),
    /// Equal Earth Projection
    Eqearth(Box<EqualEarthProjection>),
    /// Extended Transverse Mercator Projection
    Etmerc(Box<ExtendedTransverseMercatorProjection>),
    /// General Sinusoidal Series Projection
    GnSinu(Box<GeneralSinusoidalSeriesProjection>),
    /// Gnomonic Projection
    Gnom(Box<GnomonicProjection>),
    /// Goode Homolosine Projection
    Goode(Box<GoodeHomolosineProjection>),
    /// Gauss Schreiber Transverse Mercator Projection
    Gstmerc(Box<GaussSchreiberTransverseMercatorProjection>),
    /// Hotine Oblique Mercator Variant A Projection
    HotineA(Box<HotineObliqueMercatorVariantAProjection>),
    /// Hotine Oblique Mercator Variant B Projection
    HotineB(Box<HotineObliqueMercatorVariantBProjection>),
    /// Krovak Projection
    Krovak(Box<KrovakProjection>),
    /// Krovak North Oriented Projection
    KrovakNO(Box<KrovakNorthOrientedProjection>),
    /// Krovak Modified Projection
    KrovakM(Box<KrovakModifiedProjection>),
    /// Krovak Modified North Oriented Projection
    KrovakMNO(Box<KrovakModifiedNorthOrientedProjection>),
    /// Laborde Projection
    Labrd(Box<LabordeProjection>),
    /// Lambert Azimuthal Equal Area Projection
    Laea(Box<LambertAzimuthalEqualAreaProjection>),
    /// Lambert Azimuthal Equal Area Spherical Projection
    LaeaS(Box<LambertAzimuthalEqualAreaSphericalProjection>),
    /// Lambert Equal Area Conic Projection
    Leac(Box<LambertEqualAreaConicProjection>),
    /// Lambert Conformal Conic 1SP Projection
    Lcc1SP(Box<LambertConformalConic1SPProjection>),
    /// Lambert Conformal Conic 2SP Projection
    Lcc2SP(Box<LambertConformalConic2SPProjection>),
    /// Lambert Conformal Conic Alternative Projection
    LccA(Box<LambertConformalConicAlternativeProjection>),
    /// McBryde Thomas Flat Polar Sinusoidal Projection
    MBTfps(Box<McBrydeThomasFlatPolarSinusoidalProjection>),
    /// Mercator Projection
    Merc(Box<MercatorProjection>),
    /// Miller Cylindrical Projection
    Mill(Box<MillerCylindricalProjection>),
    /// Mollweide Projection
    Moll(Box<MollweideProjection>),
    /// New Zealand Map Grid Projection
    Nzmg(Box<NewZealandMapGridProjection>),
    /// Oblique Cylindrical Equal Area Projection
    Ocea(Box<ObliqueCylindricalEqualAreaProjection>),
    /// Oblated Equal Area Projection
    Oea(Box<OblatedEqualAreaProjection>),
    /// Orthographic Projection
    Ortho(Box<OrthographicProjection>),
    /// Polar Stereographic Variant A Projection
    PSterA(Box<PolarStereographicVariantAProjection>),
    /// Polar Stereographic Variant B Projection
    PSterB(Box<PolarStereographicVariantBProjection>),
    /// Polar Stereographic Variant C Projection
    PSterC(Box<PolarStereographicVariantCProjection>),
    /// Polyconic Projection
    Poly(Box<PolyconicProjection>),
    /// Robinson Projection
    Robin(Box<RobinsonProjection>),
    /// Sinusoidal Projection
    Sinu(Box<SinusoidalProjection>),
    /// Swiss OblMercator Projection
    Somerc(Box<SwissOblMercatorProjection>),
    /// Stereographic Projection
    Stere(Box<StereographicProjection>),
    /// Oblique Stereographic Alternative Projection
    Sterea(Box<ObliqueStereographicAlternativeProjection>),
    /// Transverse CentralCylindrical Projection
    Tcc(Box<TransverseCentralCylindricalProjection>),
    /// Transverse Cylindrical Equal Areal Projection
    Tcea(Box<TransverseCylindricalEqualArealProjection>),
    /// Transverse Mercator Projection
    Tmerc(Box<TransverseMercatorProjection>),
    /// Transverse Mercator South Oriented Projection
    TmercSO(Box<TransverseMercatorSouthOrientedProjection>),
    /// Universal Transverse Mercator Projection
    Utm(Box<UniversalTransverseMercatorProjection>),
    /// Van Der Grinten (I) Projection
    Vandg(Box<VanDerGrintenIProjection>),
    /// Wagner IV Projection
    WagIV(Box<WagnerIVProjection>),
    /// Wagner V Projection
    WagV(Box<WagnerVProjection>),
    /// WebMercatorProjection
    WebMerc(Box<WebMercatorProjection>),
}
impl Step {
    /// Check if there is another step that has the same name
    pub fn same_step(&self, other: &Step) -> bool {
        self.name() == other.name()
    }
    /// Get the name of the step
    pub fn name(&self) -> &str {
        dispatch_name!(
            self,
            [
                AxisSwap, Cartesian, GeoLat, Geocentric, Aea, Aeqd, Airy, Base, Bonne, Cass, Cea,
                Eck6, Eqc, Eqdc, Eqearth, Etmerc, Gnom, GnSinu, Goode, Gstmerc, HotineA, HotineB,
                Krovak, KrovakNO, KrovakM, KrovakMNO, Labrd, Laea, LaeaS, Leac, Lcc1SP, Lcc2SP,
                LccA, MBTfps, Merc, Mill, Moll, Nzmg, Ocea, Oea, Ortho, PSterA, PSterB, PSterC,
                Poly, Robin, Sinu, Somerc, Stere, Sterea, Tcc, Tcea, Tmerc, TmercSO, Utm, Vandg,
                WagIV, WagV, WebMerc
            ]
        )
    }
    /// forward conversion
    pub fn forward<P: TransformCoordinates>(&self, point: &mut P) {
        dispatch_step!(
            self,
            point,
            forward,
            [
                AxisSwap, Cartesian, GeoLat, Geocentric, Aea, Aeqd, Airy, Base, Bonne, Cass, Cea,
                Eck6, Eqc, Eqdc, Eqearth, Etmerc, Gnom, GnSinu, Goode, Gstmerc, HotineA, HotineB,
                Krovak, KrovakNO, KrovakM, KrovakMNO, Labrd, Laea, LaeaS, Leac, Lcc1SP, Lcc2SP,
                LccA, MBTfps, Merc, Mill, Moll, Nzmg, Ocea, Oea, Ortho, PSterA, PSterB, PSterC,
                Poly, Robin, Sinu, Somerc, Stere, Sterea, Tcc, Tcea, Tmerc, TmercSO, Utm, Vandg,
                WagIV, WagV, WebMerc
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
                AxisSwap, Cartesian, GeoLat, Geocentric, Aea, Aeqd, Airy, Base, Bonne, Cass, Cea,
                Eck6, Eqc, Eqdc, Eqearth, Etmerc, Gnom, GnSinu, Goode, Gstmerc, HotineA, HotineB,
                Krovak, KrovakNO, KrovakM, KrovakMNO, Labrd, Laea, LaeaS, Leac, Lcc1SP, Lcc2SP,
                LccA, MBTfps, Merc, Mill, Moll, Nzmg, Ocea, Oea, Ortho, PSterA, PSterB, PSterC,
                Poly, Robin, Sinu, Somerc, Stere, Sterea, Tcc, Tcea, Tmerc, TmercSO, Utm, Vandg,
                WagIV, WagV, WebMerc
            ]
        );
    }
    /// Create a Step from JSON Method
    pub fn from_method(method: &Method, proj: Rc<RefCell<Proj>>) -> Option<Step> {
        // first try ID
        if let Some(id) = method.id.as_ref()
            && let Some(step) = Step::from_id(id.code.i64(), proj.clone())
        {
            return Some(step);
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
    pub fn from_id(id: i64, proj: Rc<RefCell<Proj>>) -> Option<Step> {
        if id == 0 {
            return Some(Step::Base(BaseProjection::new(proj).into()));
        }
        match_ids!(
            id,
            proj,
            [
                (ALBERS_EQUAL_AREA, Aea, AlbersConicEqualAreaProjection),
                (AZIMUTHAL_EQUIDISTANT, Aeqd, AzimuthalEquidistantProjection),
                (BONNE, Bonne, BonneProjection),
                (CASSINI, Cass, CassiniProjection),
                (EQUIDISTANT_CYLINDRICAL, Eqc, EquidistantCylindricalProjection),
                (EQUIDISTANT_CONIC, Eqdc, EquidistantConicProjection),
                (EQUAL_EARTH, Eqearth, EqualEarthProjection),
                (
                    HOTINE_OBLIQUE_MERCATOR_VARIANT_A,
                    HotineA,
                    HotineObliqueMercatorVariantAProjection
                ),
                (
                    HOTINE_OBLIQUE_MERCATOR_VARIANT_B,
                    HotineB,
                    HotineObliqueMercatorVariantBProjection
                ),
                (KROVAK, Krovak, KrovakProjection),
                (KROVAK_NORTH_ORIENTED, KrovakNO, KrovakNorthOrientedProjection),
                (KROVAK_MODIFIED, KrovakM, KrovakModifiedProjection),
                (KROVAK_MODIFIED_NORTH_ORIENTED, KrovakMNO, KrovakModifiedNorthOrientedProjection),
                (LABORDE, Labrd, LabordeProjection),
                (LAMBERT_AZIMUTHAL_EQUAL_AREA, Laea, LambertAzimuthalEqualAreaProjection),
                (
                    LAMBERT_AZIMUTHAL_EQUAL_AREA_SPHERICAL,
                    LaeaS,
                    LambertAzimuthalEqualAreaSphericalProjection
                ),
                (LAMBERT_CONFORMAL_CONIC_1SP, Lcc1SP, LambertConformalConic1SPProjection),
                (LAMBERT_CONFORMAL_CONIC_2SP, Lcc2SP, LambertConformalConic2SPProjection),
                (MERCATOR, Merc, MercatorProjection),
                (OBLIQUE_STEREOGRAPHIC, Sterea, ObliqueStereographicAlternativeProjection),
                (ORTHOGRAPHIC, Ortho, OrthographicProjection),
                (POLAR_STEREOGRAPHIC_VARIANT_A, PSterA, PolarStereographicVariantAProjection),
                (POLAR_STEREOGRAPHIC_VARIANT_B, PSterB, PolarStereographicVariantBProjection),
                (POLAR_STEREOGRAPHIC_VARIANT_C, PSterC, PolarStereographicVariantCProjection),
                (POLYCONIC, Poly, PolyconicProjection),
                (SOMERC, Somerc, SwissOblMercatorProjection),
                (TRANSVERSE_MERCATOR, Tmerc, TransverseMercatorProjection),
                (
                    TRANSVERSE_MERCATOR_SOUTH_ORIENTATED,
                    TmercSO,
                    TransverseMercatorSouthOrientedProjection
                ),
                (WEB_MERCATOR, WebMerc, WebMercatorProjection)
            ]
        )
    }
    /// Create a Step from Projection name
    pub fn from_name(name: &str, proj: Rc<RefCell<Proj>>) -> Option<Step> {
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
                (Eck6, EckertVIProjection),
                (Eqc, EquidistantCylindricalProjection),
                (Eqdc, EquidistantConicProjection),
                (Eqearth, EqualEarthProjection),
                (Etmerc, ExtendedTransverseMercatorProjection),
                (Gnom, GnomonicProjection),
                (GnSinu, GeneralSinusoidalSeriesProjection),
                (Goode, GoodeHomolosineProjection),
                (Gstmerc, GaussSchreiberTransverseMercatorProjection),
                (HotineA, HotineObliqueMercatorVariantAProjection),
                (HotineB, HotineObliqueMercatorVariantBProjection),
                (Krovak, KrovakProjection),
                (KrovakNO, KrovakNorthOrientedProjection),
                (KrovakM, KrovakModifiedProjection),
                (KrovakMNO, KrovakModifiedNorthOrientedProjection),
                (Labrd, LabordeProjection),
                (Laea, LambertAzimuthalEqualAreaProjection),
                (LaeaS, LambertAzimuthalEqualAreaSphericalProjection),
                (Leac, LambertEqualAreaConicProjection),
                (Lcc1SP, LambertConformalConic1SPProjection),
                (Lcc2SP, LambertConformalConic2SPProjection),
                (LccA, LambertConformalConicAlternativeProjection),
                (MBTfps, McBrydeThomasFlatPolarSinusoidalProjection),
                (Merc, MercatorProjection),
                (Mill, MillerCylindricalProjection),
                (Moll, MollweideProjection),
                (Nzmg, NewZealandMapGridProjection),
                (Ocea, ObliqueCylindricalEqualAreaProjection),
                (Oea, OblatedEqualAreaProjection),
                (Ortho, OrthographicProjection),
                (PSterA, PolarStereographicVariantAProjection),
                (PSterB, PolarStereographicVariantBProjection),
                (PSterC, PolarStereographicVariantCProjection),
                (Poly, PolyconicProjection),
                (Robin, RobinsonProjection),
                (Sinu, SinusoidalProjection),
                (Somerc, SwissOblMercatorProjection),
                (Stere, StereographicProjection),
                (Sterea, ObliqueStereographicAlternativeProjection),
                (Tcc, TransverseCentralCylindricalProjection),
                (Tcea, TransverseCylindricalEqualArealProjection),
                (Tmerc, TransverseMercatorProjection),
                (TmercSO, TransverseMercatorSouthOrientedProjection),
                (Utm, UniversalTransverseMercatorProjection),
                (Vandg, VanDerGrintenIProjection),
                (WagIV, WagnerIVProjection),
                (WagV, WagnerVProjection),
                (WebMerc, WebMercatorProjection),
            ]
        )
    }
}

/// Conversion trait for modifying a Point
pub trait CoordinateStep {
    /// Create a new Converter
    fn new(proj: Rc<RefCell<Proj>>) -> Self;
    /// forward conversion
    fn forward<P: TransformCoordinates>(&self, point: &mut P);
    /// inverse conversion
    fn inverse<P: TransformCoordinates>(&self, point: &mut P);
}
