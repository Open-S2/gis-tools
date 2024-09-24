export * from './color';

/** Raster container with convenience methods */
export class Raster {
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

  /** @returns the length of the data */
  get length() {
    return this.data.length;
  }
}
