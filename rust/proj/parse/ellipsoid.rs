use crate::proj::{Proj, RA4, RA6, SIXTH};
use libm::{asin, cos, sin, sqrt, tan};

/// Builds a sphere with ellipsoid parameters
///
/// ## Parameters
/// - `proj`: an object with/wihtout sphere properties and builds the sphere
pub fn derive_sphere(proj: &mut Proj) {
    if proj.a == 0.0 {
        // do we have an ellipsoid? Then update the ellipsoid
        let ellipse = get_ellipsoid(&proj.ellps).unwrap_or(WGS84);
        proj.a = ellipse.a;
        if let Some(b) = ellipse.b {
            proj.b = b;
        }
        if let Some(rf) = ellipse.rf {
            proj.rf = rf;
        }
    }
    if proj.b == 0.0 && proj.rf != 0.0 {
        proj.b = (1. - 1. / proj.rf) * proj.a;
    }
    if proj.b != 0.0 && proj.rf == 0.0 {
        let a = proj.a;
        let b = proj.b;
        proj.rf = (a - b) / a;
    }
    let rf = proj.rf;
    let a = proj.a;
    if rf == 0.0 || (proj.b != 0.0 && (a - proj.b).abs() < f64::EPSILON) {
        proj.sphere = true;
        proj.b = proj.a;
    }

    derive_eccentricity(proj);
}

/// Derives an ellipsoid's eccentricity for an object
///
/// ## Parameters
/// - `el`: ellipsoid object to modify
pub fn derive_eccentricity(proj: &mut Proj) {
    let mut a = proj.a;
    let b = proj.b;
    let mut a2 = a * a; // used in geocentric
    let b2 = b * b; // used in geocentric
    let mut es = (a2 - b2) / a2; // e ^ 2
    let mut e = 0.0;
    if proj.sphere {
        a *= 1. - es * (SIXTH + es * (RA4 + es * RA6));
        a2 = a * a;
        es = 0.;
    } else {
        e = sqrt(es); // eccentricity
    }
    proj.a = a;
    proj.b = b;
    proj.es = es;
    proj.e = e;

    // Angular eccentricity
    let alpha = asin(e);
    proj.alpha = alpha;

    // Derived eccentricities
    proj.e2 = tan(alpha);
    proj.e2s = proj.e2 * proj.e2;

    let sin_alpha = sin(alpha);
    proj.e3 = if alpha != 0.0 { sin_alpha / sqrt(2.0 - sin_alpha * sin_alpha) } else { 0.0 };
    proj.e3s = proj.e3 * proj.e3;

    // Flattening and reciprocals
    let cos_alpha = cos(alpha);
    proj.f = 1. - cos_alpha;
    proj.rf = if proj.f != 0.0 { 1. / proj.f } else { f64::INFINITY };
    proj.f2 = if cos_alpha != 0.0 { 1. / cos_alpha - 1. } else { 0.0 };
    proj.rf2 = if proj.f2 != 0.0 { 1. / proj.f2 } else { f64::INFINITY };

    // Third flattening
    proj.n = (a - b) / (a + b);
    proj.rn = if proj.n != 0.0 { 1. / proj.n } else { f64::INFINITY };

    // Inverse semiaxes
    proj.ra = 1. / a;
    proj.rb = 1. / b;

    // One minus es and its reciprocal
    proj.one_es = 1. - es;
    if proj.one_es == 0.0 {
        // caller must handle invalid case
        return;
    }
    proj.rone_es = 1. / proj.one_es;

    // Second eccentricity squared
    proj.e2 = (a2 - b2) / b2;
}

/// ellipsoid constants
#[derive(Debug)]
pub struct Ellipsoidal {
    /// semi-major axis
    pub a: f64,
    /// semi-minor axis
    pub b: Option<f64>,
    /// inverse flattening
    pub rf: Option<f64>,
}

/// [CGCS2000 - 1024](https://epsg.org/ellipsoid_1024/CGCS2000.html)
pub const CGCS2000: Ellipsoidal = Ellipsoidal { a: 6_378_137.0, b: None, rf: Some(298.257222101) };

/// [GSK-2011 - 1025](https://epsg.org/ellipsoid_1025/GSK-2011.html)
pub const GSK_2011: Ellipsoidal = Ellipsoidal { a: 6_378_136.5, b: None, rf: Some(298.2564151) };

/// [Zach 1812 - 1026](https://epsg.org/ellipsoid_1026/Zach-1812.html)
pub const ZACH: Ellipsoidal = Ellipsoidal { a: 6_376_045.0, b: None, rf: Some(310.0) };

/// [Airy 1830 - 7001](https://epsg.org/ellipsoid_7001/Airy-1830.html)
pub const AIRY: Ellipsoidal = Ellipsoidal { a: 6_377_563.396, b: None, rf: Some(299.3249646) };

/// [Airy Modified 1849 - 7002](https://epsg.org/ellipsoid_7002/Airy-Modified-1849.html)
pub const MOD_AIRY: Ellipsoidal = Ellipsoidal { a: 6_377_340.189, b: None, rf: Some(299.3249646) };

/// [Australian National Spheroid (Australian Natl & S. Amer. 1969) - 7003](https://epsg.org/ellipsoid_7003/Australian-National-Spheroid.html)
pub const AUST_SA: Ellipsoidal = Ellipsoidal { a: 6_378_160.0, b: None, rf: Some(298.25) };

/// [Bessel 1841 - 7004](https://epsg.org/ellipsoid_7004/Bessel-1841.html)
pub const BESSEL: Ellipsoidal = Ellipsoidal { a: 6_377_397.155, b: None, rf: Some(299.1528128) };

/// [Bessel 1841 (Modified) - 7005](https://epsg.org/ellipsoid_7005/Bessel-Modified.html)
pub const MOD_BESSEL: Ellipsoidal = Ellipsoidal { a: 6_377_390.0, b: None, rf: Some(299.1528128) };

/// [Clarke 1858 - 7007](https://epsg.org/ellipsoid_7007/Clarke-1858.html)
pub const CLRK58: Ellipsoidal =
    Ellipsoidal { a: 6_378_293.645_208_759, b: Some(6_356_617.987_679_838), rf: None };

/// [Clarke 1866 - 7008](https://epsg.org/ellipsoid_7008/Clarke-1866.html)
pub const CLRK66: Ellipsoidal = Ellipsoidal { a: 6_378_206.4, b: Some(6_356_583.8), rf: None };

/// [Clarke 1880 (Benoit) - 7010](https://epsg.org/ellipsoid_7010/Clarke-1880-Benoit.html)
pub const CLRK80BEN: Ellipsoidal =
    Ellipsoidal { a: 6_378_300.789, b: Some(6_356_566.435), rf: None };

/// [Clarke 1880 (IGN) - 7011](https://epsg.org/ellipsoid_7011/Clarke-1880-IGN.html)
pub const CLRK80IGN: Ellipsoidal = Ellipsoidal { a: 6378249.2, b: Some(6356515.0), rf: None };

/// [Clarke 1880 (RGS) - 7012](https://epsg.org/ellipsoid_7012/Clarke-1880-RGS.html)
pub const CLRK80RGS: Ellipsoidal = Ellipsoidal { a: 6_378_249.145, b: None, rf: Some(293.465) };

/// [Clarke 1880 (Arc) - 7013](https://epsg.org/ellipsoid_7013/Clarke-1880-Arc.html)
pub const CLRK80ARC: Ellipsoidal = Ellipsoidal { a: 6_378_249.145, b: None, rf: Some(293.4663077) };

/// [Clarke 1880 (SGA) - 7014](https://epsg.org/ellipsoid_7014/Clarke-1880-SGA-1922.html)
pub const CLRK80SGA: Ellipsoidal = Ellipsoidal { a: 6_378_249.2, b: None, rf: Some(293.46598) };

/// [Everest 1830 (1937 Adjustment) - 7015](https://epsg.org/ellipsoid_7015/Everest-1830-1937-Adjustment.html)
pub const EVRST30: Ellipsoidal = Ellipsoidal { a: 6_377_276.345, b: None, rf: Some(300.8017) };

/// Everest (Sabah & Sarawak)
pub const EVRSTSS: Ellipsoidal = Ellipsoidal { a: 6_377_298.556, b: None, rf: Some(300.8017) };

/// [Everest 1830 (1967 Definition) - 7016](https://epsg.org/ellipsoid_7016/Everest-1830-1967-Definition.html)
pub const EVRST67: Ellipsoidal = EVRSTSS;

/// Everest 1948
pub const EVRST48: Ellipsoidal = Ellipsoidal { a: 6_377_304.063, b: None, rf: Some(300.8017) };

/// [Everest 1830 Modified - 7018](https://epsg.org/ellipsoid_7018/Everest-1830-Modified.html)
pub const EVRST30_MOD: Ellipsoidal = EVRST48;

/// [GRS 1980 (IUGG, 1980) - 7019](https://epsg.org/ellipsoid_7019/GRS-1980.html)
pub const GRS80: Ellipsoidal = Ellipsoidal { a: 6_378_137.0, b: None, rf: Some(298.257222101) };

/// [Helmert 1906 - 7020](https://epsg.org/ellipsoid_7020/Helmert-1906.html)
pub const HELMERT: Ellipsoidal = Ellipsoidal { a: 6_378_200.0, b: None, rf: Some(298.3) };

/// [Indonesian National Spheroid - 7021](https://epsg.org/ellipsoid_7021/Indonesian-National-Spheroid.html)
pub const INDONESIAN: Ellipsoidal = Ellipsoidal { a: 6_378_160.0, b: None, rf: Some(298.247) };

/// [International 1924 - 7022](https://epsg.org/ellipsoid_7022/International-1924.html)
pub const INTL24: Ellipsoidal = Ellipsoidal { a: 6_378_388.0, b: None, rf: Some(297.0) };

/// [Krassowsky 1940 - 7024](https://epsg.org/ellipsoid_7024/Krassowsky-1940.html)
pub const KRASS: Ellipsoidal = Ellipsoidal { a: 6_378_245.0, b: None, rf: Some(298.3) };

/// [Naval Weapons Lab., 1965 (NWL 9D) - 7025](https://epsg.org/ellipsoid_7025/NWL-9D.html)
pub const NWL9D: Ellipsoidal = Ellipsoidal { a: 6_378_145.0, b: None, rf: Some(298.25) };

/// Naval Weapons Lab., 1965
pub const NWL10D: Ellipsoidal = NWL9D;

/// [Plessis 1817 (France) - 7027](https://epsg.org/ellipsoid_7027/Plessis-1817.html)
pub const PLESSIS: Ellipsoidal = Ellipsoidal { a: 6_376_523.0, b: None, rf: Some(308.64) };

/// [Struve 1860 - 7028](https://epsg.org/ellipsoid_7028/Struve-1860.html)
pub const STRUVE: Ellipsoidal = Ellipsoidal { a: 6_378_298.3, b: None, rf: Some(294.73) };

/// [War Office - 7029](https://epsg.org/ellipsoid_7029/War-Office.html)
pub const WARO: Ellipsoidal = Ellipsoidal { a: 6_378_300.0, b: None, rf: Some(296.0) };

/// [WGS 84 - 7030](https://epsg.org/ellipsoid_7030/WGS-84.html)
pub const WGS84: Ellipsoidal = Ellipsoidal { a: 6_378_137.0, b: None, rf: Some(298.257223563) };

/// [GEM 10C - 7031](https://epsg.org/ellipsoid_7031/GEM-10C.html)
pub const GEM10C: Ellipsoidal = WGS84;

/// [OSU86F - 7032](https://epsg.org/ellipsoid_7032/OSU86F.html)
pub const OSU86F: Ellipsoidal = Ellipsoidal { a: 6_378_136.2, b: None, rf: Some(298.257223563) };

/// [OSU91A - 7033](https://epsg.org/ellipsoid_7033/OSU91A.html)
pub const OSU91A: Ellipsoidal = Ellipsoidal { a: 6_378_136.3, b: None, rf: Some(298.257223563) };

/// [Clarke 1880 - 7034](https://epsg.org/ellipsoid_7034/Clarke-1880.html)
pub const CLRK80: Ellipsoidal =
    Ellipsoidal { a: 6_378_249.144_808_011, b: Some(6_356_514.966204133), rf: None };

/// [GRS 1967 (IUGG 1967) - 7036](https://epsg.org/ellipsoid_7036/GRS-1967.html)
pub const GRS67: Ellipsoidal = Ellipsoidal { a: 6_378_160.0, b: None, rf: Some(298.247167427) };

/// [Average Terrestrial System 1977 - 7041](https://epsg.org/ellipsoid_7041/Average-Terrestrial-System-1977.html)
pub const ATS77: Ellipsoidal = Ellipsoidal { a: 6_378_135.0, b: None, rf: Some(298.257) };

/// [Everest 1830 (1830 Definition) - 7042](https://epsg.org/ellipsoid_7042/Everest-1830-Definition.html)
pub const EVRST1830: Ellipsoidal =
    Ellipsoidal { a: 6_377_299.365_595_443, b: Some(6_356_098.359_005_22), rf: None };

/// [WGS 72 - 7043](https://epsg.org/ellipsoid_7043/WGS-72.html)
pub const WGS7: Ellipsoidal = Ellipsoidal { a: 6_378_135.0, b: None, rf: Some(298.26) };

/// [Everest 1830 (1962 Definition) - 7044](https://epsg.org/ellipsoid_7044/Everest-1830-1962-Definition.html)
pub const EVRST62: Ellipsoidal = Ellipsoidal { a: 6_377_301.243, b: None, rf: Some(300.8017255) };

/// [Everest 1830 (1975 Definition) - 7045](https://epsg.org/ellipsoid_7045/Everest-1830-1975-Definition.html)
pub const EVRST75: Ellipsoidal = Ellipsoidal { a: 6_377_299.151, b: None, rf: Some(300.8017255) };

/// [Bessel 1841 (Namibia GLM) - 7046](https://epsg.org/ellipsoid_7046/Bessel-Namibia-GLM.html)
pub const BESS_NAM: Ellipsoidal =
    Ellipsoidal { a: 6_377_483.865280418, b: None, rf: Some(299.1528128) };

/// [GRS 1980 Authalic Sphere - 7048](https://epsg.org/ellipsoid_7048/GRS-1980-Authalic-Sphere.html)
///
/// NOTE: Not an ellipse why does this exist?
pub const GRS80_AUTH: Ellipsoidal = Ellipsoidal { a: 6_371_007.0, b: Some(6_371_007.0), rf: None };

/// [IAG 1975 - 7049](https://epsg.org/ellipsoid_7049/IAG-1975.html)
pub const IAG75: Ellipsoidal = Ellipsoidal { a: 6_378_140.0, b: None, rf: Some(298.257) };

/// [GRS 1967 Modified - 7050](https://epsg.org/ellipsoid_7050/GRS-1967-Modified.html)
pub const GRS_MOD: Ellipsoidal = Ellipsoidal { a: 6_378_160.0, b: None, rf: Some(298.25) };

/// [Danish 1876 - 7051](https://epsg.org/ellipsoid_7051/Danish-1876.html)
pub const DANISH: Ellipsoidal = Ellipsoidal { a: 6_377_019.27, b: None, rf: Some(300.0) };

/// [Clarke 1866 Authalic Sphere - 7052](https://epsg.org/ellipsoid_7052/Clarke-1866-Authalic-Sphere.html)
///
/// NOTE: Not an ellipse why does this exist?
pub const CLRK_AUTH: Ellipsoidal = Ellipsoidal { a: 6_370_997.0, b: Some(6_370_997.0), rf: None };

/// [Hough 1960 - 7053](https://epsg.org/ellipsoid_7053/Hough-1960.html)
pub const HOUGH: Ellipsoidal = Ellipsoidal { a: 6_378_270.0, b: None, rf: Some(297.0) };

/// [PZ-90 - 7054](https://epsg.org/ellipsoid_7054/PZ-90.html)
pub const PZ90: Ellipsoidal = Ellipsoidal { a: 6_378_136.0, b: None, rf: Some(298.257839303) };

/// [Clarke 1880 (international foot) - 7055](https://epsg.org/ellipsoid_7055/Clarke-1880-international-foot.html)
pub const CLRK80FOOT: Ellipsoidal =
    Ellipsoidal { a: 6_378_306.369_6, b: Some(6_356_571.996), rf: None };

/// [Everest 1830 (RSO 1969) - 7056](https://epsg.org/ellipsoid_7056/Everest-1830-RSO-1969.html)
pub const EVRST_RSO: Ellipsoidal = Ellipsoidal { a: 6_377_295.664, b: None, rf: Some(300.8017) };

/// [International 1924 Authalic Sphere - 7057](https://epsg.org/ellipsoid_7057/International-1924-Authalic-Sphere.html)
///
/// NOTE: Not an ellipse why does this exist?
pub const INTL1924_AUTH: Ellipsoidal =
    Ellipsoidal { a: 6_371_228.0, b: Some(6_371_228.0), rf: None };

/// [Hughes 1980 - 7058](https://epsg.org/ellipsoid_7058/Hughes-1980.html)
pub const HUGHES: Ellipsoidal = Ellipsoidal { a: 6_378_273.0, b: Some(6_356_889.449), rf: None };

// Everything after this I have no clue if they are accurate

/// MERIT 1983
pub const MERIT: Ellipsoidal = Ellipsoidal { a: 6_378_137.0, b: None, rf: Some(298.257) };

/// Soviet Geodetic System 85
pub const SGS85: Ellipsoidal = Ellipsoidal { a: 6_378_136.0, b: None, rf: Some(298.257) };

/// IAU 1976
pub const IAU76: Ellipsoidal =
    Ellipsoidal { a: 6_378_140.0, b: Some(6_356_755.29), rf: Some(298.257) };

/// Appl. Physics. 1965
pub const APL4: Ellipsoidal = Ellipsoidal { a: 6_378_137.0, b: None, rf: Some(298.25) };

/// Andrae 1876 (Den., Iclnd.)
pub const ANDRAE: Ellipsoidal = Ellipsoidal { a: 6_377_104.43, b: None, rf: Some(300.0) };

/// Clarke 1866 (Michigan)
pub const CLRK80MICH: Ellipsoidal =
    Ellipsoidal { a: 6_378_450.0475489, b: Some(6_356_826.62148844), rf: None };

/// Comm. des Poids et Mesures 1799
pub const CPM: Ellipsoidal = Ellipsoidal { a: 6_375_738.7, b: None, rf: Some(334.29) };

/// Delambre 1810 (Belgium)
pub const DELMBR: Ellipsoidal = Ellipsoidal { a: 6_376_428.0, b: None, rf: Some(311.5) };

/// Engelis 1985
pub const ENGELIS: Ellipsoidal = Ellipsoidal { a: 6_378_136.05, b: None, rf: Some(298.2566) };

/// Everest 1956
pub const EVRST56: Ellipsoidal = Ellipsoidal { a: 6_377_301.243, b: None, rf: Some(300.8017) };

/// Everest 1969
pub const EVRST69: Ellipsoidal = Ellipsoidal { a: 6_377_295.664, b: None, rf: Some(300.8017) };

/// Fischer (Mercury Datum) 1960
pub const FSCHR60: Ellipsoidal = Ellipsoidal { a: 6_378_166.0, b: None, rf: Some(298.3) };

/// Fischer 1960
pub const FSCHR60M: Ellipsoidal = Ellipsoidal { a: 6_378_155.0, b: None, rf: Some(298.3) };

/// Fischer 1968
pub const FSCHR68: Ellipsoidal = Ellipsoidal { a: 6_378_150.0, b: None, rf: Some(298.3) };

/// International 1909 (Hayford)
pub const INTL: Ellipsoidal = Ellipsoidal { a: 6_378_388.0, b: None, rf: Some(297.0) };

/// International 1909 (Hayford)
pub const INTL09: Ellipsoidal = INTL;

/// International 1967
pub const INTL67: Ellipsoidal = AUST_SA;

/// Kaula 1961
pub const KAULA: Ellipsoidal = Ellipsoidal { a: 6_378_163.0, b: None, rf: Some(298.24) };

/// Lerch 1979
pub const LERCH: Ellipsoidal = Ellipsoidal { a: 6_378_139.0, b: None, rf: Some(298.257) };

/// Maupertius 1738
pub const MPRTS: Ellipsoidal = Ellipsoidal { a: 6_397_300.0, b: None, rf: Some(191.0) };

/// New International 1967
pub const NEW_INTL: Ellipsoidal = Ellipsoidal { a: 6_378_157.5, b: Some(6_356_772.2), rf: None };

/// Southeast Asia
pub const SEASIA: Ellipsoidal = Ellipsoidal { a: 6_378_155.0, b: Some(6_356_773.320_5), rf: None };

/// Walbeck
pub const WALBECK: Ellipsoidal = Ellipsoidal { a: 6_376_896.0, b: Some(6_355_834.846_7), rf: None };

/// WGS 60
pub const WGS60: Ellipsoidal = Ellipsoidal { a: 6_378_165.0, b: None, rf: Some(298.3) };

/// WGS 66
pub const WGS66: Ellipsoidal = Ellipsoidal { a: 6_378_145.0, b: None, rf: Some(298.25) };

/// Normal Sphere (r=6370997)
pub const SPHERE: Ellipsoidal = Ellipsoidal { a: 6_370_997.0, b: Some(6_370_997.0), rf: None };

/// Given a name, return the corresponding ellipsoid
#[cfg_attr(feature = "nightly", coverage(off))]
pub fn get_ellipsoid_from_id(id: i64) -> Option<Ellipsoidal> {
    match id {
        1024 => Some(CGCS2000),
        1025 => Some(GSK_2011),
        1026 => Some(ZACH),
        7001 => Some(AIRY),
        7002 => Some(MOD_AIRY),
        7003 => Some(AUST_SA),
        7004 => Some(BESSEL),
        7005 => Some(MOD_BESSEL),
        7007 => Some(CLRK58),
        7008 => Some(CLRK66),
        7010 => Some(CLRK80BEN),
        7011 => Some(CLRK80IGN),
        7012 => Some(CLRK80RGS),
        7013 => Some(CLRK80ARC),
        7014 => Some(CLRK80SGA),
        7015 => Some(EVRST30),
        7016 => Some(EVRST67),
        7018 => Some(EVRST30_MOD),
        7019 => Some(GRS80),
        7020 => Some(HELMERT),
        7021 => Some(INDONESIAN),
        7022 => Some(INTL24),
        7024 => Some(KRASS),
        7025 => Some(NWL9D),
        7027 => Some(PLESSIS),
        7028 => Some(STRUVE),
        7029 => Some(WARO),
        7030 => Some(WGS84),
        7031 => Some(GEM10C),
        7032 => Some(OSU86F),
        7033 => Some(OSU91A),
        7034 => Some(CLRK80),
        7036 => Some(GRS67),
        7041 => Some(ATS77),
        7042 => Some(EVRST1830),
        7043 => Some(WGS7),
        7044 => Some(EVRST62),
        7045 => Some(EVRST75),
        7046 => Some(BESS_NAM),
        7048 => Some(GRS80_AUTH),
        7049 => Some(IAG75),
        7050 => Some(GRS_MOD),
        7051 => Some(DANISH),
        7052 => Some(CLRK_AUTH),
        7053 => Some(HOUGH),
        7054 => Some(PZ90),
        7055 => Some(CLRK80FOOT),
        7056 => Some(EVRST_RSO),
        7057 => Some(INTL1924_AUTH),
        7058 => Some(HUGHES),
        _ => None,
    }
}

/// Given a name, return the corresponding ellipsoid
#[cfg_attr(feature = "nightly", coverage(off))]
pub fn get_ellipsoid(name: &str) -> Option<Ellipsoidal> {
    // fix name to remove _ and convert to uppercase
    let name = name.to_uppercase().replace("_", "");
    match name.as_str() {
        "AIRY" => Some(AIRY),
        "APL4" => Some(APL4),
        "NWL9D" => Some(NWL9D),
        "NWL10D" => Some(NWL10D),
        "MODAIRY" => Some(MOD_AIRY),
        "ANDRAE" => Some(ANDRAE),
        "AUSTSA" => Some(AUST_SA),
        "GRS67" => Some(GRS67),
        "BESSEL" => Some(BESSEL),
        "MODBESSEL" => Some(MOD_BESSEL),
        "BESSNAM" => Some(BESS_NAM),
        "CLRK58" => Some(CLRK58),
        "CLRK66" => Some(CLRK66),
        "CLRK80" => Some(CLRK80),
        "CLRK80MICH" => Some(CLRK80MICH),
        "CLRK80BEN" => Some(CLRK80BEN),
        "CLRK80IGN" => Some(CLRK80IGN),
        "CLRK80RGS" => Some(CLRK80RGS),
        "CLRK80ARC" => Some(CLRK80ARC),
        "CLRK80SGA" => Some(CLRK80SGA),
        "CPM" => Some(CPM),
        "DELMBR" => Some(DELMBR),
        "ENGLIS" => Some(ENGELIS),
        "EVRST30" => Some(EVRST30),
        "EVRST48" => Some(EVRST48),
        "EVRST30MOD" => Some(EVRST30_MOD),
        "EVRST56" => Some(EVRST56),
        "EVRSTSS" => Some(EVRSTSS),
        "EVRST67" => Some(EVRST67),
        "EVRST69" => Some(EVRST69),
        "EVRST75" => Some(EVRST75),
        "FSCHR60" => Some(FSCHR60),
        "FSCHR60M" => Some(FSCHR60M),
        "FSCHR68" => Some(FSCHR68),
        "GEM10C" => Some(GEM10C),
        "GRS80" => Some(GRS80),
        "HELMERT" => Some(HELMERT),
        "HOUGH" => Some(HOUGH),
        "IAU76" => Some(IAU76),
        "INDONESIAN" => Some(INDONESIAN),
        "INTL" => Some(INTL),
        "INTL09" => Some(INTL09),
        "INTL24" => Some(INTL24),
        "INTL67" => Some(INTL67),
        "KAULA" => Some(KAULA),
        "LERCH" => Some(LERCH),
        "MPRTS" => Some(MPRTS),
        "NEWINTL" => Some(NEW_INTL),
        "PLESSIS" => Some(PLESSIS),
        "KRASS" => Some(KRASS),
        "SEASIA" => Some(SEASIA),
        "STRUVE" => Some(STRUVE),
        "MERIT" => Some(MERIT),
        "SGS85" => Some(SGS85),
        "OSU86F" => Some(OSU86F),
        "OSU91A" => Some(OSU91A),
        "WGS66" => Some(WGS66),
        "WGS7" => Some(WGS7),
        "WGS84" => Some(WGS84),
        "WGS60" => Some(WGS60),
        "SPHERE" => Some(SPHERE),
        "WARO" => Some(WARO),
        "WALBECK" => Some(WALBECK),
        _ => None,
    }
}
