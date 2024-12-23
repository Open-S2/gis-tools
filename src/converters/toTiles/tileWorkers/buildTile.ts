declare let self: Worker;

// TWO WAYS TO BUILD TILES:
// * Vector Tiles
// * Raster Tiles

/**
 * A worker that sorts a chunk of a file and sends it to an output directory
 * @param _event - the sort chunk message
 */
self.onmessage = (_event: Bun.MessageEvent<unknown>): void => {
  // void sortChunk(event.data as SortChunk).then((outFile): void => {
  //   postMessage(outFile);
  // });
};
