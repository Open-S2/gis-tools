import type { PriorityCompare } from './priorityQueue';

/** A basic node for a splay tree */
class SplayTreeNode<T> {
  left?: SplayTreeNode<T>;
  right?: SplayTreeNode<T>;
  /** @param key - the element to store */
  constructor(readonly key: T) {}
}

/**
 * # Splay Tree Set
 *
 * ## Description
 * A splay tree set is a self-balancing binary search tree that does not allow duplicate elements.
 * The value of a splay tree is in it's amortized O(log n) for all insert, delete min/max,
 * and find min/max operations.
 *
 * ## Usage
 *
 * ```ts
 * import { SplayTreeSet } from 'gis-tools-ts';
 *
 * const tree = new SplayTreeSet<number>([], (a, b) => a - b);
 *
 * tree.add(1);
 * tree.add(2);
 *
 * const current = tree.peek(); // 1
 * console.log(tree.length); // 2
 * let next = tree.pop(); // 1
 * console.log(tree.length); // 1
 * next = tree.pop(); // 2
 * console.log(tree.length); // 0
 * ```
 */
export class SplayTreeSet<T> {
  #root?: SplayTreeNode<T>;
  #length: number = 0;

  /** @param compare - compare function */
  constructor(
    private compare: PriorityCompare<T> = (a: T, b: T): number => (a < b ? -1 : a > b ? 1 : 0),
  ) {}

  /** @returns - the number of elements in the tree */
  get length(): number {
    return this.#length;
  }

  /**
   * Add an element
   * @param element - the element to add
   * @returns - the added element OR if the element already exists, the existing element
   */
  add(element: T): T | undefined {
    const compare = this.#splay(element);
    if (compare !== 0) this.#addNewRoot(new SplayTreeNode(element), compare);
    return this.#root?.key;
  }

  /**
   * Check if an element exists
   * @param key - the element
   * @returns - true if the element exists
   */
  has(key: T): boolean {
    return this.#splay(key) === 0;
  }

  /**
   * Delete an element. Return the deleted element if it exists otherwise undefined
   * @param key - the element
   * @returns - the deleted element
   */
  delete(key: T): SplayTreeNode<T> | undefined {
    if (this.#root === undefined) return undefined;
    const comp = this.#splay(key);
    if (comp !== 0) return undefined;
    let root = this.#root;
    const result = root;
    const left = root.left;
    this.#length--;
    if (left === undefined) {
      this.#root = root.right;
    } else {
      const right = root.right;
      root = this.#splayMax(left);

      root.right = right;
      this.#root = root;
    }

    return result;
  }

  /**
   * Get the first element
   * @returns - the first element, undefined if the queue is empty
   */
  first(): T | undefined {
    if (this.#length === 0) return undefined;
    return this.#first()?.key;
  }

  /**
   * Get the last element
   * @returns - the last element, undefined if the queue is empty
   */
  last(): T | undefined {
    if (this.#length === 0) return undefined;
    return this.#last()?.key;
  }

  /**
   * Get the last element in the set that is strictly smaller than element.
   * Returns undefined if no element was not found.
   * @param element - the element to compare against
   * @returns - the last element before the comparison element provided
   */
  lastBefore(element: T): T | undefined {
    if (this.#root === undefined) return undefined;
    const comp = this.#splay(element);
    if (comp < 0) return this.#root.key;
    let node = this.#root.left;
    if (node === undefined) return undefined;
    let nodeRight = node.right;
    while (nodeRight !== undefined) {
      node = nodeRight;
      nodeRight = node.right;
    }

    return node!.key;
  }

  /**
   * Get the first element in the set that is strictly larger than element.
   * Returns undefined if no element was not found.
   * @param element - the element to compare against
   * @returns - the first element after the comparison element provided
   */
  firstAfter(element: T): T | undefined {
    if (this.#root === undefined) return undefined;
    const comp = this.#splay(element);
    if (comp > 0) return this.#root.key;
    let node = this.#root.right;
    if (node === undefined) return undefined;
    let nodeLeft = node.left;
    while (nodeLeft !== undefined) {
      node = nodeLeft;
      nodeLeft = node.left;
    }

    return node!.key;
  }

  /**
   * Add a new root value
   * @param node - the new root
   * @param comp - the comparison result
   */
  #addNewRoot(node: SplayTreeNode<T>, comp: number): void {
    this.#length++;
    const root = this.#root;
    if (root === undefined) {
      this.#root = node;
      return;
    }
    if (comp < 0) {
      node.left = root;
      node.right = root.right;
      root.right = undefined;
    } else {
      node.right = root;
      node.left = root.left;
      root.left = undefined;
    }
    this.#root = node;
  }

  /**
   * Get the first element
   * @returns - the first element if it exists
   */
  #first(): SplayTreeNode<T> | undefined {
    if (this.#root === undefined) return undefined;
    this.#root = this.#splayMin(this.#root);
    return this.#root;
  }

  /**
   * Get the last element
   * @returns - the last element if it exists
   */
  #last(): SplayTreeNode<T> | undefined {
    if (this.#root === undefined) return undefined;
    this.#root = this.#splayMax(this.#root);
    return this.#root;
  }

  /**
   * Splay the element
   * @param key - the element
   * @returns - the comparison result
   */
  #splay(key: T): number {
    const root = this.#root;
    if (root === undefined) {
      this.compare(key, key);
      return -1;
    }

    let right: SplayTreeNode<T> | undefined = undefined;
    let newTreeRight: SplayTreeNode<T> | undefined = undefined;
    let left: SplayTreeNode<T> | undefined = undefined;
    let newTreeLeft: SplayTreeNode<T> | undefined = undefined;
    let current = root;
    const compare = this.compare;
    let comp: number;
    while (true) {
      comp = compare(current.key, key);
      if (comp > 0) {
        let currentLeft = current.left;
        if (currentLeft === undefined) break;
        comp = compare(currentLeft.key, key);
        if (comp > 0) {
          current.left = currentLeft.right;
          currentLeft.right = current;
          current = currentLeft;
          currentLeft = current.left;
          if (currentLeft === undefined) break;
        }
        if (right === undefined) {
          newTreeRight = current;
        } else {
          right.left = current;
        }
        right = current;
        current = currentLeft;
      } else if (comp < 0) {
        let currentRight = current.right;
        if (currentRight === undefined) break;
        comp = compare(currentRight.key, key);
        if (comp < 0) {
          current.right = currentRight.left;
          currentRight.left = current;
          current = currentRight;
          currentRight = current.right;
          if (currentRight === undefined) break;
        }
        if (left === undefined) {
          newTreeLeft = current;
        } else {
          left.right = current;
        }
        left = current;
        current = currentRight;
      } else {
        break;
      }
    }
    if (left !== undefined) {
      left.right = current.left;
      current.left = newTreeLeft;
    }
    if (right !== undefined) {
      right.left = current.right;
      current.right = newTreeRight;
    }
    if (this.#root !== current) this.#root = current;

    return comp;
  }

  /**
   * Splay the smallest element
   * @param node - the starting node
   * @returns - the smallest element
   */
  #splayMin(node: SplayTreeNode<T>): SplayTreeNode<T> {
    let current = node;
    let nextLeft = current.left;
    while (nextLeft !== undefined) {
      const left = nextLeft;
      current.left = left.right;
      left.right = current;
      current = left;
      nextLeft = current.left;
    }
    return current;
  }

  /**
   * Splay the largest element
   * @param node - the starting node
   * @returns - the largest element
   */
  #splayMax(node: SplayTreeNode<T>): SplayTreeNode<T> {
    let current = node;
    let nextRight = current.right;
    while (nextRight !== undefined) {
      const right = nextRight;
      current.right = right.left;
      right.left = current;
      current = right;
      nextRight = current.right;
    }
    return current;
  }
}
