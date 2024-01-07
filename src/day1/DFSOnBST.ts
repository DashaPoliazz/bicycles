import { BinaryNode } from "global";

export default function dfs(head: BinaryNode<number>, needle: number): boolean {
    if (!head) return false;
    if (head.value === needle) return true;

    if (head.left && head.value > needle) return dfs(head.left, needle);
    if (head.right && head.value < needle) return dfs(head.right, needle);

    return false;
}
