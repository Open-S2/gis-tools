/* eslint-disable no-var */
import sharp from 'sharp';

import type { Blob } from 'node:buffer';

// Augmenting global types
declare global {
  /** Declare the ImageBitmap class globally */
  interface ImageBitmap {
    readonly width: number;
    readonly height: number;
  }

  /** Declare the ImageData class globally */
  interface ImageData {
    readonly data: Uint8ClampedArray;
    readonly width: number;
    readonly height: number;
  }

  /** What the OffscreenCanvas interface looks like */
  interface OffscreenCanvasInterface {
    readonly width: number;
    readonly height: number;
    getContext(type: string): null | OffscreenCanvasRenderingContext2D;
  }

  /** Declare the OffscreenCanvas class globally */
  var OffscreenCanvas: {
    prototype: OffscreenCanvasInterface;
    new (width: number, height: number): OffscreenCanvasInterface;
  };

  /**
   * Declare the createImageBitmap function globally
   * @param blob - the blob input
   * @returns an ImageBitmap
   */
  function createImageBitmap(blob: Blob): Promise<ImageBitmap>;

  /** Declare the OffscreenCanvasRenderingContext2D class globally */
  interface OffscreenCanvasRenderingContext2D {
    drawImage(image: ImageBitmap, dx: number, dy: number): void;
    getImageData(x: number, y: number, width: number, height: number): ImageData;
  }
}

/** Build a local ImageBitmap for reading the data */
class ImageBitmap {
  /**
   * @param data - the image data
   * @param width - the image width
   * @param height - the image height
   */
  constructor(
    public data: Uint8Array,
    public width: number,
    public height: number,
  ) {}
}

/**
 * @param blob - the blob input data
 * @returns an ImageBitmap
 */
async function createImageBitmap(blob: Blob): Promise<ImageBitmap> {
  const imageBuffer = Buffer.from(await blob.arrayBuffer());

  // Decode image using sharp
  const decodedImage = await sharp(imageBuffer).raw().toBuffer({ resolveWithObject: true });
  const {
    data,
    info: { width, height },
  } = decodedImage;

  return new ImageBitmap(new Uint8Array(data), width, height);
}

/** An offscreen canvas polyfill */
class OffscreenCanvasPolyfill {
  /**
   * @param width - the canvas width
   * @param height - the canvas height
   */
  constructor(
    public width: number,
    public height: number,
  ) {}

  /**
   * @param type - expect '2d' or throw null
   * @returns the OffscreenCanvasRenderingContext2D
   */
  getContext(type: string): null | OffscreenCanvasRenderingContext2D {
    if (type !== '2d') return null;

    return new OffscreenCanvasRenderingContext2D(this.width, this.height);
  }
}

/** An offscreen canvas rendering context polyfill */
class OffscreenCanvasRenderingContext2D {
  data: Uint8ClampedArray;
  /**
   * @param width - the canvas width
   * @param height - the canvas height
   */
  constructor(
    public width: number,
    public height: number,
  ) {
    this.data = new Uint8ClampedArray(width * height * 4);
  }

  /**
   * Draw an ImageBitmap onto the canvas at position (dx, dy).
   * Copies the pixel data from the ImageBitmap to the canvas buffer.
   * @param image - the image to draw
   * @param dx - the x position on the canvas
   * @param dy - the y position on the canvas
   */
  drawImage(image: ImageBitmap, dx: number, dy: number): void {
    const { width: imgWidth, height: imgHeight, data: imgData } = image;

    for (let y = 0; y < imgHeight; y++) {
      for (let x = 0; x < imgWidth; x++) {
        const imgIndex = (y * imgWidth + x) * 4; // Index in the image data (RGBA)
        const canvasX = dx + x;
        const canvasY = dy + y;

        if (canvasX >= 0 && canvasX < this.width && canvasY >= 0 && canvasY < this.height) {
          const canvasIndex = (canvasY * this.width + canvasX) * 4; // Index in the canvas data

          // Copy RGBA values from the image to the canvas
          this.data[canvasIndex] = imgData[imgIndex]; // R
          this.data[canvasIndex + 1] = imgData[imgIndex + 1]; // G
          this.data[canvasIndex + 2] = imgData[imgIndex + 2]; // B
          this.data[canvasIndex + 3] = imgData[imgIndex + 3]; // A
        }
      }
    }
  }

  /**
   * Get the ImageData for a specific region of the canvas.
   * @param x - The x coordinate of the region's top-left corner
   * @param y - The y coordinate of the region's top-left corner
   * @param width - The width of the region
   * @param height - The height of the region
   * @returns the ImageData
   */
  getImageData(x: number, y: number, width: number, height: number): ImageData {
    const size = width * height * 4;
    if (this.data.length === size) return { data: this.data.slice(0, size), width, height };

    const imageData = new Uint8ClampedArray(size);
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const canvasX = x + col;
        const canvasY = y + row;
        const canvasIndex = (canvasY * this.width + canvasX) * 4;
        const imageDataIndex = (row * width + col) * 4;

        // Copy RGBA values from the canvas to the imageData
        imageData[imageDataIndex] = this.data[canvasIndex]; // R
        imageData[imageDataIndex + 1] = this.data[canvasIndex + 1]; // G
        imageData[imageDataIndex + 2] = this.data[canvasIndex + 2]; // B
        imageData[imageDataIndex + 3] = this.data[canvasIndex + 3]; // A
      }
    }

    return { data: imageData, width, height };
  }
}

globalThis.createImageBitmap ??= createImageBitmap;
globalThis.OffscreenCanvas ??= OffscreenCanvasPolyfill;
