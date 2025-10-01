import { FlatQueue } from './flatqueue.js';

import type { BBox } from '../index.js';

/** Function describing how to access the minX, minY, maxX, and maxY properties */
export type BoxIndexAccessor<T> = (item: T) => BBox;

/**
 * # BoxIndex
 *
 * ## Description
 * An Index for points and rectangles
 *
 * A really fast static spatial index for 2D points and rectangles in JavaScript.
 * Uses either a fast simple Hilbert curve algorithm or a more complex Hilbert curve (S2) algorithm.
 *
 * This is a partial port/typescript port of the [flatbush](https://github.com/mourner/flatbush)
 * codebase.
 *
 * ## Usage
 * ```ts
 * import { BoxIndex } from 'gis-tools-ts';
 * import type { BoxIndexAccessor } from 'gis-tools-ts';
 *
 * interface Item { minX: number; minY: number; maxX: number; maxY: number }
 *
 * // define how to access the minX, minY, maxX, and maxY properties
 * const accessor: BoxIndexAccessor<Item> = (item: Item) => [item.minX, item.minY, item.maxX, item.maxY];
 * // create the index
 * const flatbush = new BoxIndex<Item>(items, accessor);
 *
 * // make a bounding box query
 * const found = index.search(minX, minY, maxX, maxY);
 *
 * // make a k-nearest-neighbors query
 * const neighborIds = index.neighbors(x, y, 5);
 * ```
 *
 * ## Links
 * - <https://github.com/mourner/flatbush>
 */
export class BoxIndex<T> {
  #nodeSize: number;
  #numItems: number;
  #levelBounds: number[];
  #pos = 0;
  minX = Infinity;
  minY = Infinity;
  maxX = -Infinity;
  maxY = -Infinity;
  #boxes: number[];
  #indices: number[];
  // a priority queue for k-nearest-neighbors queries
  #queue: FlatQueue<number> = new FlatQueue<number>();
  #indexed = false;
  /**
   * Create a BoxIndex index that will hold a given number of items.
   * @param items - The items to index.
   * @param accessor - A function for accessing the minX, minY, maxX, and maxY properties of the items.
   * @param [nodeSize] Size of the tree node (16 by default).
   */
  constructor(
    private readonly items: T[],
    private readonly accessor: BoxIndexAccessor<T>,
    nodeSize = 16,
  ) {
    this.#numItems = items.length;
    this.#nodeSize = Math.min(Math.max(nodeSize, 2), 65535);

    // calculate the total number of nodes in the R-tree to allocate space for
    // and the index of each tree level (used in search later)
    let n = this.#numItems;
    let numNodes = n;
    this.#levelBounds = [n * 4];
    if (n !== 0) {
      do {
        n = Math.ceil(n / this.#nodeSize);
        numNodes += n;
        this.#levelBounds.push(numNodes * 4);
      } while (n !== 1);
    }

    // setup the index buffers
    this.#boxes = new Array(numNodes * 4);
    this.#indices = new Array(numNodes);

    // add the items
    for (const item of this.items) {
      const [minX, minY, maxX, maxY] = this.accessor(item);
      this.#add(minX, minY, maxX, maxY);
    }
    // spatially index the items
    this.#index();
  }

  /**
   * Add a given rectangle to the index.
   * @param minX - The minimum x coordinate of the query point.
   * @param minY - The minimum y coordinate of the query point.
   * @param maxX - The maximum x coordinate of the query point.
   * @param maxY - The maximum y coordinate of the query point.
   * @returns A zero-based, incremental number that represents the newly added rectangle.
   */
  #add(minX: number, minY: number, maxX: number, maxY: number): number {
    const index = this.#pos >> 2;
    const boxes = this.#boxes;
    this.#indices[index] = index;
    boxes[this.#pos++] = minX;
    boxes[this.#pos++] = minY;
    boxes[this.#pos++] = maxX;
    boxes[this.#pos++] = maxY;

    if (minX < this.minX) this.minX = minX;
    if (minY < this.minY) this.minY = minY;
    if (maxX > this.maxX) this.maxX = maxX;
    if (maxY > this.maxY) this.maxY = maxY;

    return index;
  }

  /** Perform indexing of the added rectangles/points after all data has been added about the items. */
  #index(): void {
    if (this.#indexed || this.items.length === 0) {
      return;
    }
    this.#indexed = true;
    if (this.#pos >> 2 !== this.#numItems) {
      throw new Error(`Added ${this.#pos >> 2} items when expected ${this.#numItems}.`);
    }
    const boxes = this.#boxes;

    if (this.#numItems <= this.#nodeSize) {
      // only one node, skip sorting and just fill the root box
      boxes[this.#pos++] = this.minX;
      boxes[this.#pos++] = this.minY;
      boxes[this.#pos++] = this.maxX;
      boxes[this.#pos++] = this.maxY;
      return;
    }

    const width = this.maxX - this.minX;
    const height = this.maxY - this.minY;
    const hilbertValues = new Uint32Array(this.#numItems);
    const hilbertMax = (1 << 16) - 1;

    // map item centers into Hilbert coordinate space and calculate Hilbert values
    for (let i = 0, pos = 0; i < this.#numItems; i++) {
      const minX = boxes[pos++];
      const minY = boxes[pos++];
      const maxX = boxes[pos++];
      const maxY = boxes[pos++];
      const x = Math.floor((hilbertMax * ((minX + maxX) / 2 - this.minX)) / width);
      const y = Math.floor((hilbertMax * ((minY + maxY) / 2 - this.minY)) / height);
      hilbertValues[i] = hilbert(x, y);
    }

    // sort items by their Hilbert value (for packing later)
    sort(hilbertValues, boxes, this.#indices, 0, this.#numItems - 1, this.#nodeSize);

    // generate nodes at each tree level, bottom-up
    for (let i = 0, pos = 0; i < this.#levelBounds.length - 1; i++) {
      const end = this.#levelBounds[i];

      // generate a parent node for each block of consecutive <nodeSize> nodes
      while (pos < end) {
        const nodeIndex = pos;

        // calculate bbox for the new node
        let nodeMinX = boxes[pos++];
        let nodeMinY = boxes[pos++];
        let nodeMaxX = boxes[pos++];
        let nodeMaxY = boxes[pos++];
        for (let j = 1; j < this.#nodeSize && pos < end; j++) {
          nodeMinX = Math.min(nodeMinX, boxes[pos++]);
          nodeMinY = Math.min(nodeMinY, boxes[pos++]);
          nodeMaxX = Math.max(nodeMaxX, boxes[pos++]);
          nodeMaxY = Math.max(nodeMaxY, boxes[pos++]);
        }

        // add the new node to the tree data
        this.#indices[this.#pos >> 2] = nodeIndex;
        boxes[this.#pos++] = nodeMinX;
        boxes[this.#pos++] = nodeMinY;
        boxes[this.#pos++] = nodeMaxX;
        boxes[this.#pos++] = nodeMaxY;
      }
    }
  }

  /**
   * Search the index by a bounding box.
   * @param minX - The minimum x coordinate of the query point.
   * @param minY - The minimum y coordinate of the query point.
   * @param maxX - The maximum x coordinate of the query point.
   * @param maxY - The maximum y coordinate of the query point.
   * @param [filterFn] An optional function that is called on every found item; if supplied, only items for which this function returns true will be included in the results array.
   * @returns An array of indices of items intersecting or touching the given bounding box.
   */
  search(
    minX: number,
    minY: number,
    maxX: number,
    maxY: number,
    filterFn?: (item: T) => boolean,
  ): T[] {
    let nodeIndex: undefined | number = this.#boxes.length - 4;
    const queue = [];
    const results: T[] = [];

    while (nodeIndex !== undefined) {
      // find the end index of the node
      const end = Math.min(
        nodeIndex + this.#nodeSize * 4,
        upperBound(nodeIndex, this.#levelBounds),
      );

      // search through child nodes
      for (let pos = nodeIndex ?? 0; pos < end; pos += 4) {
        // check if node bbox intersects with query bbox
        const x0 = this.#boxes[pos];
        if (maxX < x0) continue;
        const y0 = this.#boxes[pos + 1];
        if (maxY < y0) continue;
        const x1 = this.#boxes[pos + 2];
        if (minX > x1) continue;
        const y1 = this.#boxes[pos + 3];
        if (minY > y1) continue;

        const index: number = this.#indices[pos >> 2] | 0;

        if (nodeIndex >= this.#numItems * 4) {
          queue.push(index); // node; add it to the search queue
        } else {
          const item = this.items.at(index);
          if (item !== undefined && (filterFn?.(item) ?? true)) results.push(item); // leaf item
        }
      }

      nodeIndex = queue.pop();
    }

    return results;
  }

  /**
   * Search items in order of distance from the given point.
   * @param x - The x coordinate of the query point.
   * @param y - The y coordinate of the query point.
   * @param [maxResults] - The maximum number of results to return.
   * @param [maxDistance] - The maximum distance to search.
   * @param [filterFn] An optional function for filtering the results.
   * @returns An array of indices of items found.
   */
  neighbors(
    x: number,
    y: number,
    maxResults = Infinity,
    maxDistance = Infinity,
    filterFn?: (item: T) => boolean,
  ): T[] {
    let nodeIndex: undefined | number = this.#boxes.length - 4;
    const q = this.#queue;
    const results: T[] = [];
    const maxDistSquared = maxDistance * maxDistance;

    outer: while (nodeIndex !== undefined) {
      // find the end index of the node
      const end = Math.min(
        nodeIndex + this.#nodeSize * 4,
        upperBound(nodeIndex, this.#levelBounds),
      );

      // add child nodes to the queue
      for (let pos = nodeIndex; pos < end; pos += 4) {
        const index = this.#indices[pos >> 2] | 0;
        const minX = this.#boxes[pos];
        const minY = this.#boxes[pos + 1];
        const maxX = this.#boxes[pos + 2];
        const maxY = this.#boxes[pos + 3];
        const dx = x < minX ? minX - x : x > maxX ? x - maxX : 0;
        const dy = y < minY ? minY - y : y > maxY ? y - maxY : 0;
        const dist = dx * dx + dy * dy;
        if (dist > maxDistSquared) continue;

        if (nodeIndex >= this.#numItems * 4) {
          q.push(index << 1, dist); // node (use even id)
        } else {
          q.push((index << 1) + 1, dist); // leaf item (use odd id)
        }
      }

      // pop items from the queue
      // @ts-expect-error q.length check eliminates undefined values
      while (q.length !== 0 && (q.peek() & 1) !== 0) {
        const dist = q.peekValue();
        if (dist === undefined || dist > maxDistSquared) break outer;
        const idx = (q.pop() ?? 0) >> 1;
        const item = this.items.at(idx);
        if (item !== undefined && (filterFn?.(item) ?? true)) results.push(item);
        if (results.length === maxResults) break outer;
      }

      nodeIndex = q.length !== 0 ? (q.pop() ?? 0) >> 1 : undefined;
    }

    q.clear();
    return results;
  }
}

/**
 * Binary search for the first value in the array bigger than the given.
 * @param value - the value to search for
 * @param arr - the array to search
 * @returns the first value in the array bigger than the given
 */
function upperBound(value: number, arr: number[]): number {
  let i = 0;
  let j = arr.length - 1;
  while (i < j) {
    const m = (i + j) >> 1;
    if (arr[m] > value) {
      j = m;
    } else {
      i = m + 1;
    }
  }
  return arr[i];
}

/**
 * Custom quicksort that partially sorts bbox data alongside the hilbert values.
 * @param values - the hilbert values
 * @param boxes - the boxes
 * @param indices - the indices
 * @param left - the left index
 * @param right - the right index
 * @param nodeSize - the node size
 */
function sort(
  values: Uint32Array,
  boxes: number[],
  indices: number[],
  left: number,
  right: number,
  nodeSize: number,
): void {
  if (Math.floor(left / nodeSize) >= Math.floor(right / nodeSize)) return;

  // apply median of three method
  const start = values[left];
  const mid = values[(left + right) >> 1];
  const end = values[right];

  let pivot = end;

  const x = Math.max(start, mid);
  if (end > x) {
    pivot = x;
  } else if (x === start) {
    pivot = Math.max(mid, end);
  } else if (x === mid) {
    pivot = Math.max(start, end);
  }

  let i = left - 1;
  let j = right + 1;

  while (true) {
    do i++;
    while (values[i] < pivot);
    do j--;
    while (values[j] > pivot);
    if (i >= j) break;
    swap(values, boxes, indices, i, j);
  }

  sort(values, boxes, indices, left, j, nodeSize);
  sort(values, boxes, indices, j + 1, right, nodeSize);
}

/**
 * Swap two values and two corresponding boxes.
 * @param values - the hilbert values
 * @param boxes - the boxes
 * @param indices - the indices
 * @param i - index
 * @param j - index
 */
function swap(values: Uint32Array, boxes: number[], indices: number[], i: number, j: number): void {
  const temp = values[i];
  values[i] = values[j];
  values[j] = temp;

  const k = 4 * i;
  const m = 4 * j;

  const a = boxes[k];
  const b = boxes[k + 1];
  const c = boxes[k + 2];
  const d = boxes[k + 3];
  boxes[k] = boxes[m];
  boxes[k + 1] = boxes[m + 1];
  boxes[k + 2] = boxes[m + 2];
  boxes[k + 3] = boxes[m + 3];
  boxes[m] = a;
  boxes[m + 1] = b;
  boxes[m + 2] = c;
  boxes[m + 3] = d;

  const e = indices[i];
  indices[i] = indices[j];
  indices[j] = e;
}

/**
 * Fast Hilbert curve algorithm by http://threadlocalmutex.com/
 * Ported from C++ https://github.com/rawrunprotected/hilbert_curves (public domain)
 * @param x - x coordinate
 * @param y - y coordinate
 * @returns the Hilbert value (32-bit integer)
 */
function hilbert(x: number, y: number): number {
  let a = x ^ y;
  let b = 0xffff ^ a;
  let c = 0xffff ^ (x | y);
  let d = x & (y ^ 0xffff);

  let A = a | (b >> 1);
  let B = (a >> 1) ^ a;
  let C = (c >> 1) ^ (b & (d >> 1)) ^ c;
  let D = (a & (c >> 1)) ^ (d >> 1) ^ d;

  a = A;
  b = B;
  c = C;
  d = D;
  A = (a & (a >> 2)) ^ (b & (b >> 2));
  B = (a & (b >> 2)) ^ (b & ((a ^ b) >> 2));
  C ^= (a & (c >> 2)) ^ (b & (d >> 2));
  D ^= (b & (c >> 2)) ^ ((a ^ b) & (d >> 2));

  a = A;
  b = B;
  c = C;
  d = D;
  A = (a & (a >> 4)) ^ (b & (b >> 4));
  B = (a & (b >> 4)) ^ (b & ((a ^ b) >> 4));
  C ^= (a & (c >> 4)) ^ (b & (d >> 4));
  D ^= (b & (c >> 4)) ^ ((a ^ b) & (d >> 4));

  a = A;
  b = B;
  c = C;
  d = D;
  C ^= (a & (c >> 8)) ^ (b & (d >> 8));
  D ^= (b & (c >> 8)) ^ ((a ^ b) & (d >> 8));

  a = C ^ (C >> 1);
  b = D ^ (D >> 1);

  let i0 = x ^ y;
  let i1 = b | (0xffff ^ (i0 | a));

  i0 = (i0 | (i0 << 8)) & 0x00ff00ff;
  i0 = (i0 | (i0 << 4)) & 0x0f0f0f0f;
  i0 = (i0 | (i0 << 2)) & 0x33333333;
  i0 = (i0 | (i0 << 1)) & 0x55555555;

  i1 = (i1 | (i1 << 8)) & 0x00ff00ff;
  i1 = (i1 | (i1 << 4)) & 0x0f0f0f0f;
  i1 = (i1 | (i1 << 2)) & 0x33333333;
  i1 = (i1 | (i1 << 1)) & 0x55555555;

  return ((i1 << 1) | i0) >>> 0;
}
