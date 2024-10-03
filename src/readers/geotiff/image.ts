import { applyPredictor } from './predictor';
import { buildTransform } from './proj';
import { getDecoder } from './decoder';
import { buildSamples, convertColorSpace } from './color';
import { needsNormalization, normalizeArray, sampleSum, toArrayType } from './imageUtil';

import type { Reader } from '..';
import type { Transformer } from '../../proj4';
import type { ArrayTypes, Decoder, GridReader, ImageFileDirectory } from '.';
import type {
  Properties,
  VectorMultiPoint,
  VectorMultiPointGeometry,
  VectorPoint,
} from '../../geometry';

/** Metadata for a GeoTIFF image */
export interface GeoTIFFMetadata {
  height: number;
  width: number;
  alpha: boolean;
}

/**
 *
 */
export interface VectorMultiPointResult {
  geometry: VectorMultiPointGeometry<RGBA>;
  width: number;
  height: number;
  alpha: boolean;
}

/** An RGBA color */
export interface RGBA extends Properties {
  r: number;
  g: number;
  b: number;
  a: number;
}

/** Raster data */
export interface Raster {
  width: number;
  height: number;
  data: ArrayTypes;
  alpha: boolean;
}

/** A tiepoint structured for decoding images */
export interface TiePoint {
  i: number;
  j: number;
  k: number;
  x: number;
  y: number;
  z: number;
}

/** A Container for a GeoTIFF image */
export class GeoTIFFImage {
  #reader: Reader;
  #imageDirectory: ImageFileDirectory;
  #littleEndian: boolean;
  #isTiled = false;
  #planarConfiguration = 1;
  #transformer: Transformer;
  /**
   * @param reader
   * @param imageDirectory
   * @param littleEndian
   * @param gridStore
   */
  constructor(
    reader: Reader,
    imageDirectory: ImageFileDirectory,
    littleEndian: boolean,
    gridStore: GridReader[],
  ) {
    this.#reader = reader;
    this.#imageDirectory = imageDirectory;
    this.#littleEndian = littleEndian;
    if (imageDirectory.StripOffsets === undefined) this.#isTiled = true;
    if (imageDirectory.PlanarConfiguration !== undefined)
      this.#planarConfiguration = imageDirectory.PlanarConfiguration;
    this.#transformer = buildTransform(this.#imageDirectory.geoKeyDirectory, gridStore);
  }

  /**
   *
   */
  get width(): number {
    return this.#imageDirectory.ImageWidth ?? 0;
  }

  /**
   *
   */
  get height(): number {
    return this.#imageDirectory.ImageLength ?? 0;
  }

  /**
   *
   */
  get tileWidth(): number {
    return this.#isTiled ? (this.#imageDirectory.TileWidth ?? 0) : this.width;
  }

  /**
   *
   */
  get tileHeight(): number {
    const { TileLength, RowsPerStrip } = this.#imageDirectory;
    return this.#isTiled ? (TileLength ?? 0) : Math.min(this.height, RowsPerStrip ?? Infinity);
  }

  /**
   *
   */
  get blockWidth(): number {
    return this.tileWidth;
  }

  /**
   * @param y
   */
  getBlockHeight(y: number): number {
    if (this.#isTiled || (y + 1) * this.tileHeight <= this.height) {
      return this.tileHeight;
    } else {
      return this.height - y * this.tileHeight;
    }
  }

  /**
   * Calculates the number of bytes for each pixel across all samples. Only full
   * bytes are supported, an exception is thrown when this is not the case.
   * @returns the bytes per pixel
   */
  get bytesPerPixel(): number {
    const bitsPerSample = this.#imageDirectory.BitsPerSample ?? [];
    let bytes = 0;
    for (let i = 0; i < bitsPerSample.length; ++i) {
      bytes += Math.ceil(bitsPerSample[i] / 8);
    }
    return bytes;
  }

  /**
   * Returns the number of samples per pixel.
   * @returns the number of samples per pixel
   */
  get samplesPerPixel() {
    const { SamplesPerPixel } = this.#imageDirectory;
    return SamplesPerPixel !== undefined ? SamplesPerPixel : 1;
  }

  /**
   * @param sampleIndex
   */
  getSampleFormat(sampleIndex = 0): number {
    const { SampleFormat } = this.#imageDirectory;
    return Array.isArray(SampleFormat) ? SampleFormat[sampleIndex] : 1;
  }

  /**
   * @param sampleIndex
   */
  getBitsPerSample(sampleIndex = 0): number {
    const { BitsPerSample } = this.#imageDirectory;
    return (BitsPerSample ?? [])[sampleIndex];
  }

  /**
   * @param raster
   * @param sampleIndex
   */
  rasterToArrayType(raster: number[]): ArrayTypes {
    const format = this.getSampleFormat();
    const bitsPerSample = this.getBitsPerSample();
    return toArrayType(raster, format, bitsPerSample);
  }

  /**
   * Returns an array of tiepoints.
   * @returns - An array of tiepoints
   */
  get tiePoints(): TiePoint[] {
    const tiepoint = this.#imageDirectory.tiepoint ?? [];
    const tiePoints = [];
    for (let i = 0; i < tiepoint.length; i += 6) {
      tiePoints.push({
        i: tiepoint[i],
        j: tiepoint[i + 1],
        k: tiepoint[i + 2],
        x: tiepoint[i + 3],
        y: tiepoint[i + 4],
        z: tiepoint[i + 5],
      });
    }
    return tiePoints;
  }

  /**
   * Returns the image origin as a XYZ-vector. When the image has no affine
   * transformation, then an exception is thrown.
   * @returns The origin as a vector
   */
  get origin(): VectorPoint {
    const { tiepoint, ModelTransformation: transform } = this.#imageDirectory;
    if (Array.isArray(tiepoint) && tiepoint.length === 6) {
      return { x: tiepoint[3], y: tiepoint[4], z: tiepoint[5] };
    } else if (transform !== undefined) {
      return { x: transform[3], y: transform[7], z: transform[11] };
    }
    throw new Error('The image does not have an affine transformation.');
  }

  /**
   * Returns the image origin as a XYZ-vector in lon-lat space. When the image has no affine
   * transformation, then an exception is thrown.
   * @returns The origin as a lon-lat vector
   */
  get originLL(): VectorPoint {
    const { origin } = this;
    return this.#transformer.forward(origin);
  }

  /**
   * Returns the image resolution as a XYZ-vector. When the image has no affine
   * transformation, then an exception is thrown. in cases when the current image does
   * not have the required tags on its own.
   * @returns The resolution as a vector
   */
  get resolution(): VectorPoint {
    const { sqrt } = Math;
    const { pixelScale, ModelTransformation: transform } = this.#imageDirectory;

    if (Array.isArray(pixelScale)) {
      return { x: pixelScale[0], y: -pixelScale[1], z: pixelScale[2] };
    }
    if (transform !== undefined) {
      if (transform[1] === 0 && transform[4] === 0) {
        return { x: transform[0], y: -transform[5], z: transform[10] };
      }
      return {
        x: sqrt(transform[0] * transform[0] + transform[4] * transform[4]),
        y: -sqrt(transform[1] * transform[1] + transform[5] * transform[5]),
        z: transform[10],
      };
    }

    throw new Error('The image does not have an affine transformation.');
  }

  /**
   * Returns the image resolution as a XYZ-vector in lon-lat space. When the image has no affine
   * transformation, then an exception is thrown. in cases when the current image does not
   * have the required tags on its own.
   * @returns The resolution as a lon-lat vector
   */
  get resolutionLL(): VectorPoint {
    const { resolution } = this;
    return this.#transformer.forward(resolution);
  }

  /**
   * Returns whether or not the pixels of the image depict an area (or point).
   * @returns Whether the pixels are a point
   */
  get pixelIsArea(): boolean {
    return this.#imageDirectory.geoKeyDirectory?.GTRasterTypeGeoKey === 1;
  }

  /**
   * Returns the image bounding box as an array of 4 values: min-x, min-y,
   * max-x and max-y. When the image has no affine transformation, then an
   * exception is thrown.
   * @param transform - apply affine transformation or proj4 transformation
   * @returns The bounding box
   */
  getBoundingBox(transform = true): [minX: number, minY: number, maxX: number, maxY: number] {
    const { height, width } = this;
    const { ModelTransformation } = this.#imageDirectory;

    if (ModelTransformation !== undefined && transform) {
      const [a, b, _c, d, e, f, _g, h] = ModelTransformation;

      const corners = [
        [0, 0],
        [0, height],
        [width, 0],
        [width, height],
      ];

      const projected = corners.map(([I, J]) => [d + a * I + b * J, h + e * I + f * J]);

      const xs = projected.map((pt) => pt[0]);
      const ys = projected.map((pt) => pt[1]);

      return [Math.min(...xs), Math.min(...ys), Math.max(...xs), Math.max(...ys)];
    } else {
      const { x: x1, y: y1 } = this.origin;
      const { x: r1, y: r2 } = this.resolution;
      const x2 = x1 + r1 * width;
      const y2 = y1 + r2 * height;
      const minX = Math.min(x1, x2);
      const minY = Math.min(y1, y2);
      const maxX = Math.max(x1, x2);
      const maxY = Math.max(y1, y2);

      if (transform) {
        const { x: tminX, y: tminY } = this.#transformer.forward({ x: minX, y: minY });
        const { x: tmaxX, y: tmaxY } = this.#transformer.forward({ x: maxX, y: maxY });
        return [tminX, tminY, tmaxX, tmaxY];
      } else {
        return [minX, minY, maxX, maxY];
      }
    }
  }

  /**
   * @param samples - Samples to read
   */
  async rasterData(samples: number[] = []): Promise<Raster> {
    const { tileWidth, tileHeight, width, height, samplesPerPixel } = this;
    const bitsPerSample = this.#imageDirectory.BitsPerSample ?? [];
    const decodeFn = getDecoder(this.#imageDirectory.Compression);
    if (samples.length === 0) samples = [...Array(samplesPerPixel).keys()];
    let bytesPerPixel = this.bytesPerPixel;

    const srcSampleOffsets: number[] = [];
    const sampleReaders: ((offset: number, littleEndian: boolean) => number)[] = [];
    for (let i = 0; i < samples.length; ++i) {
      if (this.#planarConfiguration === 1) {
        srcSampleOffsets.push(sampleSum(bitsPerSample, 0, samples[i]) / 8);
      } else {
        srcSampleOffsets.push(0);
      }
      sampleReaders.push(this.getReaderForSample(samples[i]));
    }

    const res: number[] = Array(width * height * samplesPerPixel);
    const maxXTile = Math.ceil(width / tileWidth);
    const maxYTile = Math.ceil(height / tileHeight);
    for (let yTile = 0; yTile < maxYTile; ++yTile) {
      for (let xTile = 0; xTile < maxXTile; ++xTile) {
        let data: ArrayBufferLike | undefined;
        if (this.#planarConfiguration === 1) {
          data = await this.getTileOrStrip(xTile, yTile, 0, decodeFn);
        }
        for (let sampleIndex = 0; sampleIndex < samples.length; ++sampleIndex) {
          const si = sampleIndex;
          const sample = samples[sampleIndex];
          if (this.#planarConfiguration === 2) {
            bytesPerPixel = Math.ceil(bitsPerSample[sample] / 8);
            data = await this.getTileOrStrip(xTile, yTile, sample, decodeFn);
          }
          if (data === undefined) throw new Error('data failed to load');
          const dataView = new DataView(data);
          const blockHeight = this.getBlockHeight(yTile);
          const firstLine = yTile * tileHeight;
          const firstCol = xTile * tileWidth;
          const lastLine = firstLine + blockHeight;
          const lastCol = (xTile + 1) * tileWidth;
          const reader = sampleReaders[si];

          const ymax = Math.min(blockHeight, blockHeight - (lastLine - height), height - firstLine);
          const xmax = Math.min(tileWidth, tileWidth - (lastCol - width), width - firstCol);

          for (let y = 0; y < ymax; ++y) {
            for (let x = 0; x < xmax; ++x) {
              const pixelOffset = (y * tileWidth + x) * bytesPerPixel;
              const value = reader.call(
                dataView,
                pixelOffset + srcSampleOffsets[si],
                this.#littleEndian,
              );
              const windowCoordinate =
                (y + firstLine) * width * samples.length + (x + firstCol) * samples.length + si;
              res[windowCoordinate] = value;
            }
          }
        }
      }
    }

    return { data: this.rasterToArrayType(res), width, height, alpha: false };
  }

  /**
   *
   */
  async getRGBA(): Promise<Raster> {
    const bitsPerSample = this.#imageDirectory.BitsPerSample ?? [0];
    const extraSamples = (this.#imageDirectory.ExtraSamples ?? [0])[0];
    const pi = this.#imageDirectory.PhotometricInterpretation;
    const samples = buildSamples(pi, bitsPerSample, extraSamples);

    const rasterData = await this.rasterData(samples);
    const max = 2 ** this.getBitsPerSample();
    convertColorSpace(pi, rasterData, max, this.#imageDirectory.ColorMap);
    rasterData.alpha = extraSamples !== 0;

    return rasterData;
  }

  /**
   * Build a vector feature from the image
   * @returns - The vector feature with rgba values incoded into the points
   */
  async getMultiPointVector(): Promise<VectorMultiPointResult> {
    const { width, height, alpha, data } = await this.getRGBA();
    const [minX, minY, maxX, maxY] = this.getBoundingBox(false);
    const coordinates: VectorMultiPoint<RGBA> = [];
    const rgbaStride = alpha ? 4 : 3;
    const boundX = minX === maxX ? 1 : maxX - minX;
    const boundY = minY === maxY ? 1 : maxY - minY;
    const areaXStride = this.pixelIsArea ? (0.5 / width) * boundX : 0;
    const areaYStride = this.pixelIsArea ? (0.5 / height) * boundY : 0;
    const pixelXStride = width === 1 ? 1 : width - 1;
    const pixelYStride = height === 1 ? 1 : height - 1;

    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        // Adjust xPos and yPos relative to the bounding box
        const xPos = minX + (x / pixelXStride) * boundX + areaXStride;
        const yPos = minY + (y / pixelYStride) * boundY + areaYStride;
        // Extract RGBA values
        const r = data[y * width * rgbaStride + x * rgbaStride];
        const g = data[y * width * rgbaStride + x * rgbaStride + 1];
        const b = data[y * width * rgbaStride + x * rgbaStride + 2];
        const a = alpha ? data[y * width * rgbaStride + x * rgbaStride + 3] : 255;
        // find the lon-lat coordinates of the point
        const { x: lon, y: lat } = this.#transformer.forward({ x: xPos, y: yPos });
        // Add point to coordinates array
        coordinates.push({
          x: lon,
          y: lat,
          m: { r, g, b, a },
        });
      }
    }

    return {
      geometry: {
        type: 'MultiPoint',
        is3D: false,
        coordinates,
      },
      width,
      height,
      alpha,
    };
  }

  /**
   * @param sampleIndex
   */
  getReaderForSample(sampleIndex: number): (offset: number, littleEndian: boolean) => number {
    const bitsPerSample = (this.#imageDirectory.BitsPerSample ?? [])[sampleIndex];
    const format =
      this.#imageDirectory.SampleFormat !== undefined
        ? this.#imageDirectory.SampleFormat[sampleIndex]
        : 1;
    switch (format) {
      case 1: // unsigned integer data
        if (bitsPerSample <= 8) {
          return DataView.prototype.getUint8;
        } else if (bitsPerSample <= 16) {
          return DataView.prototype.getUint16;
        } else if (bitsPerSample <= 32) {
          return DataView.prototype.getUint32;
        }
        break;
      case 2: // twos complement signed integer data
        if (bitsPerSample <= 8) {
          return DataView.prototype.getInt8;
        } else if (bitsPerSample <= 16) {
          return DataView.prototype.getInt16;
        } else if (bitsPerSample <= 32) {
          return DataView.prototype.getInt32;
        }
        break;
      case 3:
        switch (bitsPerSample) {
          case 16:
            return DataView.prototype.getFloat16;
          case 32:
            return DataView.prototype.getFloat32;
          case 64:
            return DataView.prototype.getFloat64;
          default:
            break;
        }
        break;
    }
    throw Error('Unsupported data format/bitsPerSample');
  }

  /**
   * @param x
   * @param y
   * @param sample
   * @param decodeFn
   */
  async getTileOrStrip(
    x: number,
    y: number,
    sample: number,
    decodeFn: Decoder,
  ): Promise<ArrayBufferLike> {
    const { TileOffsets, TileByteCounts, StripOffsets, StripByteCounts } = this.#imageDirectory;
    const numTilesPerRow = Math.ceil(this.width / this.tileWidth);
    const numTilesPerCol = Math.ceil(this.height / this.tileHeight);
    const index =
      this.#planarConfiguration === 1
        ? y * numTilesPerRow + x
        : this.#planarConfiguration === 2
          ? sample * numTilesPerRow * numTilesPerCol + y * numTilesPerRow + x
          : 0;

    const offset = this.#isTiled ? (TileOffsets ?? [])[index] : (StripOffsets ?? [])[index];
    const byteCount = this.#isTiled
      ? (TileByteCounts ?? [])[index]
      : (StripByteCounts ?? [])[index];
    const slice = this.#reader.slice(offset, offset + byteCount).buffer;
    let data = await decodeFn(slice, this.#imageDirectory.JPEGTables);
    data = this.maybeApplyPredictor(data);
    const sampleFormat = this.getSampleFormat();
    const bitsPerSample = this.getBitsPerSample();
    if (needsNormalization(sampleFormat, bitsPerSample)) {
      data = normalizeArray(
        data,
        sampleFormat,
        this.#planarConfiguration,
        this.samplesPerPixel,
        bitsPerSample,
        this.tileWidth,
        this.getBlockHeight(y),
      );
    }
    return data;
  }

  /**
   * @param data
   */
  maybeApplyPredictor(data: ArrayBufferLike): ArrayBufferLike {
    const predictor = this.#imageDirectory.Predictor ?? 1;
    if (predictor === 1) {
      return data;
    } else {
      const tileWidth = this.#isTiled ? this.tileWidth : this.width;
      const tileHeight = this.#isTiled
        ? this.tileHeight
        : (this.#imageDirectory.RowsPerStrip ?? this.height);
      return applyPredictor(
        data,
        predictor,
        tileWidth,
        tileHeight,
        this.#imageDirectory.BitsPerSample ?? [],
        this.#planarConfiguration,
      );
    }
  }
}
