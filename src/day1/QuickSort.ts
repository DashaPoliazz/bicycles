function qs(arr: number[], lo: number, hi: number): void {
    if (lo >= hi) {
        return;
    }

    const pivotIdx = partition(arr, lo, hi);

    qs(arr, lo, pivotIdx - 1);
    qs(arr, pivotIdx + 1, hi);
}
// Returns the pivot point
function partition(arr: number[], lo: number, hi: number): number {
    // insertion index
    let idx = lo - 1;

    const pivot = arr[hi];

    // hi not included because in the end of sort
    // it moves to the pivot
    for (let i = lo; i < hi; i++) {
        if (arr[i] <= pivot) {
            idx++;
            // swap
            const tmp = arr[i];
            arr[i] = arr[idx];
            arr[idx] = tmp;
        }
    }

    // Moving pivot to it's position
    idx++;
    arr[hi] = arr[idx];
    arr[idx] = pivot;

    return idx;
}

export default function quick_sort(arr: number[]): void {
    qs(arr, 0, arr.length - 1);
}

quick_sort([3, 2, 5, 0, 1, 8, 7, 6, 9, 4]);
