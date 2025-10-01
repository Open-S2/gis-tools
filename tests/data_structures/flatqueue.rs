#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::data_structures::FlatQueue;
    use rand::Rng;

    #[test]
    fn maintains_a_priority_queue() {
        let mut data: Vec<i32> = Vec::new();
        for _ in 0..100 {
            data.push(rand::rng().random_range(0..100));
        }

        let mut sorted = data.clone();
        sorted.sort();

        let mut queue = FlatQueue::new();
        for (i, &priority) in data.iter().enumerate() {
            queue.push(i, priority as f64);
        }

        assert_eq!(queue.peek_value(), Some(sorted[0] as f64));
        assert_eq!(data[queue.peek().copied().unwrap()], sorted[0]);

        let mut result: Vec<i32> = Vec::new();
        while queue.len() != 0 {
            result.push(data[queue.pop().unwrap()]);
        }

        assert_eq!(result, sorted);
    }

    #[test]
    fn handles_edge_cases_with_few_elements() {
        let mut queue = FlatQueue::new();

        queue.push(0, 2.0);
        queue.push(1, 1.0);
        queue.pop();
        queue.pop();
        queue.pop();
        queue.push(2, 2.0);
        queue.push(3, 1.0);

        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.peek_value(), None);
    }

    #[test]
    fn shrinks_internal_arrays_when_calling_shrink() {
        let mut queue = FlatQueue::new();

        for i in 0..10 {
            queue.push(i, i as f64);
        }

        while queue.len() != 0 {
            queue.pop();
        }

        assert_eq!(queue.len(), 0);

        queue.shrink();

        // internal ids/values are private, so just checking no panic + len = 0
    }
}
