export default function two_crystal_balls(breaks: boolean[]): number {
    const n = breaks.length;
    const distance = Math.floor(Math.sqrt(n));
    let i = distance;

    for (; i < n; i += distance) {
        if (breaks[i]) {
            break;
        }
    }

    i -= distance;

    for (let j = 0; j <= distance && i < n; i++, j++) {
        if (breaks[i]) {
            return i;
        }
    }

    return -1;
}
