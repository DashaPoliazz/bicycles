struct Heap<T, F>
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    lookup: Vec<T>,
    comparator: F,
}

impl<T, F> Heap<T, F>
where
    T: PartialOrd,
    F: Fn(&T, &T) -> bool,
{
    fn new(comparator: F) -> Self {
        Heap {
            lookup: vec![],
            comparator,
        }
    }

    pub fn push(&mut self, item: T) {
        self.lookup.push(item);
        let idx = self.lookup.len() - 1;
        self.heapify_up(idx);
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.lookup.is_empty() {
            return None;
        }

        let last_el_idx = self.lookup.len() - 1;
        self.lookup.swap(0, last_el_idx);
        let out = self.lookup.pop();
        self.heapify_down(0);

        out
    }
    fn heapify_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let child_idx = idx;
            let parent_idx = self.get_parent_idx(child_idx);

            if let (Some(child), Some(parent)) =
                (self.lookup.get(child_idx), self.lookup.get(parent_idx))
            {
                if (self.comparator)(child, parent) {
                    self.lookup.swap(child_idx, parent_idx);
                    idx = parent_idx;
                } else {
                    break;
                }
            }
        }
    }
    fn heapify_down(&mut self, idx: usize) {
        let left_child = self.get_left_child(idx);
        let right_child = self.get_right_child(idx);

        match (left_child, right_child) {
            (Some(left), Some(right)) => {
                // Defining the most relevant element between left_child and right_child
                if (self.comparator)(left, right) {
                    self.swap_with_left(idx);
                } else {
                    self.swap_with_right(idx);
                }
            }
            (Some(_), None) => {
                self.swap_with_left(idx);
            }
            (None, Some(_)) => {
                self.swap_with_right(idx);
            }
            (None, None) => (),
        }
    }

    fn swap_with_left(&mut self, parent_idx: usize) {
        let left_child_idx = self.get_left_child_idx(parent_idx);
        self.lookup.swap(left_child_idx, parent_idx);
        self.heapify_down(left_child_idx);
    }
    fn swap_with_right(&mut self, parent_idx: usize) {
        let right_child_idx = self.get_right_child_idx(parent_idx);
        self.lookup.swap(right_child_idx, parent_idx);
        self.heapify_down(right_child_idx);
    }
    fn get_left_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }
    fn get_left_child(&self, idx: usize) -> Option<&T> {
        self.lookup.get(self.get_left_child_idx(idx))
    }
    fn get_right_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 2
    }
    fn get_right_child(&self, idx: usize) -> Option<&T> {
        self.lookup.get(self.get_right_child_idx(idx))
    }
    fn get_parent_idx(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }
    fn get_parent(&self, idx: usize) -> Option<&T> {
        self.lookup.get(self.get_left_child_idx(idx))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    fn validate_heap<T, F>(heap: &Heap<T, F>) -> bool
    where
        T: PartialOrd + Debug,
        F: Fn(&T, &T) -> bool,
    {
        heap.lookup
            .iter()
            .enumerate()
            .skip(1)
            .all(|(child_idx, child)| {
                let parent_idx = heap.get_parent_idx(child_idx);
                if let Some(parent) = heap.lookup.get(parent_idx) {
                    (heap.comparator)(parent, child)
                } else {
                    true
                }
            })
    }

    #[test]
    fn it_should_heapify_elements_up() {
        let max_heap_comparator: fn(&usize, &usize) -> bool = |a, b| a >= b;
        let mut h = Heap::new(max_heap_comparator);
        for n in 0..1000 {
            h.push(n);
        }
        assert_eq!(validate_heap(&h), true);
    }

    #[test]
    fn it_should_heapify_elements_down() {
        let max_heap_comparator: fn(&usize, &usize) -> bool = |a, b| a >= b;
        let mut h = Heap::new(max_heap_comparator);
        for n in 0..10 {
            h.push(n);
        }
        for n in 0..11 {
            h.pop();
        }
        assert_eq!(validate_heap(&h), true);
    }

    #[test]
    fn it_should_pop_elements() {
        let max_heap_comparator: fn(&usize, &usize) -> bool = |a, b| a >= b;
        let mut h = Heap::new(max_heap_comparator);
        for n in 0..10 {
            h.push(n);
        }
        let head = h.pop().unwrap();
        assert_eq!(head, 9);
    }
}
