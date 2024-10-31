declare let self: Worker;

import { sortChunk } from './sortChunk';

import type { SortChunk } from '.';

/**
 * A worker that sorts a chunk of a file and sends it to an output directory
 * @param event - the sort chunk message
 */
self.onmessage = (event: Bun.MessageEvent<SortChunk>): void => {
  void sortChunk(event.data as SortChunk).then((outFile): void => {
    postMessage(outFile);
  });
};
