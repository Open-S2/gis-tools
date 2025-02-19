import { copyImage, createImage, resizeImage } from '..';

/**
 * # Lanczos Resampling
 *
 * ## Description
 * Perform a 2D Lanczos filter on an image. the Lanczos resampler creates some regions that are
 * darker than any in the original and others that are lighter than any in the original. This
 * Fourier transform method is useful in spatial/GIS applications, especially downscaling raster
 * data.
 *
 * ## Usage
 * ```ts
 * import sharp from 'sharp';
 * import { lanczos, createImage } from 'gis-tools-ts';
 * // pull in the image you want to downscale
 * const pattern = await sharp('./pattern.png')
 * .raw()
 * .toBuffer({ resolveWithObject: true });
 * // setup the destianation image
 * const patternHalf = createImage(4, 4);
 * // resize down into patternHalf
 * lanczos(pattern, patternHalf);
 * ```
 *
 * ## Links
 * - https://en.wikipedia.org/wiki/Lanczos_algorithm
 * - https://en.wikipedia.org/wiki/Lanczos_resampling
 * - https://gis.stackexchange.com/questions/10931/what-is-lanczos-resampling-useful-for-in-a-spatial-context
 * - https://icess.eri.ucsb.edu/gem/Duchon_1979_JAM_Lanczos.pdf
 * @param source - the source image
 * @param dest - the destination image
 * @param sx - source starting x point [Default: 0]
 * @param sy - source starting y point [Default: 0]
 * @param sw - source width to use [Default: source width - sx]
 * @param sh - source height to use [Default: source height - sy]
 * @param dx - destination starting x point [Default: 0]
 * @param dy - destination starting y point [Default: 0]
 * @param dw - destination width to use [Default: destination width - dx]
 * @param dh - destination height to use [Default: destination height - dy]
 * @param use2 - use 2nd lanczos filter instead of 3rd [Default: false]
 */
export function lanczos(
  source: ImageData,
  dest: ImageData,
  sx = 0,
  sy = 0,
  sw = source.width - sx,
  sh = source.height - sy,
  dx = 0,
  dy = 0,
  dw = dest.width - dx,
  dh = dest.height - dy,
  use2 = false,
): void {
  sx |= 0;
  sy |= 0;
  sw |= 0;
  sh |= 0;
  dx |= 0;
  dy |= 0;
  dw |= 0;
  dh |= 0;

  if (sw <= 0 || sh <= 0 || dw <= 0 || dh <= 0) return;

  if (
    sx === 0 &&
    sy === 0 &&
    sw === source.width &&
    sh === source.height &&
    dx === 0 &&
    dy === 0 &&
    dw === dest.width &&
    dh === dest.height
  ) {
    resizeImage(source, dest, use2);
    return;
  }

  const croppedSource = createImage(sw, sh);
  const croppedDest = createImage(dw, dh);
  copyImage(source, croppedSource, sx, sy);
  resizeImage(croppedSource, croppedDest, use2);
  copyImage(croppedDest, dest, 0, 0, croppedDest.width, croppedDest.height, dx, dy);
}
