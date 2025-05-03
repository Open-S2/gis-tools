use alloc::{vec, vec::Vec};
use core::f64;
use geometry::{incirclefast, orient2d};
use libm::{ceil, fabs, floor, sqrt};
use s2json::{Point, VectorPoint};

/// # NO_REF means it's not pointing to a location in memory
pub static NO_REF: usize = usize::MAX;

/// # Delaunator
///
/// ## Description
/// An incredibly fast and robust Typescript library for Delaunay triangulation of 2D points.
///
/// ## Links
/// - https://en.wikipedia.org/wiki/Delaunay_triangulation
#[derive(Debug)]
pub struct Delaunator {
    edge_stack: Vec<usize>,
    /// coordinates of each point
    pub coords: Vec<f64>,
    /// indexes to each triangle. (triangle[i * 3], triangle[(i * 3) + 1], triangle[(i * 3) + 2])
    /// makes a triangle
    pub triangles: Vec<usize>,
    /// indexes to each half edge. (halfedge[i], halfedge[(i + 1) % 3], halfedge[(i + 2) % 3])
    pub halfedges: Vec<usize>,
    hash_size: usize,
    hull_prev: Vec<usize>,
    hull_next: Vec<usize>,
    hull_tri: Vec<usize>,
    hull_hash: Vec<usize>,
    ids: Vec<usize>,
    dists: Vec<f64>,
    hull_start: usize,
    cx: f64,
    cy: f64,
    /// indexes to each point on the convex hull
    pub hull: Vec<usize>,
    /// length of the triangles array
    pub triangles_len: usize,
}
impl Delaunator {
    /// Constructs a delaunay triangulation object given an array of point coordinates of the form:
    /// [x0, y0, x1, y1, ...] (use a typed array for best performance).
    /// @param coords - flattened array of x,y points. e.g. [x1, y1, x2, y2, ...]
    pub fn new(coords: Vec<f64>) -> Delaunator {
        let n = coords.len() >> 1;
        let max_triangles: usize = if n < 3 { 0 } else { usize::max(2 * n - 5, 0) };
        let hash_size = ceil(sqrt(n as f64)) as usize;

        let mut del = Delaunator {
            coords,
            // arrays that will store the triangulation graph
            triangles: vec![0; max_triangles * 3],
            halfedges: vec![0; max_triangles * 3],
            // temporary arrays for tracking the edges of the advancing convex hull
            hash_size,
            hull_prev: vec![0; n],         // edge to prev edge
            hull_next: vec![0; n],         // edge to next edge
            hull_tri: vec![0; n],          // edge to adjacent triangle
            hull_hash: vec![0; hash_size], // angular edge hash
            // temporary arrays for sorting points
            ids: vec![0; n],
            dists: vec![0.; n],
            // setup the initial hull
            edge_stack: vec![0; 512],
            hull: vec![],
            hull_start: 0,
            cx: 0.0,
            cy: 0.0,
            triangles_len: 0,
        };

        del.update();

        del
    }

    /// Given a flattened array of x,y points. e.g. [[x1, y1], [x2, y2], ...]
    /// return a Delaunator class to do Delaunay triangulation
    pub fn from_points(points: &[Point]) -> Delaunator {
        let n = points.len();
        let mut coords = vec![0.; n * 2];

        for i in 0..n {
            let Point(x, y) = points[i];
            coords[2 * i] = x;
            coords[2 * i + 1] = y;
        }

        Delaunator::new(coords)
    }

    /// @param points - flattened array of x,y vector points. e.g. [{ x1, y1 }, { x2, y2 }, ...]
    /// @returns - a Delaunator class to do Delaunay triangulation
    pub fn from_vector_points<M: Clone>(points: &[VectorPoint<M>]) -> Delaunator {
        let n = points.len();
        let mut coords = vec![0.; n * 2];

        for i in 0..n {
            let VectorPoint { x, y, .. } = &points[i];
            coords[2 * i] = *x;
            coords[2 * i + 1] = *y;
        }

        Delaunator::new(coords)
    }

    /// Updates the triangulation if you modified delaunay.coords values in place, avoiding expensive
    /// memory allocations. Useful for iterative relaxation algorithms such as
    /// [Lloyd's](https://en.wikipedia.org/wiki/Lloyd%27s_algorithm).
    pub fn update(&mut self) {
        let n = self.coords.len() >> 1;
        if n == 0 {
            return;
        }
        let epsilon = f64::EPSILON * 2.;

        // populate an array of point indices; calculate input data bbox
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for i in 0..n {
            let x = self.coords[2 * i];
            let y = self.coords[2 * i + 1];
            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
            self.ids[i] = i;
        }
        let cx = (min_x + max_x) / 2.;
        let cy = (min_y + max_y) / 2.;

        let mut i0 = 0;
        let mut i1 = 0;
        let mut i2 = 0;

        // pick a seed point close to the center
        let mut min_dist = f64::INFINITY;
        for i in 0..n {
            let d = dist(cx, cy, self.coords[2 * i], self.coords[2 * i + 1]);
            if d < min_dist {
                i0 = i;
                min_dist = d;
            }
        }
        let i0x = self.coords[2 * i0];
        let i0y = self.coords[2 * i0 + 1];

        // find the point closest to the seed
        min_dist = f64::INFINITY;
        for i in 0..n {
            if i == i0 {
                continue;
            }
            let d = dist(i0x, i0y, self.coords[2 * i], self.coords[2 * i + 1]);
            if d < min_dist && d > 0. {
                i1 = i;
                min_dist = d;
            }
        }
        let mut i1x = self.coords[2 * i1];
        let mut i1y = self.coords[2 * i1 + 1];

        let mut min_radius = f64::INFINITY;
        // find the third point which forms the smallest circumcircle with the first two
        for i in 0..n {
            if i == i0 || i == i1 {
                continue;
            }
            let r = circumradius(i0x, i0y, i1x, i1y, self.coords[2 * i], self.coords[2 * i + 1]);
            if r < min_radius {
                i2 = i;
                min_radius = r;
            }
        }
        let mut i2x = self.coords[2 * i2];
        let mut i2y = self.coords[2 * i2 + 1];

        if min_radius == f64::INFINITY {
            // order collinear points by dx (or dy if all x are identical)
            // and return the list as a hull
            for i in 0..n {
                let dx = self.coords[2 * i] - self.coords[0];
                let dy = self.coords[2 * i + 1] - self.coords[1];
                self.dists[i] = if dx > 0. { dx } else { dy };
            }
            quicksort(&mut self.ids, &mut self.dists, 0, n - 1);
            let mut hull = vec![0; n];
            let mut j = 0;
            let mut d0 = f64::NEG_INFINITY;
            for i in 0..n {
                let id = self.ids[i];
                let d = self.dists[id];
                if d > d0 {
                    hull[j] = id;
                    j += 1;
                    d0 = d;
                }
            }
            self.hull = hull[0..j].to_vec();
            self.triangles = vec![];
            self.halfedges = vec![];
            return;
        }

        // swap the order of the seed points for counter-clockwise orientation
        if orient2d(i0x, i0y, i1x, i1y, i2x, i2y) < 0. {
            let i = i1;
            let x = i1x;
            let y = i1y;
            i1 = i2;
            i1x = i2x;
            i1y = i2y;
            i2 = i;
            i2x = x;
            i2y = y;
        }

        let center = circumcenter(i0x, i0y, i1x, i1y, i2x, i2y);
        (self.cx, self.cy) = center;

        for i in 0..n {
            self.dists[i] = dist(self.coords[2 * i], self.coords[2 * i + 1], center.0, center.1);
        }

        // sort the points by distance from the seed triangle circumcenter
        quicksort(&mut self.ids, &mut self.dists, 0, n - 1);

        // set up the seed triangle as the starting hull
        self.hull_start = i0;
        let mut hull_size = 3;

        self.hull_prev[i2] = i1;
        self.hull_next[i0] = i1;
        self.hull_prev[i0] = i2;
        self.hull_next[i1] = i2;
        self.hull_prev[i1] = i0;
        self.hull_next[i2] = i0;

        self.hull_tri[i0] = 0;
        self.hull_tri[i1] = 1;
        self.hull_tri[i2] = 2;

        self.hull_hash.fill(NO_REF);
        let io_hash = self.hash_key(i0x, i0y);
        self.hull_hash[io_hash] = i0;
        let i1_hash = self.hash_key(i1x, i1y);
        self.hull_hash[i1_hash] = i1;
        let i2_hash = self.hash_key(i2x, i2y);
        self.hull_hash[i2_hash] = i2;

        self.triangles_len = 0;
        self.add_triangle(i0, i1, i2, NO_REF, NO_REF, NO_REF);

        let mut xp = 0.;
        let mut yp = 0.;
        for k in 0..self.ids.len() {
            let i = self.ids[k];
            let x = self.coords[2 * i];
            let y = self.coords[2 * i + 1];

            // skip near-duplicate points
            if k > 0 && fabs(x - xp) <= epsilon && fabs(y - yp) <= epsilon {
                continue;
            }
            xp = x;
            yp = y;

            // skip seed triangle points
            if i == i0 || i == i1 || i == i2 {
                continue;
            }

            // find a visible edge on the convex hull using edge hash
            let mut start = 0;
            let key = self.hash_key(x, y);
            //   for (let j = 0, key = self.hash_key(x, y); j < self.hash_size; j++) {
            for j in 0..self.hash_size {
                start = self.hull_hash[(key + j) % self.hash_size];
                if start != NO_REF && start != self.hull_next[start] {
                    break;
                }
            }

            start = self.hull_prev[start];
            let mut e = start;
            let mut q;
            while {
                q = self.hull_next[e];
                orient2d(
                    x,
                    y,
                    self.coords[2 * e],
                    self.coords[2 * e + 1],
                    self.coords[2 * q],
                    self.coords[2 * q + 1],
                ) >= 0.
            } {
                e = q;
                if e == start {
                    e = NO_REF;
                    break;
                }
            }
            if e == NO_REF {
                continue;
            } // likely a near-duplicate point; skip it

            // add the first triangle from the point
            let mut t =
                self.add_triangle(e, i, self.hull_next[e], NO_REF, NO_REF, self.hull_tri[e]);

            // recursively flip triangles from the point until they satisfy the Delaunay condition
            self.hull_tri[i] = self.legalize(t + 2);
            self.hull_tri[e] = t; // keep track of boundary triangles on the hull
            hull_size += 1;

            // walk forward through the hull, adding more triangles and flipping recursively
            let mut n = self.hull_next[e];
            while {
                q = self.hull_next[n];
                orient2d(
                    x,
                    y,
                    self.coords[2 * n],
                    self.coords[2 * n + 1],
                    self.coords[2 * q],
                    self.coords[2 * q + 1],
                ) < 0.
            } {
                t = self.add_triangle(n, i, q, self.hull_tri[i], NO_REF, self.hull_tri[n]);
                self.hull_tri[i] = self.legalize(t + 2);
                self.hull_next[n] = n; // mark as removed
                hull_size -= 1;
                n = q;
            }

            // walk backward from the other side, adding more triangles and flipping
            if e == start {
                while {
                    q = self.hull_prev[e];
                    orient2d(
                        x,
                        y,
                        self.coords[2 * q],
                        self.coords[2 * q + 1],
                        self.coords[2 * e],
                        self.coords[2 * e + 1],
                    ) < 0.
                } {
                    t = self.add_triangle(q, i, e, NO_REF, self.hull_tri[e], self.hull_tri[q]);
                    self.legalize(t + 2);
                    self.hull_tri[q] = t;
                    self.hull_next[e] = e; // mark as removed
                    hull_size -= 1;
                    e = q;
                }
            }

            // update the hull indices
            self.hull_prev[i] = e;
            self.hull_start = e;
            self.hull_prev[n] = i;
            self.hull_next[e] = i;
            self.hull_next[i] = n;

            // save the two new edges in the hash table
            let i_hash = self.hash_key(x, y);
            self.hull_hash[i_hash] = i;
            let e_hash = self.hash_key(self.coords[2 * e], self.coords[2 * e + 1]);
            self.hull_hash[e_hash] = e;
        }

        self.hull = vec![0; hull_size];
        let mut e = self.hull_start;
        for i in 0..hull_size {
            self.hull[i] = e;
            e = self.hull_next[e];
        }

        // trim typed triangle mesh arrays
        self.triangles = self.triangles[0..self.triangles_len].to_vec();
        self.halfedges = self.halfedges[0..self.triangles_len].to_vec();
    }

    /// @param x - x coordinate
    /// @param y - y coordinate
    /// @returns - a hash value corresponding to the point (x, y)
    fn hash_key(&self, x: f64, y: f64) -> usize {
        let hash_size = self.hash_size as f64;
        (floor(pseudo_angle(x - self.cx, y - self.cy) * hash_size) % hash_size) as usize
    }

    /// given an index of triangle vertex
    /// returns the index of previous triangle vertex
    fn legalize(&mut self, mut a: usize) -> usize {
        let mut i = 0;

        // recursion eliminated with a fixed-size stack
        loop {
            let b = self.halfedges[a];

            // if the pair of triangles doesn't satisfy the Delaunay condition
            // (p1 is inside the circumcircle of [p0, pl, pr]), flip them,
            // then do the same check/flip recursively for the new pair of triangles
            //
            //           pl                    pl
            //          /||\                  /  \
            //       al/ || \bl            al/    \a
            //        /  ||  \              /      \
            //       /  a||b  \    flip    /___ar___\
            //     p0\   ||   /p1   =>   p0\---bl---/p1
            //        \  ||  /              \      /
            //       ar\ || /br             b\    /br
            //          \||/                  \  /
            //           pr                    pr
            let a0 = a - (a % 3);
            let ar = a0 + ((a + 2) % 3);

            if b == NO_REF {
                // convex hull edge
                if i == 0 {
                    return ar;
                }
                i -= 1;
                a = self.edge_stack[i];
                continue;
            }

            let b0 = b - (b % 3);
            let al = a0 + ((a + 1) % 3);
            let bl = b0 + ((b + 2) % 3);

            let p0 = self.triangles[ar];
            let pr = self.triangles[a];
            let pl = self.triangles[al];
            let p1 = self.triangles[bl];

            let illegal = incirclefast(
                self.coords[2 * p0],
                self.coords[2 * p0 + 1],
                self.coords[2 * pr],
                self.coords[2 * pr + 1],
                self.coords[2 * pl],
                self.coords[2 * pl + 1],
                self.coords[2 * p1],
                self.coords[2 * p1 + 1],
            ) < 0.;

            if illegal {
                self.triangles[a] = p1;
                self.triangles[b] = p0;

                let hbl = self.halfedges[bl];

                // edge swapped on the other side of the hull (rare); fix the halfedge reference
                if hbl == NO_REF {
                    let mut e = self.hull_start;
                    loop {
                        if self.hull_tri[e] == bl {
                            self.hull_tri[e] = a;
                            break;
                        }
                        e = self.hull_prev[e];
                        if e == self.hull_start {
                            break;
                        }
                    }
                }
                self.link(a, hbl);
                self.link(b, self.halfedges[ar]);
                self.link(ar, bl);

                let br = b0 + ((b + 1) % 3);

                // don't worry about hitting the cap: it can only happen on extremely degenerate input
                if i < self.edge_stack.len() {
                    self.edge_stack[i] = br;
                    i += 1;
                }
            } else {
                if i == 0 {
                    return ar;
                }
                i -= 1;
                a = self.edge_stack[i];
            }
        }
    }

    /// @param a - index of triangle vertex
    /// @param b - index of next triangle vertex
    fn link(&mut self, a: usize, b: usize) {
        self.halfedges[a] = b;
        if b != NO_REF {
            self.halfedges[b] = a;
        }
    }

    /// add a new triangle given vertex indices and adjacent half-edge ids
    /// @param i0 - index of triangle vertex
    /// @param i1 - index of next triangle vertex
    /// @param i2 - index of previous triangle vertex
    /// @param a - adjacent half-edge id
    /// @param b - adjacent half-edge id
    /// @param c - adjacent half-edge id
    /// @returns - index of new triangle
    fn add_triangle(
        &mut self,
        i0: usize,
        i1: usize,
        i2: usize,
        a: usize,
        b: usize,
        c: usize,
    ) -> usize {
        let t = self.triangles_len;
        if t + 3 >= self.triangles.len() {
            self.triangles.resize((t + 3) * 2, NO_REF);
            self.halfedges.resize((t + 3) * 2, NO_REF);
        }

        self.triangles[t] = i0;
        self.triangles[t + 1] = i1;
        self.triangles[t + 2] = i2;

        self.link(t, a);
        self.link(t + 1, b);
        self.link(t + 2, c);

        self.triangles_len += 3;

        t
    }
}

/// monotonically increases with real angle, but doesn't need expensive trigonometry
/// returns the pseudo angle
fn pseudo_angle(dx: f64, dy: f64) -> f64 {
    let p = dx / (fabs(dx) + fabs(dy));
    (if dy > 0. { 3. - p } else { 1. + p }) / 4. // [0..1]
}

/// returns the squared distance between the two points
fn dist(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    let dx = ax - bx;
    let dy = ay - by;
    dx * dx + dy * dy
}

/// returns the squared radius of the circumscribed circle
fn circumradius(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> f64 {
    let dx = bx - ax;
    let dy = by - ay;
    let ex = cx - ax;
    let ey = cy - ay;

    let bl = dx * dx + dy * dy;
    let cl = ex * ex + ey * ey;
    let d = 0.5 / (dx * ey - dy * ex);

    let x = (ey * bl - dy * cl) * d;
    let y = (dx * cl - ex * bl) * d;

    x * x + y * y
}

/// A Voronoi diagram is built by connecting the Delaunay triangle circumcenters together using the
/// dual of the Delaunay graph.
/// 1. Calculate the circumcenters of each triangle
/// 2. Construct the Voronoi edges from two circumcenters
/// 3. Connect the edges into Voronoi cells
fn circumcenter(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> (f64, f64) {
    let dx = bx - ax;
    let dy = by - ay;
    let ex = cx - ax;
    let ey = cy - ay;

    let bl = dx * dx + dy * dy;
    let cl = ex * ex + ey * ey;
    let d = 0.5 / (dx * ey - dy * ex);

    let x = ax + (ey * bl - dy * cl) * d;
    let y = ay + (dx * cl - ex * bl) * d;

    (x, y)
}

/// sort both the points and ids at the same time
fn quicksort(ids: &mut [usize], dists: &mut [f64], left: usize, right: usize) {
    if right - left <= 20 {
        for i in left + 1..=right {
            let temp = ids[i];
            let temp_dist = dists[temp];
            let mut j = i - 1;
            while j >= left && dists[ids[j]] > temp_dist {
                ids[j + 1] = ids[j];
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            ids[j + 1] = temp;
        }
    } else {
        let median = (left + right) >> 1;
        let mut i = left + 1;
        let mut j = right;
        ids.swap(median, i);
        if dists[ids[left]] > dists[ids[right]] {
            ids.swap(left, right);
        }
        if dists[ids[i]] > dists[ids[right]] {
            ids.swap(i, right);
        }
        if dists[ids[left]] > dists[ids[i]] {
            ids.swap(left, i);
        }

        let temp = ids[i];
        let temp_dist = dists[temp];
        loop {
            loop {
                i += 1;
                if dists[ids[i]] >= temp_dist {
                    break;
                }
            }
            loop {
                if j == 0 {
                    break;
                }
                j -= 1;
                if dists[ids[j]] <= temp_dist {
                    break;
                }
            }
            if j < i {
                break;
            }
            ids.swap(i, j);
        }
        ids[left + 1] = ids[j];
        ids[j] = temp;

        if right - i + 1 >= j - left {
            quicksort(ids, dists, i, right);
            quicksort(ids, dists, left, j - 1);
        } else {
            quicksort(ids, dists, left, j - 1);
            quicksort(ids, dists, i, right);
        }
    }
}
