function merge(left, right) {
    const result = [];

    while (left.length && right.length) {
        if (left[0] <= right[0]) {
            result.push(left.shift());
        } else {
            result.push(right.shift());
        }
    }

    return result.concat(left, right);
}

function mergeSort(list) {
    const len = list.length;

    if (len < 2) {
        return list;
    }

    const middle = Math.floor(len / 2);
    const left = list.slice(0, middle);
    const right = list.slice(middle);

    const leftPart = mergeSort(left);
    const rightPart = mergeSort(right);

    return merge(leftPart, rightPart);
}

const data = [5, 2, 7, 6, 8, 8, 55, 14];
const sortedData = mergeSort(data);
console.log(sortedData);
