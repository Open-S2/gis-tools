<h1 style="text-align: center;">
  <div align="center">Data Structure - Splay Tree</div>
</h1>

<p align="center">
  <img src="../../assets/badges/splayTree-file.svg" alt="splayTree-file-ts">
  <img src="../../assets/badges/splayTree-gzip.svg" alt="splayTree-gzip-ts">
  <img src="../../assets/badges/splayTree-brotli.svg" alt="splayTree-brotli-ts">
</p>

## Description

A splay tree set is a self-balancing binary search tree that does not allow duplicate elements.
The value of a splay tree is in it's amortized O(log n) for all insert, delete min/max,
and find min/max operations.

## Usage

```ts
import { SplayTreeSet } from 'gis-tools-ts';

const tree = new SplayTreeSet<number>([], (a, b) => a - b);

// If the element already exists, the existing element will be returned otherwise
// the new element will be both added and returned
let element = tree.add(1);
tree.add(2);

console.log(tree.length); // 2
// Get first and last elements
let firstElement = tree.first(); // 1
let lastElement = tree.last(); // 2
// check if a value exists
console.log(tree.has(1)); // true
// look for a value right before one provided
console.log(tree.lastBefore(2)); // 1
// look for a value right after one provided
console.log(tree.firstAfter(1)); // 2
```

## Links

- https://en.wikipedia.org/wiki/Splay_tree
- [Splay Tree Visualizer](http://slmoore.github.io/SplayTreeVisualizer/)
- [Visualizer Source Code](https://github.com/slmoore/SplayTreeVisualizer)
