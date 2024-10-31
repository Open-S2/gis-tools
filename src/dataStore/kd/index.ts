import type { Stringifiable } from '..';

/** The kind of input required to store a vector for proper indexing */
export interface KDKV<T = Stringifiable> {
  x: number;
  y: number;
  data: T;
}

/** Represents a vector store or an array */
export interface KDStore<T = Stringifiable> {
  push: (value: KDKV<T>) => void;
  get: (index: number) => KDKV<T>;
  getRange: (indexStart: number, indexEnd: number) => KDKV<T>[];
  length: number;
  values: () => Generator<KDKV<T>>;
  sort: () => void;
  [Symbol.iterator]: () => Generator<KDKV<T>>;
  close: () => void;
}

/** A constructor for a vector store */
export type KDStoreConstructor<T = Stringifiable> = new (nodeSize: number) => KDStore<T>;

/** A local KD key-value store */
export class KDSpatialIndex<T = Stringifiable> implements KDStore<T> {
  #store: KDKV<T>[] = [];

  /**
   * @param nodeSize - the size of each kd-tree node
   */
  constructor(private readonly nodeSize = 64) {}

  /** @returns the length of the store */
  get length(): number {
    return this.#store.length;
  }

  /**
   * Push a value into the store
   * @param value - the value to store
   */
  push(value: KDKV<T>): void {
    this.#store.push(value);
  }

  /**
   * Get a value at index
   * @param index - the position in the store to get the value from
   * @returns the value
   */
  get(index: number): KDKV<T> {
    return this.#store[index];
  }

  /**
   * Get a range of values within an index range
   * @param indexStart - the start index
   * @param indexEnd - the end index
   * @returns the values
   */
  getRange(indexStart: number, indexEnd: number): KDKV<T>[] {
    return this.#store.slice(indexStart, indexEnd);
  }

  /**
   * iterate through the values
   * @yields an iterator
   */
  *values(): Generator<KDKV<T>> {
    for (const value of this.#store) yield value;
  }

  /** Sort the store in place */
  sort(): void {
    this.#sort(0, this.#store.length - 1, 0);
  }

  /**
   * iterate through the values
   * @returns an iterator
   */
  [Symbol.iterator](): Generator<KDKV<T>> {
    return this.values();
  }

  /** Closes the store */
  close(): void {
    this.#store = [];
  }

  /**
   * Recursively kd-sort the store
   * @param left - the leftmost index
   * @param right - the rightmost index
   * @param axis - 0 for x, 1 for y
   */
  #sort(left: number, right: number, axis: number): void {
    if (right - left <= this.nodeSize) return;

    const m = (left + right) >> 1; // middle index
    // sort ids and coords around the middle index so that the halves lie
    // either left/right or top/bottom correspondingly (taking turns)
    this.#select(m, left, right, axis);
    // recursively kd-sort first half and second half on the opposite axis
    this.#sort(left, m - 1, 1 - axis);
    this.#sort(m + 1, right, 1 - axis);
  }

  /**
   * Custom Floyd-Rivest selection algorithm: sort coords so that
   * [left..k-1] items are smaller than k-th item (on either x or y axis)
   * @param k - the sorting anchor index between left and right
   * @param left - the leftmost index
   * @param right - the rightmost index
   * @param axis - 0 for x, 1 for y
   */
  #select(k: number, left: number, right: number, axis: number): void {
    while (right > left) {
      if (right - left > 600) {
        const n = right - left + 1;
        const m = k - left + 1;
        const z = Math.log(n);
        const s = 0.5 * Math.exp((2 * z) / 3);
        const sd = 0.5 * Math.sqrt((z * s * (n - s)) / n) * (m - n / 2 < 0 ? -1 : 1);
        const newLeft = Math.max(left, Math.floor(k - (m * s) / n + sd));
        const newRight = Math.min(right, Math.floor(k + ((n - m) * s) / n + sd));
        this.#select(k, newLeft, newRight, axis);
      }

      const p = this.#store[k];
      const t = axis === 0 ? p.x : p.y;
      let i = left;
      let j = right;

      this.#swap(left, k);
      const rp = this.#store[right];
      if ((axis === 0 ? rp.x : rp.y) > t) this.#swap(left, right);

      while (i < j) {
        this.#swap(i, j);
        i++;
        j--;
        while (i < this.length && (axis === 0 ? this.#store[i].x : this.#store[i].y) < t) i++;
        while (j >= 0 && (axis === 0 ? this.#store[j].x : this.#store[j].y) > t) j--;
      }

      if ((axis === 0 ? this.#store[left].x : this.#store[left].y) === t) {
        this.#swap(left, j);
      } else {
        j++;
        this.#swap(j, right);
      }

      if (j <= k) left = j + 1;
      if (k <= j) right = j - 1;
    }
  }

  /**
   * swap two values given their indices
   * @param i - the first index
   * @param j - the second index
   */
  #swap(i: number, j: number): void {
    const store = this.#store;
    const tmp = store[i];
    store[i] = store[j];
    store[j] = tmp;
  }
}
