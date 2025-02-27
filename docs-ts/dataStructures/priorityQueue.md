<h1 style="text-align: center;">
  <div align="center">Data Structure - Priority Queue</div>
</h1>

<p align="center">
  <img src="../../assets/badges/priorityQueue-file.svg" alt="priorityQueue-file-ts">
  <img src="../../assets/badges/priorityQueue-gzip.svg" alt="priorityQueue-gzip-ts">
  <img src="../../assets/badges/priorityQueue-brotli.svg" alt="priorityQueue-brotli-ts">
</p>

## Description

A priority queue is a data structure that stores elements in a specific order.

## Usage

```ts
import { PriorityQueue } from 'gis-tools-ts';

const queue = new PriorityQueue<number>();

queue.push(1);
queue.push(2);

const current = queue.peek(); // 1
console.log(queue.length); // 2
let next = queue.pop(); // 1
console.log(queue.length); // 1
next = queue.pop(); // 2
console.log(queue.length); // 0
```
