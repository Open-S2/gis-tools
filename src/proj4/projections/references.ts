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
