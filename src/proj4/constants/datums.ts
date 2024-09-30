/** Description of a WGS84 datum */
export interface ToWGS84Datum {
  datumParams: number[];
  ellipse: string;
  datumName: string;
}

/** Description of a NADGRIDS datum */
export interface NADGRIDSDatum {
  nadgrids: string[];
  ellipse: string;
  datumName: string;
}

/** WGS84 Datum */
const wgs84: ToWGS84Datum = {
  datumParams: [0, 0, 0, 0, 0, 0, 0],
  ellipse: 'WGS84',
  datumName: 'WGS84',
};

/** Swiss Datum */
const ch1903: ToWGS84Datum = {
  datumParams: [674.374, 15.056, 405.346],
  ellipse: 'bessel',
  datumName: 'swiss',
};

/** Greek_Geodetic_Reference_System_1987 Datum */
const ggrs87: ToWGS84Datum = {
  datumParams: [-199.87, 74.79, 246.62],
  ellipse: 'GRS80',
  datumName: 'Greek_Geodetic_Reference_System_1987',
};

/** North_American_Datum_1983 Datum */
const nad83: ToWGS84Datum = {
  datumParams: [0, 0, 0],
  ellipse: 'GRS80',
  datumName: 'North_American_Datum_1983',
};

/** Potsdam Rauenberg 1950 DHDN Datum */
const potsdam: ToWGS84Datum = {
  datumParams: [598.1, 73.7, 418.2, 0.202, 0.045, -2.455, 6.7],
  ellipse: 'bessel',
  datumName: 'Potsdam Rauenberg 1950 DHDN',
};

/** Carthage 1934 Tunisia Datum */
const carthage: ToWGS84Datum = {
  datumParams: [-263.0, 6.0, 431.0],
  ellipse: 'clark80',
  datumName: 'Carthage 1934 Tunisia',
};

/** Hermannskogel Datum */
const hermannskogel: ToWGS84Datum = {
  datumParams: [577.326, 90.129, 463.919, 5.137, 1.474, 5.297, 2.4232],
  ellipse: 'bessel',
  datumName: 'Hermannskogel',
};

/** Militar-Geographische Institut Datum */
const militargeographischeInstitut: ToWGS84Datum = {
  datumParams: [577.326, 90.129, 463.919, 5.137, 1.474, 5.297, 2.4232],
  ellipse: 'bessel',
  datumName: 'Militar-Geographische Institut',
};

/** Irish National Datum */
const osni52: ToWGS84Datum = {
  datumParams: [482.53, -130.596, 564.557, -1.042, -0.214, -0.631, 8.15],
  ellipse: 'airy',
  datumName: 'Irish National',
};

/** Ireland 1965 Datum */
const ire65: ToWGS84Datum = {
  datumParams: [482.53, -130.596, 564.557, -1.042, -0.214, -0.631, 8.15],
  ellipse: 'mod_airy',
  datumName: 'Ireland 1965',
};

/** Rassadiran Datum */
const rassadiran: ToWGS84Datum = {
  datumParams: [-133.63, -157.5, -158.62],
  ellipse: 'intl',
  datumName: 'Rassadiran',
};

/** New Zealand Geodetic Datum 1949 Datum */
const nzgd49: ToWGS84Datum = {
  datumParams: [59.47, -5.04, 187.44, 0.47, -0.1, 1.024, -4.5993],
  ellipse: 'intl',
  datumName: 'New Zealand Geodetic Datum 1949',
};

/** Airy 1830 Datum */
const osgb36: ToWGS84Datum = {
  datumParams: [446.448, -125.157, 542.06, 0.1502, 0.247, 0.8421, -20.4894],
  ellipse: 'airy',
  datumName: 'Airy 1830',
};

/** S-JTSK (Ferro) Datum */
const s_jtsk: ToWGS84Datum = {
  datumParams: [589, 76, 480],
  ellipse: 'bessel',
  datumName: 'S-JTSK (Ferro)',
};

/** Beduaram Datum */
const beduaram: ToWGS84Datum = {
  datumParams: [-106, -87, 188],
  ellipse: 'clrk80',
  datumName: 'Beduaram',
};

/** Gunung Segara Jakarta Datum */
const gunung_segara: ToWGS84Datum = {
  datumParams: [-403, 684, 41],
  ellipse: 'bessel',
  datumName: 'Gunung Segara Jakarta',
};

/** Reseau National Belge 1972 */
const rnb72: ToWGS84Datum = {
  datumParams: [106.869, -52.2978, 103.724, -0.33657, 0.456955, -1.84218, 1],
  ellipse: 'intl',
  datumName: 'Reseau National Belge 1972',
};

export const DATUMS: Record<string, ToWGS84Datum> = {
  wgs84,
  ch1903,
  ggrs87,
  nad83,
  rassadiran,
  nzgd49,
  osgb36,
  s_jtsk,
  beduaram,
  potsdam,
  carthage,
  hermannskogel,
  militargeographischeInstitut,
  osni52,
  ire65,
  rnb72,
  gunung_segara,
};

/** North_American_Datum_1927 Datum */
const nad27: NADGRIDSDatum = {
  nadgrids: ['@conus', '@alaska', '@ntv2_0.gsb', '@ntv1_can.dat'],
  ellipse: 'clrk66',
  datumName: 'North_American_Datum_1927',
};

export const NAD_GRIDS: Record<string, NADGRIDSDatum> = {
  nad27,
};
