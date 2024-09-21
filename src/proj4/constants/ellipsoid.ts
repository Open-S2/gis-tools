/** ellipsoid constants */
export interface Ellipsoid {
  /** semi-major axis */
  a: number;
  /** semi-minor axis */
  b?: number;
  /** inverse flattening */
  rf?: number;
}

/** MERIT 1983 */
const MERIT: Ellipsoid = {
  a: 6_378_137.0,
  rf: 298.257,
};

/** Soviet Geodetic System 85 */
const SGS85: Ellipsoid = {
  a: 6_378_136.0,
  rf: 298.257,
};

/** GRS 1980(IUGG, 1980) */
const GRS80: Ellipsoid = {
  a: 6_378_137.0,
  rf: 298.257222101,
};

/** IAU 1976 */
const IAU76: Ellipsoid = {
  a: 6_378_140.0,
  b: 6_356_755.29,
  rf: 298.257,
};

/** Airy 1830 */
const airy: Ellipsoid = {
  a: 6_377_563.396,
  b: 6_356_256.91,
};

/** Appl. Physics. 1965 */
const APL4: Ellipsoid = {
  a: 6_378_137,
  rf: 298.25,
};

/** Naval Weapons Lab., 1965 */
const NWL9D: Ellipsoid = {
  a: 6_378_145.0,
  rf: 298.25,
};

/** Modified Airy */
const mod_airy: Ellipsoid = {
  a: 6_377_340.189,
  b: 6_356_034.446,
};

/** Andrae 1876 (Den., Iclnd.) */
const andrae: Ellipsoid = {
  a: 6_377_104.43,
  rf: 300.0,
};

/** Australian Natl & S. Amer. 1969 */
const aust_SA: Ellipsoid = {
  a: 6_378_160.0,
  rf: 298.25,
};

/** GRS 67(IUGG 1967) */
const GRS67: Ellipsoid = {
  a: 6_378_160.0,
  rf: 298.247167427,
};

/** Bessel 1841 */
const bessel: Ellipsoid = {
  a: 6_377_397.155,
  rf: 299.1528128,
};

/** Bessel 1841 (Namibia) */
const bess_nam: Ellipsoid = {
  a: 6_377_483.865,
  rf: 299.1528128,
};

/** Clarke 1866 */
const clrk66: Ellipsoid = {
  a: 6_378_206.4,
  b: 6_356_583.8,
};

/** Clarke 1880 mod. */
const clrk80: Ellipsoid = {
  a: 6_378_249.145,
  rf: 293.4663,
};

/** Clarke 1880 (IGN) */
const clrk80ign: Ellipsoid = {
  a: 6378249.2,
  b: 6356515,
  rf: 293.4660213,
};

/** Clarke 1858 */
const clrk58: Ellipsoid = {
  a: 6_378_293.645208759,
  rf: 294.2606763692654,
};

/** Comm. des Poids et Mesures 1799 */
const CPM: Ellipsoid = {
  a: 6_375_738.7,
  rf: 334.29,
};

/** Delambre 1810 (Belgium) */
const delmbr: Ellipsoid = {
  a: 6_376_428.0,
  rf: 311.5,
};

/** Engelis 1985 */
const engelis: Ellipsoid = {
  a: 6_378_136.05,
  rf: 298.2566,
};

/** Everest 1830 (Angola) */
const evrst30: Ellipsoid = {
  a: 6_377_276.345,
  rf: 300.8017,
};

/** Everest 1948 */
const evrst48: Ellipsoid = {
  a: 6_377_304.063,
  rf: 300.8017,
};

/** Everest 1956 */
const evrst56: Ellipsoid = {
  a: 6_377_301.243,
  rf: 300.8017,
};

/** Everest 1969 */
const evrst69: Ellipsoid = {
  a: 6_377_295.664,
  rf: 300.8017,
};

/** Everest (Sabah & Sarawak) */
const evrstSS: Ellipsoid = {
  a: 6_377_298.556,
  rf: 300.8017,
};

/** Fischer (Mercury Datum) 1960 */
const fschr60: Ellipsoid = {
  a: 6_378_166.0,
  rf: 298.3,
};

/** Fischer 1960 */
const fschr60m: Ellipsoid = {
  a: 6_378_155.0,
  rf: 298.3,
};

/** Fischer 1968 */
const fschr68: Ellipsoid = {
  a: 6_378_150.0,
  rf: 298.3,
};

/** Helmert 1906 */
const helmert: Ellipsoid = {
  a: 6_378_200.0,
  rf: 298.3,
};

/** Hough */
const hough: Ellipsoid = {
  a: 6_378_270.0,
  rf: 297.0,
};

/** International 1909 (Hayford) */
const intl: Ellipsoid = {
  a: 6_378_388.0,
  rf: 297.0,
};

/** Kaula 1961 */
const kaula: Ellipsoid = {
  a: 6_378_163.0,
  rf: 298.24,
};

/** Lerch 1979 */
const lerch: Ellipsoid = {
  a: 6_378_139.0,
  rf: 298.257,
};

/** Maupertius 1738 */
const mprts: Ellipsoid = {
  a: 6_397_300.0,
  rf: 191.0,
};

/** New International 1967 */
const new_intl: Ellipsoid = {
  a: 6_378_157.5,
  b: 6_356_772.2,
};

/** Plessis 1817 (France) */
const plessis: Ellipsoid = {
  a: 6_376_523.0,
  rf: 308.533,
};

/** Krassovsky, 1942 */
const krass: Ellipsoid = {
  a: 6_378_245.0,
  rf: 298.3,
};

/** Southeast Asia */
const SEasia: Ellipsoid = {
  a: 6_378_155.0,
  b: 6_356_773.3205,
};

/** Walbeck */
const walbeck: Ellipsoid = {
  a: 6_376_896.0,
  b: 6_355_834.8467,
};

/** WGS 60 */
const WGS60: Ellipsoid = {
  a: 6_378_165.0,
  rf: 298.3,
};

/** WGS 66 */
const WGS66: Ellipsoid = {
  a: 6_378_145.0,
  rf: 298.25,
};

/** WGS 72 */
const WGS7: Ellipsoid = {
  a: 6_378_135.0,
  rf: 298.26,
};

/** WGS 84 */
const WGS84: Ellipsoid = {
  a: 6_378_137.0,
  rf: 298.257223563,
};

/** Normal Sphere (r=6370997) */
const SPHERE: Ellipsoid = {
  a: 6370997.0,
  b: 6370997.0,
};

const ellipsoids: Record<string, Ellipsoid> = {
  MERIT,
  SGS85,
  GRS80,
  IAU76,
  airy,
  APL4,
  NWL9D,
  mod_airy,
  andrae,
  aust_SA,
  GRS67,
  bessel,
  bess_nam,
  clrk66,
  clrk80,
  clrk80ign,
  clrk58,
  CPM,
  delmbr,
  engelis,
  evrst30,
  evrst48,
  evrst56,
  evrst69,
  evrstSS,
  fschr60,
  fschr60m,
  fschr68,
  helmert,
  hough,
  intl,
  kaula,
  lerch,
  mprts,
  new_intl,
  plessis,
  krass,
  SEasia,
  walbeck,
  WGS60,
  WGS66,
  WGS7,
  WGS84,
  SPHERE,
};

export default ellipsoids;
