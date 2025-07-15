#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::data_structures::PriorityQueue;

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
