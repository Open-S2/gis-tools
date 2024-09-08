import { incirclefast, orient2d } from '../../geometry/predicates';

const EPSILON = Math.pow(2, -52);

/** An incredibly fast and robust Typescript library for Delaunay triangulation of 2D points. */
export default class Delaunator {
  edgeStack = new Array(512);
  coords: number[];
  #triangles: number[];
  #halfedges: number[];
  #hashSize: number;
  #hullPrev: number[];
  #hullNext: number[];
  #hullTri: number[];
  #hullHash: number[];
  #ids: number[];
  #dists: number[];

  #hullStart = 0;
  #cx = 0;
  #cy = 0;

  hull!: number[];
  triangles!: number[];
  halfedges!: number[];

  trianglesLen = 0;

  /**
   * Constructs a delaunay triangulation object given an array of point coordinates of the form:
   * [x0, y0, x1, y1, ...] (use a typed array for best performance).
   * @param coords - flattened array of x,y points. e.g. [x1, y1, x2, y2, ...]
   */
  constructor(coords: number[]) {
    const n = coords.length >> 1;
    this.coords = coords;

    // arrays that will store the triangulation graph
    const maxTriangles = Math.max(2 * n - 5, 0);
    this.#triangles = new Array(maxTriangles * 3);
    this.#halfedges = new Array(maxTriangles * 3);

    // temporary arrays for tracking the edges of the advancing convex hull
    this.#hashSize = Math.ceil(Math.sqrt(n));
    this.#hullPrev = new Array(n); // edge to prev edge
    this.#hullNext = new Array(n); // edge to next edge
    this.#hullTri = new Array(n); // edge to adjacent triangle
    this.#hullHash = new Array(this.#hashSize); // angular edge hash

    // temporary arrays for sorting points
    this.#ids = new Array(n);
    this.#dists = new Array(n);

    this.update();
  }

  /**
   * @param points - flattened array of x,y points. e.g. [x1, y1, x2, y2, ...]
   * @param getX - function to pull in x values for each point
   * @param getY - function to pull in y values for each point
   * @returns - a Delaunator class to do Delaunay triangulation
   */
  static from(
    points: [x: number, y: number][],
    getX = defaultGetX,
    getY = defaultGetY,
  ): Delaunator {
    const n = points.length;
    const coords = new Array(n * 2);

    for (let i = 0; i < n; i++) {
      const p = points[i];
      coords[2 * i] = getX(p);
      coords[2 * i + 1] = getY(p);
    }

    return new Delaunator(coords);
  }

  /**
   * Updates the triangulation if you modified delaunay.coords values in place, avoiding expensive
   * memory allocations. Useful for iterative relaxation algorithms such as [Lloyd's](https://en.wikipedia.org/wiki/Lloyd%27s_algorithm).
   */
  update() {
    const { coords } = this;
    const hullPrev = this.#hullPrev;
    const hullNext = this.#hullNext;
    const hullTri = this.#hullTri;
    const hullHash = this.#hullHash;
    const n = coords.length >> 1;

    // populate an array of point indices; calculate input data bbox
    let minX = Infinity;
    let minY = Infinity;
    let maxX = -Infinity;
    let maxY = -Infinity;

    for (let i = 0; i < n; i++) {
      const x = coords[2 * i];
      const y = coords[2 * i + 1];
      if (x < minX) minX = x;
      if (y < minY) minY = y;
      if (x > maxX) maxX = x;
      if (y > maxY) maxY = y;
      this.#ids[i] = i;
    }
    const cx = (minX + maxX) / 2;
    const cy = (minY + maxY) / 2;

    let i0 = 0;
    let i1 = 0;
    let i2 = 0;

    // pick a seed point close to the center
    for (let i = 0, minDist = Infinity; i < n; i++) {
      const d = dist(cx, cy, coords[2 * i], coords[2 * i + 1]);
      if (d < minDist) {
        i0 = i;
        minDist = d;
      }
    }
    const i0x = coords[2 * i0];
    const i0y = coords[2 * i0 + 1];

    // find the point closest to the seed
    for (let i = 0, minDist = Infinity; i < n; i++) {
      if (i === i0) continue;
      const d = dist(i0x, i0y, coords[2 * i], coords[2 * i + 1]);
      if (d < minDist && d > 0) {
        i1 = i;
        minDist = d;
      }
    }
    let i1x = coords[2 * i1];
    let i1y = coords[2 * i1 + 1];

    let minRadius = Infinity;

    // find the third point which forms the smallest circumcircle with the first two
    for (let i = 0; i < n; i++) {
      if (i === i0 || i === i1) continue;
      const r = circumradius(i0x, i0y, i1x, i1y, coords[2 * i], coords[2 * i + 1]);
      if (r < minRadius) {
        i2 = i;
        minRadius = r;
      }
    }
    let i2x = coords[2 * i2];
    let i2y = coords[2 * i2 + 1];

    if (minRadius === Infinity) {
      // order collinear points by dx (or dy if all x are identical)
      // and return the list as a hull
      for (let i = 0; i < n; i++) {
        this.#dists[i] = coords[2 * i] - coords[0] || coords[2 * i + 1] - coords[1];
      }
      quicksort(this.#ids, this.#dists, 0, n - 1);
      const hull = new Array(n);
      let j = 0;
      for (let i = 0, d0 = -Infinity; i < n; i++) {
        const id = this.#ids[i];
        const d = this.#dists[id];
        if (d > d0) {
          hull[j++] = id;
          d0 = d;
        }
      }
      this.hull = hull.slice(0, j);
      this.triangles = new Array(0);
      this.halfedges = new Array(0);
      return;
    }

    // swap the order of the seed points for counter-clockwise orientation
    if (orient2d(i0x, i0y, i1x, i1y, i2x, i2y) < 0) {
      const i = i1;
      const x = i1x;
      const y = i1y;
      i1 = i2;
      i1x = i2x;
      i1y = i2y;
      i2 = i;
      i2x = x;
      i2y = y;
    }

    const center = circumcenter(i0x, i0y, i1x, i1y, i2x, i2y);
    this.#cx = center.x;
    this.#cy = center.y;

    for (let i = 0; i < n; i++) {
      this.#dists[i] = dist(coords[2 * i], coords[2 * i + 1], center.x, center.y);
    }

    // sort the points by distance from the seed triangle circumcenter
    quicksort(this.#ids, this.#dists, 0, n - 1);

    // set up the seed triangle as the starting hull
    this.#hullStart = i0;
    let hullSize = 3;

    hullNext[i0] = hullPrev[i2] = i1;
    hullNext[i1] = hullPrev[i0] = i2;
    hullNext[i2] = hullPrev[i1] = i0;

    hullTri[i0] = 0;
    hullTri[i1] = 1;
    hullTri[i2] = 2;

    hullHash.fill(-1);
    hullHash[this.#hashKey(i0x, i0y)] = i0;
    hullHash[this.#hashKey(i1x, i1y)] = i1;
    hullHash[this.#hashKey(i2x, i2y)] = i2;

    this.trianglesLen = 0;
    this.#addTriangle(i0, i1, i2, -1, -1, -1);

    for (let k = 0, xp = 0, yp = 0; k < this.#ids.length; k++) {
      const i = this.#ids[k];
      const x = coords[2 * i];
      const y = coords[2 * i + 1];

      // skip near-duplicate points
      if (k > 0 && Math.abs(x - xp) <= EPSILON && Math.abs(y - yp) <= EPSILON) continue;
      xp = x;
      yp = y;

      // skip seed triangle points
      if (i === i0 || i === i1 || i === i2) continue;

      // find a visible edge on the convex hull using edge hash
      let start = 0;
      for (let j = 0, key = this.#hashKey(x, y); j < this.#hashSize; j++) {
        start = hullHash[(key + j) % this.#hashSize];
        if (start !== -1 && start !== hullNext[start]) break;
      }

      start = hullPrev[start];
      let e = start,
        q;
      while (
        ((q = hullNext[e]),
        orient2d(x, y, coords[2 * e], coords[2 * e + 1], coords[2 * q], coords[2 * q + 1]) >= 0)
      ) {
        e = q;
        if (e === start) {
          e = -1;
          break;
        }
      }
      if (e === -1) continue; // likely a near-duplicate point; skip it

      // add the first triangle from the point
      let t = this.#addTriangle(e, i, hullNext[e], -1, -1, hullTri[e]);

      // recursively flip triangles from the point until they satisfy the Delaunay condition
      hullTri[i] = this.#legalize(t + 2);
      hullTri[e] = t; // keep track of boundary triangles on the hull
      hullSize++;

      // walk forward through the hull, adding more triangles and flipping recursively
      let n = hullNext[e];
      while (
        ((q = hullNext[n]),
        orient2d(x, y, coords[2 * n], coords[2 * n + 1], coords[2 * q], coords[2 * q + 1]) < 0)
      ) {
        t = this.#addTriangle(n, i, q, hullTri[i], -1, hullTri[n]);
        hullTri[i] = this.#legalize(t + 2);
        hullNext[n] = n; // mark as removed
        hullSize--;
        n = q;
      }

      // walk backward from the other side, adding more triangles and flipping
      if (e === start) {
        while (
          ((q = hullPrev[e]),
          orient2d(x, y, coords[2 * q], coords[2 * q + 1], coords[2 * e], coords[2 * e + 1]) < 0)
        ) {
          t = this.#addTriangle(q, i, e, -1, hullTri[e], hullTri[q]);
          this.#legalize(t + 2);
          hullTri[q] = t;
          hullNext[e] = e; // mark as removed
          hullSize--;
          e = q;
        }
      }

      // update the hull indices
      this.#hullStart = hullPrev[i] = e;
      hullNext[e] = hullPrev[n] = i;
      hullNext[i] = n;

      // save the two new edges in the hash table
      hullHash[this.#hashKey(x, y)] = i;
      hullHash[this.#hashKey(coords[2 * e], coords[2 * e + 1])] = e;
    }

    this.hull = new Array(hullSize);
    for (let i = 0, e = this.#hullStart; i < hullSize; i++) {
      this.hull[i] = e;
      e = hullNext[e];
    }

    // trim typed triangle mesh arrays
    this.triangles = this.#triangles.slice(0, this.trianglesLen);
    this.#halfedges = this.#halfedges.slice(0, this.trianglesLen);
  }

  /**
   * @param x - x coordinate
   * @param y - y coordinate
   * @returns - a hash value corresponding to the point (x, y)
   */
  #hashKey(x: number, y: number) {
    return Math.floor(pseudoAngle(x - this.#cx, y - this.#cy) * this.#hashSize) % this.#hashSize;
  }

  /**
   * @param a - index of triangle vertex
   * @returns - index of previous triangle vertex
   */
  #legalize(a: number): number {
    const { coords } = this;
    const triangles = this.#triangles;
    const halfedges = this.#halfedges;

    let i = 0;
    let ar = 0;

    // recursion eliminated with a fixed-size stack
    while (true) {
      const b = halfedges[a];

      /* if the pair of triangles doesn't satisfy the Delaunay condition
       * (p1 is inside the circumcircle of [p0, pl, pr]), flip them,
       * then do the same check/flip recursively for the new pair of triangles
       *
       *           pl                    pl
       *          /||\                  /  \
       *       al/ || \bl            al/    \a
       *        /  ||  \              /      \
       *       /  a||b  \    flip    /___ar___\
       *     p0\   ||   /p1   =>   p0\---bl---/p1
       *        \  ||  /              \      /
       *       ar\ || /br             b\    /br
       *          \||/                  \  /
       *           pr                    pr
       */
      const a0 = a - (a % 3);
      ar = a0 + ((a + 2) % 3);

      if (b === -1) {
        // convex hull edge
        if (i === 0) break;
        a = this.edgeStack[--i];
        continue;
      }

      const b0 = b - (b % 3);
      const al = a0 + ((a + 1) % 3);
      const bl = b0 + ((b + 2) % 3);

      const p0 = triangles[ar];
      const pr = triangles[a];
      const pl = triangles[al];
      const p1 = triangles[bl];

      const illegal =
        incirclefast(
          coords[2 * p0],
          coords[2 * p0 + 1],
          coords[2 * pr],
          coords[2 * pr + 1],
          coords[2 * pl],
          coords[2 * pl + 1],
          coords[2 * p1],
          coords[2 * p1 + 1],
        ) < 0;

      if (illegal) {
        triangles[a] = p1;
        triangles[b] = p0;

        const hbl = halfedges[bl];

        // edge swapped on the other side of the hull (rare); fix the halfedge reference
        if (hbl === -1) {
          let e = this.#hullStart;
          do {
            if (this.#hullTri[e] === bl) {
              this.#hullTri[e] = a;
              break;
            }
            e = this.#hullPrev[e];
          } while (e !== this.#hullStart);
        }
        this.#link(a, hbl);
        this.#link(b, halfedges[ar]);
        this.#link(ar, bl);

        const br = b0 + ((b + 1) % 3);

        // don't worry about hitting the cap: it can only happen on extremely degenerate input
        if (i < this.edgeStack.length) {
          this.edgeStack[i++] = br;
        }
      } else {
        if (i === 0) break;
        a = this.edgeStack[--i];
      }
    }

    return ar;
  }

  /**
   * @param a - index of triangle vertex
   * @param b - index of next triangle vertex
   */
  #link(a: number, b: number): void {
    this.#halfedges[a] = b;
    if (b !== -1) this.#halfedges[b] = a;
  }

  /**
   * add a new triangle given vertex indices and adjacent half-edge ids
   * @param i0 - index of triangle vertex
   * @param i1 - index of next triangle vertex
   * @param i2 - index of previous triangle vertex
   * @param a - adjacent half-edge id
   * @param b - adjacent half-edge id
   * @param c - adjacent half-edge id
   * @returns - index of new triangle
   */
  #addTriangle(i0: number, i1: number, i2: number, a: number, b: number, c: number): number {
    const t = this.trianglesLen;

    this.#triangles[t] = i0;
    this.#triangles[t + 1] = i1;
    this.#triangles[t + 2] = i2;

    this.#link(t, a);
    this.#link(t + 1, b);
    this.#link(t + 2, c);

    this.trianglesLen += 3;

    return t;
  }
}

/**
 * monotonically increases with real angle, but doesn't need expensive trigonometry
 * @param dx - delta x
 * @param dy - delta y
 * @returns - angle
 */
function pseudoAngle(dx: number, dy: number): number {
  const p = dx / (Math.abs(dx) + Math.abs(dy));
  return (dy > 0 ? 3 - p : 1 + p) / 4; // [0..1]
}

/**
 * @param ax - x coordinate of first point
 * @param ay - y coordinate of first point
 * @param bx - x coordinate of second point
 * @param by - y coordinate of second point
 * @returns - squared distance between the two points
 */
function dist(ax: number, ay: number, bx: number, by: number): number {
  const dx = ax - bx;
  const dy = ay - by;
  return dx * dx + dy * dy;
}

/**
 * @param ax - x coordinate of first point
 * @param ay - y coordinate of first point
 * @param bx - x coordinate of second point
 * @param by - y coordinate of second point
 * @param cx - x coordinate of third point
 * @param cy - y coordinate of third point
 * @returns - squared radius of the circumscribed circle
 */
function circumradius(
  ax: number,
  ay: number,
  bx: number,
  by: number,
  cx: number,
  cy: number,
): number {
  const dx = bx - ax;
  const dy = by - ay;
  const ex = cx - ax;
  const ey = cy - ay;

  const bl = dx * dx + dy * dy;
  const cl = ex * ex + ey * ey;
  const d = 0.5 / (dx * ey - dy * ex);

  const x = (ey * bl - dy * cl) * d;
  const y = (dx * cl - ex * bl) * d;

  return x * x + y * y;
}

/**
 * @param ax - x coordinate of first point
 * @param ay - y coordinate of first point
 * @param bx - x coordinate of second point
 * @param by - y coordinate of second point
 * @param cx - x coordinate of third point
 * @param cy - y coordinate of third point
 * @returns - center of the circumscribed circle
 */
function circumcenter(
  ax: number,
  ay: number,
  bx: number,
  by: number,
  cx: number,
  cy: number,
): { x: number; y: number } {
  const dx = bx - ax;
  const dy = by - ay;
  const ex = cx - ax;
  const ey = cy - ay;

  const bl = dx * dx + dy * dy;
  const cl = ex * ex + ey * ey;
  const d = 0.5 / (dx * ey - dy * ex);

  const x = ax + (ey * bl - dy * cl) * d;
  const y = ay + (dx * cl - ex * bl) * d;

  return { x, y };
}

/**
 * @param ids - array of point ids
 * @param dists - array of point distances
 * @param left - leftmost index
 * @param right - rightmost index
 */
function quicksort(ids: number[], dists: number[], left: number, right: number): void {
  if (right - left <= 20) {
    for (let i = left + 1; i <= right; i++) {
      const temp = ids[i];
      const tempDist = dists[temp];
      let j = i - 1;
      while (j >= left && dists[ids[j]] > tempDist) ids[j + 1] = ids[j--];
      ids[j + 1] = temp;
    }
  } else {
    const median = (left + right) >> 1;
    let i = left + 1;
    let j = right;
    swap(ids, median, i);
    if (dists[ids[left]] > dists[ids[right]]) swap(ids, left, right);
    if (dists[ids[i]] > dists[ids[right]]) swap(ids, i, right);
    if (dists[ids[left]] > dists[ids[i]]) swap(ids, left, i);

    const temp = ids[i];
    const tempDist = dists[temp];
    while (true) {
      do i++;
      while (dists[ids[i]] < tempDist);
      do j--;
      while (dists[ids[j]] > tempDist);
      if (j < i) break;
      swap(ids, i, j);
    }
    ids[left + 1] = ids[j];
    ids[j] = temp;

    if (right - i + 1 >= j - left) {
      quicksort(ids, dists, i, right);
      quicksort(ids, dists, left, j - 1);
    } else {
      quicksort(ids, dists, left, j - 1);
      quicksort(ids, dists, i, right);
    }
  }
}

/**
 * @param arr - array of numbers to swap
 * @param i - index at position i
 * @param j - index at position j
 */
function swap(arr: number[], i: number, j: number): void {
  const tmp = arr[i];
  arr[i] = arr[j];
  arr[j] = tmp;
}

/**
 * @param p - point: [x, y]
 * @returns - x
 */
function defaultGetX(p: [x: number, y: number]): number {
  return p[0];
}
/**
 * @param p - point: [x, y]
 * @returns - y
 */
function defaultGetY(p: [x: number, y: number]): number {
  return p[1];
}
