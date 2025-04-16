import type { PriorityCompare } from './priorityQueue.js';

/**
 * # Splay Tree Set
 *
 * ## Description
 * A splay tree set is a self-balancing binary search tree that does not allow duplicate items.
 * The value of a splay tree is in it's amortized O(log n) for all insert, delete min/max,
 * and find min/max operations.
 *
 * ## Usage
 *
 * ```ts
 * import { VectorSet } from 'gis-tools-ts';
 *
 * const vecSet = new VectorSet<number>([], (a, b) => a - b);
 *
 * // If the item already exists, the existing item will be returned otherwise
 * // the new item will be both added and returned
 * let item = vecSet.add(1);
 * vecSet.add(2);
 *
 * console.log(vecSet.length); // 2
 * // Get first and last items
 * let firstitem = vecSet.first(); // 1
 * let lastitem = vecSet.last(); // 2
 * // check if a value exists
 * console.log(vecSet.has(1)); // true
 * // look for a value right before one provided
 * console.log(vecSet.lastBefore(2)); // 1
 * // look for a value right after one provided
 * console.log(vecSet.firstAfter(1)); // 2
 * ```
 *
 * ## Links
 * - TODO!()
 */
export class VectorSet<T> {
  #set: T[] = [];

  /** @param compare - compare function */
  constructor(
    private compare: PriorityCompare<T> = (a: T, b: T): number => (a < b ? -1 : a > b ? 1 : 0),
  ) {}

  /** @returns - the number of items in the set */
  get length(): number {
    return this.#set.length;
  }

  /**
   * Add an item
   * @param item - the item to add
   * @returns - the added item OR if the item already exists, the existing item
   */
  add(item: T): T {
    // if the item already exists just return that one
    const lowerBound = this.#lowerBound(item);
    if (lowerBound < this.length && this.compare(this.#set[lowerBound], item) === 0) {
      return this.#set[lowerBound];
    }
    // otherwise, add, sort, and return the input item
    this.#set.push(item);
    this.#set.sort(this.compare);

    return item;
  }

  /**
   * Check if an item exists
   * @param key - the item
   * @returns - true if the item exists
   */
  has(key: T): boolean {
    const lowerBound = this.#lowerBound(key);
    return lowerBound < this.length && this.compare(this.#set[lowerBound], key) === 0;
  }

  /**
   * Delete an item. Return the deleted item if it exists otherwise undefined
   * @param key - the item
   * @returns - the deleted item
   */
  delete(key: T): T | undefined {
    const lowerBound = this.#lowerBound(key);
    if (lowerBound < this.length && this.compare(this.#set[lowerBound], key) === 0) {
      return this.#set.splice(lowerBound, 1)[0];
    }
    return undefined;
  }

  /**
   * Get the first item
   * @returns - the first item, undefined if the set is empty
   */
  first(): T | undefined {
    return this.#set.at(0);
  }

  /**
   * Get the last item
   * @returns - the last item, undefined if the set is empty
   */
  last(): T | undefined {
    if (this.#set.length === 0) return undefined;
    return this.#set.at(-1);
  }

  /**
   * Get the last item in the set that is strictly smaller than item.
   * Returns undefined if no item was not found.
   * @param item - the item to compare against
   * @returns - the last item before the comparison item provided
   */
  lastBefore(item: T): T | undefined {
    const lowerBounds = this.#lowerBound(item);
    return this.#set[lowerBounds - 1];
  }

  /**
   * Get the first item in the set that is strictly larger than item.
   * Returns undefined if no item was not found.
   * @param item - the item to compare against
   * @returns - the first item after the comparison item provided
   */
  firstAfter(item: T): T | undefined {
    const lowerBounds = this.#lowerBound(item);
    const foundItem = this.#set[lowerBounds];
    if (this.compare(foundItem, item) === 0) return this.#set[lowerBounds + 1];
    return foundItem;
  }

  /**
   * @param id - the id to search for
   * @returns the starting index from the lower bound of the id
   */
  #lowerBound(id: T): number {
    // lower bound search
    let lo: number = 0;
    let hi: number = this.length;
    let mid: number;

    while (lo < hi) {
      mid = Math.floor(lo + (hi - lo) / 2);
      const loHi = this.#set[mid];
      if (this.compare(loHi, id) < 0) {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }

    return lo;
  }
}
