use std::cmp::min;

#[derive(Debug)]
struct PriorityQueue<T> where T: PartialOrd + Clone {
    pq: Vec<T>,
}

impl <T> PriorityQueue<T> where T: PartialOrd + Clone {
    fn new() -> PriorityQueue<T> {
        PriorityQueue { pq: Vec::new() }
    }

    fn len(&self) -> usize {
        self.pq.len()
    }

    fn is_empty(&self) -> bool {
        self.pq.len() == 0
    }

    fn insert(&mut self, value: T) {
        self.pq.push(value);
    }

    fn max(&self) -> Option<T> {
        if self.is_empty() { return None }
        let max = self.max_index();
        Some(self.pq[max].clone())
    }

    fn min(&self) -> Option<T> {
        if self.is_empty() { return None }
        let min = self.min_index();
        Some(self.pq[min].clone())
    }

    fn delete_max(&mut self) -> Option<T> {
        if self.is_empty() { return None }
        let max = self.max_index();
        Some(self.pq.remove(max).clone())
    }

    fn delete_min(&mut self) -> Option<T> {
        if self.is_empty() { return None }
        let min = self.min_index();
        Some(self.pq.remove(min).clone())
    }

    fn max_index(&self) -> usize {
        let mut max = 0;
        for i in 1..self.len() {
            if self.pq[max] < self.pq[i] {
                max = i;
            }
        }
        max
    }

    fn min_index(&self) -> usize {
        let mut min = 0;
        for i in 0..self.pq.len() - 1 {
            if self.pq[i] < self.pq[i + 1] {
                min = i;
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keep_min() {
        let mut pq = PriorityQueue::new();
        pq.insert(3);
        pq.insert(2);
        pq.insert(1);
        pq.insert(4);
        assert_eq!(pq.min().unwrap(), 1);
    }

    #[test]
    fn test_keep_max() {
        let mut pq = PriorityQueue::new();
        pq.insert(3);
        pq.insert(2);
        pq.insert(1);
        pq.insert(4);
        assert_eq!(pq.max().unwrap(), 4);
    }

    #[test]
    fn test_is_empty() {
        let mut pq = PriorityQueue::new();
        assert!(pq.is_empty());
        pq.insert(1);
        assert!(!pq.is_empty());
    }

    #[test]
    fn test_len() {
        let mut pq = PriorityQueue::new();
        assert_eq!(pq.len(), 0);
        pq.insert(2);
        pq.insert(3);
        pq.insert(4);
        assert_eq!(pq.len(), 3);
    }

    #[test]
    fn test_delete_min() {
        let mut pq = PriorityQueue::new();
        pq.insert(3);
        pq.insert(4);
        assert_eq!(pq.len(), 2);
        assert_eq!(pq.delete_min().unwrap(), 3);
        assert_eq!(pq.len(), 1);
    }

    #[test]
    fn test_delete_max() {
        let mut pq = PriorityQueue::new();
        pq.insert(1);
        pq.insert(3);
        pq.insert(5);
        assert_eq!(pq.len(), 3);
        assert_eq!(pq.delete_max().unwrap(), 5);
        assert_eq!(pq.len(), 2);
    }

}
