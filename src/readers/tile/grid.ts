import type { TileID } from '../../index.js';

export interface TileGridGuide {
  /** Final grid position to start at */
  destOffsets: [x: number, y: number];
  /** Where in the tile to start */
  srcOffsets: [x: number, y: number];
  /** Write size */
  writeSize: [width: number, height: number];
  /** What tile to use */
  tile: TileID;
  /** Notify that we have a clamp */
  clamp?: boolean;
  /** Image data (added later) */
  image?: ImageData;
}

/**
 * Given the WebMercator source tile and the padding, build a grid of tiles to render
 *
 * @param tile - metadata for the position in the quad-tree
 * @param padding - amount of padding to add to the tile
 * @param size - the size of each tile (size x size)
 * @param wantedSize - the size of the rendered tile. So if the source tiles are 256x256, you can set this to 512
 * @param isTMS - if the tile scheme is TMS
 * @returns - the grid design to build/render a resultant tile
 */
export function buildTileGridWM(
  tile: TileID,
  padding: number,
  size: number,
  wantedSize: number,
  isTMS: boolean,
): TileGridGuide[] {
  if (wantedSize % size !== 0) throw new Error('wantedSize must be a multiple of size');

  const scale = wantedSize / size;
  const depthShift = Math.log2(scale);
  if (!Number.isInteger(depthShift)) throw new Error('Scale must be a power of 2');

  const { min } = Math;
  const mod = (n: number, m: number) => ((n % m) + m) % m;

  // Increment the zoom level by the size shift
  const zHi = tile.zoom + depthShift;
  const zoomTileSize = 1 << zHi;

  // In TMS, Y=0 is bottom. In WM, Y=0 is top.
  // When subdividing, the "top" index changes based on the scheme.
  const xHiStart = tile.x * scale;
  const yHiOrigin = isTMS ? tile.y * scale + (scale - 1) : tile.y * scale;

  const horizontalStrip: TileGridGuide[] = [];

  // Center tiles of the strip
  for (let i = 0; i < scale; i++) {
    horizontalStrip.push({
      destOffsets: [padding + i * size, 0], // Y adjusted later
      srcOffsets: [0, 0],
      writeSize: [size, size],
      tile: { zoom: zHi, x: mod(xHiStart + i, zoomTileSize), y: yHiOrigin },
    });
  }

  // Left padding
  let remainingLeft = padding;
  let currentXLeft = xHiStart;
  let currentOffsetLeft = padding;
  while (remainingLeft > 0) {
    currentXLeft = mod(currentXLeft - 1, zoomTileSize);
    const writeWidth = min(remainingLeft, size);
    currentOffsetLeft -= writeWidth;
    horizontalStrip.push({
      destOffsets: [currentOffsetLeft, 0],
      srcOffsets: [size - writeWidth, 0],
      writeSize: [writeWidth, size],
      tile: { zoom: zHi, x: currentXLeft, y: yHiOrigin },
    });
    remainingLeft -= writeWidth;
  }

  // Right padding
  let remainingRight = padding;
  let currentXRight = xHiStart + (scale - 1);
  let currentOffsetRight = padding + wantedSize;
  while (remainingRight > 0) {
    currentXRight = mod(currentXRight + 1, zoomTileSize);
    const writeWidth = min(remainingRight, size);
    horizontalStrip.push({
      destOffsets: [currentOffsetRight, 0],
      srcOffsets: [0, 0],
      writeSize: [writeWidth, size],
      tile: { zoom: zHi, x: currentXRight, y: yHiOrigin },
    });
    currentOffsetRight += writeWidth;
    remainingRight -= writeWidth;
  }

  // Expand the horizontal strip vertically
  const finalGrid: TileGridGuide[] = [];

  for (const h of horizontalStrip) {
    const { x: hX } = h.tile;
    const [hDestX] = h.destOffsets;
    const [hSrcX] = h.srcOffsets;
    const [hWriteW] = h.writeSize;

    // Vertical: The "Wanted" Center Rows
    for (let i = 0; i < scale; i++) {
      // In TMS, as we go "down" the screen, Y decreases.
      const currentY = isTMS ? yHiOrigin - i : yHiOrigin + i;
      finalGrid.push({
        destOffsets: [hDestX, padding + i * size],
        srcOffsets: [hSrcX, 0],
        writeSize: [hWriteW, size],
        tile: { zoom: zHi, x: hX, y: currentY },
      });
    }

    // Vertical: Padding Up
    let remainingTop = padding;
    let currentYTop = isTMS ? yHiOrigin : yHiOrigin;
    let currentOffsetTop = padding;
    while (remainingTop > 0) {
      const writeHeight = min(remainingTop, size);
      currentOffsetTop -= writeHeight;

      const nextY = isTMS ? currentYTop + 1 : currentYTop - 1;
      const isOOB = isTMS ? nextY >= zoomTileSize : nextY < 0;
      if (!isOOB) currentYTop = nextY;

      finalGrid.push({
        destOffsets: [hDestX, currentOffsetTop],
        srcOffsets: [hSrcX, isOOB ? (isTMS ? size - 1 : 0) : size - writeHeight],
        writeSize: [hWriteW, isOOB ? remainingTop : writeHeight],
        tile: { zoom: zHi, x: hX, y: currentYTop },
        clamp: isOOB,
      });
      remainingTop -= writeHeight;
      if (isOOB) remainingTop = 0;
    }

    // Vertical: Padding Down
    let remainingBottom = padding;
    // Bottom edge of the center block
    let currentYBottom = isTMS ? yHiOrigin - (scale - 1) : yHiOrigin + (scale - 1);
    let currentOffsetBottom = padding + wantedSize;
    while (remainingBottom > 0) {
      const writeHeight = min(remainingBottom, size);

      const nextY = isTMS ? currentYBottom - 1 : currentYBottom + 1;
      const isOOB = isTMS ? nextY < 0 : nextY >= zoomTileSize;
      if (!isOOB) currentYBottom = nextY;

      finalGrid.push({
        destOffsets: [hDestX, currentOffsetBottom],
        srcOffsets: [hSrcX, isOOB ? (isTMS ? 0 : size - 1) : 0],
        writeSize: [hWriteW, isOOB ? remainingBottom : writeHeight],
        tile: { zoom: zHi, x: hX, y: currentYBottom },
        clamp: isOOB,
      });
      currentOffsetBottom += writeHeight;
      remainingBottom -= writeHeight;
      if (isOOB) remainingBottom = 0;
    }
  }

  return finalGrid;
}

/**
 * Given input tile grid guide, merge the images into a single image
 *
 * @param grid - the grid guides to merge with
 * @param size - the size of the final image
 * @param padding - the amount of padding that was applied
 * @returns The merged image
 */
export function mergeTileGridWM(grid: TileGridGuide[], size: number, padding: number): ImageData {
  const destSize = size + padding * 2;
  const data = new Uint8ClampedArray(destSize * destSize * 4);

  for (const guide of grid) {
    const { destOffsets, srcOffsets, writeSize, image, clamp } = guide;
    if (image === undefined) continue;
    const sourceChannels = image.data.length / (image.width * image.height);
    const sourceAlpha = sourceChannels >= 4;
    const [width, height] = writeSize;
    const [imageX, imageY] = srcOffsets;
    for (let y = 0; y < height; y++) {
      const sourceY = clamp === true ? imageY : imageY + y;
      const targetY = destOffsets[1] + y;
      for (let x = 0; x < width; x++) {
        const sourceX = imageX + x;
        const targetX = destOffsets[0] + x;
        const sourceIndex = (sourceY * image.width + sourceX) * sourceChannels;
        const targetIndex = (targetY * destSize + targetX) * 4;
        data[targetIndex] = image.data[sourceIndex];
        data[targetIndex + 1] = image.data[sourceIndex + 1];
        data[targetIndex + 2] = image.data[sourceIndex + 2];
        data[targetIndex + 3] = sourceAlpha ? image.data[sourceIndex + 3] : 255;
      }
    }
  }

  return { colorSpace: 'srgb' as const, width: destSize, height: destSize, data };
}
