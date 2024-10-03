import mergeSortedChunks from './mergeSortedChunks';
import sortChunk from './sortChunk';
import { availableParallelism, tmpdir } from 'os';
import { stat, unlink } from 'fs/promises';

/**
 * Sorts an array using external-sorting.
 * @param inputs - a list of input files
 * @param output - output file
 * @param maxHeap - max instance of the parsed entity in memory
 * @param threadCount - number of workers
 * @param tmpDir - temporary directory
 */
export async function externalSort(
  inputs: string[],
  output: string,
  maxHeap = 100_000,
  threadCount = 1,
  tmpDir = tmpdir(),
): Promise<void> {
  // 1) Get the size of the input
  const sizes = await getSizes(inputs);
  // 2) Build chunk list
  const chunks = buildChunks(sizes, tmpDir, maxHeap);
  // 3) Sort chunks - using either workers or single threaded
  let sortedFiles: string[] = [];
  if (threadCount === 1 || chunks.length <= 10) {
    for (const chunk of chunks) sortedFiles.push(await sortChunk(chunk));
  } else {
    sortedFiles = await sortChunksWithWorkers(chunks, threadCount);
  }
  // 4) Merge chunks
  await mergeSortedChunks(sortedFiles, output).catch((err) => console.error(err));
  // 5) Cleanup
  for (const file of sortedFiles) await unlink(file);
}

/** A File name and it's size */
export interface FileSize {
  name: string;
  input: string;
  size: number;
}

/**
 * @param inputs - a list of file's
 * @returns - a list of file names, inputs, and sizes
 */
async function getSizes(inputs: string[]): Promise<FileSize[]> {
  const sizes: FileSize[] = [];

  for (const input of inputs) {
    sizes.push({
      name: input.split('/').pop()!,
      input,
      size: await stat(input).then((stat) => stat.size),
    });
  }

  return sizes;
}

/** A chunk of a file to be sorted */
export interface SortChunk {
  name: string; // name of the input (there could be multiple input files to sort)
  input: string;
  outDir: string;
  start: number;
  end: number;
}

/**
 * @param fileSizes - a list of file names and sizes
 * @param outDir - output directory to store temporary sorted files
 * @param maxHeap - max number of keys in memory
 * @returns - a list of chunks
 */
function buildChunks(fileSizes: FileSize[], outDir: string, maxHeap: number): SortChunk[] {
  const chunks: SortChunk[] = [];

  for (const { name, input, size } of fileSizes) {
    for (let start = 0; start < size; start += maxHeap * 16) {
      const end = Math.min(start + maxHeap * 16, size);
      chunks.push({ name, input, outDir, start, end });
    }
  }

  return chunks;
}

/**
 * @param chunks - a list of chunks
 * @param tc - user defined thread count
 * @returns - a list of sorted files
 */
async function sortChunksWithWorkers(chunks: SortChunk[], tc: number): Promise<string[]> {
  const sortedFiles: string[] = [];
  const threadCount = Math.min(tc, availableParallelism());

  // Have workers sort chunks
  await new Promise<void>((resolve) => {
    // begin the workers and ship chunks
    const chunkLength = chunks.length;
    for (let i = 0; i < Math.min(threadCount, chunkLength); i++) {
      // @ts-expect-error - import.meta.url is a string
      const worker = new Worker(new URL('./worker', import.meta.url).href, { type: 'module' });
      worker.postMessage(chunks.shift());
      /** @param msg - a sorted file */
      worker.onmessage = (msg: Bun.MessageEvent<string>) => {
        sortedFiles.push(msg.data);
        if (chunks.length === 0) {
          worker.terminate();
          resolve();
        } else {
          worker.postMessage(chunks.shift());
        }
      };
    }
  });

  return sortedFiles;
}
