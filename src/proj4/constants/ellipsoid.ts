/** ellipsoid constants */
export interface Ellipsoid {
  /** semi-major axis */
  a: number;
  /** semi-minor axis */
  b: number;
  /** inverse flattening */
  rf: number;
}

/** MERIT 1983 */
export const MERIT: Ellipsoid = {
  a: 6_378_137.0,
  b: 6_356_752.31,
  rf: 298.257,
};

/** Soviet Geodetic System 85 */
export const SGS85: Ellipsoid = {
  a: 6_378_136.0,
  b: 6_356_751.99,
  rf: 298.257,
};

/** GRS 1980(IUGG, 1980) */
export const GRS80: Ellipsoid = {
  a: 6_378_137.0,
  b: 6_356_752.3141,
  rf: 298.257222101,
};

/** IAU 1976 */
export const IAU76: Ellipsoid = {
  a: 6_378_140.0,
  b: 6_356_755.29,
  rf: 298.257,
};

/** Airy 1830 */
export const airy: Ellipsoid = {
  a: 6_377_563.396,
  b: 6_356_256.91,
  rf: 299.325,
};

/** Appl. Physics. 1965 */
export const APL4: Ellipsoid = {
  a: 6_378_137,
  b: 6_356_752.31,
  rf: 298.25,
};

/** Naval Weapons Lab., 1965 */
export const NWL9D: Ellipsoid = {
  a: 6_378_145.0,
  b: 6_356_760.298,
  rf: 298.25,
};

/** Modified Airy */
export const mod_airy: Ellipsoid = {
  a: 6_377_340.189,
  b: 6_356_034.446,
  rf: 299.325,
};

/** Andrae 1876 (Den., Iclnd.) */
export const andrae: Ellipsoid = {
  a: 6_377_104.43,
  b: 6_356_913.06,
  rf: 300.0,
};

/** Australian Natl & S. Amer. 1969 */
export const aust_SA: Ellipsoid = {
  a: 6_378_160.0,
  b: 6_356_752.31,
  rf: 298.25,
};

/** GRS 67(IUGG 1967) */
export const GRS67: Ellipsoid = {
  a: 6_378_160.0,
  b: 6_356_750.197,
  rf: 298.247167427,
};

/** Bessel 1841 */
export const bessel: Ellipsoid = {
  a: 6_377_397.155,
  b: 6_356_078.24,
  rf: 299.1528128,
};

/** Bessel 1841 (Namibia) */
export const bess_nam: Ellipsoid = {
  a: 6_377_483.865,
  b: 6_356_292.65,
  rf: 299.1528128,
};

/** Clarke 1866 */
export const clrk66: Ellipsoid = {
  a: 6_378_206.4,
  b: 6_356_583.8,
  rf: 294.979,
};

/** Clarke 1880 mod. */
export const clrk80: Ellipsoid = {
  a: 6_378_249.145,
  b: 6_356_592.34,
  rf: 293.4663,
};

/** Clarke 1880 (IGN) */
export const clrk80ign: Ellipsoid = {
  a: 6378249.2,
  b: 6356515,
  rf: 293.4660213,
};

/** Clarke 1858 */
export const clrk58: Ellipsoid = {
  a: 6_378_293.645208759,
  b: 6_356_670.51,
  rf: 294.2606763692654,
};

/** Comm. des Poids et Mesures 1799 */
export const CPM: Ellipsoid = {
  a: 6_375_738.7,
  b: 6_356_204.15,
  rf: 334.29,
};

/** Delambre 1810 (Belgium) */
export const delmbr: Ellipsoid = {
  a: 6_376_428.0,
  b: 6_356_225.95,
  rf: 311.5,
};

/** Engelis 1985 */
export const engelis: Ellipsoid = {
  a: 6_378_136.05,
  b: 6_356_752.27,
  rf: 298.2566,
};

/** Everest 1830 (Angola) */
export const evrst30: Ellipsoid = {
  a: 6_377_276.345,
  b: 6_356_626.27,
  rf: 300.8017,
};

/** Everest 1948 */
export const evrst48: Ellipsoid = {
  a: 6_377_304.063,
  b: 6_356_622.48,
  rf: 300.8017,
};

/** Everest 1956 */
export const evrst56: Ellipsoid = {
  a: 6_377_301.243,
  b: 6_356_622.78,
  rf: 300.8017,
};

/** Everest 1969 */
export const evrst69: Ellipsoid = {
  a: 6_377_295.664,
  b: 6_356_628.28,
  rf: 300.8017,
};

/** Everest (Sabah & Sarawak) */
export const evrstSS: Ellipsoid = {
  a: 6_377_298.556,
  b: 6_356_628.91,
  rf: 300.8017,
};

/** Fischer (Mercury Datum) 1960 */
export const fschr60: Ellipsoid = {
  a: 6_378_166.0,
  b: 6_356_741.84,
  rf: 298.3,
};

/** Fischer 1960 */
export const fschr60m: Ellipsoid = {
  a: 6_378_155.0,
  b: 6_356_733.14,
  rf: 298.3,
};

/** Fischer 1968 */
export const fschr68: Ellipsoid = {
  a: 6_378_150.0,
  b: 6_356_728.78,
  rf: 298.3,
};

/** Helmert 1906 */
export const helmert: Ellipsoid = {
  a: 6_378_200.0,
  b: 6_356_730.91,
  rf: 298.3,
};

/** Hough */
export const hough: Ellipsoid = {
  a: 6_378_270.0,
  b: 6_356_735.96,
  rf: 297.0,
};

/** International 1909 (Hayford) */
export const intl: Ellipsoid = {
  a: 6_378_388.0,
  b: 6_356_847.3,
  rf: 297.0,
};

/** Kaula 1961 */
export const kaula: Ellipsoid = {
  a: 6_378_163.0,
  b: 6_356_751.95,
  rf: 298.24,
};

/** Lerch 1979 */
export const lerch: Ellipsoid = {
  a: 6_378_139.0,
  b: 6_356_751.82,
  rf: 298.257,
};

/** Maupertius 1738 */
export const mprts: Ellipsoid = {
  a: 6_397_300.0,
  b: 6_365_965.29,
  rf: 191.0,
};

/** New International 1967 */
export const new_intl: Ellipsoid = {
  a: 6_378_157.5,
  b: 6_356_772.2,
  rf: 298.255,
};

/** Plessis 1817 (France) */
export const plessis: Ellipsoid = {
  a: 6_376_523.0,
  b: 6_355_863.0,
  rf: 308.533,
};

/** Krassovsky, 1942 */
export const krass: Ellipsoid = {
  a: 6_378_245.0,
  b: 6_356_738.58,
  rf: 298.3,
};

/** Southeast Asia */
export const SEasia: Ellipsoid = {
  a: 6_378_155.0,
  b: 6_356_773.3205,
  rf: 298.257,
};

/** Walbeck */
export const walbeck: Ellipsoid = {
  a: 6_376_896.0,
  b: 6_355_834.8467,
  rf: 302.081,
};

/** WGS 60 */
export const WGS60: Ellipsoid = {
  a: 6_378_165.0,
  b: 6_356_748.16,
  rf: 298.3,
};

/** WGS 66 */
export const WGS66: Ellipsoid = {
  a: 6_378_145.0,
  b: 6_356_746.7,
  rf: 298.25,
};

/** WGS 72 */
export const WGS7: Ellipsoid = {
  a: 6_378_135.0,
  b: 6_356_756.81,
  rf: 298.26,
};

/** WGS 84 */
export const WGS84: Ellipsoid = {
  a: 6_378_137.0,
  b: 6_356_752.314,
  rf: 298.257223563,
};

/** Normal Sphere (r=6370997) */
export const SPHERE: Ellipsoid = {
  a: 6370997.0,
  b: 6370997.0,
  rf: Infinity, // Indicates a sphere with no flattening
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
