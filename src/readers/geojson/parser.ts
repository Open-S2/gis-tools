import fs from 'fs';

const LEFT_BRACE = 0x7b;
const RIGHT_BRACE = 0x7d;
const BACKSLASH = 0x5c;
const STRING = 0x22;

/**
 *
 */
export default class Parser {
  chunkSize = 0;
  buffer: Buffer = Buffer.alloc(665536);
  offset = 0;
  pos = 0;
  fd = 0;
  fileSize = 0;
  braceDepth = 0;
  feature: Buffer[] = [];
  start: null | number = null;
  end: null | number = null;
  isObject = true;

  /**
   * @param file
   */
  parse(file: string): Iterator<Buffer> {
    return _parse(this, file);
  }

  // each line is a geojson object
  /**
   *
   */
  nextValueFast(): void {
    fs.readSync(this.fd, this.buffer, this.offset, this.chunkSize, this.offset);
  }

  // since we know that a '{' is the start of a feature after we read a '"features"',
  // than we start there to avoid reading in values that are not features.
  // This is a modified Knuth–Morris–Pratt algorithm
  /**
   *
   */
  setStartPosition(): boolean {
    const features = Buffer.from('"features":');
    const featuresSize = features.length;

    let k = 0;
    while (this.pos < this.chunkSize) {
      if (features[k] === this.buffer[this.pos]) {
        k++;
        this.pos++;
        if (k === featuresSize) {
          return true;
        }
      } else {
        k = 0;
        this.pos++;
      }
    }
    // if we made it here, we need to read in the next buffer chunk.
    // If we hit the end of the file, return false
    this.offset += this.chunkSize;
    if (this.offset < this.fileSize) {
      this.pos = 0;
      if (this.offset + this.chunkSize < this.fileSize) {
        this.chunkSize = this.fileSize - this.offset;
      }
      fs.readSync(this.fd, this.buffer, this.offset, this.chunkSize, this.offset);
      return this.setStartPosition();
    } else {
      return false;
    }
  }

  // everytime we see a "{" we start 'recording' the feature. If we see more "{" on our journey, we increment.
  // Once we find the end of the feature, store the "start" and "end" indexes, slice the buffer and send out
  // as a return. If we run out of buffer to read AKA we finish the file, we return a null. If we run
  // out of the buffer, but we still have file left to read, just read into the buffer and continue on
  /**
   *
   */
  nextValue(): null | Buffer {
    // get started
    while (this.pos < this.chunkSize) {
      if (this.buffer[this.pos] === BACKSLASH) {
        this.pos++;
      } else if (this.buffer[this.pos] === STRING) {
        if (!this.isObject) this.isObject = true;
        else this.isObject = false;
      } else if (this.buffer[this.pos] === LEFT_BRACE && this.isObject) {
        if (this.braceDepth === 0) this.start = this.pos;
        this.braceDepth++; // first brace is the start of the feature
      } else if (this.buffer[this.pos] === RIGHT_BRACE && this.isObject) {
        this.braceDepth--; // if this hits zero, we are at the end of the feature
        if (this.braceDepth === 0) {
          this.end = this.pos;
          break;
        }
      }
      this.pos++;
    }

    // what if the last char in current buffer was a BACKSLASH?
    // we need to make sure in the next buffer we account for increment
    const incrementSpace = this.pos - this.chunkSize;

    if (this.start !== null && this.end !== null) {
      this.pos++;
      try {
        this.feature.push(this.buffer.slice(this.start, this.end + 1));
        // this.feature = Buffer.concat(this.feature)
        // this.feature = this.feature.toString()
        // this.feature = this.feature.replace(/(\r\n|\n|\r)/gm, '')
        // const feature = JSON.parse(this.feature)
        const feature = Buffer.concat(this.feature);
        // reset variables
        this.feature = [];
        this.start = null;
        this.end = null;
        this.braceDepth = 0;
        this.isObject = true;
        // return
        return feature;
      } catch (err) {
        console.error(new Error('could not parse feature'), this.feature.toString());
        // reset variables
        this.feature = [];
        this.start = null;
        this.end = null;
        this.braceDepth = 0;
        this.isObject = true;
        return null;
      }
    } else {
      // if offset isn't at filesize, increment buffer and start again
      if (this.start !== null) {
        this.feature.push(this.buffer.slice(this.start));
        this.start = 0;
      }
      this.offset += this.chunkSize;
      if (this.offset < this.fileSize) {
        this.pos = incrementSpace > 0 ? incrementSpace : 0;
        if (this.offset + this.chunkSize > this.fileSize) {
          this.chunkSize = this.fileSize - this.offset;
        }
        this.buffer = Buffer.alloc(this.chunkSize);
        fs.readSync(this.fd, this.buffer, 0, this.chunkSize, this.offset);
        return this.nextValue();
      } else {
        return null;
      } // end of file
    }
  }
}

/**
 * @param parser
 * @param file
 */
function* _parse(parser: Parser, file: string): Iterator<Buffer> {
  // setup
  parser.fd = fs.openSync(file, 'r');
  const stats = fs.fstatSync(parser.fd);
  parser.fileSize = stats.size;
  parser.chunkSize = 665536;
  // buffer the first chunk
  fs.readSync(parser.fd, parser.buffer, parser.offset, parser.chunkSize, parser.offset);
  // find out starting position
  const set = parser.setStartPosition();
  if (!set) throw Error('File is not geojson');
  while (true) {
    const feature = parser.nextValue();
    if (feature !== null)
      yield feature; // may have removed the feature for some reason
    else break;
  }
  // finish by closing the file
  fs.closeSync(parser.fd);
}
