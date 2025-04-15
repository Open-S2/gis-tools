/// ellipsoid constants
#[derive(Debug)]
pub struct Ellipsoid {
    /// semi-major axis
    pub a: f64,
    /// semi-minor axis
    pub b: Option<f64>,
    /// inverse flattening
    pub rf: Option<f64>,
}

/// MERIT 1983
pub const MERIT: Ellipsoid = Ellipsoid { a: 6_378_137.0, b: None, rf: Some(298.257) };

/// Soviet Geodetic System 85
pub const SGS85: Ellipsoid = Ellipsoid { a: 6_378_136.0, b: None, rf: Some(298.257) };

/// GRS 1980(IUGG, 1980)
pub const GRS80: Ellipsoid = Ellipsoid { a: 6_378_137.0, b: None, rf: Some(298.257222101) };

/// IAU 1976
pub const IAU76: Ellipsoid = Ellipsoid { a: 6_378_140.0, b: Some(6_356_755.29), rf: Some(298.257) };

/// Airy 1830
pub const AIRY: Ellipsoid = Ellipsoid { a: 6_377_563.396, b: Some(6_356_256.91), rf: None };

/// Appl. Physics. 1965
pub const APL4: Ellipsoid = Ellipsoid { a: 6_378_137.0, b: None, rf: Some(298.25) };

/// Naval Weapons Lab., 1965
pub const NWL9D: Ellipsoid = Ellipsoid { a: 6_378_145.0, b: None, rf: Some(298.25) };

/// Naval Weapons Lab., 1965
pub const NWL10D: Ellipsoid = NWL9D;

/// Modified Airy
pub const MOD_AIRY: Ellipsoid = Ellipsoid { a: 6_377_340.189, b: Some(6_356_034.446), rf: None };

/// Andrae 1876 (Den., Iclnd.)
pub const ANDRAE: Ellipsoid = Ellipsoid { a: 6_377_104.43, b: None, rf: Some(300.0) };

/// Australian Natl & S. Amer. 1969
pub const AUST_SA: Ellipsoid = Ellipsoid { a: 6_378_160.0, b: None, rf: Some(298.25) };

/// GRS 67(IUGG 1967)
pub const GRS67: Ellipsoid = Ellipsoid { a: 6_378_160.0, b: None, rf: Some(298.247167427) };

/// Bessel 1841
pub const BESSEL: Ellipsoid = Ellipsoid { a: 6_377_397.155, b: None, rf: Some(299.1528128) };

/// Bessel 1841 (Modified)
pub const MOD_BESSEL: Ellipsoid = Ellipsoid { a: 6_377_390.0, b: None, rf: Some(299.1528128) };

/// Bessel 1841 (Namibia)
pub const BESS_NAM: Ellipsoid = Ellipsoid { a: 6_377_483.865, b: None, rf: Some(299.1528128) };

/// Clarke 1858
pub const CLRK58: Ellipsoid =
    Ellipsoid { a: 6_378_293.645208759, b: None, rf: Some(294.2606763692654) };

/// Clarke 1866
pub const CLRK66: Ellipsoid = Ellipsoid { a: 6_378_206.4, b: Some(6_356_583.8), rf: None };

/// Clarke 1880 mod.
pub const CLRK80: Ellipsoid = Ellipsoid { a: 6_378_249.145, b: None, rf: Some(293.4663) };

/// Clarke 1866 (Michigan)
pub const CLRK80MICH: Ellipsoid =
    Ellipsoid { a: 6_378_450.0475489, b: Some(6_356_826.62148844), rf: None };

/// Clarke 1880 (Benoit)
pub const CLRK80BEN: Ellipsoid = Ellipsoid { a: 6378300.789, b: Some(6356566.435), rf: None };

/// Clarke 1880 (IGN)
pub const CLRK80IGN: Ellipsoid =
    Ellipsoid { a: 6378249.2, b: Some(6356515.0), rf: Some(293.4660213) };

/// Clarke 1880 (RGS)
pub const CLRK80RGS: Ellipsoid = Ellipsoid { a: 6_378_249.145, b: None, rf: Some(293.465) };

/// Clarke 1880 (Arc)
pub const CLRK80ARC: Ellipsoid = Ellipsoid { a: 6_378_249.145, b: None, rf: Some(293.4663077) };

/// Clarke 1880 (SGA)
pub const CLRK80SGA: Ellipsoid = Ellipsoid { a: 6_378_249.2, b: None, rf: Some(293.46598) };

/// Comm. des Poids et Mesures 1799
pub const CPM: Ellipsoid = Ellipsoid { a: 6_375_738.7, b: None, rf: Some(334.29) };

/// Delambre 1810 (Belgium)
pub const DELMBR: Ellipsoid = Ellipsoid { a: 6_376_428.0, b: None, rf: Some(311.5) };

/// Engelis 1985
pub const ENGELIS: Ellipsoid = Ellipsoid { a: 6_378_136.05, b: None, rf: Some(298.2566) };

/// Everest 1830 (1937 Adjustment)
pub const EVRST30: Ellipsoid = Ellipsoid { a: 6_377_276.345, b: None, rf: Some(300.8017) };

/// Everest 1948
pub const EVRST48: Ellipsoid = Ellipsoid { a: 6_377_304.063, b: None, rf: Some(300.8017) };

/// Everest 1830 Modified
pub const EVRST30_MOD: Ellipsoid = EVRST48;

/// Everest 1956
pub const EVRST56: Ellipsoid = Ellipsoid { a: 6_377_301.243, b: None, rf: Some(300.8017) };

/// Everest (Sabah & Sarawak)
pub const EVRSTSS: Ellipsoid = Ellipsoid { a: 6_377_298.556, b: None, rf: Some(300.8017) };

/// Everest 1830 (1967 Definition)
pub const EVRST67: Ellipsoid = EVRSTSS;

/// Everest 1969
pub const EVRST69: Ellipsoid = Ellipsoid { a: 6_377_295.664, b: None, rf: Some(300.8017) };

/// Everest 1830 (1975 Definition)
pub const EVRST75: Ellipsoid = Ellipsoid { a: 6_377_299.151, b: None, rf: Some(300.8017255) };

/// Fischer (Mercury Datum) 1960
pub const FSCHR60: Ellipsoid = Ellipsoid { a: 6_378_166.0, b: None, rf: Some(298.3) };

/// Fischer 1960
pub const FSCHR60M: Ellipsoid = Ellipsoid { a: 6_378_155.0, b: None, rf: Some(298.3) };

/// Fischer 1968
pub const FSCHR68: Ellipsoid = Ellipsoid { a: 6_378_150.0, b: None, rf: Some(298.3) };

/// Helmert 1906
pub const HELMERT: Ellipsoid = Ellipsoid { a: 6_378_200.0, b: None, rf: Some(298.3) };

/// Hough
pub const HOUGH: Ellipsoid = Ellipsoid { a: 6_378_270.0, b: None, rf: Some(297.0) };

/// Indonesian National Spheroid
pub const INDONESIAN: Ellipsoid = Ellipsoid { a: 6_378_160.0, b: None, rf: Some(298.247) };

/// International 1909 (Hayford)
pub const INTL: Ellipsoid = Ellipsoid { a: 6_378_388.0, b: None, rf: Some(297.0) };

/// International 1909 (Hayford)
pub const INTL09: Ellipsoid = INTL;

/// International 1924
pub const INTL24: Ellipsoid = Ellipsoid { a: 6_378_388.0, b: None, rf: Some(297.0) };

/// International 1967
pub const INTL67: Ellipsoid = AUST_SA;

/// Kaula 1961
pub const KAULA: Ellipsoid = Ellipsoid { a: 6_378_163.0, b: None, rf: Some(298.24) };

/// Lerch 1979
pub const LERCH: Ellipsoid = Ellipsoid { a: 6_378_139.0, b: None, rf: Some(298.257) };

/// Maupertius 1738
pub const MPRTS: Ellipsoid = Ellipsoid { a: 6_397_300.0, b: None, rf: Some(191.0) };

/// New International 1967
pub const NEW_INTL: Ellipsoid = Ellipsoid { a: 6_378_157.5, b: Some(6_356_772.2), rf: None };

/// Plessis 1817 (France)
pub const PLESSIS: Ellipsoid = Ellipsoid { a: 6_376_523.0, b: None, rf: Some(308.533) };

/// Krassovsky, 1942
pub const KRASS: Ellipsoid = Ellipsoid { a: 6_378_245.0, b: None, rf: Some(298.3) };

/// Southeast Asia
pub const SEASIA: Ellipsoid = Ellipsoid { a: 6_378_155.0, b: Some(6_356_773.320_5), rf: None };

/// Struve 1860
pub const STRUVE: Ellipsoid = Ellipsoid { a: 6_378_298.3, b: None, rf: Some(294.73) };

/// Walbeck
pub const WALBECK: Ellipsoid = Ellipsoid { a: 6_376_896.0, b: Some(6_355_834.846_7), rf: None };

/// War Office
pub const WARO: Ellipsoid = Ellipsoid { a: 6_378_300.0, b: None, rf: Some(296.0) };

/// WGS 60
pub const WGS60: Ellipsoid = Ellipsoid { a: 6_378_165.0, b: None, rf: Some(298.3) };

/// WGS 66
pub const WGS66: Ellipsoid = Ellipsoid { a: 6_378_145.0, b: None, rf: Some(298.25) };

/// WGS 72
pub const WGS7: Ellipsoid = Ellipsoid { a: 6_378_135.0, b: None, rf: Some(298.26) };

/// WGS 84
pub const WGS84: Ellipsoid = Ellipsoid { a: 6_378_137.0, b: None, rf: Some(298.257223563) };

/// GEM 10C
pub const GEM10C: Ellipsoid = WGS84;

/// OSU86F
pub const OSU86F: Ellipsoid = Ellipsoid { a: 6_378_136.2, b: None, rf: Some(298.257223563) };

/// OSU91A
pub const OSU91A: Ellipsoid = Ellipsoid { a: 6_378_136.3, b: None, rf: Some(298.257223563) };

/// Normal Sphere (r=6370997)
pub const SPHERE: Ellipsoid = Ellipsoid { a: 6370997.0, b: Some(6370997.0), rf: None };

/// Given a name, return the corresponding ellipsoid
pub fn get_ellipsoid(name: &str) -> Option<Ellipsoid> {
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
