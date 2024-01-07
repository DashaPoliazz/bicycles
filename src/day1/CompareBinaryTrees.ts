import { BinaryNode } from "global";

export default function compare(
    a: BinaryNode<number> | null,
    b: BinaryNode<number> | null,
): boolean {
    if (!a && !b) return true;
    if (!a && b) return false;
    if (a && !b) return false;

    return (
        compare(a && a.left, b && b.left) && compare(a && a.right, b && b.right)
    );
}
