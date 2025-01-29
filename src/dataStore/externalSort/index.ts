import { mergeSortedChunks } from './mergeSortedChunks';
import { sortChunk } from './sortChunk';
import { availableParallelism, tmpdir } from 'os';
import { createReadStream, createWriteStream } from 'fs';
import { exists, stat, unlink } from 'fs/promises';

/**
 * Sorts an array using external-sorting.
 * @param inputs - a list of input files without their extensions. e.g. './file1', './file2', './file3'
 * @param output - output folder to place the sorted keys
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
  await mergeValues(output, sizes);
  // 5) Cleanup
  for (const file of sortedFiles) await unlink(file);
}

/** A File name and it's size */
export interface FileSize {
  /** Name of the folder */
  name: string;
  /** Name of the input (there could be multiple input files to sort) */
  input: string;
  /** Total size of the key store */
  keySize: number;
  /** Total size of the item store */
  valueSize: number;
  /** Offset for values */
  valueOffset: number;
}

/**
 * @param inputs - a list of file's
 * @returns - a list of file names, inputs, and sizes
 */
async function getSizes(inputs: string[]): Promise<FileSize[]> {
  const sizes: FileSize[] = [];
  let valueOffset = 0;

  for (const input of inputs) {
    const valueSize = (await exists(`${input}.values`))
      ? await stat(`${input}.values`).then((stat) => stat.size)
      : 0;
    sizes.push({
      name: input.split('/').pop()!,
      input,
      keySize: await stat(`${input}.keys`).then((stat) => stat.size),
      valueSize,
      valueOffset,
    });
    valueOffset += valueSize;
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
  valueOffset: number;
}

/**
 * @param fileSizes - a list of file names and sizes
 * @param outDir - output directory to store temporary sorted files
 * @param maxHeap - max number of keys in memory
 * @returns - a list of chunks
 */
function buildChunks(fileSizes: FileSize[], outDir: string, maxHeap: number): SortChunk[] {
  const chunks: SortChunk[] = [];

  for (const { name, input, keySize, valueOffset } of fileSizes) {
    for (let start = 0; start < keySize; start += maxHeap * 16) {
      const end = Math.min(start + maxHeap * 16, keySize);
      chunks.push({ name, input: `${input}.keys`, outDir, start, end, valueOffset });
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
    const threads = Math.min(threadCount, chunkLength);
    let threadsComplete = 0;
    for (let i = 0; i < threads; i++) {
      const worker = new Worker(new URL('./worker', import.meta.url).href, { type: 'module' });
      worker.postMessage(chunks.shift());
      /** @param msg - a sorted file */
      worker.onmessage = (msg: Bun.MessageEvent<string>) => {
        sortedFiles.push(msg.data);
        if (chunks.length === 0) {
          worker.terminate();
          threadsComplete++;
          if (threadsComplete === threads) resolve();
        } else {
          worker.postMessage(chunks.shift());
        }
      };
    }
  });

  return sortedFiles;
}

/**
 * merge the values files since the sorted key indexes have been merged as well.
 * @param output - name of the output folder
 * @param sizes - list of unique input values
 */
async function mergeValues(output: string, sizes: FileSize[]): Promise<void> {
  if (sizes.length <= 1) return;
  const values = sizes
    .sort((a, b) => a.valueOffset - b.valueOffset)
    .filter((c) => c.input !== output && c.valueSize > 0)
    .map((c) => c.input);

  if (values.length === 0) return;

  const writeStream = createWriteStream(`${output}.values`, { flags: 'a' }); // Open output file in append mode

  for (const value of values) {
    const readStream = createReadStream(`${value}.values`); // Create a read stream for each file
    readStream.pipe(writeStream, { end: false }); // Pipe data to the write stream
    await new Promise((resolve, reject) => {
      // @ts-expect-error - recent typescript bug
      readStream.on('end', resolve); // Resolve when reading ends
      readStream.on('error', reject); // Reject on error
    });
  }

  writeStream.end(); // Close the write stream
}
