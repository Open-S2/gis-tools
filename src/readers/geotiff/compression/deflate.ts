import BaseDecoder from './basedecoder';
import { inflate } from 'pako';

/**
 *
 */
export default class DeflateDecoder extends BaseDecoder {
  /**
   * @param buffer
   */
  decodeBlock(buffer: ArrayBuffer): ArrayBuffer {
    return inflate(new Uint8Array(buffer)).buffer;
  }
}
