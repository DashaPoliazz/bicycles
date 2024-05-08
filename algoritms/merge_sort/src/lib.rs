fn merge<T>(left: &[T], right: &[T]) -> Vec<T>
where
    T: Ord + Clone,
{
    let mut sorted = Vec::with_capacity(left.len() + right.len());

    let mut left_idx = 0;
    let mut right_idx = 0;

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            sorted.push(left[left_idx].clone());
            left_idx += 1;
        } else {
            sorted.push(right[right_idx].clone());
            right_idx += 1;
        }
    }

    sorted.extend(left[left_idx..].iter().cloned());
    sorted.extend(right[right_idx..].iter().cloned());

    sorted
}

// T -> O(n*log n)
// S -> O(n*log n)
fn merge_sort<T>(xs: Vec<T>) -> Vec<T>
where
    T: Ord + Clone,
{
    if xs.len() <= 1 {
        return xs;
    }

    let middle = xs.len() / 2;
    let left = &xs[0..middle];
    let right = &xs[middle..];

    let merged_left = merge_sort(left.to_vec());
    let merged_right = merge_sort(right.to_vec());

    merge(&merged_left, &merged_right)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_merge_sort() {
        let v = vec![5, 4, 3, 2, 1];
        let result = merge_sort(v);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
}
