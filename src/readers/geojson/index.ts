import Parser from './parser';
import { fromFile } from 'gen-readlines';

/**
 * @param filePath
 * @param lineDelimited
 */
export default function geoJSONFileIterator(
  filePath: string,
  lineDelimited = false,
): Iterator<Buffer> {
  let featureIterator: Iterator<Buffer>;

  if (lineDelimited) {
    featureIterator = fromFile(filePath);
  } else {
    const parser = new Parser();
    featureIterator = parser.parse(filePath);
  }

  return featureIterator;
}
