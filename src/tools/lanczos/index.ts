import { copy, createImage, resize } from './util';

export * from './util';

/**
 * Lanczos filter function for downscaling
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
    resize(source, dest, use2);
    return;
  }

  const croppedSource = createImage(sw, sh);
  const croppedDest = createImage(dw, dh);
  copy(source, croppedSource, sx, sy);
  resize(croppedSource, croppedDest, use2);
  copy(croppedDest, dest, 0, 0, croppedDest.width, croppedDest.height, dx, dy);
}
