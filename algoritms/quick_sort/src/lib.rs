fn partition(left: usize, right: usize, xs: &mut Vec<usize>) -> usize {
    let pivot = xs[right];
    let mut idx: isize = -1;

    for i in left..right {
        if xs[i] <= pivot {
            idx += 1;
            xs.swap(i, idx as usize);
        }
    }

    idx += 1;
    xs.swap(idx as usize, right);

    idx as usize
}
fn qs(left: usize, right: usize, xs: &mut Vec<usize>) {
    if left >= right {
        return;
    }

    let pivot_point = partition(left, right, xs);

    if pivot_point > 0 {
        qs(left, pivot_point - 1, xs);
    }
    qs(pivot_point + 1, right, xs);
}
fn quick_sort(xs: &mut Vec<usize>) {
    qs(0, xs.len() - 1, xs);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_partition_correct() {
        let mut xs: Vec<usize> = vec![3, 2, 5, 0, 1, 8, 7, 6, 9, 4];
        partition(0, xs.len() - 1, &mut xs);
        assert_eq!(xs, vec![3, 2, 0, 1, 4, 8, 7, 6, 9, 5]);
    }

    #[test]
    fn it_quick_sorts() {
        let mut xs: Vec<usize> = vec![3, 2, 5, 0, 1, 8, 7, 6, 9, 4];
        quick_sort(&mut xs);
        assert_eq!(xs, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
}
