import BaseDecoder from './basedecoder';

/**
 *
 */
export default class RawDecoder extends BaseDecoder {
  /**
   * @param buffer
   */
  decodeBlock(buffer) {
    return buffer;
  }
}
