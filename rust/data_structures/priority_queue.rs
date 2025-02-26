use alloc::vec::Vec;

/// Comparison function type for priority queue elements.
pub type PriorityCompare<T> = fn(&T, &T) -> core::cmp::Ordering;

/// # Priority Queue
///
/// ## Description
/// A priority queue is a data structure that stores elements in a specific order.
///
/// ## Usage
/// ```rust
/// use gistools::data_structures::PriorityQueue;
///
/// let mut queue = PriorityQueue::new(|a: &i32, b: &i32| a.cmp(b));
///
/// queue.push(3);
/// queue.push(1);
/// queue.push(2);
///
/// assert_eq!(queue.peek(), Some(&1));
/// assert_eq!(queue.pop(), Some(1));
/// assert_eq!(queue.len(), 2);
/// ```
pub struct PriorityQueue<T> {
    data: Vec<T>,
    compare: PriorityCompare<T>,
}

impl<T> PriorityQueue<T>
where
    T: Ord + Clone,
{
    /// Creates a new priority queue with an optional comparison function.
    pub fn new(compare: PriorityCompare<T>) -> Self {
        Self { data: Vec::new(), compare }
    }

    /// Returns the number of elements in the queue.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Pushes an item into the queue.
    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.up(self.data.len() - 1);
    }

    /// Removes and returns the highest-priority item.
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let top = self.data.swap_remove(0);
        if !self.data.is_empty() {
            self.down(0);
        }
        Some(top)
    }

    /// Peeks at the highest-priority item without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    fn up(&mut self, mut pos: usize) {
        let item = self.data[pos].clone();
        while pos > 0 {
            let parent = (pos - 1) / 2;
            if (self.compare)(&self.data[parent], &item) != core::cmp::Ordering::Greater {
                break;
            }
            self.data[pos] = self.data[parent].clone();
            pos = parent;
        }
        self.data[pos] = item;
    }

    fn down(&mut self, mut pos: usize) {
        let len = self.data.len();
        let item = self.data[pos].clone();
        let half_len = len / 2;
        while pos < half_len {
            let mut child = 2 * pos + 1;
            if child + 1 < len
                && (self.compare)(&self.data[child + 1], &self.data[child])
                    == core::cmp::Ordering::Less
            {
                child += 1;
            }
            if (self.compare)(&self.data[child], &item) != core::cmp::Ordering::Less {
                break;
            }
            self.data[pos] = self.data[child].clone();
            pos = child;
        }
        self.data[pos] = item;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_queue() {
        let mut queue = PriorityQueue::new(|a: &i32, b: &i32| a.cmp(b));
        assert!(queue.is_empty());
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.pop(), None);
        queue.push(3);
        queue.push(1);
        queue.push(2);
        assert!(!queue.is_empty());
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_priority_queue_large() {
        let mut queue = PriorityQueue::new(|a: &i32, b: &i32| a.cmp(b));

        // add 200 elements
        for i in 0..200 {
            queue.push(i);
        }

        assert_eq!(queue.pop(), Some(0));
    }

    #[test]
    fn test_priority_queue_large_inverse() {
        let mut queue = PriorityQueue::new(|a: &i32, b: &i32| a.cmp(b));

        // add 200 elements
        for i in 0..200 {
            queue.push(200 - i);
        }

        assert_eq!(queue.pop(), Some(1));
    }
}
