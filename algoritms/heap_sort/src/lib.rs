type Comparator<T> = dyn Fn(Option<&T>, Option<&T>) -> bool;

fn build_heap<T>(items: &mut Vec<T>, comparator: &Box<Comparator<T>>) {
    // comparator = |x1, x2| x1 >= x2  : MAXHEAP
    // comparator = |x1, x2| x1 <= x2  : MINHEAP
    let start = items.len() / 2 - 1;
    for idx in (0..=start).rev() {
        heapify_down(items, idx, items.len(), &comparator);
    }
}

fn heapify_down<T>(lookup: &mut Vec<T>, idx: usize, bound: usize, comparator: &Box<Comparator<T>>) {
    let left_idx = idx * 2 + 1;
    let right_idx = idx * 2 + 2;
    let mut swap_with = idx;

    if left_idx < bound && comparator(lookup.get(left_idx), lookup.get(idx)) {
        swap_with = left_idx;
    }

    if right_idx < bound && comparator(lookup.get(right_idx), lookup.get(swap_with)) {
        swap_with = right_idx;
    }

    if swap_with != idx {
        lookup.swap(idx, swap_with);
        heapify_down(lookup, swap_with, bound, comparator);
    }
}

fn sort<T>(mut items: Vec<T>, comparator: Box<Comparator<T>>) -> Vec<T> {
    // Building heap from the items
    build_heap(&mut items, &comparator);

    // Sorting
    let mut bound = items.len() - 1;
    let first_idx = 0;
    while bound > 0 {
        items.swap(first_idx, bound);
        heapify_down(&mut items, first_idx, bound, &comparator);
        bound -= 1;
    }

    items
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn it_should_weak_sort() {
        let comparator: Box<Comparator<usize>> = Box::new(|x1, x2| x1 >= x2);
        let mut arr = vec![12, 9, 27, 43, 2, 53, 17];
        build_heap(&mut arr, &comparator);
        assert_eq!(arr, vec![53, 43, 27, 9, 2, 12, 17]);
    }

    #[test]
    fn it_should_sort_small_array_correcrly() {
        let comparator: Box<Comparator<usize>> = Box::new(|x1, x2| x1 >= x2);
        let mut arr = vec![12, 9, 27, 43, 2, 53, 17];
        let sorted = sort(arr, comparator);
        assert_eq!(sorted, vec![2, 9, 12, 17, 27, 43, 53]);
    }

    #[test]
    fn it_should_sort_big_array_correctly() {
        let comparator: Box<Comparator<usize>> = Box::new(|x1, x2| x1 >= x2);
        const LENGTH: usize = 1000;
        let mut arr = Vec::with_capacity(LENGTH);
        let mut rng = rand::thread_rng();
        for _ in 0..LENGTH {
            arr.push(rng.gen_range(0..1000));
        }
        let sorted = sort(arr, comparator);
        assert_eq!(sorted.windows(2).all(|pair| pair[0] <= pair[1]), true);
    }
}
