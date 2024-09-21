/**
 * A collection of projection codes
 */

/**
 * **WGS84** projection
 * - WGS84
 * - EPSG:4326
 */
export const WGS84 =
  '+title=WGS 84 (long/lat) +proj=longlat +ellps=WGS84 +datum=WGS84 +units=degrees';

/**
 * **NAD83** projection
 * - NAD83
 * - EPSG:4269
 */
export const NAD83 =
  '+title=NAD83 (long/lat) +proj=longlat +a=6378137.0 +b=6356752.31414036 +ellps=GRS80 +datum=NAD83 +units=degrees';

/**
 * **PseudoMercator** projection
 * - PseudoMercator
 * - EPSG:3857
 * - EPSG:3785
 * - EPSG:900913
 * - EPSG:102113
 * - GOOGLE
 */
export const PseudoMercator =
  '+title=WGS 84 / Pseudo-Mercator +proj=merc +a=6378137 +b=6378137 +lat_ts=0.0 +lon_0=0.0 +x_0=0.0 +y_0=0 +k=1.0 +units=m +nadgrids=@null +no_defs';

/**
 * **UTM Zone 33N** projection (Northern Hemisphere)
 * - UTM Zone 33N
 * - EPSG:32633
 */
export const UTM33N = '+title=UTM Zone 33N +proj=utm +zone=33 +datum=WGS84 +units=m +no_defs';

/**
 * **UTM Zone 33S** projection (Southern Hemisphere)
 * - UTM Zone 33S
 * - EPSG:32733
 */
export const UTM33S = '+title=UTM Zone 33S +proj=utm +zone=33 +datum=WGS84 +units=m +no_defs';

/**
 * **British National Grid** projection
 * - OSGB36
 * - EPSG:27700
 */
export const BritishNationalGrid =
  '+title=OSGB 1936 / British National Grid +proj=tmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=400000 +y_0=-100000 +datum=OSGB36 +units=m +no_defs';

/**
 * **NAD27** projection
 * - NAD27
 * - EPSG:4267
 */
export const NAD27 = '+title=NAD27 (long/lat) +proj=longlat +datum=NAD27 +no_defs';

/**
 * **Lambert Conformal Conic** projection (France)
 * - Lambert 93
 * - EPSG:2154
 */
export const Lambert93 =
  '+title=RGF93 / Lambert-93 +proj=lcc +lat_1=44 +lat_2=49 +lat_0=46.5 +lon_0=3 +x_0=700000 +y_0=6600000 +datum=RGF93 +units=m +no_defs';

/**
 * **Swiss Grid** projection
 * - CH1903+
 * - EPSG:2056
 */
export const SwissGrid =
  '+title=CH1903+ / LV95 +proj=somerc +lat_0=46.95240555555556 +lon_0=7.439583333333333 +k_0=1 +x_0=2600000 +y_0=1200000 +ellps=bessel +units=m +no_defs';
