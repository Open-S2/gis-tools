/** How the comparison function needs to work */
export type PriorityCompare<T> = (a: T, b: T) => number;

/** A Priority Queue */
export default class PriorityQueue<T = number> {
  #length: number = 0;
  /**
   * @param data - initial data
   * @param compare - compare function
   */
  constructor(
    private data: T[] = [],
    private compare: PriorityCompare<T> = (a: T, b: T): number => (a < b ? -1 : a > b ? 1 : 0),
  ) {
    this.#length = data.length;
    if (this.#length > 0) {
      for (let i = (this.#length >> 1) - 1; i >= 0; i--) this.#down(i);
    }
  }

  /** @returns - the number of items */
  get length(): number {
    return this.#length;
  }

  /** @param item - the item to store */
  push(item: T): void {
    this.data.push(item);
    this.#up(this.#length++);
  }

  /**
   * Access the top item, remove it, and return it
   * @returns - the top item
   */
  pop(): T | undefined {
    if (this.#length === 0) return;
    const top = this.data.at(0);
    const bottom = this.data.pop();
    if (bottom === undefined) return;

    if (--this.#length > 0) {
      this.data[0] = bottom;
      this.#down(0);
    }

    return top;
  }

  /**
   * Peek at the top item without removing it
   * @returns - the top item
   */
  peek(): T | undefined {
    return this.data.at(0);
  }

  /**
   * @param pos - the position to reorder
   */
  #up(pos: number): void {
    const { data, compare } = this;
    const item = data[pos];

    while (pos > 0) {
      const parent = (pos - 1) >> 1;
      const current = data[parent];
      if (compare(item, current) >= 0) break;
      data[pos] = current;
      pos = parent;
    }

    data[pos] = item;
  }

  /** @param pos - the position to start reordering */
  #down(pos: number): void {
    const { data, compare } = this;
    const halfLength = this.#length >> 1;
    const item = data[pos];

    while (pos < halfLength) {
      let bestChild = (pos << 1) + 1; // initially it is the left child
      const right = bestChild + 1;

      if (right < this.#length && compare(data[right], data[bestChild]) < 0) {
        bestChild = right;
      }
      if (compare(data[bestChild], item) >= 0) break;

      data[pos] = data[bestChild];
      pos = bestChild;
    }

    data[pos] = item;
  }
}
