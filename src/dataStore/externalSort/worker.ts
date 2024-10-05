declare let self: Worker;

import { sortChunk } from './sortChunk';

/** A chunk of a file to be sorted */
export interface SortChunk {
  name: string; // name of the input (there could be multiple input files to sort)
  input: string;
  outDir: string;
  start: number;
  end: number;
}

/**
 * A worker that sorts a chunk of a file and sends it to an output directory
 * @param event - the sort chunk message
 */
self.onmessage = (event: Bun.MessageEvent<SortChunk>): void => {
  void sortChunk(event.data).then((outFile): void => {
    postMessage(outFile);
  });
};
