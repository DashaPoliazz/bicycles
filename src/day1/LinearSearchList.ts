export default function linear_search(
    haystack: number[],
    needle: number,
): boolean {
    // Linear search
    // T -> O(n)
    // S -> 0(1)
    for (let i = 0; i < haystack.length; i++) {
        if (haystack[i] === needle) {
            return true;
        }
    }

    return false;
}
