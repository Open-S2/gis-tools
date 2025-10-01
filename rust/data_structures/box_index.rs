use crate::{
    data_structures::FlatQueue,
    geometry::{LonLat, S2CellId, S2Point},
};
use alloc::{vec, vec::Vec};
use core::{cmp::max, ops::Div};
use libm::{floor, fmax, fmin};
use s2json::{BBox, GetXY, VectorPoint};

const HILBERT_MAX: f64 = ((1 << 16) - 1) as f64;

// TODO: boxes (Vec<f64>) could be simplifed to an actual BBox and sorting would be simpler

/// # Box Index Accessor
///
/// ## Description
/// Designed to be used with the BoxIndex.
///
/// Has [`BoxIndexAccessor::bbox`] and optionally updated [`BoxIndexAccessor::hilbert`] to properly index the item
/// for the [`BoxIndex`]. hilbert is auto-generated for items whre you only supplied the [`BoxIndexAccessor::bbox`].
/// Note that [`LonLat`] and [`S2Point`] implements the [`S2CellId`] hilbert curve for sorting.
pub trait BoxIndexAccessor {
    /// Get the bounding box of the item
    fn bbox(&self) -> BBox;
    /// Get the hilbert value of the item
    fn hilbert(&self, index_bbox: &BBox) -> u64 {
        // lets default to the local hilbert here (can use s2 later)
        let width = index_bbox.right - index_bbox.left;
        let height = index_bbox.top - index_bbox.bottom;
        let BBox { left, bottom, right, top } = self.bbox();
        let x = floor((HILBERT_MAX * ((left + right) / 2. - index_bbox.left)) / width) as u32;
        let y = floor((HILBERT_MAX * ((bottom + top) / 2. - index_bbox.bottom)) / height) as u32;
        _hilbert(x, y) as u64
    }
}
impl<M: Clone + Default> BoxIndexAccessor for LonLat<M> {
    fn bbox(&self) -> BBox {
        BBox::new(self.lon(), self.lat(), self.lon(), self.lat())
    }
    fn hilbert(&self, _index_bbox: &BBox) -> u64 {
        let s2cell: S2CellId = self.into();
        s2cell.id
    }
}
impl<M: Clone + Default> BoxIndexAccessor for VectorPoint<M> {
    fn bbox(&self) -> BBox {
        BBox::new(self.x, self.y, self.x, self.y)
    }
    fn hilbert(&self, _index_bbox: &BBox) -> u64 {
        let s2cell: S2CellId = self.into();
        s2cell.id
    }
}
impl BoxIndexAccessor for S2Point {
    fn bbox(&self) -> BBox {
        let ll: LonLat = self.into();
        BBox::new(ll.lon(), ll.lat(), ll.lon(), ll.lat())
    }
    fn hilbert(&self, _index_bbox: &BBox) -> u64 {
        let s2cell: S2CellId = self.into();
        s2cell.id
    }
}

/// # BoxIndex
///
/// ## Description
/// An Index for points and rectangles
///
/// A really fast static spatial index for 2D points and rectangles in JavaScript.
/// Uses either a fast simple Hilbert curve algorithm or a more complex Hilbert curve (S2) algorithm.
///
/// This is a partial port/rust port of the [flatbush](https://github.com/mourner/flatbush)
/// codebase. The port is designed to work with this library and also S2 cells.
///
/// The default hilbert curve ported is simpler and originally designed for simpler datasets.
/// This library also provides the S2 hilbert curve for cm precision of the Earth using S2 cells.
///
/// ## Usage
/// ```rust
/// use gistools::data_structures::{BoxIndex, BoxIndexAccessor};
/// use gistools::proj::Coords;
/// use s2json::BBox;
///
/// // Test item
/// #[derive(Debug, Clone, PartialEq)]
/// struct Item {
///     pub id: usize,
///     pub min_x: f64,
///     pub min_y: f64,
///     pub max_x: f64,
///     pub max_y: f64,
/// }
/// // define how to access the min_x, min_y, max_x, and max_y properties
/// impl BoxIndexAccessor for Item {
///     fn bbox(&self) -> BBox {
///         BBox::new(self.min_x, self.min_y, self.max_x, self.max_y)
///     }
/// }
///
/// // create the index
/// let mut index = BoxIndex::<Item>::new(vec![], None);
///
/// // make a bounding box query
/// let items = index.search(&BBox::new(40.0, 40.0, 60.0, 60.0), None::<fn(&Item) -> bool>);
///
/// // make a k-nearest-neighbors query
/// let items =
///     index.neighbors(Coords::new_xy(50., 50.), Some(3), None, None::<fn(&Item) -> bool>);
/// ```
///
/// ## Links
/// - <https://github.com/mourner/flatbush>
#[derive(Debug, Clone, PartialEq)]
pub struct BoxIndex<T: BoxIndexAccessor + Clone> {
    node_size: usize,
    num_items: usize,
    level_bounds: Vec<usize>,
    pos: usize,
    /// full bounding box of the index
    pub bbox: BBox,
    boxes: Vec<f64>,
    indices: Vec<usize>,
    items: Vec<T>,
}
impl<T: BoxIndexAccessor + Clone> BoxIndex<T> {
    /// Create a BoxIndex index that will hold a given number of items.
    ///
    /// ## Parameters
    /// - `items`: The items to index. Requires the item type to implement [`BoxIndexAccessor`].
    /// - `accessor`: A function for accessing the min_x, min_y, max_x, and max_y properties of the items.
    /// - `[node_size]`: Size of the tree node (16 by default).
    pub fn new(items: Vec<T>, node_size: Option<usize>) -> Self {
        let node_size = usize::min(usize::max(node_size.unwrap_or(16), 2), 65_535);
        let num_items = items.len();
        // calculate the total number of nodes in the R-tree to allocate space for
        // and the index of each tree level (used in search later)
        let mut n = num_items;
        let mut num_nodes = n;
        let mut level_bounds = vec![n * 4];
        if num_items > 0 {
            loop {
                n = n.div_ceil(node_size);
                num_nodes += n;
                level_bounds.push(num_nodes * 4);
                if n == 1 {
                    break;
                }
            }
        }

        let mut box_index = Self {
            node_size,
            num_items,
            level_bounds,
            pos: 0,
            bbox: BBox::default(),
            boxes: vec![0.0; num_nodes * 4],
            indices: vec![0; num_nodes],
            items,
        };

        box_index.insert_items();
        box_index.index();

        box_index
    }

    /// Insert all a given rectangle to the boxes and indices arrays
    fn insert_items(&mut self) {
        for item in &self.items {
            let index = self.pos >> 2;
            let boxes = &mut self.boxes;
            self.indices[index] = index;
            let item_bbox = item.bbox();
            self.bbox.merge_in_place(&item_bbox);
            let BBox { left, bottom, right, top } = item_bbox;
            boxes[self.pos] = left;
            boxes[self.pos + 1] = bottom;
            boxes[self.pos + 2] = right;
            boxes[self.pos + 3] = top;
            self.pos += 4;
        }
    }

    /// Perform indexing of the added rectangles/points after all data has been added about the
    /// items.
    pub fn index(&mut self) {
        if self.items.is_empty() {
            return;
        }
        if self.pos >> 2 != self.num_items {
            panic!("Added {} items when expected {}.", self.pos >> 2, self.num_items);
        }
        let boxes = &mut self.boxes;

        if self.num_items <= self.node_size {
            // only one node, skip sorting and just fill the root box
            boxes[self.pos] = self.bbox.left;
            self.pos += 1;
            boxes[self.pos] = self.bbox.bottom;
            self.pos += 1;
            boxes[self.pos] = self.bbox.right;
            self.pos += 1;
            boxes[self.pos] = self.bbox.top;
            self.pos += 1;
            return;
        }

        let mut hilbert_values = vec![0; self.num_items];
        // map item centers into Hilbert coordinate space and calculate Hilbert values
        for (i, item) in self.items.iter().enumerate() {
            hilbert_values[i] = item.hilbert(&self.bbox);
        }

        // sort items by their Hilbert value (for packing later)
        sort(&mut hilbert_values, boxes, &mut self.indices, 0, self.num_items - 1, self.node_size);

        // generate nodes at each tree level, bottom-up
        let mut i = 0;
        let mut pos = 0;
        while i < self.level_bounds.len() - 1 {
            let end = self.level_bounds[i];

            // generate a parent node for each block of consecutive <node_size> nodes
            while pos < end {
                let node_index = pos;

                // calculate bbox for the new node
                let mut node_min_x = boxes[pos];
                pos += 1;
                let mut node_min_y = boxes[pos];
                pos += 1;
                let mut node_max_x = boxes[pos];
                pos += 1;
                let mut node_max_y = boxes[pos];
                pos += 1;
                let mut j = 1;
                while j < self.node_size && pos < end {
                    node_min_x = fmin(node_min_x, boxes[pos]);
                    pos += 1;
                    node_min_y = fmin(node_min_y, boxes[pos]);
                    pos += 1;
                    node_max_x = fmax(node_max_x, boxes[pos]);
                    pos += 1;
                    node_max_y = fmax(node_max_y, boxes[pos]);
                    pos += 1;
                    j += 1;
                }

                // add the new node to the tree data
                self.indices[self.pos >> 2] = node_index;
                boxes[self.pos] = node_min_x;
                self.pos += 1;
                boxes[self.pos] = node_min_y;
                self.pos += 1;
                boxes[self.pos] = node_max_x;
                self.pos += 1;
                boxes[self.pos] = node_max_y;
                self.pos += 1;
            }

            i += 1;
        }
    }

    /// Search the index by a bounding box.
    ///
    /// ## Parameters
    /// - `min_x`: The minimum x coordinate of the query point.
    /// - `min_y`: The minimum y coordinate of the query point.
    /// - `max_x`: The maximum x coordinate of the query point.
    /// - `max_y`: The maximum y coordinate of the query point.
    /// - `[filter_fn]`: An optional function that is called on every found item; if supplied, only items
    ///   for which this function returns true will be included in the results array.
    ///
    /// ## Returns
    /// An array of indices of items intersecting or touching the given bounding box.
    pub fn search(&self, bbox: &BBox, filter_fn: Option<impl Fn(&T) -> bool>) -> Vec<T> {
        let BBox { left: min_x, bottom: min_y, right: max_x, top: max_y } = bbox;
        let mut results = vec![];
        if self.items.is_empty() {
            return results;
        }
        let mut node_index = Some(self.boxes.len() - 4);
        let mut queue = vec![];

        while let Some(n_index) = node_index {
            // find the end index of the node
            let end =
                usize::min(n_index + self.node_size * 4, upper_bound(n_index, &self.level_bounds));

            // search through child nodes
            for pos in (n_index..end).step_by(4) {
                // check if node bbox intersects with query bbox
                let x0 = self.boxes[pos];
                if *max_x < x0 {
                    continue;
                }
                let y0 = self.boxes[pos + 1];
                if *max_y < y0 {
                    continue;
                }
                let x1 = self.boxes[pos + 2];
                if *min_x > x1 {
                    continue;
                }
                let y1 = self.boxes[pos + 3];
                if *min_y > y1 {
                    continue;
                }

                let index = self.indices[pos >> 2];

                if n_index >= self.num_items * 4 {
                    queue.push(index); // node; add it to the search queue
                } else if let Some(item) = self.items.get(index)
                    && filter_fn.as_ref().is_none_or(|f| f(item))
                {
                    results.push(item.clone());
                }
            }

            node_index = queue.pop();
        }

        results
    }

    /// Search items in order of distance from the given point.
    ///
    /// ## Parameters
    /// - `x`: The x coordinate of the query point.
    /// - `y`: The y coordinate of the query point.
    /// - `[max_results]`: The maximum number of results to return.
    /// - `[max_distance]`: The maximum distance to search.
    /// - `[filter_fn]`: An optional function for filtering the results.
    ///
    /// ## Returns
    /// An array of indices of items found.
    pub fn neighbors<P: GetXY>(
        &self,
        p: P,
        max_results: Option<usize>,
        max_distance: Option<f64>,
        filter_fn: Option<impl Fn(&T) -> bool>,
    ) -> Vec<T> {
        let mut results = vec![];
        if self.items.is_empty() {
            return results;
        }
        let x = p.x();
        let y = p.y();
        let max_results = max_results.unwrap_or(usize::MAX);
        let max_distance = max_distance.unwrap_or(f64::MAX);
        let mut q = FlatQueue::new();
        let max_dist_squared = max_distance * max_distance;
        let mut node_index = Some(self.boxes.len() - 4);

        'outer: while let Some(n_index) = node_index {
            // find the end index of the node
            let end =
                usize::min(n_index + self.node_size * 4, upper_bound(n_index, &self.level_bounds));

            // add child nodes to the queue
            for pos in (n_index..end).step_by(4) {
                let index = self.indices[pos >> 2];
                let min_x = self.boxes[pos];
                let min_y = self.boxes[pos + 1];
                let max_x = self.boxes[pos + 2];
                let max_y = self.boxes[pos + 3];
                let dx = if x < min_x {
                    min_x - x
                } else if x > max_x {
                    x - max_x
                } else {
                    0.
                };
                let dy = if y < min_y {
                    min_y - y
                } else if y > max_y {
                    y - max_y
                } else {
                    0.
                };
                let dist = dx * dx + dy * dy;
                if dist > max_dist_squared {
                    continue;
                }

                if n_index >= self.num_items * 4 {
                    q.push(index << 1, dist); // node (use even id)
                } else {
                    q.push((index << 1) + 1, dist); // leaf item (use odd id)
                }
            }

            // pop items from the queue
            while !q.is_empty() && (q.peek().unwrap_or(&0) & 1) != 0 {
                let dist = q.peek_value();
                if dist.is_none() || dist.unwrap() > max_dist_squared {
                    break 'outer;
                }
                if let Some(item) = self.items.get(q.pop().unwrap_or(0) >> 1)
                    && filter_fn.as_ref().is_none_or(|f| f(item))
                {
                    results.push(item.clone());
                }
                if results.len() == max_results {
                    break 'outer;
                }
            }

            node_index = if !q.is_empty() { Some(q.pop().unwrap_or(0) >> 1) } else { None };
        }

        results
    }

    /// Get the items that have been added to the index. If taken, this index will be empty.
    pub fn take(&mut self) -> Vec<T> {
        core::mem::take(&mut self.items)
    }
}

/// Binary search for the first value in the array bigger than the given.
///
/// ## Parameters
/// - `value`: the value to search for
/// - `arr`: the array to search
///
/// ## Returns
/// The first value in the array bigger than the given
fn upper_bound(value: usize, arr: &[usize]) -> usize {
    let mut i = 0;
    let mut j = arr.len() - 1;
    while i < j {
        let m = (i + j) >> 1;
        if arr[m] > value {
            j = m;
        } else {
            i = m + 1;
        }
    }

    arr[i]
}

/// Custom quicksort that partially sorts bbox data alongside the hilbert values.
///
/// ## Parameters
/// - `values`: the hilbert values
/// - `boxes`: the boxes
/// - `indices`: the indices
/// - `left`: the left index
/// - `right`: the right index
/// - `node_size`: the node size
fn sort(
    values: &mut [u64],
    boxes: &mut [f64],
    indices: &mut [usize],
    left: usize,
    right: usize,
    node_size: usize,
) {
    if left.div(node_size) >= right.div(node_size) {
        return;
    }

    // apply median of three method
    let start = values[left];
    let mid = values[(left + right) >> 1];
    let end = values[right];

    let mut pivot = end;

    let x = max(start, mid);
    if end > x {
        pivot = x;
    } else if x == start {
        pivot = max(mid, end);
    } else if x == mid {
        pivot = max(start, end);
    }

    let i = left as isize - 1;
    let mut j = right + 1;

    loop {
        let mut i = (i + 1) as usize;
        while values[i] < pivot {
            i += 1;
        }
        j -= 1;
        while values[j] > pivot {
            j -= 1;
        }
        if i >= j {
            break;
        }
        swap(values, boxes, indices, i, j);
    }

    sort(values, boxes, indices, left, j, node_size);
    sort(values, boxes, indices, j + 1, right, node_size);
}

/// Swap two values and two corresponding boxes.
///
/// ## Parameters
/// - `values`: the hilbert values
/// - `boxes`: the boxes
/// - `indices`: the indices
/// - `i`: index
/// - `j`: index
fn swap(values: &mut [u64], boxes: &mut [f64], indices: &mut [usize], i: usize, j: usize) {
    values.swap(i, j);

    let k = 4 * i;
    let m = 4 * j;

    boxes.swap(k, m);
    boxes.swap(k + 1, m + 1);
    boxes.swap(k + 2, m + 2);
    boxes.swap(k + 3, m + 3);

    indices.swap(i, j);
}

/// Fast Hilbert curve algorithm by http://threadlocalmutex.com/
/// Ported from C++ https://github.com/rawrunprotected/hilbert_curves (public domain)
///
/// ## Parameters
/// - `x`: x coordinate
/// - `y`: y coordinate
///
/// ## Returns
/// The Hilbert value (32-bit integer)
fn _hilbert(x: u32, y: u32) -> u32 {
    let mut a = x ^ y;
    let mut b = 0xffff ^ a;
    let mut c = 0xffff ^ (x | y);
    let mut d = x & (y ^ 0xffff);

    let mut _a = a | (b >> 1);
    let mut _b = (a >> 1) ^ a;
    let mut _c = (c >> 1) ^ (b & (d >> 1)) ^ c;
    let mut _d = (a & (c >> 1)) ^ (d >> 1) ^ d;

    a = _a;
    b = _b;
    c = _c;
    d = _d;
    _a = (a & (a >> 2)) ^ (b & (b >> 2));
    _b = (a & (b >> 2)) ^ (b & ((a ^ b) >> 2));
    _c ^= (a & (c >> 2)) ^ (b & (d >> 2));
    _d ^= (b & (c >> 2)) ^ ((a ^ b) & (d >> 2));

    a = _a;
    b = _b;
    c = _c;
    d = _d;
    _a = (a & (a >> 4)) ^ (b & (b >> 4));
    _b = (a & (b >> 4)) ^ (b & ((a ^ b) >> 4));
    _c ^= (a & (c >> 4)) ^ (b & (d >> 4));
    _d ^= (b & (c >> 4)) ^ ((a ^ b) & (d >> 4));

    a = _a;
    b = _b;
    c = _c;
    d = _d;
    _c ^= (a & (c >> 8)) ^ (b & (d >> 8));
    _d ^= (b & (c >> 8)) ^ ((a ^ b) & (d >> 8));

    a = _c ^ (_c >> 1);
    b = _d ^ (_d >> 1);

    let mut i0 = x ^ y;
    let mut i1 = b | (0xffff ^ (i0 | a));

    i0 = (i0 | (i0 << 8)) & 0x00ff00ff;
    i0 = (i0 | (i0 << 4)) & 0x0f0f0f0f;
    i0 = (i0 | (i0 << 2)) & 0x33333333;
    i0 = (i0 | (i0 << 1)) & 0x55555555;

    i1 = (i1 | (i1 << 8)) & 0x00ff00ff;
    i1 = (i1 | (i1 << 4)) & 0x0f0f0f0f;
    i1 = (i1 | (i1 << 2)) & 0x33333333;
    i1 = (i1 | (i1 << 1)) & 0x55555555;

    (i1 << 1) | i0
}
