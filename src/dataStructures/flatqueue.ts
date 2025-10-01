/**
 * # FlatQueue
 *
 * ## Description
 * A priority queue implemented using a binary heap.
 *
 * A Typescript port from the [flatqueue](https://github.com/mourner/flatqueue) code.
 *
 * ## Usage
 *
 * ```ts
 * import { FlatQueue } from 'gis-tools-ts';
 *
 * const queue = new FlatQueue<number>();
 *
 * queue.push(0, 2);
 * queue.push(1, 1);
 * queue.push(2, 3);
 *
 * expect(queue.pop()).toEqual(2);
 * expect(queue.pop()).toEqual(1);
 * expect(queue.pop()).toEqual(0);
 * ```
 */
export class FlatQueue<T = number> {
  #ids: T[] = [];
  #values: number[] = [];
  #length: number = 0;

  /** @returns - the number of items */
  get length(): number {
    return this.#length;
  }

  /** Removes all items from the queue. */
  clear(): void {
    this.#length = 0;
  }

  /**
   * Adds `item` to the queue with the specified `priority`.
   *
   * `priority` must be a number. Items are sorted and returned from low to high priority. Multiple items
   * with the same priority value can be added to the queue, but there is no guaranteed order between these items.
   * @param item - the item to add
   * @param priority - the priority of the item
   */
  push(item: T, priority: number): void {
    let pos = this.#length++;

    while (pos > 0) {
      const parent = (pos - 1) >> 1;
      const parentValue = this.#values[parent];
      if (priority >= parentValue) break;
      this.#ids[pos] = this.#ids[parent];
      this.#values[pos] = parentValue;
      pos = parent;
    }

    this.#ids[pos] = item;
    this.#values[pos] = priority;
  }

  /**
   * Removes and returns the item from the head of this queue, which is one of
   * the items with the lowest priority. If this queue is empty, returns `undefined`.
   * @returns the item from the head of this queue
   */
  pop(): T | undefined {
    if (this.#length === 0) return undefined;

    const ids = this.#ids;
    const values = this.#values;
    const top = ids[0];
    const last = --this.#length;

    if (last > 0) {
      const id = ids[last];
      const value = values[last];
      let pos = 0;
      const halfLen = last >> 1;

      while (pos < halfLen) {
        const left = (pos << 1) + 1;
        const right = left + 1;
        const child = left + (Number(right < last) & Number(values[right] < values[left]));
        if (values[child] >= value) break;
        ids[pos] = ids[child];
        values[pos] = values[child];
        pos = child;
      }

      ids[pos] = id;
      values[pos] = value;
    }

    return top;
  }

  /**
   * @returns the item from the head of this queue without removing it. If this queue is empty,
   * returns `undefined`.
   */
  peek(): T | undefined {
    return this.length > 0 ? this.#ids[0] : undefined;
  }

  /**
   * @returns the priority value of the item at the head of this queue without
   * removing it. If this queue is empty, returns `undefined`.
   */
  peekValue(): number | undefined {
    return this.length > 0 ? this.#values[0] : undefined;
  }

  /**
   * Shrinks the internal arrays to `this.length`.
   *
   * `pop()` and `clear()` calls don't free memory automatically to avoid unnecessary resize operations.
   * This also means that items that have been added to the queue can't be garbage collected until
   * a new item is pushed in their place, or this method is called.
   */
  shrink() {
    this.#ids.length = this.#values.length = this.length;
  }
}
