// T -> O(n^2)
// S -> O(1)
fn bubble_sort<T>(v: &mut [T]) -> &[T]
where
    T: PartialOrd,
{
    for i in 0..v.len() - 1 {
        let mut sorted = false;
        for j in 0..v.len() - i - 1 {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
                sorted = false;
            }
        }
        if sorted == true {
            return v;
        }
    }

    v
}

#[test]
fn it_bubble_sort() {
    let mut v = vec![5, 4, 3, 2, 1];
    bubble_sort(&mut v);
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
}
