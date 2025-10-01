use alloc::vec::Vec;

/// # FlatQueue
///
/// ## Description
/// A priority queue implemented using a binary heap.
///
/// A port from the [flatqueue](https://github.com/mourner/flatqueue) code.
///
/// ## Usage
///
/// ```rust
/// use gistools::data_structures::FlatQueue;
///
/// let mut q = FlatQueue::new();
/// q.push(1, 1.0);
/// q.push(3, 3.0);
/// q.push(2, 2.0);
/// assert_eq!(q.pop(), Some(1));
/// assert_eq!(q.pop(), Some(2));
/// assert_eq!(q.pop(), Some(3));
/// assert_eq!(q.pop(), None);
/// ```
pub struct FlatQueue<T> {
    ids: Vec<T>,
    values: Vec<f64>, // priority
    len: usize,
}
impl<T: Clone> Default for FlatQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Clone> FlatQueue<T> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        Self { ids: Vec::new(), values: Vec::new(), len: 0 }
    }

    /// Returns the number of items.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Clears all items from the queue.
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Adds `item` to the queue with the specified `priority`.
    ///
    /// `priority` must be a number. Items are sorted and returned from low to high priority. Multiple items
    /// with the same priority value can be added to the queue, but there is no guaranteed order between these items.
    ///
    /// ## Parameters
    /// - `item`: the item to add
    /// - `priority`: the priority of the item
    pub fn push(&mut self, item: T, priority: f64) {
        let mut pos = self.len;
        self.len += 1;

        if self.ids.len() < self.len {
            self.ids.resize(self.len, item.clone());
            self.values.resize(self.len, priority);
        }

        while pos > 0 {
            let parent = (pos - 1) >> 1;
            let parent_value = self.values[parent];
            if priority >= parent_value {
                break;
            }
            self.ids[pos] = self.ids[parent].clone();
            self.values[pos] = parent_value;
            pos = parent;
        }

        self.ids[pos] = item;
        self.values[pos] = priority;
    }

    /// Removes and returns the item from the head of this queue, which is one of
    /// the items with the lowest priority. If this queue is empty, returns `undefined`.
    ///
    /// ## Returns
    /// the item from the head of this queue
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let top = self.ids[0].clone();
        self.len -= 1;

        if self.len > 0 {
            let id = self.ids[self.len].clone();
            let value = self.values[self.len];
            let mut pos = 0;
            let half_len = self.len >> 1;

            while pos < half_len {
                let left = (pos << 1) + 1;
                let right = left + 1;
                let mut child = left;
                if right < self.len && self.values[right] < self.values[left] {
                    child = right;
                }
                if self.values[child] >= value {
                    break;
                }
                self.ids[pos] = self.ids[child].clone();
                self.values[pos] = self.values[child];
                pos = child;
            }

            self.ids[pos] = id;
            self.values[pos] = value;
        }

        Some(top)
    }

    /// Returns the item with the lowest priority without removing it.
    pub fn peek(&self) -> Option<&T> {
        if self.len > 0 { Some(&self.ids[0]) } else { None }
    }

    /// Returns the priority of the lowest-priority item.
    pub fn peek_value(&self) -> Option<f64> {
        if self.len > 0 { Some(self.values[0]) } else { None }
    }

    /// Shrinks underlying arrays to current length.
    pub fn shrink(&mut self) {
        self.ids.truncate(self.len);
        self.values.truncate(self.len);
    }
}
